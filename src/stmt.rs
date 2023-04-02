/// Represents a statement in the program
/// Every statement has a line number from source code
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
  Label(String, usize),
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

/// Represents value that is passed to statement
///
/// # Examples
/// ```python
/// LOAD =5
/// ```
///
/// ```text
/// LOAD 5 # Register value. 
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
  /// Represents pure value
  /// ```text
  /// LOAD =5 # Loads `5` into 0s register
  /// ```
  Pure(usize),
  Register(RegisterValue),
}

/// Represents register
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RegisterValue {
  /// Register that will be operated on is specified directly
  /// ```python
  /// STORE 2 // Stores value from register 0 into register 2
  /// ```
  /// Intel syntax heristics:
  /// ```asm
  /// mov ecx, eax
  /// ```
  Direct(usize),
  /// Register that will be operated on is specified indirectly
  /// You could think of this as dereferensing in C.
  /// ```python
  /// STORE *2 // Stores value from register 0 into register that number is written in register 2
  /// ```
  /// Intel syntax heristics:
  /// ```asm
  /// mov [ecx], eax
  /// ```
  Indirect(usize),
}

/// Represent label
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(String);

impl Label {
  /// Creates new label
  #[inline]
  pub fn new(label: String) -> Self {
    Label(label)
  }
  /// Get label as string
  #[inline]
  pub fn get(&self) -> &str {
    &self.0
  }
}
