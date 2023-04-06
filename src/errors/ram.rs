/// Represents various interpretation errors that may occur during program execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InterpretError {
  /// Occurs when attempting to access memory outside the allowed range.
  SegmentationFault(usize),
  /// Occurs when a reference to an unknown label is encountered.
  UnknownLabel(usize),
  /// Occurs when invalid input is provided during program execution.
  InvalidInput(usize, String),
  /// Occurs when an invalid literal value is encountered.
  InvalidLiteral(usize),
  /// Occurs when a division by zero is attempted.
  DivisionByZero(usize),
  /// Occurs when there is an error writing to provided writer.
  IOError(usize),
  /// Occurs when the program is halted but step was made.
  Halted(usize),
}

impl std::fmt::Display for InterpretError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "InterpretError")
  }
}

impl std::error::Error for InterpretError {}
