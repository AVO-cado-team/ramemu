use crate::errors::ParseError;

use crate::stmt::Label;
use crate::stmt::RegisterValue;
use crate::stmt::Stmt;
use crate::stmt::Value;

pub fn parse(source: &str) -> impl Iterator<Item = Result<Stmt, ParseError>> + '_ {
  source
    .lines()
    .enumerate()
    .map(|(i, l)| (i, l.trim()))
    .map(|(i, l)| parse_line(l, i))
    .filter_map(|result| result.transpose())
}

pub fn parse_line(source: &str, line: usize) -> Result<Option<Stmt>, ParseError> {
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
      return Ok(Some(Stmt::Label(label.to_string(), line)));
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

fn parse_with_label(head: &str, tail: &str, line: usize) -> Result<Stmt, ParseError> {
  let label: Label = if is_valid_label(tail) {
    Label::new(tail.to_string())
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

fn is_valid_label(label: &str) -> bool {
  let Some(first) = label.chars().next() else { return false };

  if !first.is_ascii_alphabetic() && first != '_' {
    return false;
  }

  label
    .chars()
    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c.is_ascii_digit())
}
