//! The `parser` module is responsible for parsing the source code of the
//! assembly language into a statements. It provides methods for parsing
//! individual lines of source code as well as entire programs.
//!

use crate::errors::ParseError;
use crate::program::LabelId;
use rustc_hash::FxHashMap as HashMap;

use crate::stmt::Label;
use crate::stmt::Op::*;
use crate::stmt::RegisterValue;
use crate::stmt::Stmt;
use crate::stmt::Value;

/// Parses the source code and returns an iterator over [`Result<Stmt, ParseError>`].
///
/// This function processes each line of the source code, parsing it into a [`Stmt`] or
/// a [`ParseError`] if an error occurs. It skips empty lines and comments.
pub fn parse(source: &str) -> impl Iterator<Item = Result<Stmt, ParseError>> + '_ {
    let mut label_ids: HashMap<String, LabelId> = HashMap::default();
    source
        .lines()
        .enumerate()
        .map(|(i, l)| (i + 1, l.trim()))
        .map(move |(i, l)| parse_line(l, i, &mut label_ids))
        .filter_map(|result| result.transpose())
}

/// Parses a single line of source code and returns a [`Result`] containing an [`Option<Stmt>`]
/// or a [`ParseError`].
///
/// This function processes a single line of source code, returning `None` for empty lines
/// or lines containing only comments. If the line contains an instruction or label, it returns
/// a [`Stmt`] wrapped in a `Some`. In case of a parsing error, it returns a [`ParseError`]
pub fn parse_line(
    source: &str,
    line: usize,
    label_ids: &mut HashMap<String, LabelId>,
) -> Result<Option<Stmt>, ParseError> {
    let facts: Vec<_> = source
        .split('#')
        .next()
        .unwrap_or("")
        .split_whitespace()
        .collect();

    if facts.len() > 2 {
        return Err(ParseError::UnsupportedSyntax(line));
    }

    if facts.is_empty() {
        return Ok(None);
    }

    let head = facts[0];
    let tail = facts.get(1);

    if let Some(label) = head.strip_suffix(':') {
        if is_valid_label(label) {
            let len = label_ids.len();
            let id = *label_ids.entry(label.to_string()).or_insert(len);
            return Ok(Some(Stmt::new(Label(id), line)));
        }
        return Err(ParseError::LabelIsNotValid(line));
    }

    let opcode = head.to_uppercase();

    let stmt = match opcode.as_str() {
        "LOAD" | "ADD" | "SUB" | "MULT" | "MUL" | "DIV" | "WRITE" | "OUTPUT" => parse_with_value(
            &opcode,
            tail.ok_or(ParseError::ArgumentIsRequired(line))?,
            line,
        )?,
        "JUMP" | "JMP" | "JZ" | "JZERO" | "JGZ" | "JGTZ" => parse_with_label(
            &opcode,
            tail.ok_or(ParseError::ArgumentIsRequired(line))?,
            line,
            label_ids,
        )?,
        "STORE" | "INPUT" | "READ" => parse_with_register(
            &opcode,
            tail.ok_or(ParseError::ArgumentIsRequired(line))?,
            line,
        )?,
        "HALT" => Stmt::new(Halt, line),
        _ => return Err(ParseError::UnsupportedOpcode(line, opcode)),
    };

    Ok(Some(stmt))
}

fn parse_with_register(opcode: &str, tail: &str, line: usize) -> Result<Stmt, ParseError> {
    let arg: RegisterValue = {
        if let Some(tail) = tail.strip_prefix('*') {
            RegisterValue::Indirect(
                tail.parse()
                    .map_err(|_| ParseError::argument_value_must_be_numeric(line))?,
            )
        } else if let Ok(arg) = tail.parse::<usize>() {
            RegisterValue::Direct(arg)
        } else if tail.starts_with('=') {
            return Err(ParseError::pure_argument_not_allowed(line));
        } else {
            return Err(ParseError::not_valid_argument(line));
        }
    };
    let op = match opcode {
        "STORE" => Store(arg),
        "INPUT" | "READ" => Input(arg),
        _ => unreachable!("Opcodes were changed in parse function, but not there"),
    };
    Ok(Stmt::new(op, line))
}

