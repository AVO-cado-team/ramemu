//! # RAM Machine Simulation
//!
//! This module provides a [`Ram`] struct and related functionalities
//! The RAM machine is a theoretical model of computation that consists of an
//! for simulating the behavior of a RAM (Random Access Machine) machine.
//! infinite array of memory cells, a finite set of registers, and a program counter.
//! The [`Ram`] struct holds the program, registers, program counter, line
//! number, halt state, error state, input reader, and output writer.
//!
//! The main functionality includes the following methods:
//!
//! 1. Creating a new [`Ram`] instance.
//! 2. Getting the current state of registers.
//! 3. Fetching the current instruction.
//! 4. Running the program.
//! 5. Executing a single instruction.
//! 6. Getting the current error state.
//! 7. Evaluating a given statement.
//! 8. Evaluating the current statement.
//!
//! The [`Ram`] struct also implements the [`Debug`] trait for better debug
//! outputs and the [`Iterator`] trait, which allows the RAM machine to be used
//! as an iterator, with each step yielding a [`RamState`].
//!
//! [`RamState`] is a struct that holds a snapshot of the RAM machine's state.
//! It is used to track the RAM machine's state at different points in time. It
//! can be created from a `Ram` instance or a reference to a `Ram` instance
//! using the [`From`] trait. It also provides a [`RamState::create_ram()`]
//! method to create a new [`Ram`] instance from a [`RamState`].
//!
//! # Examples
//!
//! ```
//! use ramemu::program::Program;
//! use ramemu::registers::RegisterId;
//! use ramemu::ram::Ram;
//! use ramemu::stmt::{Stmt, Value, Op::*};
//! use std::io::BufReader;
//! use std::io::BufWriter;
//!
//! let instructions = vec![
//!   Stmt::new(Load(Value::Pure(2)), 1),
//!   Stmt::new(Add(Value::Pure(2)), 3),
//!   Stmt::new(Output(Value::Pure(0)), 4),
//!   Stmt::new(Halt, 5),
//! ];
//! let labels = Default::default();
//! let program = Program::from(instructions, labels);
//!
//! let reader = BufReader::new(std::io::empty());
//! let writer = BufWriter::new(std::io::sink());
//! let mut ram = Ram::new(program, Box::new(reader), Box::new(writer));
//!
//! ram.run().unwrap();
//! assert_eq!(ram.get_registers().get(RegisterId(0)), 4);
//! ```
//!
//! This module enables the creation of a RAM machine and provides the necessary functionalities to execute, debug, and manage its state.
use std::fmt::Debug;
use std::fmt::Formatter;
use std::io::BufRead;
use std::io::Write;
use std::iter::FusedIterator;

use crate::errors::InterpretError;
use crate::program::CodeAddress;
use crate::program::Program;
use crate::registers::RegisterId;
use crate::registers::Registers;
use crate::stmt::Op::{
    Add, Div, Halt, Input, Jump, JumpGreatherZero, JumpIfZero, Load, Mult, Output, Store, Sub,
};
use crate::stmt::RegisterValue;
use crate::stmt::Stmt;
use crate::stmt::Value;

use crate::errors::InterpretErrorKind::{
    DivisionByZero, Halted, IOError, InvalidInput, InvalidLiteral, SegmentationFault, UnknownLabel,
};

/// The [`Ram`] struct represents a Random Access Machine (RAM).
///
/// It holds the program, registers, program counter, line number, halt state, error state, input reader, and output writer.
pub struct Ram {
    program: Program,
    registers: Registers<i64>,
    pc: CodeAddress,
    line: usize,
    halt: bool,
    error: Option<InterpretError>,
    reader: Box<dyn BufRead>,
    writer: Box<dyn Write>,
}

impl Ram {
    /// Creates a new [`Ram`] instance with the given program, input reader, and output writer.
    #[inline]
    #[must_use]
    pub fn new(program: Program, reader: Box<dyn BufRead>, writer: Box<dyn Write>) -> Self {
        Self {
            program,
            registers: [0; 100].into(),
            pc: CodeAddress::default(),
            line: 0,
            halt: false,
            error: None,
            reader,
            writer,
        }
    }

