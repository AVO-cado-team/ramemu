#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Op {
  Load(Value),
  Store(Value),
  Add(Value),
  Sub(Value),
  Mul(Value),
  Div(Value),
  Jump(Value),
  JumpIfZero(Value),
  JumpIfNeg(Value),
  Input(Value),
  Output(Value),
  Label(String),
  Halt,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
  Direct(usize),
  Indirect(usize),
  DoubleIndirect(usize),
  Label(String),
}

