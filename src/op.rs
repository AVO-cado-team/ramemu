#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Op {
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

impl Op {
  pub fn get_line(&self) -> usize {
    match self {
      Op::Load(_, line) => *line,
      Op::Store(_, line) => *line,
      Op::Add(_, line) => *line,
      Op::Sub(_, line) => *line,
      Op::Mul(_, line) => *line,
      Op::Div(_, line) => *line,
      Op::Jump(_, line) => *line,
      Op::JumpIfZero(_, line) => *line,
      Op::JumpGreatherZero(_, line) => *line,
      Op::Input(_, line) => *line,
      Op::Output(_, line) => *line,
      Op::Label(_, line) => *line,
      Op::Halt(line) => *line,
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
  pub fn new(label: String) -> Self {
    Label(label)
  }
  pub fn get(&self) -> &str {
    &self.0
  }
}
