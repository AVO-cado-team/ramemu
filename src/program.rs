//! The [`Program`] module represents a program in the assembly language. It contains
//! the instructions and labels of the program, and provides methods for creating,
//! modifying, and querying the program structure.
use rustc_hash::FxHashMap as HashMap;

use crate::{
  errors::{InterpretError, ParseError},
  parser,
  stmt::{
    Label,
    Stmt::{self, *},
  },
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
  /// Label name -> Label id mapping.
  /// This is used for parsing dynamicly added commands.
  pub label_ids: HashMap<String, usize>,
  /// Label id -> Code Address mapping.
  pub labels: Vec<usize>,
}

impl Program {
  /// Creates a new [`Program`] from the vector of [`Stmt`].
  ///
  /// This method initializes the labels in the program.
  pub fn from(instructions: Vec<Stmt>) -> Program {
    Program {
      instructions,
      label_ids: Default::default(),
      labels: Default::default(),
    }
  }

  /// Creates a new [`Program`] from the source code.
  ///
  /// This method parses the source code, creating a [`Program`] with the resulting
  /// instructions and labels.
  pub fn from_source(source: &str) -> Result<Program, ParseError> {
    let mut label_ids: HashMap<String, usize> = HashMap::default();
    let stmts: Result<Vec<Stmt>, ParseError> = parser::parse(source, &mut label_ids).collect();
    let instructions = stmts?;
    let mut p = Program {
      instructions,
      label_ids,
      labels: Default::default(),
    };
    p.init_labels().map_err(|err| match err {
      InterpretError::LabelIsNotValid(line) => ParseError::LabelIsNotValid(line),
      _ => unreachable!("Unexpected error while initializing labels"),
    })?;

    Ok(p)
  }

  /// Initializes labels of the program.
  ///
  /// This method updates the internal label mapping based on the current instructions.
  #[inline]
  pub fn init_labels(self: &mut Program) -> Result<(), InterpretError> {
    let mut labels_idx = HashMap::default();

    for (pc, stmt) in self.instructions.iter().enumerate() {
      if let Stmt::Label(id, _) = stmt {
        if labels_idx.insert(*id, pc).is_some() {
          return Err(InterpretError::LabelIsNotValid(stmt.get_line()));
        }
      }
    }

    if labels_idx.len() != self.label_ids.len() {
      let bad_label = self
        .label_ids
        .values()
        .find(|id| !labels_idx.contains_key(*id))
        .expect("Duplicate label id while parsing!");
      let bad_jump = self.instructions.iter().find(|stmt| match stmt {
        Jump(id, _) | JumpIfZero(id, _) | JumpGreatherZero(id, _) => *id == *bad_label,
        _ => false,
      });
      return Err(InterpretError::LabelIsNotValid(
        bad_jump
          .expect("Found bad label id, but not where it used.")
          .get_line(),
      ));
    }

    self.labels = (0..labels_idx.len()).map(|id| labels_idx[&id]).collect();
    Ok(())
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
  pub fn decode_label(&self, label: Label) -> Option<usize> {
    self.labels.get(label).copied()
  }

  /// Injects an instruction at given index.
  #[inline]
  pub fn inject_instruction(&mut self, instruction: Stmt, index: usize) -> Result<(), InterpretError> {
    self.instructions.insert(index, instruction);
    self.init_labels()?;
    Ok(())
  }

  /// Removes an instruction at given index.
  #[inline]
  pub fn remove_instruction(&mut self, index: usize) -> Result<(), InterpretError> {
    self.instructions.remove(index);
    self.init_labels()?;
    Ok(())
  }

  /// Injects instructions at given index.
  #[inline]
  pub fn inject_instructions<T>(&mut self, instructions: T, index: usize) -> Result<(), InterpretError>
  where
    T: IntoIterator<Item = Stmt>,
  {
    let tail = self.instructions.split_off(index);
    self.instructions.extend(instructions.into_iter());
    self.instructions.extend(tail);
    self.init_labels()?;
    Ok(())
  }

  /// Removes instructions at given indexies.
  #[inline]
  pub fn remove_instructions<T>(&mut self, indexes: T) -> Result<(), InterpretError>
  where
    T: IntoIterator<Item = usize>,
  {
    // type Collector = HashSet<Stmt>;
    type Collector = Vec<Stmt>;
    let to_remove: Collector = indexes
      .into_iter()
      .filter_map(|i| self.instructions.get(i))
      .cloned()
      .collect();

    self.instructions.retain(|op| to_remove.contains(op));
    self.init_labels()?;
    Ok(())
  }
}
