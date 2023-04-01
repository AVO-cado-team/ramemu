use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum ParseError {
  LabelIsNotValid(usize),

  UnsupportedSyntax(usize),
  UnsupportedOpcode(usize, String),

  ArgumentIsRequired(usize),
  ArgumentIsNotValid(usize, InvalidArgument),

  UnknownError(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum InvalidArgument {
  LabelIsNotValid,
  ArgumentIsRequired,
  ArgumentValueMustBeNumberic,
  PureArgumentIsNotAllowed,

  ArgumentIsNotValid,
}

impl ParseError {
  #[inline]
  pub(crate) fn pure_argument_not_allowed(index: usize) -> Self {
    ParseError::ArgumentIsNotValid(index, InvalidArgument::PureArgumentIsNotAllowed)
  }

  #[inline]
  pub(crate) fn not_valid_argument(index: usize) -> Self {
    ParseError::ArgumentIsNotValid(index, InvalidArgument::ArgumentIsNotValid)
  }

  #[inline]
  pub(crate) fn argument_value_must_be_numeric(index: usize) -> Self {
    ParseError::ArgumentIsNotValid(index, InvalidArgument::ArgumentValueMustBeNumberic)
  }
}

impl std::fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Parse error")
  }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parser::parse_line;

  #[test]
  fn test_label_is_not_valid() {
    let line = "фывфыфыв:";
    let result = parse_line(line, 0);

    assert_eq!(result, Err(ParseError::LabelIsNotValid(0)));
  }

  #[test]
  fn test_unsupported_syntax() {
    let line = "LOAD 1 2";
    let result = parse_line(line, 0);

    assert_eq!(result, Err(ParseError::UnsupportedSyntax(0)));
  }

  #[test]
  fn test_unsupported_opcode() {
    let line = "KoKotinf 1 2";
    let result = parse_line(line, 0);

    assert_eq!(result, Err(ParseError::UnsupportedSyntax(0)));
  }

  #[test]
  fn test_argument_is_required() {
    let line = "LOAD";
    let result = parse_line(line, 0);

    assert_eq!(result, Err(ParseError::ArgumentIsRequired(0)));
  }

  #[test]
  fn test_pure_argument_not_allowed() {
    let line = "STORE =1";
    let result = parse_line(line, 0);

    assert_eq!(result, Err(ParseError::pure_argument_not_allowed(0)));
  }

  #[test]
  fn test_argument_value_must_be_numeric() {
    let line = "STORE *a";
    let result = parse_line(line, 0);

    assert_eq!(result, Err(ParseError::argument_value_must_be_numeric(0)));
  }

  #[test]
  fn test_argument_is_not_valid() {
    let line = "STORE a";
    let result = parse_line(line, 0);

    assert_eq!(result, Err(ParseError::not_valid_argument(0)));
  }
}