    /// Returns a reference to the registers of the [`Ram`] instance.
    #[inline]
    #[must_use]
    pub fn get_registers(&self) -> &Registers<i64> {
        &self.registers
    }

    /// Returns the current instruction of the program as an [`Option<Stmt>`].
    #[inline]
    #[must_use]
    pub fn get_current_instruction(&self) -> Option<&Stmt> {
        self.program.get(self.pc)
    }

    /// Runs the program until it halts or encounters an error.
    /// # Errors
    /// Returns an [`InterpretError`] if the program encounters an error.
    pub fn run(&mut self) -> Result<(), InterpretError> {
        while !self.halt {
            self.step()?;
        }
        Ok(())
    }

    /// Executes one step of the program and advances the program counter.
    // Maybe, it should return `R0` as `Ok`?
    /// # Errors
    /// Returns an [`InterpretError`] if the program encounters an error.
    #[inline]
    pub fn step(&mut self) -> Result<(), InterpretError> {
        let result = self.eval_current();

        match &result {
            &Ok(next_pc) => self.pc = next_pc,
            Err(error) => {
                self.error = Some(error.clone());
                self.halt = true;
            }
        };

        result.map(|_| ())
    }

    /// Returns the current error state of the [`Ram`] instance as an
    /// [`Option<InterpretError>`].
    #[inline]
    #[must_use]
    pub fn get_error(&self) -> Option<&InterpretError> {
        self.error.as_ref()
    }

    /// Returns the next program counter.
    ///
    /// Evaluates the given statement without affecting the program counter.
    /// Changes `line`
    /// # Errors
    /// Returns an [`InterpretError`] if the program encounters an error.
    #[inline]
    pub fn eval(&mut self, stmt: Stmt) -> Result<CodeAddress, InterpretError> {
        self.line = stmt.line;
        let mut next_pc = self.pc + 1;

        match stmt.op {
            Load(value) => self.set_first(self.get_with_value(value)?),
            Store(value) => {
                let index: usize = self
                    .get_with_register(value)?
                    .try_into()
                    .map_err(|_| InterpretError::new(SegmentationFault, self.line))?;
                self.registers.set(RegisterId(index), self.first());
            }
            Add(value) => self.set_first(self.first() + self.get_with_value(value)?),
            Sub(value) => self.set_first(self.first() - self.get_with_value(value)?),
            Mult(value) => self.set_first(self.first() * self.get_with_value(value)?),
            Div(value) => {
                self.set_first(
                    self.first()
                        .checked_div(self.get_with_value(value)?)
                        .ok_or(InterpretError::new(DivisionByZero, self.line))?,
                );
            }
            Jump(label) => {
                next_pc = self
                    .program
                    .decode_label(label)
                    .ok_or(InterpretError::new(UnknownLabel, self.line))?;
            }
            JumpIfZero(label) => {
                if self.first() == 0 {
                    next_pc = self
                        .program
                        .decode_label(label)
                        .ok_or(InterpretError::new(UnknownLabel, self.line))?;
                }
            }
            JumpGreatherZero(label) => {
                if self.first() > 0 {
                    next_pc = self
                        .program
                        .decode_label(label)
                        .ok_or(InterpretError::new(UnknownLabel, self.line))?;
                }
            }
            Output(value) => {
                let value = self.get_with_value(value)?;
                write!(&mut self.writer, "{value}")
                    .map_err(|_| InterpretError::new(IOError, self.line))?;
            }
            Input(value) => {
                let mut input = String::new();
                self.reader
                    .read_line(&mut input)
                    .map_err(|_| InterpretError::new(IOError, self.line))?;
                let index: usize = self
                    .get_with_register(value)?
                    .try_into()
                    .map_err(|_| InterpretError::new(SegmentationFault, self.line))?;
                self.registers.set(
                    RegisterId(index),
                    input.trim().parse().map_err(|_| {
                        InterpretError::new(InvalidInput(input.trim().into()), self.line)
                    })?,
                );
            }
            Halt => self.halt = true,
        };

        Ok(next_pc)
    }

    fn eval_current(&mut self) -> Result<CodeAddress, InterpretError> {
        if self.halt {
            return Err(InterpretError::new(Halted, self.line));
        }

        let Some(&stmt) = self.program.get(self.pc) else {
            return Err(InterpretError::new(SegmentationFault, self.line));
        };

        self.eval(stmt)
    }

