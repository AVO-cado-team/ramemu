use std::collections::HashMap;

use crate::{
  errors::ParseError,
  parser,
  stmt::{Label, Stmt},
};

#[derive(Default, Debug, Clone)]
pub struct Program {
  pub instructions: Vec<Stmt>,
  pub labels: HashMap<String, usize>,
}

impl Program {
  pub fn new(instructions: Vec<Stmt>) -> Self {
    let mut p = Program {
      instructions,
      labels: HashMap::new(),
    };
    p.init_labels();
    p
  }
  pub fn from_source(source: &str) -> Result<Program, ParseError> {
    let stmts: Result<Vec<Stmt>, ParseError> = parser::parse(source).collect();
    let stmts = stmts?;

    Ok(Program::new(stmts))
  }
  #[inline]
  pub fn init_labels(&mut self) {
    self.labels.clear();
    for (index, op) in self.instructions.iter().enumerate() {
      if let Stmt::Label(label, _) = op {
        self.labels.insert(label.clone(), index);
      }
    }
  }
  #[inline]
  pub fn get(&self, index: usize) -> Option<&Stmt> {
    self.instructions.get(index)
  }
  #[inline]
  pub fn decode_label(&self, label: &Label) -> Option<usize> {
    self.labels.get(label.get()).copied()
  }
  #[inline]
  pub fn inject_instruction(&mut self, instruction: Stmt, index: usize) {
    self.instructions.insert(index, instruction);
    self.init_labels();
  }
  #[inline]
  pub fn remove_instruction(&mut self, index: usize) {
    self.instructions.remove(index);
    self.init_labels();
  }
  #[inline]
  pub fn inject_instructions(&mut self, instruction: Box<[Stmt]>, index: usize) {
    let tail = self.instructions.split_off(index);
    self.instructions.extend(instruction.into_vec().into_iter());
    self.instructions.extend(tail);
    self.init_labels();
  }
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
