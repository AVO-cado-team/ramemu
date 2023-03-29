use std::collections::HashMap;

use crate::op::{Label, Op};

#[derive(Debug, Clone)]
pub struct Program {
  pub instructions: Vec<Op>,
  pub labels: HashMap<String, usize>,
}

impl Program {
  pub fn new(instructions: Vec<Op>) -> Self {
    let mut p = Program {
      instructions,
      labels: HashMap::new(),
    };
    p.init_labels();
    p
  }
  pub fn init_labels(&mut self) {
    self.labels.clear();
    for (index, op) in self.instructions.iter().enumerate() {
      if let Op::Label(label, _) = op {
        self.labels.insert(label.clone(), index);
      }
    }
  }
  pub fn get(&self, index: usize) -> Option<&Op> {
    self.instructions.get(index)
  }
  pub fn decode_label(&self, label: &Label) -> Option<usize> {
    self.labels.get(label.get()).copied()
  }
  pub fn inject_instructions(&mut self, instruction: Vec<Op>, index: usize) {
    let tail = self.instructions.split_off(index);
    self.instructions.extend(instruction);
    self.instructions.extend(tail);
    self.init_labels();
  }
  pub fn remove_instructions(&mut self, indexes: Vec<usize>) {
    let to_remove: Vec<Op> = self
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
