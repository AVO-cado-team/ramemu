//! The `parser` module is responsible for parsing the source code of the
//! assembly language into a statements. It provides methods for parsing
//! individual lines of source code as well as entire programs.
//!

use std::collections::HashMap;
use crate::errors::ParseError;

use crate::stmt::Label;
use crate::stmt::RegisterValue;
use crate::stmt::Stmt;
use crate::stmt::Value;

/// Parses the source code and returns an iterator over [`Result<Stmt, ParseError>`].
///
/// This function processes each line of the source code, parsing it into a [`Stmt`] or
/// a [`ParseError`] if an error occurs. It skips empty lines and comments.
pub fn parse<'a>(
  source: &'a str,
  label_ids: &'a mut HashMap<String, usize>,
) -> impl Iterator<Item = Result<Stmt, ParseError>> + 'a {
  source
    .lines()
    .enumerate()
    .map(|(i, l)| (i + 1, l.trim()))
    .map(move |(i, l)| parse_line(l, i, label_ids))
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
  label_ids: &mut HashMap<String, usize>,
) -> Result<Option<Stmt>, ParseError> {
  let facts: Vec<_> = source
    .split('#')
    .next()
    .unwrap_or("")
    .split_whitespace()
    .collect();

  if facts.len() > 2 {
    Err(ParseError::UnsupportedSyntax(line))?
  }

  if facts.is_empty() {
    return Ok(None);
  }

  let head = facts[0].trim();
  let tail = facts.get(1);

  if let Some(label) = head.strip_suffix(':') {
    if is_valid_label(label) {
      let len = label_ids.len();
      let id = *label_ids.entry(label.to_string()).or_insert(len);
      return Ok(Some(Stmt::Label(Label::from(id), line)));
    }
    Err(ParseError::LabelIsNotValid(line))?
  }

  let opcode = head.to_uppercase();

  let stmt = match opcode.as_str() {
    "LOAD" | "ADD" | "SUB" | "MUL" | "DIV" | "WRITE" | "OUTPUT" => parse_with_value(
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
    "HALT" => Stmt::Halt(line),
    _ => Err(ParseError::UnsupportedOpcode(line, opcode))?,
  };

  Ok(Some(stmt))
}

fn parse_with_register(opcode: &str, tail: &str, line: usize) -> Result<Stmt, ParseError> {
  let arg: RegisterValue = {
    if let Some(tail) = tail.strip_prefix('*') {
      RegisterValue::Indirect(
        tail
          .parse()
          .map_err(|_| ParseError::argument_value_must_be_numeric(line))?,
      )
    } else if let Ok(arg) = tail.parse::<usize>() {
      RegisterValue::Direct(arg)
    } else if tail.starts_with('=') {
      Err(ParseError::pure_argument_not_allowed(line))?
    } else {
      Err(ParseError::not_valid_argument(line))?
    }
  };
  match opcode {
    "STORE" => Ok(Stmt::Store(arg, line)),
    "INPUT" | "READ" => Ok(Stmt::Input(arg, line)),
    _ => unreachable!("Opcodes were chenged in parse function, but not there"),
  }
}

fn parse_with_value(head: &str, tail: &str, line: usize) -> Result<Stmt, ParseError> {
  let arg: Value = {
    if let Some(tail) = tail.strip_prefix('=') {
      Value::Pure(
        tail
          .parse()
          .map_err(|_| ParseError::argument_value_must_be_numeric(line))?,
      )
    } else if let Some(tail) = tail.strip_prefix('*') {
      Value::Register(RegisterValue::Indirect(
        tail
          .parse()
          .map_err(|_| ParseError::argument_value_must_be_numeric(line))?,
      ))
    } else if let Ok(arg) = tail.parse::<usize>() {
      Value::Register(RegisterValue::Direct(arg))
    } else {
      Err(ParseError::not_valid_argument(line))?
    }
  };

  match head {
    "LOAD" => Ok(Stmt::Load(arg, line)),
    "OUTPUT" | "WRITE" => Ok(Stmt::Output(arg, line)),
    "ADD" => Ok(Stmt::Add(arg, line)),
    "SUB" => Ok(Stmt::Sub(arg, line)),
    "MUL" => Ok(Stmt::Mul(arg, line)),
    "DIV" => Ok(Stmt::Div(arg, line)),
    _ => unreachable!("Opcodes were chenged in parse function, but not there"),
  }
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
    Label::from(*id)
  } else {
    Err(ParseError::LabelIsNotValid(line))?
  };

  match head {
    "JUMP" | "JMP" => Ok(Stmt::Jump(label, line)),
    "JZ" | "JZERO" => Ok(Stmt::JumpIfZero(label, line)),
    "JGZ" | "JGTZ" => Ok(Stmt::JumpGreatherZero(label, line)),
    _ => unreachable!("Opcodes were chenged in parse function, but not there"),
  }
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
