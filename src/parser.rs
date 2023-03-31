use std::error::Error;

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

pub fn parse_line(line: &str, index: usize) -> Result<Option<Stmt>, ParseError> {
  let facts: Vec<_> = line
    .split('#')
    .next()
    .unwrap_or("")
    .split_whitespace()
    .collect();

  if facts.len() > 2 {
    Err(ParseError {})?
  }

  if facts.is_empty() {
    return Ok(None);
  }

  let head = facts[0].trim();
  let tail = facts.get(1);

  if let Some(label) = head.strip_suffix(':') {
    if is_valid_label(label) {
      return Ok(Some(Stmt::Label(label.to_string(), index)));
    }
    Err(ParseError {})?
  }

  let opcode = head.to_uppercase();

  let stmt = match opcode.as_str() {
    "LOAD" | "ADD" | "SUB" | "MUL" | "DIV" | "WRITE" | "OUTPUT" => {
      parse_with_value(&opcode, tail.ok_or(ParseError {})?, index)?
    }
    "JUMP" | "JMP" | "JZ" | "JZERO" | "JGZ" | "JGTZ" => {
      parse_with_label(&opcode, tail.ok_or(ParseError {})?, index)?
    }
    "STORE" | "INPUT" | "READ" => parse_with_register(&opcode, tail.ok_or(ParseError {})?, index)?,
    "HALT" => Stmt::Halt(index),
    _ => Err(ParseError {})?,
  };

  Ok(Some(stmt))
}

fn parse_with_register(head: &str, tail: &str, index: usize) -> Result<Stmt, ParseError> {
  let arg: RegisterValue = {
    if let Some(tail) = tail.strip_prefix('*') {
      RegisterValue::Indirect(tail.parse().map_err(|_| ParseError {})?)
    } else if let Ok(arg) = tail.parse::<usize>() {
      RegisterValue::Direct(arg)
    } else {
      Err(ParseError {})?
    }
  };
  match head {
    "STORE" => Ok(Stmt::Store(arg, index)),
    "INPUT" | "READ" => Ok(Stmt::Input(arg, index)),
    _ => Err(ParseError {})?,
  }
}

fn parse_with_value(head: &str, tail: &str, index: usize) -> Result<Stmt, ParseError> {
  let arg: Value = {
    if let Some(tail) = tail.strip_prefix('=') {
      Value::Pure(tail.parse().map_err(|_| ParseError {})?)
    } else if let Some(tail) = tail.strip_prefix('*') {
      Value::Register(RegisterValue::Indirect(
        tail.parse().map_err(|_| ParseError {})?,
      ))
    } else if let Ok(arg) = tail.parse::<usize>() {
      Value::Register(RegisterValue::Direct(arg))
    } else {
      Err(ParseError {})?
    }
  };

  match head {
    "LOAD" => Ok(Stmt::Load(arg, index)),
    "OUTPUT" | "WRITE" => Ok(Stmt::Output(arg, index)),
    "ADD" => Ok(Stmt::Add(arg, index)),
    "SUB" => Ok(Stmt::Sub(arg, index)),
    "MUL" => Ok(Stmt::Mul(arg, index)),
    "DIV" => Ok(Stmt::Div(arg, index)),
    _ => Err(ParseError {})?,
  }
}

fn parse_with_label(head: &str, tail: &str, index: usize) -> Result<Stmt, ParseError> {
  let label: Label = if is_valid_label(tail) {
    Label::new(tail.to_string())
  } else {
    Err(ParseError {})?
  };

  match head {
    "JUMP" | "JMP" => Ok(Stmt::Jump(label, index)),
    "JZ" | "JZERO" => Ok(Stmt::JumpIfZero(label, index)),
    "JGZ" | "JGTZ" => Ok(Stmt::JumpGreatherZero(label, index)),
    _ => Err(ParseError {})?,
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

#[derive(Debug)]
pub struct ParseError {}

impl std::fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Parse error")
  }
}

impl Error for ParseError {}