    #[inline]
    fn get_with_value(&self, value: Value) -> Result<i64, InterpretError> {
        match value {
            Value::Pure(index) => (index)
                .try_into()
                .map_err(|_| InterpretError::new(InvalidLiteral, self.line)),
            Value::Register(RegisterValue::Direct(index)) => self.get::<1>(index),
            Value::Register(RegisterValue::Indirect(index)) => self.get::<2>(index),
        }
    }

    #[inline]
    fn get_with_register(&self, value: RegisterValue) -> Result<i64, InterpretError> {
        match value {
            RegisterValue::Direct(index) => self.get::<0>(index),
            RegisterValue::Indirect(index) => self.get::<1>(index),
        }
    }

    #[inline]
    fn set_first(&mut self, value: i64) {
        self.registers.set(RegisterId(0), value);
    }

    #[inline]
    fn first(&self) -> i64 {
        self.registers.get(RegisterId(0))
    }

    fn get<const N: usize>(&self, index: usize) -> Result<i64, InterpretError> {
        if N == 0 {
            return index
                .try_into()
                .map_err(|_| InterpretError::new(InvalidLiteral, self.line));
        }

        let mut index = index;
        for _ in 0..N - 1 {
            index = self
                .registers
                .get(RegisterId(index))
                .try_into()
                .map_err(|_| InterpretError::new(SegmentationFault, self.line))?;
        }
        Ok(self.registers.get(RegisterId(index)))
    }
}

