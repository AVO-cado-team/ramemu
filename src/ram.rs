use crate::program::Program;
use crate::registers::Registers;
use crate::stmt::RegisterValue;
use crate::stmt::Stmt;
use crate::stmt::Value;

#[derive(Debug, Clone)]
pub struct Ram {
  program: Program,
  registers: Registers<i64>,
  pc: usize,
  line: usize,
  halt: bool,
}

impl Ram {
  pub fn new(program: Program) -> Self {
    Ram {
      program,
      registers: [0; 100].into(),
      pc: 0,
      line: 0,
      halt: false,
    }
  }

  #[inline]
  pub fn get_registers(&self) -> &Registers<i64> {
    &self.registers
  }

  #[inline]
  pub fn get_current_instruction(&self) -> Option<Stmt> {
    self.program.get(self.pc).cloned()
  }

  pub fn run(&mut self) -> Result<(), InterpretError> {
    while !self.halt {
      self.step()?;
    }
    Ok(())
  }

  pub fn step(&mut self) -> Result<(), InterpretError> {
    if self.halt {
      return Err(InterpretError::Halted(self.line));
    }

    let Some(stmt) = self.program.get(self.pc) else {
      return Err(InterpretError::SegmentationFault(self.line));
    };

    let mut should_increment = true;

    self.line = stmt.get_line();

    match stmt {
      Stmt::Label(..) => {}
      Stmt::Load(value, _) => self.set_first(self.get_with_value(value)?),
      Stmt::Store(value, _) => {
        let index: usize = self
          .get_with_register(&value.clone())?
          .try_into()
          .map_err(|_| InterpretError::SegmentationFault(self.line))?;
        self.registers.set(index, self.first());
      }
      Stmt::Add(value, _) => self.set_first(self.first() + self.get_with_value(value)?),
      Stmt::Sub(value, _) => self.set_first(self.first() - self.get_with_value(value)?),
      Stmt::Mul(value, _) => self.set_first(self.first() * self.get_with_value(value)?),
      Stmt::Div(value, _) => {
        self.set_first(
          self
            .first()
            .checked_div(self.get_with_value(value)?)
            .ok_or(InterpretError::DivisionByZero(self.line))?,
        );
      }
      Stmt::Jump(label, _) => {
        self.pc = self
          .program
          .decode_label(label)
          .ok_or(InterpretError::UnknownLabel(self.line))?;
        should_increment = false;
      }
      Stmt::JumpIfZero(label, _) => {
        if self.first() == 0 {
          self.pc = self
            .program
            .decode_label(label)
            .ok_or(InterpretError::UnknownLabel(self.line))?;

          should_increment = false;
        }
      }
      Stmt::JumpGreatherZero(label, _) => {
        if self.first() > 0 {
          self.pc = self
            .program
            .decode_label(label)
            .ok_or(InterpretError::UnknownLabel(self.line))?;

          should_increment = false;
        }
      }
      Stmt::Output(value, _) => println!("{}", self.get_with_value(value)?),
      Stmt::Input(value, _) => {
        let mut input = String::new();
        std::io::stdin()
          .read_line(&mut input)
          .map_err(|_| InterpretError::InvalidInput(self.line, input.clone()))?;
        let index: usize = self
          .get_with_register(&value.clone())?
          .try_into()
          .map_err(|_| InterpretError::SegmentationFault(self.line))?;
        self.registers.set(
          index,
          input
            .trim()
            .parse()
            .map_err(|_| InterpretError::SegmentationFault(self.line))?,
        );
      }
      Stmt::Halt(_) => self.halt = true,
    };

    if should_increment {
      self.pc += 1;
    }

    Ok(())
  }

  #[inline]
  fn get_with_value(&self, value: &Value) -> Result<i64, InterpretError> {
    match value {
      Value::Pure(index) => self.get::<0>(*index),
      Value::Register(RegisterValue::Direct(index)) => self.get::<1>(*index),
      Value::Register(RegisterValue::Indirect(index)) => self.get::<2>(*index),
    }
  }

  #[inline]
  fn get_with_register(&self, value: &RegisterValue) -> Result<i64, InterpretError> {
    match value {
      RegisterValue::Direct(index) => self.get::<0>(*index),
      RegisterValue::Indirect(index) => self.get::<1>(*index),
    }
  }

  #[inline]
  fn set_first(&mut self, value: i64) {
    self.registers.set(0, value);
  }

  #[inline]
  fn first(&self) -> i64 {
    self.registers.get(0)
  }

  fn get<const N: usize>(&self, index: usize) -> Result<i64, InterpretError> {
    if N == 0 {
      return index
        .try_into()
        .map_err(|_| InterpretError::InvalidLiteral(self.line));
    }

    let mut index = index;
    for _ in 0..N - 1 {
      index = self
        .registers
        .get(index)
        .try_into()
        .map_err(|_| InterpretError::SegmentationFault(self.line))?
    }
    Ok(self.registers.get(index))
  }
}

#[derive(Debug)]
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

impl Iterator for Ram {
  type Item = Result<RamState, InterpretError>;
  fn next(&mut self) -> Option<Self::Item> {
    let out = self.step().map(|_| RamState::from(&*self));
    if self.halt {
      None
    } else {
      Some(out)
    }
  }
}

#[derive(Default, Debug, Clone)]
pub struct RamState {
  pub program: Program,
  pub registers: Registers<i64>,
  pub pc: usize,
  pub line: usize,
  pub halt: bool,
}

impl From<&Ram> for RamState {
  fn from(ram: &Ram) -> Self {
    Self {
      program: ram.program.clone(),
      registers: ram.registers.clone(),
      pc: ram.pc,
      line: ram.line,
      halt: ram.halt,
    }
  }
}

impl From<RamState> for Ram {
  fn from(state: RamState) -> Self {
    Self {
      program: state.program,
      registers: state.registers,
      pc: state.pc,
      line: state.line,
      halt: state.halt,
    }
  }
}

