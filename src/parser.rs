use std::error::Error;

use crate::op::Label;
use crate::op::Op;
use crate::op::RegisterValue;
use crate::op::Value;

pub fn parse(source: &str) -> impl Iterator<Item = Result<Op, ParseError>> + '_ {
  source
    .lines()
    .enumerate()
    .map(|(i, l)| (i, l.trim()))
    .filter(|(_, l)| !l.is_empty() && !l.starts_with('#'))
    .map(|(i, l)| parse_line(l, i))
}

pub fn parse_line(line: &str, index: usize) -> Result<Op, ParseError> {
  let facts: Vec<_> = line.split_whitespace().collect();

  if facts.len() > 2 && !facts[2].starts_with('#') {
    Err(ParseError {})?
  }

  let first = facts.first().ok_or(ParseError {})?.trim();
  let tail = facts.get(1);

  if let Some(label) = first.strip_suffix(':') {
    if is_valid_label(label) {
      return Ok(Op::Label(label.to_string(), index));
    }
    Err(ParseError {})?
  }

  let opcode = first.to_uppercase();

  let op = match opcode.as_str() {
    "LOAD" | "ADD" | "SUB" | "MUL" | "DIV" | "WRITE" | "OUTPUT" => {
      parse_with_value(&opcode, tail.ok_or(ParseError {})?, index)?
    }
    "JUMP" | "JMP" | "JZ" | "JZERO" | "JGZ" | "JGTZ" => {
      parse_with_label(&opcode, tail.ok_or(ParseError {})?, index)?
    }
    "STORE" | "INPUT" | "READ" => parse_with_register(&opcode, tail.ok_or(ParseError {})?, index)?,
    "HALT" => Op::Halt(index),
    _ => Err(ParseError {})?,
  };

  Ok(op)
}

fn parse_with_register(head: &str, tail: &str, index: usize) -> Result<Op, ParseError> {
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
    "STORE" => Ok(Op::Store(arg, index)),
    "INPUT" | "READ" => Ok(Op::Input(arg, index)),
    _ => Err(ParseError {})?,
  }
}

fn parse_with_value(head: &str, tail: &str, index: usize) -> Result<Op, ParseError> {
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
    "LOAD" => Ok(Op::Load(arg, index)),
    "OUTPUT" | "WRITE" => Ok(Op::Output(arg, index)),
    "ADD" => Ok(Op::Add(arg, index)),
    "SUB" => Ok(Op::Sub(arg, index)),
    "MUL" => Ok(Op::Mul(arg, index)),
    "DIV" => Ok(Op::Div(arg, index)),
    _ => Err(ParseError {})?,
  }
}

fn parse_with_label(head: &str, tail: &str, index: usize) -> Result<Op, ParseError> {
  let label: Label = if is_valid_label(tail) {
    Label::new(tail.to_string())
  } else {
    Err(ParseError {})?
  };

  match head {
    "JUMP" | "JMP" => Ok(Op::Jump(label, index)),
    "JZ" | "JZERO" => Ok(Op::JumpIfZero(label, index)),
    "JGZ" | "JGTZ" => Ok(Op::JumpGreatherZero(label, index)),
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