impl AsRef<Self> for Ram {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for Ram {
    fn as_mut(&mut self) -> &mut Self {
        self
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
            .finish_non_exhaustive()
    }
}

impl Iterator for Ram {
    type Item = RamState;
    fn next(&mut self) -> Option<Self::Item> {
        self.step().ok().and_then(|_| {
            if self.halt {
                None
            } else {
                Some(RamState::from(self))
            }
        })
    }
}

impl FusedIterator for Ram {}

/// The [`RamState`] struct represents a snapshot of a RAM machine's state.
#[derive(Default, Debug, Clone)]
pub struct RamState {
    /// The program of the RAM machine.
    pub program: Program,
    /// The registers of the RAM machine.
    pub registers: Registers<i64>,
    /// The program counter of the RAM machine.
    pub pc: CodeAddress,
    /// The current line of source code of the RAM machine.
    pub line: usize,
    /// Whether the RAM machine is halted.
    pub halt: bool,
    /// The error of the RAM machine.
    pub error: Option<InterpretError>,
}

impl From<Ram> for RamState {
    /// Creates a [`RamState`] instance from a given [`Ram`] instance.
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
    /// Creates a [`RamState`] instance from a reference to a [`Ram`] instance.
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

impl From<&mut Ram> for RamState {
    /// Creates a [`RamState`] instance from a reference to a [`Ram`] instance.
    fn from(ram: &mut Ram) -> Self {
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

impl AsRef<Self> for RamState {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for RamState {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl RamState {
    /// Creates a new [`Ram`] instance from the given [`RamState`], input reader, and output writer.
    #[must_use]
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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::io::{Result as IoResult, Write};
    use std::rc::Rc;

    use super::*;
    use std::io::BufReader;
    use std::io::BufWriter;

    pub struct CustomWriter {
        buffer: Rc<RefCell<Vec<u8>>>,
    }

    impl CustomWriter {
        pub fn new(buffer: Rc<RefCell<Vec<u8>>>) -> Self {
            Self { buffer }
        }
    }

    impl Write for CustomWriter {
        fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
            self.buffer.borrow_mut().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> IoResult<()> {
            Ok(())
        }
    }

    fn get_test_program() -> Program {
        let instructions = vec![
            Stmt::new(Load(Value::Pure(2)), 1),
            Stmt::new(Add(Value::Pure(2)), 2),
            Stmt::new(Output(Value::Pure(4)), 3),
            Stmt::new(Halt, 4),
        ];
        let labels = Default::default();
        Program::from(instructions, labels)
    }

    #[test]
    fn ram_new_test() {
        let program = get_test_program();
        let reader = BufReader::new(std::io::empty());
        let writer = BufWriter::new(std::io::sink());
        let ram = Ram::new(program.clone(), Box::new(reader), Box::new(writer));

        assert_eq!(ram.program, program);
        assert_eq!(ram.pc, CodeAddress(0));
        assert_eq!(ram.line, 0);
        assert!(!ram.halt);
        assert_eq!(ram.error, None);
    }

    #[test]
    fn ram_run_test() {
        let program = get_test_program();
        let reader = BufReader::new(std::io::empty());
        let output = Vec::new();
        let writer = BufWriter::new(output);

        let mut ram = Ram::new(program, Box::new(reader), Box::new(writer));
        ram.run().unwrap();

        assert_eq!(ram.get_registers().get(0), 4);
        assert_eq!(ram.pc, 4.into());
        assert_eq!(ram.line, 4);
        assert!(ram.halt);
        assert_eq!(ram.error, None);
    }

    #[test]
    fn ram_step_test() {
        let program = get_test_program();
        let reader = BufReader::new(std::io::empty());
        let output = Vec::new();
        let writer = BufWriter::new(output);

        let mut ram = Ram::new(program, Box::new(reader), Box::new(writer));

        assert_eq!(ram.step(), Ok(()));
        assert_eq!(ram.get_registers().get(0), 2);
        assert_eq!(ram.pc, 1.into());
        assert_eq!(ram.line, 1);

        assert_eq!(ram.step(), Ok(()));
        assert_eq!(ram.get_registers().get(0), 4);
        assert_eq!(ram.pc, 2.into());
        assert_eq!(ram.line, 2);

        assert_eq!(ram.step(), Ok(()));
        assert_eq!(ram.pc, 3.into());
        assert_eq!(ram.line, 3);

        assert_eq!(ram.step(), Ok(()));
        assert_eq!(ram.pc, 4.into());
        assert_eq!(ram.line, 4);
        assert!(ram.halt);
    }
    #[test]
    fn ram_output_test() {
        let program = get_test_program();
        let reader = BufReader::new(std::io::empty());
        let output = Rc::new(RefCell::new(Vec::new()));
        let writer = CustomWriter::new(output.clone());

        let mut ram = Ram::new(program, Box::new(reader), Box::new(writer));
        ram.run().unwrap();

        let output_vec = output.borrow();
        assert_eq!(String::from_utf8(output_vec.to_vec()).unwrap(), "4");
    }
    #[test]
    fn ram_eval_test() {
        let instructions = vec![
            Stmt::new(Load(Value::Pure(2)), 1),
            Stmt::new(Add(Value::Pure(3)), 2),
            Stmt::new(Sub(Value::Pure(1)), 3),
            Stmt::new(Mult(Value::Pure(2)), 4),
            Stmt::new(Div(Value::Pure(2)), 5),
            Stmt::new(Halt, 6),
        ];
        let labels = Default::default();
        let program = Program::from(instructions, labels);

        let reader = BufReader::new(std::io::empty());
        let output = Vec::new();
        let writer = BufWriter::new(output);

        let mut ram = Ram::new(program, Box::new(reader), Box::new(writer));

        // Test Load
        assert_eq!(ram.eval(Stmt::new(Load(Value::Pure(2)), 1)), Ok(1.into()));
        assert_eq!(ram.get_registers().get(0), 2);

        // Test Add
        assert_eq!(ram.eval(Stmt::new(Add(Value::Pure(3)), 2)), Ok(1.into()));
        assert_eq!(ram.get_registers().get(0), 5);

        // Test Sub
        assert_eq!(ram.eval(Stmt::new(Sub(Value::Pure(1)), 3)), Ok(1.into()));
        assert_eq!(ram.get_registers().get(0), 4);

        // Test Mult
        assert_eq!(ram.eval(Stmt::new(Mult(Value::Pure(2)), 4)), Ok(1.into()));
        assert_eq!(ram.get_registers().get(0), 8);

        // Test Div
        assert_eq!(ram.eval(Stmt::new(Div(Value::Pure(2)), 5)), Ok(1.into()));
        assert_eq!(ram.get_registers().get(0), 4);

        // Test Halt
        assert_eq!(ram.eval(Stmt::new(Halt, 6)), Ok(1.into()));
        assert!(ram.halt);
    }
}
