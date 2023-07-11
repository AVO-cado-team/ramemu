//! Stmt module defines the [`Stmt`] enum, which represents the various statements
//! in the RAM assembly language, as well as related types [`Value`],
//! [`RegisterValue`], and [`Label`].
//!
//! The [`Stmt`] enum contains variants for each of the supported assembly language statements,
//! such as Load, Store, Add, and Jump, along with the line number from the source code.
//!
//! The [`Value`] enum represents a value that is passed to a statement. It can be either a pure value
//! or a value in a register.
//!
//! The [`RegisterValue`] enum represents a reference to a register, either directly or indirectly.
//! Direct references are register numbers, while indirect references are register numbers that
//! point to other registers.
//!
//! The [`Label`] struct is used to represent labels in the assembly language. Labels are used for
//! defining targets for jump statements.
//!
//! # Examples
//!
//! Here's an example of RAM assembly language code:
//!
//! ```text
//! load =0
//!
//! read  0
//! store 1 # number
//!
//! sub =1
//! jz quit_1
//!
//! loop_1:
//!     load  1
//!     div   3
//!     mul   3
//!     sub   1
//!     jz quit_2
//!     load  3
//!     sub   2
//!     jz quit_1
//!     load  3
//!     add  =1
//!     store 3
//!     jmp loop_1
//!
//! quit_1:
//! write =1
//! jmp quit
//!
//! quit_2:
//! write =0
//! jmp quit
//!
//! quit:
//!
//! halt
//! ```
//!
//! This code demonstrates the use of various statements, including Load, Store, Jump, and Label.
//! It also demonstrates the use of `Value` and `RegisterValue` for specifying operands in the
//! assembly language code.

use crate::program::LabelId;

/// Represents a statement in the program, along with its line number from the source code.
/// Statements are the basic building blocks of a program and define the operations to be performed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Stmt {
    /// The operation to be performed by the statement.
    pub op: Op,
    /// The line number from the source code.
    pub line: usize,
}

impl AsRef<Self> for Stmt {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for Stmt {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl PartialOrd for Stmt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.line.cmp(&other.line))
    }
}

impl Ord for Stmt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.line.cmp(&other.line)
    }
}

impl Stmt {
    /// Creates a new statement with the specified operation and line number.
    #[must_use]
    pub fn new(op: Op, line: usize) -> Self {
        Self { op, line }
    }
}

/// Represents an operation that can be performed by the program.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Op {
    /// Loads value into register `0`
    Load(Value),
    /// Stores value from register `0` into register
    Store(RegisterValue),
    /// Adds value to register `0`
    Add(Value),
    /// Subtracts value from register `0`
    Sub(Value),
    /// Multiplies value with register `0`
    Mult(Value),
    /// Divides register `0` by value
    Div(Value),
    /// Jumps to label
    Jump(LabelId),
    /// Jumps to label if register `0` is equal to `0`
    JumpIfZero(LabelId),
    /// Jumps to label if register `0` is greater than `0`
    JumpGreatherZero(LabelId),
    /// Inputs value from `reader`
    Input(RegisterValue),
    /// Outputs value to `writer`
    Output(Value),
    /// Halts program
    Halt,
}

impl AsRef<Self> for Op {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for Op {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

/// Represents a value that can be passed to a statement.
/// The value can be a pure numeric value or a value stored in a register.
///
/// Examples:
/// - `LOAD =5`: Loads the pure numeric value `5` into register 0.
/// - `LOAD 5`: Loads the value stored in register 5 into register 0.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    /// Represents a pure numeric value, indicated by an equal sign (`=`) before the number.
    /// For example, `LOAD =5` loads the pure numeric value `5` into register 0.
    Pure(isize),
    /// Represents the value stored in a specific register.
    // /// For example, `LOAD 5` loads the value stored in register 5 into register 0.
    Register(RegisterValue),
}

/// Represents a register that can be operated on directly or indirectly.
///
/// There are two ways to specify the register to be operated on:
/// - Direct: The register is specified directly, e.g., `STORE 2` stores the value from register 0 into register 2.
/// - Indirect: The register is specified indirectly, e.g., `STORE *2` stores the value from register 0 into the register whose number is stored in register 2.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RegisterValue {
    /// Specifies the register to be operated on directly.
    /// Example: `STORE 2` stores the value from register 0 into register 2.
    Direct(usize),
    /// Specifies the register to be operated on indirectly.
    /// Example: `STORE *2` stores the value from register 0 into the register whose number is stored in register 2.
    Indirect(usize),
}

impl AsRef<Self> for Value {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for Value {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl AsRef<Self> for RegisterValue {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for RegisterValue {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
