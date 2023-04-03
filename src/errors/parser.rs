use std::error::Error;

/// Represents various parsing errors that may occur during parsing and validating input.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum ParseError {
  /// Occurs when a label is not valid.
  LabelIsNotValid(usize),

  /// Occurs when unsupported syntax is encountered.
  UnsupportedSyntax(usize),
  /// Occurs when an unsupported opcode is encountered.
  UnsupportedOpcode(usize, String),

  /// Occurs when an argument is required but not provided.
  ArgumentIsRequired(usize),
  /// Occurs when an argument is not valid.
  ArgumentIsNotValid(usize, InvalidArgument),

  /// Represents an unknown error that occurred at a specific index.
  UnknownError(usize),
}

/// Represents various invalid argument errors that may occur during parsing and validating input.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum InvalidArgument {
  LabelIsNotValid,
  ArgumentIsRequired,
  ArgumentValueMustBeNumberic,
  PureArgumentIsNotAllowed,

  ArgumentIsNotValid,
}

impl ParseError {
  /// Creates a new `ParseError` for the `PureArgumentIsNotAllowed` case.
  #[inline]
  pub(crate) fn pure_argument_not_allowed(index: usize) -> Self {
    ParseError::ArgumentIsNotValid(index, InvalidArgument::PureArgumentIsNotAllowed)
  }

  /// Creates a new `ParseError` for the `ArgumentIsNotValid` case.
  #[inline]
  pub(crate) fn not_valid_argument(index: usize) -> Self {
    ParseError::ArgumentIsNotValid(index, InvalidArgument::ArgumentIsNotValid)
  }
  /// Creates a new `ParseError` for the `ArgumentValueMustBeNumberic` case.
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
  use rustc_hash::FxHashMap;

use super::*;
  use crate::parser::parse_line;

  #[test]
  fn test_label_is_not_valid() {
    let line = "фывфыфыв:";
    let result = parse_line(line, 0, &mut FxHashMap::default());

    assert_eq!(result, Err(ParseError::LabelIsNotValid(0)));
  }

  #[test]
  fn test_unsupported_syntax() {
    let line = "LOAD 1 2";
    let result = parse_line(line, 0, &mut FxHashMap::default());

    assert_eq!(result, Err(ParseError::UnsupportedSyntax(0)));
  }

  #[test]
  fn test_unsupported_opcode() {
    let line = "KoKotinf 1 2";
    let result = parse_line(line, 0, &mut FxHashMap::default());

    assert_eq!(result, Err(ParseError::UnsupportedSyntax(0)));
  }

  #[test]
  fn test_argument_is_required() {
    let line = "LOAD";
    let result = parse_line(line, 0, &mut FxHashMap::default());

    assert_eq!(result, Err(ParseError::ArgumentIsRequired(0)));
  }

  #[test]
  fn test_pure_argument_not_allowed() {
    let line = "STORE =1";
    let result = parse_line(line, 0, &mut FxHashMap::default());

    assert_eq!(result, Err(ParseError::pure_argument_not_allowed(0)));
  }

  #[test]
  fn test_argument_value_must_be_numeric() {
    let line = "STORE *a";
    let result = parse_line(line, 0, &mut FxHashMap::default());

    assert_eq!(result, Err(ParseError::argument_value_must_be_numeric(0)));
  }

  #[test]
  fn test_argument_is_not_valid() {
    let line = "STORE a";
    let result = parse_line(line, 0, &mut FxHashMap::default());

    assert_eq!(result, Err(ParseError::not_valid_argument(0)));
  }
}
