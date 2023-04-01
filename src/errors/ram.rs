#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InterpretError {
  SegmentationFault(usize),
  UnknownLabel(usize),
  InvalidInput(usize, String),
  InvalidLiteral(usize),
  DivisionByZero(usize),
  Halted(usize),
}

impl std::fmt::Display for InterpretError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "InterpretError")
  }
}

impl std::error::Error for InterpretError {}

