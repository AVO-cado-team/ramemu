//! The `parser` module is responsible for parsing the source code of the
//! assembly language into a statements. It provides methods for parsing
//! individual lines of source code as well as entire programs.
//!

use crate::errors::ParseError;
use crate::errors::ParseErrorKind;
use crate::program::CodeAddress;
use crate::program::LabelId;
use crate::program::Program;
use crate::stmt::Op;
use rustc_hash::FxHashMap as HashMap;

use crate::stmt::Op::{
    Add, Div, Halt, Input, Jump, JumpGreatherZero, JumpIfZero, Load, Mult, Output, Store, Sub,
};
use crate::stmt::RegisterValue;
use crate::stmt::Stmt;
use crate::stmt::Value;

/// Parses the source code and returns an iterator over [`Result<Stmt, ParseError>`].
///
/// This function processes each line of the source code, parsing it into a [`Stmt`] or
/// a [`ParseError`] if an error occurs. It skips empty lines and comments.
/// # Errors
/// Returns all errors that occurred while parsing the source code.
pub fn parse(source: &str) -> Result<Program, Vec<ParseError>> {
    let mut label_to_address: HashMap<LabelId, CodeAddress> = HashMap::default();
    let mut errors = Vec::new();
    let mut instructions = Vec::new();

    let mut label_ids: HashMap<String, LabelId> = HashMap::default();
    let lines = source.lines().enumerate().map(|(i, l)| (i + 1, l));

    for (line, source) in lines {
        let (op, label) = match parse_line(source, &mut label_ids) {
            Ok(ParsedLine { op, label }) => (op, label),
            Err(kind) => {
                errors.push(ParseError { kind, line });
                continue;
            }
        };

        let labels_code_address = CodeAddress(instructions.len());

        if let Some(op) = op {
            instructions.push(Stmt { op, line });
        }

        if let Some(label) = label {
            label_to_address.insert(label, labels_code_address);
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(Program {
        instructions,
        labels: label_to_address,
    })
}

/// Represents a parsed line of source code.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ParsedLine {
    /// Op code in the line
    pub op: Option<Op>,
    /// Label code in the line
    pub label: Option<LabelId>,
}

impl ParsedLine {
    /// Returns a new [`ParsedLine`] with the given op code and label code.
    fn new(op: Option<Op>, label: Option<LabelId>) -> Self {
        Self { op, label }
    }
}

/// Parses a single line of source code and returns a [`Result`] containing an [`Option<Stmt>`]
/// or a [`ParseError`].
///
/// This function processes a single line of source code, returning `None` for empty lines
/// or lines containing only comments. If the line contains an instruction or label, it returns
/// a [`Stmt`] wrapped in a `Some`. In case of a parsing error, it returns a [`ParseError`]
/// # Errors
/// Returns a [`ParseError`] if the line contains an invalid instruction or label.
#[allow(clippy::implicit_hasher)]
pub fn parse_line(
    source: &str,
    label_ids: &mut HashMap<String, LabelId>,
) -> Result<ParsedLine, ParseErrorKind> {
    let source = source.trim();

    let (source, label_id) = match parse_label(source) {
        (Ok(Some(label)), source) => {
            let len = label_ids.len();
            let id = *label_ids.entry(label.to_string()).or_insert(LabelId(len));
            (source, Some(id))
        }
        (Ok(None), source) => (source, None),
        (Err(_), _) => return Err(ParseErrorKind::LabelIsNotValid),
    };

    let mut facts = source.split('#').next().unwrap_or("").split_whitespace();

    let facts = (facts.next(), facts.next(), facts.next());

    let (head, tail) = match facts {
        (None, _, _) => return Ok(ParsedLine::new(None, label_id)),
        (Some(head), tail, None) => (head, tail),
        (_, _, Some(_)) => return Err(ParseErrorKind::UnsupportedSyntax),
    };

    let opcode = head.to_uppercase();

    let opcode = match opcode.as_str() {
        "LOAD" | "ADD" | "SUB" | "MULT" | "MUL" | "DIV" | "WRITE" | "OUTPUT" => {
            parse_with_value(&opcode, tail.ok_or(ParseErrorKind::ArgumentIsRequired)?)?
        }
        "JUMP" | "JMP" | "JZ" | "JZERO" | "JGZ" | "JGTZ" => parse_with_label_arg(
            &opcode,
            tail.ok_or(ParseErrorKind::ArgumentIsRequired)?,
            label_ids,
        )?,
        "STORE" | "INPUT" | "READ" => {
            parse_with_register(&opcode, tail.ok_or(ParseErrorKind::ArgumentIsRequired)?)?
        }
        "HALT" => Halt,
        _ => return Err(ParseErrorKind::UnsupportedOpcode(opcode)),
    };

    Ok(ParsedLine::new(Some(opcode), label_id))
}

fn parse_with_register(opcode: &str, tail: &str) -> Result<Op, ParseErrorKind> {
    let arg: RegisterValue = {
        if let Some(tail) = tail.strip_prefix('*') {
            RegisterValue::Indirect(
                tail.parse()
                    .map_err(|_| ParseErrorKind::argument_value_must_be_numeric())?,
            )
        } else if let Ok(arg) = tail.parse::<usize>() {
            RegisterValue::Direct(arg)
        } else if tail.starts_with('=') {
            return Err(ParseErrorKind::pure_argument_not_allowed());
        } else {
            return Err(ParseErrorKind::not_valid_argument());
        }
    };

    Ok(match opcode {
        "STORE" => Store(arg),
        "INPUT" | "READ" => Input(arg),
        _ => unreachable!("Opcodes were changed in parse function, but not there"),
    })
}

fn parse_with_value(head: &str, tail: &str) -> Result<Op, ParseErrorKind> {
    let arg: Value = {
        if let Some(tail) = tail.strip_prefix('=') {
            Value::Pure(
                tail.parse()
                    .map_err(|_| ParseErrorKind::argument_value_must_be_numeric())?,
            )
        } else if let Some(tail) = tail.strip_prefix('*') {
            Value::Register(RegisterValue::Indirect(
                tail.parse()
                    .map_err(|_| ParseErrorKind::argument_value_must_be_numeric())?,
            ))
        } else if let Ok(arg) = tail.parse::<usize>() {
            Value::Register(RegisterValue::Direct(arg))
        } else {
            return Err(ParseErrorKind::not_valid_argument());
        }
    };

    Ok(match head {
        "LOAD" => Load(arg),
        "OUTPUT" | "WRITE" => Output(arg),
        "ADD" => Add(arg),
        "SUB" => Sub(arg),
        "MUL" | "MULT" => Mult(arg),
        "DIV" => Div(arg),
        _ => unreachable!("Opcodes were changed in parse function, but not there"),
    })
}

fn parse_with_label_arg(
    head: &str,
    tail: &str,
    label_ids: &mut HashMap<String, LabelId>,
) -> Result<Op, ParseErrorKind> {
    let label: LabelId = if is_valid_label(tail) {
        let label = tail;
        let len = label_ids.len();
        *label_ids.entry(label.to_string()).or_insert(LabelId(len))
    } else {
        return Err(ParseErrorKind::LabelIsNotValid);
    };

    Ok(match head {
        "JUMP" | "JMP" => Jump(label),
        "JZ" | "JZERO" => JumpIfZero(label),
        "JGZ" | "JGTZ" => JumpGreatherZero(label),
        _ => unreachable!("Opcodes were changed in parse function, but not there"),
    })
}

fn parse_label(source: &str) -> (Result<Option<&str>, ParseErrorKind>, &str) {
    match source.split_once(':') {
        Some((label, tail)) if is_valid_label(label) => (Ok(Some(label)), tail),
        Some((_, tail)) => (Err(ParseErrorKind::LabelIsNotValid), tail),
        None => (Ok(None), source),
    }
}

/// Checks if the given string is a valid label.
///
/// A valid label must start with an ASCII alphabetic character or an underscore,
/// and can contain ASCII alphanumeric characters, underscores, or digits.
fn is_valid_label(label: &str) -> bool {
    let Some(first) = label.chars().next() else {
        return false;
    };

    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }

    label.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stmt::{RegisterValue, Value};

    #[test]
    fn test_parse_line_load() {
        let mut label_ids = HashMap::default();
        let line = "LOAD 1";
        let res = parse_line(line, &mut label_ids).unwrap();
        assert_eq!(
            res.op,
            Some(Load(Value::Register(RegisterValue::Direct(1))))
        );
        assert_eq!(res.label, None);
    }

    #[test]
    fn test_parse_line_add() {
        let mut label_ids = HashMap::default();
        let line = "ADD *2";
        let res = parse_line(line, &mut label_ids).unwrap();
        assert_eq!(
            res.op,
            Some(Add(Value::Register(RegisterValue::Indirect(2))))
        );
        assert_eq!(res.label, None);
    }

    #[test]
    fn test_parse_line_jump() {
        let mut label_ids = HashMap::default();
        let line = "JUMP start";
        let res = parse_line(line, &mut label_ids).unwrap();
        assert_eq!(res.op, Some(Jump(0.into())));
        assert_eq!(res.label, None);

        let line = "JUMP start";
        let res = parse_line(line, &mut label_ids).unwrap();
        assert_eq!(res.op, Some(Jump(0.into())));
        assert_eq!(res.label, None);
    }

    #[test]
    fn test_parse_line_invalid_label() {
        let mut label_ids = HashMap::default();
        let line = "JUMP 1start";
        let error = parse_line(line, &mut label_ids).unwrap_err();
        assert_eq!(error, ParseErrorKind::LabelIsNotValid);
    }

    #[test]
    fn test_parse_line_unsupported_opcode() {
        let mut label_ids = HashMap::default();
        let line = "NOP";
        let error = parse_line(line, &mut label_ids).unwrap_err();
        assert_eq!(error, ParseErrorKind::UnsupportedOpcode("NOP".to_string()));
    }

    #[test]
    fn test_parse_line_missing_argument() {
        let mut label_ids = HashMap::default();
        let line = "LOAD";
        let error = parse_line(line, &mut label_ids).unwrap_err();
        assert_eq!(error, ParseErrorKind::ArgumentIsRequired);
    }

    #[test]
    fn test_parse_comment() {
        let mut label_ids = HashMap::default();
        let line = "# This is a comment";
        let res = parse_line(line, &mut label_ids).unwrap();
        assert_eq!(res.op, None);
        assert_eq!(res.label, None);
    }

    #[test]
    fn test_parse_line_load_with_comment() {
        let mut label_ids = HashMap::default();
        let line = "Load 1 # This is a comment";
        let res = parse_line(line, &mut label_ids).unwrap();
        assert_eq!(
            res.op,
            Some(Load(Value::Register(RegisterValue::Direct(1))))
        );
        assert_eq!(res.label, None);
    }

    #[test]
    fn test_parse_with_only_label() {
        let mut label_ids = HashMap::default();
        let line = "start:";
        let res = parse_line(line, &mut label_ids).unwrap();
        let label_id = label_ids.get("start").unwrap();
        assert_eq!(res.op, None);
        assert_eq!(res.label, Some(*label_id));
    }

    #[test]
    fn test_parse_with_label_and_opcode() {
        let mut label_ids = HashMap::default();
        let line = "start: LOAD 1";
        let res = parse_line(line, &mut label_ids).unwrap();
        let label_id = label_ids.get("start").unwrap();
        assert_eq!(
            res.op,
            Some(Load(Value::Register(RegisterValue::Direct(1))))
        );
        assert_eq!(res.label, Some(*label_id));
    }
}