fn parse_with_value(head: &str, tail: &str, line: usize) -> Result<Stmt, ParseError> {
    let arg: Value = {
        if let Some(tail) = tail.strip_prefix('=') {
            Value::Pure(
                tail.parse()
                    .map_err(|_| ParseError::argument_value_must_be_numeric(line))?,
            )
        } else if let Some(tail) = tail.strip_prefix('*') {
            Value::Register(RegisterValue::Indirect(
                tail.parse()
                    .map_err(|_| ParseError::argument_value_must_be_numeric(line))?,
            ))
        } else if let Ok(arg) = tail.parse::<usize>() {
            Value::Register(RegisterValue::Direct(arg))
        } else {
            return Err(ParseError::not_valid_argument(line));
        }
    };

    let op = match head {
        "LOAD" => Load(arg),
        "OUTPUT" | "WRITE" => Output(arg),
        "ADD" => Add(arg),
        "SUB" => Sub(arg),
        "MUL" => Mult(arg),
        "MULT" => Mult(arg),
        "DIV" => Div(arg),
        _ => unreachable!("Opcodes were changed in parse function, but not there"),
    };
    Ok(Stmt::new(op, line))
}

fn parse_with_label(
    head: &str,
    tail: &str,
    line: usize,
    label_ids: &mut HashMap<String, usize>,
) -> Result<Stmt, ParseError> {
    let label: Label = if is_valid_label(tail) {
        let label = tail;
        let len = label_ids.len();
        let id = label_ids.entry(label.to_string()).or_insert(len);
        *id
    } else {
        return Err(ParseError::LabelIsNotValid(line));
    };

    let op = match head {
        "JUMP" | "JMP" => Jump(label),
        "JZ" | "JZERO" => JumpIfZero(label),
        "JGZ" | "JGTZ" => JumpGreatherZero(label),
        _ => unreachable!("Opcodes were changed in parse function, but not there"),
    };
    Ok(Stmt::new(op, line))
}

/// Checks if the given string is a valid label.
///
/// A valid label must start with an ASCII alphabetic character or an underscore,
/// and can contain ASCII alphanumeric characters, underscores, or digits.
fn is_valid_label(label: &str) -> bool {
    let Some(first) = label.chars().next() else { return false };

    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }

    label
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stmt::{RegisterValue, Stmt, Value};

    #[test]
    fn test_parse_line_load() {
        let mut label_ids = HashMap::default();
        let line = "LOAD 1";
        let stmt = parse_line(line, 1, &mut label_ids).unwrap();
        assert_eq!(
            stmt.unwrap(),
            Stmt::new(Load(Value::Register(RegisterValue::Direct(1))), 1)
        );
    }

    #[test]
    fn test_parse_line_add() {
        let mut label_ids = HashMap::default();
        let line = "ADD *2";
        let stmt = parse_line(line, 1, &mut label_ids).unwrap();
        assert_eq!(
            stmt.unwrap(),
            Stmt::new(Add(Value::Register(RegisterValue::Indirect(2))), 1)
        );
    }

    #[test]
    fn test_parse_line_jump() {
        let mut label_ids = HashMap::default();
        let line = "JUMP start";
        let stmt = parse_line(line, 1, &mut label_ids).unwrap();
        assert_eq!(stmt, Some(Stmt::new(Jump(0), 1)));

        let line = "JUMP start";
        let stmt = parse_line(line, 2, &mut label_ids).unwrap();
        assert_eq!(stmt, Some(Stmt::new(Jump(0), 2)));
    }

    #[test]
    fn test_parse_line_invalid_label() {
        let mut label_ids = HashMap::default();
        let line = "JUMP 1start";
        let error = parse_line(line, 1, &mut label_ids).unwrap_err();
        assert_eq!(error, ParseError::LabelIsNotValid(1));
    }

    #[test]
    fn test_parse_line_unsupported_opcode() {
        let mut label_ids = HashMap::default();
        let line = "NOP";
        let error = parse_line(line, 1, &mut label_ids).unwrap_err();
        assert_eq!(error, ParseError::UnsupportedOpcode(1, "NOP".to_string()));
    }

    #[test]
    fn test_parse_line_missing_argument() {
        let mut label_ids = HashMap::default();
        let line = "LOAD";
        let error = parse_line(line, 1, &mut label_ids).unwrap_err();
        assert_eq!(error, ParseError::ArgumentIsRequired(1));
    }

    #[test]
    fn test_parse_comment() {
        let mut label_ids = HashMap::default();
        let line = "# This is a comment";
        let stmt = parse_line(line, 1, &mut label_ids).unwrap();
        assert_eq!(stmt, None);
    }

    #[test]
    fn test_parse_line_load_with_comment() {
        let mut label_ids = HashMap::default();
        let line = "Load 1 # This is a comment";
        let stmt = parse_line(line, 1, &mut label_ids).unwrap();
        assert_eq!(
            stmt,
            Some(Stmt::new(
                Load(Value::Register(RegisterValue::Direct(1))),
                1
            ))
        );
    }
}
