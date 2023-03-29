use crate::op::Op;
use crate::op::RegisterValue;
use crate::op::Value;
use crate::program::Program;
use crate::registers::Registers;

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
      registers: vec![0; 100].into(),
      pc: 0,
      line: 0,
      halt: false,
    }
  }

  pub fn get_registers(&self) -> &Registers<i64> {
    &self.registers
  }

  pub fn get_current_instruction(&self) -> Option<Op> {
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
      Op::Label(..) => {}
      Op::Load(value, _) => *self.first_mut() = self.get_with_value(&value.clone())?,
      Op::Store(value, _) => {
        let index: usize = self
          .get_with_register(&value.clone())?
          .try_into()
          .map_err(|_| InterpretError::SegmentationFault(self.line))?;
        *self.registers.get_mut(index) = *self.first();
      }
      Op::Add(value, _) => *self.first_mut() += self.get_with_value(&value.clone())?,
      Op::Sub(value, _) => *self.first_mut() -= self.get_with_value(&value.clone())?,
      Op::Mul(value, _) => *self.first_mut() *= self.get_with_value(&value.clone())?,
      Op::Div(value, _) => {
        let value = self.get_with_value(&value.clone())?;
        *self.first_mut() = self
          .first()
          .checked_div(value)
          .ok_or(InterpretError::DivisionByZero(self.line))?;
      }
      Op::Jump(label, _) => {
        self.pc = self
          .program
          .decode_label(label)
          .ok_or(InterpretError::UnknownLabel(self.line))?;
        should_increment = false;
      }
      Op::JumpIfZero(label, _) => {
        if *self.first() == 0 {
          self.pc = self
            .program
            .decode_label(label)
            .ok_or(InterpretError::UnknownLabel(self.line))?;

          should_increment = false;
        }
      }
      Op::JumpGreatherZero(label, _) => {
        if *self.first() > 0 {
          self.pc = self
            .program
            .decode_label(label)
            .ok_or(InterpretError::UnknownLabel(self.line))?;

          should_increment = false;
        }
      }
      Op::Output(value, _) => println!("{}", self.get_with_value(&value.clone())?),
      Op::Input(value, _) => {
        let mut input = String::new();
        std::io::stdin()
          .read_line(&mut input)
          .map_err(|_| InterpretError::InvalidInput(self.line, input.clone()))?;
        let index: usize = self
          .get_with_register(&value.clone())?
          .try_into()
          .map_err(|_| InterpretError::SegmentationFault(self.line))?;
        *self.registers.get_mut(index) = input
          .trim()
          .parse()
          .map_err(|_| InterpretError::SegmentationFault(self.line))?;
      }
      Op::Halt(_) => self.halt = true,
    };

    if should_increment {
      self.pc += 1;
    }

    Ok(())
  }

  fn get_with_value(&mut self, value: &Value) -> Result<i64, InterpretError> {
    match value {
      Value::Pure(index) => self.get::<0>(*index),
      Value::Register(RegisterValue::Direct(index)) => self.get::<1>(*index),
      Value::Register(RegisterValue::Indirect(index)) => self.get::<2>(*index),
    }
  }

  fn get_with_register(&mut self, value: &RegisterValue) -> Result<i64, InterpretError> {
    match value {
      RegisterValue::Direct(index) => self.get::<0>(*index),
      RegisterValue::Indirect(index) => self.get::<1>(*index),
    }
  }

  fn first_mut(&mut self) -> &mut i64 {
    self.registers.first_mut()
  }

  fn first(&self) -> &i64 {
    self.registers.first()
  }

  fn get<const N: usize>(&mut self, index: usize) -> Result<i64, InterpretError> {
    if N == 0 {
      return index
        .try_into()
        .map_err(|_| InterpretError::InvalidLiteral(self.line));
    }

    let mut index = index;
    for _ in 0..N - 1 {
      index = (*self.registers.get_mut(index))
        .try_into()
        .map_err(|_| InterpretError::SegmentationFault(self.line))?
    }
    Ok(*self.registers.get_mut(index))
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
