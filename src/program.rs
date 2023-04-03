//! The [`Program`] module represents a program in the assembly language. It contains
//! the instructions and labels of the program, and provides methods for creating,
//! modifying, and querying the program structure.
use rustc_hash::FxHashMap as HashMap;

use crate::{
  errors::ParseError,
  parser,
  stmt::{Label, Stmt},
};

/// Represents a program code.
///
/// The [`Program`] struct contains the instructions and labels of a program,
/// and provides methods for creating, modifying, and querying the program
/// structure.
#[derive(Default, Debug, Clone)]
pub struct Program {
  /// Instructions of the program.
  pub instructions: Vec<Stmt>,
  /// Labels of the program.
  pub labels: HashMap<String, usize>,
}

impl Program {
  /// Creates a new [`Program`] from the vector of [`Stmt`].
  ///
  /// This method initializes the labels in the program.
  pub fn from(instructions: Vec<Stmt>) -> Self {
    let mut p = Program {
      instructions,
      labels: HashMap::default(),
    };
    p.init_labels();
    p
  }

  /// Creates a new [`Program`] from the source code.
  ///
  /// This method parses the source code, creating a [`Program`] with the resulting
  /// instructions and labels.
  pub fn from_source(source: &str) -> Result<Program, ParseError> {
    let stmts: Result<Vec<Stmt>, ParseError> = parser::parse(source).collect();
    let stmts = stmts?;

    Ok(Program::from(stmts))
  }

  /// Initializes labels of the program.
  ///
  /// This method updates the internal label mapping based on the current instructions.
  #[inline]
  pub fn init_labels(&mut self) {
    self.labels.clear();
    for (index, op) in self.instructions.iter().enumerate() {
      if let Stmt::Label(label, _) = op {
        self.labels.insert(label.clone(), index);
      }
    }
  }


  /// Returns the instruction at the given index.
  ///
  /// If the index is out of bounds, returns `None`.
  #[inline]
  pub fn get(&self, index: usize) -> Option<&Stmt> {
    self.instructions.get(index)
  }

  /// Decodes the label into the instruction index.
  ///
  /// If the label is not found, returns `None`.
  #[inline]
  pub fn decode_label(&self, label: &Label) -> Option<usize> {
    self.labels.get(label.get()).copied()
  }

  /// Injects an instruction at given index.
  #[inline]
  pub fn inject_instruction(&mut self, instruction: Stmt, index: usize) {
    self.instructions.insert(index, instruction);
    self.init_labels();
  }

  /// Removes an instruction at given index.
  #[inline]
  pub fn remove_instruction(&mut self, index: usize) {
    self.instructions.remove(index);
    self.init_labels();
  }

  /// Injects instructions at given index.
  #[inline]
  pub fn inject_instructions<T>(&mut self, instructions: T, index: usize)
  where
    T: IntoIterator<Item = Stmt>,
  {
    let tail = self.instructions.split_off(index);
    self.instructions.extend(instructions.into_iter());
    self.instructions.extend(tail);
    self.init_labels();
  }

  /// Removes instructions at given indexies.
  #[inline]
  pub fn remove_instructions(&mut self, indexes: &[usize]) {
    let to_remove: Vec<Stmt> = self
      .instructions
      .iter()
      .enumerate()
      .filter(|(i, _)| indexes.contains(i))
      .map(|(_, op)| op.clone())
      .collect();

    self.instructions.retain(|op| to_remove.contains(op));
    self.init_labels();
  }
}
