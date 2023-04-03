//! The [`errors`] module provides error types for various parsing and interpretation errors
//! that may occur during the parsing, validation, and execution of a program.
//!
//! This module includes the following error types:
//! - [`ParseError`] for parsing errors that may occur during parsing and validating input.
//! - [`InterpretError`] for interpretation errors that may occur during program execution.
//!
//! It also includes error-related types:
//! - [`InvalidArgument`] for representing various invalid argument errors.
//!
//! [`ParseError`]: enum.ParseError.html
//! [`InterpretError`]: enum.InterpretError.html
//! [`InvalidArgument`]: enum.InvalidArgument.html
//! [`errors`]: errors/index.html
mod parser;
mod ram;

pub use ram::*;
pub use parser::*;
