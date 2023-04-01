#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
  Load(Value, usize),
  Store(RegisterValue, usize),
  Add(Value, usize),
  Sub(Value, usize),
  Mul(Value, usize),
  Div(Value, usize),
  Jump(Label, usize),
  JumpIfZero(Label, usize),
  JumpGreatherZero(Label, usize),
  Input(RegisterValue, usize),
  Output(Value, usize),
  Label(String, usize),
  Halt(usize),
}

impl Stmt {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
  Pure(usize),
  Register(RegisterValue),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RegisterValue {
  Direct(usize),
  Indirect(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(String);

impl Label {
  #[inline]
  pub fn new(label: String) -> Self {
    Label(label)
  }
  #[inline]
  pub fn get(&self) -> &str {
    &self.0
  }
}
