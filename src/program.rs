//! The [`Program`] module represents a program in the assembly language. It contains
//! the instructions and labels of the program, and provides methods for creating,
//! modifying, and querying the program structure.

use rustc_hash::FxHashMap as HashMap;

use crate::{
  errors::{InterpretError, ParseError},
  parser,
  stmt::{Label, Op::*, Stmt},
};

/// Represents a program code.
///
/// The [`Program`] struct contains the instructions and labels of a program,
/// and provides methods for creating, modifying, and querying the program
/// structure.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Program {
  /// Instructions of the program.
  pub instructions: Vec<Stmt>,
  /// Labels of the program.
  /// Label name -> Label id mapping.
  /// This is used for parsing dynamicly added commands.
  /// Should not have equal keys or values.
  pub label_ids: HashMap<String, usize>,
  /// Label id -> Code Address mapping.
  /// Should not have equal elements.
  pub labels: Vec<usize>,
}

impl Program {
  /// Creates a new [`Program`] from an iterable of [`Stmt`] and a precomputed label mapping.
  ///
  /// This method initializes the labels in the program using the provided `label_ids`.
  ///
  /// # Examples
  ///
  /// ```
  /// use ramemu::program::Program;
  /// use ramemu::stmt::Stmt;
  /// use ramemu::stmt::Op::*;
  /// use rustc_hash::FxHashMap as HashMap;
  ///
  /// let instructions = vec![
  ///     Stmt::new(Label(0), 1),
  ///     Stmt::new(Label(1), 2),
  /// ];
  ///
  /// let mut label_ids = HashMap::default();
  /// label_ids.insert("L0".to_string(), 0);
  /// label_ids.insert("L1".to_string(), 1);
  ///
  /// let program = Program::from_with_label_ids(instructions, label_ids);
  ///
  /// assert_eq!(program.label_ids.get("L0"), Some(&0));
  /// assert_eq!(program.label_ids.get("L1"), Some(&1));
  /// ```
  pub fn from_with_label_ids<T: IntoIterator<Item = Stmt>>(
    instructions: T,
    label_ids: HashMap<String, usize>,
  ) -> Program {
    let instructions: Vec<Stmt> = instructions.into_iter().collect();

    Program {
      instructions,
      label_ids,
      labels: Default::default(),
    }
  }
  /// Creates a new [`Program`] from an iterable of [`Stmt`].
  ///
  /// This method initializes the labels in the program. Labels are assigned a
  /// unique name based on their position in the iterable, using the format "L{index}",
  /// where "{index}" is the position of the label in the input iterator.
  ///
  /// # Examples
  ///
  /// ```
  /// use ramemu::program::Program;
  /// use ramemu::stmt::Stmt;
  /// use ramemu::stmt::Op::*;
  ///
  /// let instructions = vec![
  ///     Stmt::new(Label(0), 1),
  ///     Stmt::new(Label(1), 2),
  /// ];
  ///
  /// let program = Program::from(instructions).unwrap();
  ///
  /// assert_eq!(program.label_ids.get("L0"), Some(&0));
  /// assert_eq!(program.label_ids.get("L1"), Some(&1));
  /// ```
  pub fn from<T: IntoIterator<Item = Stmt>>(instructions: T) -> Result<Program, InterpretError> {
    let instructions: Vec<Stmt> = instructions.into_iter().collect();

    let label_ids: HashMap<String, usize> = instructions
      .iter()
      .filter_map(|stmt| match stmt.op {
        Label(id) => Some((format!("L{}", id), id)),
        _ => None,
      })
      .collect();

    let mut program = Program {
      instructions,
      label_ids,
      labels: Default::default(),
    };

    program.init_labels()?;

    Ok(program)
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
      if let Label(id) = stmt.op {
        if labels_idx.insert(id, pc).is_some() {
          return Err(InterpretError::LabelIsNotValid(stmt.line));
        }
      }
    }

    if labels_idx.len() != self.label_ids.len() {
      let bad_label = self
        .label_ids
        .values()
        .find(|id| !labels_idx.contains_key(*id))
        .expect("Duplicate label id in provided program!");
      let bad_jump = self.instructions.iter().find(|stmt| match stmt.op {
        Jump(id) | JumpIfZero(id) | JumpGreatherZero(id) => id == *bad_label,
        _ => false,
      });
      return Err(InterpretError::LabelIsNotValid(
        bad_jump
          .expect("Found bad label id, but not where it used.")
          .line,
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
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::stmt::{RegisterValue, Stmt, Value};

  fn get_test_program() -> Program {
    let instructions = vec![
      Stmt::new(Load(Value::Pure(42)), 1),
      Stmt::new(Label(0), 2),
      Stmt::new(Add(Value::Register(RegisterValue::Direct(0))), 3),
      Stmt::new(Sub(Value::Register(RegisterValue::Direct(1))), 4),
      Stmt::new(Jump(0), 5),
    ];
    let mut program = Program::from(instructions).unwrap();
    program.init_labels().unwrap();
    program
  }

  #[test]
  fn init_labels_test() {
    let program = get_test_program();
    assert_eq!(program.labels, vec![1]);
  }

  #[test]
  fn get_instruction_test() {
    let program = get_test_program();
    assert_eq!(program.get(0), Some(&Stmt::new(Load(Value::Pure(42)), 1)));
    assert_eq!(program.get(1), Some(&Stmt::new(Label(0), 2)));
    assert_eq!(program.get(6), None);
  }

  #[test]
  fn decode_label_test() {
    let program = get_test_program();
    assert_eq!(program.decode_label(0), Some(1));
    assert_eq!(program.decode_label(1), None);
  }
}
