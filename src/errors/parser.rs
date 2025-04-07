use std::error::Error;

/// Represents various parsing error kinds that may occur during parsing and validating input.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum ParseErrorKind {
    /// Occurs when a label is not valid.
    LabelIsNotValid,
    /// Occurs when unsupported syntax is encountered.
    UnsupportedSyntax,
    /// Occurs when an unsupported opcode is encountered.
    UnsupportedOpcode(String),
    /// Occurs when an argument is required but not provided.
    ArgumentIsRequired,
    /// Occurs when an argument is not valid.
    ArgumentIsNotValid(InvalidArgument),
    /// Represents an unknown error that occurred at a specific index.
    UnknownError,
}

/// Represents various parsing errors that may occur during parsing and validating input.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct ParseError {
    /// Kind of the error.
    pub kind: ParseErrorKind,
    /// The line number from the source code.
    pub line: usize,
}

/// Represents various invalid argument errors that may occur during parsing and validating input.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum InvalidArgument {
    /// Occurs when a label is not valid.
    LabelIsNotValid,
    /// Occurs when an argument is required but not provided.
    ArgumentIsRequired,
    /// Occurs when an argument muts be numeric but it is not.
    ArgumentValueMustBeNumberic,

    /// Occurs when a pure argument is not allowed.
    PureArgumentIsNotAllowed,

    /// Occurs when an argument is not valid.
    ArgumentIsNotValid,
}

impl ParseErrorKind {
    /// Creates a new `ParseError` for the `PureArgumentIsNotAllowed` case.
    #[inline]
    pub(crate) fn pure_argument_not_allowed() -> Self {
        Self::ArgumentIsNotValid(InvalidArgument::PureArgumentIsNotAllowed)
    }

    /// Creates a new `ParseError` for the `ArgumentIsNotValid` case.
    #[inline]
    pub(crate) fn not_valid_argument() -> Self {
        Self::ArgumentIsNotValid(InvalidArgument::ArgumentIsNotValid)
    }
    /// Creates a new `ParseError` for the `ArgumentValueMustBeNumberic` case.
    #[inline]
    pub(crate) fn argument_value_must_be_numeric() -> Self {
        Self::ArgumentIsNotValid(InvalidArgument::ArgumentValueMustBeNumberic)
    }
}

impl std::fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::LabelIsNotValid => write!(f, "Invalid Label"),
            Self::UnsupportedSyntax => write!(f, "Unsupported Syntax"),
            Self::UnsupportedOpcode(opcode) => write!(f, "Unsupported Opcode: {opcode}"),
            Self::ArgumentIsRequired => write!(f, "Argument is required"),
            Self::ArgumentIsNotValid(arg) => write!(f, "Argument is not valid: {arg}"),
            Self::UnknownError => todo!(),
        }
    }
}

impl std::fmt::Display for InvalidArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LabelIsNotValid => write!(f, "Invalid Label"),
            Self::ArgumentIsRequired => write!(f, "Argument is required"),
            Self::ArgumentValueMustBeNumberic => write!(f, "Argument must be numeric"),
            Self::PureArgumentIsNotAllowed => write!(f, "Pure argument is not allowed"),
            Self::ArgumentIsNotValid => write!(f, "Argument is not valid"),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at line {}: {:?}", self.line, self.kind)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_line;
    use std::collections::HashMap;

    #[test]
    fn test_label_is_not_valid() {
        let line = "фывфыфыв:";
        let result = parse_line(line, &mut HashMap::default());

        assert_eq!(result, Err(ParseErrorKind::LabelIsNotValid));
    }

    #[test]
    fn test_unsupported_syntax() {
        let line = "LOAD 1 2";
        let result = parse_line(line, &mut HashMap::default());

        assert_eq!(result, Err(ParseErrorKind::UnsupportedSyntax));
    }

    #[test]
    fn test_unsupported_opcode() {
        let line = "KoKotinf 1 2";
        let result = parse_line(line, &mut HashMap::default());

        assert_eq!(result, Err(ParseErrorKind::UnsupportedSyntax));
    }

    #[test]
    fn test_argument_is_required() {
        let line = "LOAD";
        let result = parse_line(line, &mut HashMap::default());

        assert_eq!(result, Err(ParseErrorKind::ArgumentIsRequired));
    }

    #[test]
    fn test_pure_argument_not_allowed() {
        let line = "STORE =1";
        let result = parse_line(line, &mut HashMap::default());

        assert_eq!(result, Err(ParseErrorKind::pure_argument_not_allowed()));
    }

    #[test]
    fn test_argument_value_must_be_numeric() {
        let line = "STORE *a";
        let result = parse_line(line, &mut HashMap::default());

        assert_eq!(
            result,
            Err(ParseErrorKind::argument_value_must_be_numeric())
        );
    }

    #[test]
    fn test_argument_is_not_valid() {
        let line = "STORE a";
        let result = parse_line(line, &mut HashMap::default());

        assert_eq!(result, Err(ParseErrorKind::not_valid_argument()));
    }
}
