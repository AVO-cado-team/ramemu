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

/// Represents a statement in the program, along with its line number from the source code.
/// Statements are the basic building blocks of a program and define the operations to be performed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
  /// Loads value into register `0`
  Load(Value, usize),
  /// Stores value from register `0` into register
  Store(RegisterValue, usize),
  /// Adds value to register `0`
  Add(Value, usize),
  /// Subtracts value from register `0`
  Sub(Value, usize),
  /// Multiplies value with register `0`
  Mul(Value, usize),
  /// Divides register `0` by value
  Div(Value, usize),
  /// Jumps to label
  Jump(Label, usize),
  /// Jumps to label if register `0` is equal to `0`
  JumpIfZero(Label, usize),
  /// Jumps to label if register `0` is greater than `0`
  JumpGreatherZero(Label, usize),
  /// Inputs value from `reader`
  Input(RegisterValue, usize),
  /// Outputs value to `writer`
  Output(Value, usize),
  /// Represents label
  Label(Label, usize),
  /// Halts program
  Halt(usize),
}

impl Stmt {
  /// Returns line number of statement in source code
  #[inline]
  pub fn get_line(&self) -> usize {
    match self {
      Stmt::Load(_, line) => *line,
      Stmt::Store(_, line) => *line,
      Stmt::Add(_, line) => *line,
      Stmt::Sub(_, line) => *line,
      Stmt::Mul(_, line) => *line,
      Stmt::Div(_, line) => *line,
      Stmt::Jump(_, line) => *line,
      Stmt::JumpIfZero(_, line) => *line,
      Stmt::JumpGreatherZero(_, line) => *line,
      Stmt::Input(_, line) => *line,
      Stmt::Output(_, line) => *line,
      Stmt::Label(_, line) => *line,
      Stmt::Halt(line) => *line,
    }
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
  Pure(usize),
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

/// Represent label
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Label(usize);

impl From<usize> for Label {
  #[inline]
  fn from(label: usize) -> Self {
    Label(label)
  }
}

impl From<Label> for usize {
  #[inline]
  fn from(label: Label) -> Self {
    label.0
  }
}

