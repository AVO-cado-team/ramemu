use crate::op::Op;
use crate::op::Value;

pub fn parse(source: &str) -> impl Iterator<Item = Result<Op, ParseError>> + '_ {
  source.lines().map(parse_line)
}

pub fn parse_line(line: &str) -> Result<Op, ParseError> {
  let facts: Vec<_> = line.split_whitespace().collect();
  let first = facts.first().ok_or(ParseError {})?.trim().to_uppercase();

  if let Some(label) = first.strip_suffix(':') {
    if is_valid_label(label) {
      return Ok(Op::Label(label.to_string()));
    }
    return Err(ParseError {});
  }

  let opcode = first;
  let arg = facts.get(1).ok_or(ParseError {})?;
  let arg: Value = {
    if let Some(tail) = arg.strip_prefix('=') {
      Value::Direct(tail.parse().map_err(|_| ParseError {})?)
    } else if let Some(tail) = arg.strip_prefix('*') {
      Value::DoubleIndirect(tail.parse().map_err(|_| ParseError {})?)
    } else if let Ok(arg) = arg.parse::<usize>() {
      Value::Indirect(arg)
    } else if is_valid_label(arg) {
      Value::Label(arg.to_string())
    } else {
      Err(ParseError {})?
    }
  };

  let op = match opcode.as_str() {
    "LOAD" => Op::Load(arg),
    "STORE" => Op::Store(arg),
    "ADD" => Op::Add(arg),
    "SUB" => Op::Sub(arg),
    "MUL" => Op::Mul(arg),
    "DIV" => Op::Div(arg),
    "JUMP" => Op::Jump(arg),
    "JUMPIFZERO" => Op::JumpIfZero(arg),
    "JUMPIFNEG" => Op::JumpIfNeg(arg),
    "INPUT" => Op::Input(arg),
    "OUTPUT" => Op::Output(arg),
    "HALT" => Op::Halt,
    _ => return Err(ParseError {}),
  };

  Ok(op)
}

pub struct ParseError {}

fn is_valid_label(label: &str) -> bool {
  let Some(first) = label.chars().next() else { return false };

  if !first.is_ascii_alphabetic() && first != '_' {
    return false;
  }

  label
    .chars()
    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c.is_ascii_digit())
}
