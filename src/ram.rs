use std::fmt::Debug;
use std::fmt::Formatter;
use std::io::BufRead;
use std::io::Write;

use crate::errors::InterpretError;
use crate::program::Program;
use crate::registers::Registers;
use crate::stmt::RegisterValue;
use crate::stmt::Stmt;
use crate::stmt::Value;

pub struct Ram {
  program: Program,
  registers: Registers<i64>,
  pc: usize,
  line: usize,
  halt: bool,
  error: Option<InterpretError>,
  reader: Box<dyn BufRead>,
  writer: Box<dyn Write>,
}

impl Ram {
  #[inline]
  pub fn new(program: Program, reader: Box<dyn BufRead>, writer: Box<dyn Write>) -> Self {
    Ram {
      program,
      registers: [0; 100].into(),
      pc: 0,
      line: 0,
      halt: false,
      error: None,
      reader,
      writer,
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

  // proxy to halt on error
  pub fn step(&mut self) -> Result<(), InterpretError> {
    let result = self.eval_current();
    if let Ok(next_pc) = result {
      self.pc = next_pc;
    } else {
      self.halt = true;
    }
    result.map(|_| ())
  }

  #[inline]
  pub fn get_error(&self) -> Option<InterpretError> {
    self.error.clone()
  }

  #[inline]
  pub fn eval(&mut self, stmt: Stmt) -> Result<(), InterpretError> {
    let inject_into = self.pc;
    self.program.inject_instruction(stmt, inject_into);
    let _next_pc = self.eval_current()?;
    self.program.remove_instruction(inject_into);
    Ok(())
  }

  fn eval_current(&mut self) -> Result<usize, InterpretError> {
    if self.halt {
      return Err(InterpretError::Halted(self.line));
    }

    let Some(stmt) = self.program.get(self.pc) else {
      return Err(InterpretError::SegmentationFault(self.line));
    };

    self.line = stmt.get_line();
    let mut next_pc = self.pc + 1;

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
        next_pc = self
          .program
          .decode_label(label)
          .ok_or(InterpretError::UnknownLabel(self.line))?;
      }
      Stmt::JumpIfZero(label, _) => {
        if self.first() == 0 {
          next_pc = self
            .program
            .decode_label(label)
            .ok_or(InterpretError::UnknownLabel(self.line))?;
        }
      }
      Stmt::JumpGreatherZero(label, _) => {
        if self.first() > 0 {
          next_pc = self
            .program
            .decode_label(label)
            .ok_or(InterpretError::UnknownLabel(self.line))?;
        }
      }
      Stmt::Output(value, _) => {
        let value = self.get_with_value(value)?;
        writeln!(&mut self.writer, "{}", value)
          .map_err(|_| InterpretError::WriteError(self.line))?
      }
      Stmt::Input(value, _) => {
        let mut input = String::new();
        self
          .reader
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

    Ok(next_pc)
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

impl Debug for Ram {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Ram")
      .field("program", &self.program)
      .field("registers", &self.registers)
      .field("pc", &self.pc)
      .field("line", &self.line)
      .field("halt", &self.halt)
      .field("error", &self.error)
      .finish()
  }
}

impl Iterator for Ram {
  type Item = Result<RamState, InterpretError>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.halt {
      return None;
    }
    Some(self.step().map(|_| RamState::from(&*self)))
  }
}

#[derive(Default, Debug, Clone)]
pub struct RamState {
  pub program: Program,
  pub registers: Registers<i64>,
  pub pc: usize,
  pub line: usize,
  pub halt: bool,
  pub error: Option<InterpretError>,
}

impl From<Ram> for RamState {
  fn from(ram: Ram) -> Self {
    Self {
      program: ram.program,
      registers: ram.registers,
      pc: ram.pc,
      line: ram.line,
      halt: ram.halt,
      error: ram.error,
    }
  }
}

impl From<&Ram> for RamState {
  fn from(ram: &Ram) -> Self {
    Self {
      program: ram.program.clone(),
      registers: ram.registers.clone(),
      pc: ram.pc,
      line: ram.line,
      halt: ram.halt,
      error: ram.error.clone(),
    }
  }
}

impl RamState {
  pub fn create_ram(self, reader: Box<dyn BufRead>, writer: Box<dyn Write>) -> Ram {
    Ram {
      program: self.program,
      registers: self.registers,
      pc: self.pc,
      line: self.line,
      halt: self.halt,
      error: self.error,
      reader,
      writer,
    }
  }
}
