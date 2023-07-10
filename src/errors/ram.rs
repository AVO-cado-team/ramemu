/// Represents various interpretation errors that may occur during program execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InterpretErrorKind {
    /// Occurs when attempting to access memory outside the allowed range.
    SegmentationFault,
    /// Occurs when a reference to an unknown label is encountered.
    UnknownLabel,
    /// Occurs when invalid input is provided during program execution.
    InvalidInput(Box<str>),
    /// Occurs when an invalid literal value is encountered.
    InvalidLiteral,
    /// Occurs when a division by zero is attempted.
    DivisionByZero,
    /// Occurs when there is an error writing to provided writer.
    IOError,
    /// Occurs when the program is halted but step was made.
    Halted,
}

/// Represents various interpretation errors that may occur during program execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InterpretError {
    /// Kind of the error.
    pub kind: InterpretErrorKind,
    /// The line number from the source code.
    pub line: usize,
}

impl InterpretError {
    /// Creates a new `InterpretError` for the `InvalidInput` case.
    #[inline]
    pub(crate) fn new(kind: InterpretErrorKind, line: usize) -> Self {
        Self { kind, line }
    }
}

impl std::fmt::Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InterpretError")
    }
}

impl std::error::Error for InterpretError {}
