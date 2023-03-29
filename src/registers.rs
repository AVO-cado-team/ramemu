use std::collections::HashMap;

/// INVARIANT: Should have at least one element.

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Registers<T> {
  registers: HashMap<usize, T>,
}

impl<T: Clone + Default> Registers<T> {
  pub(crate) fn get_mut(&mut self, index: usize) -> &mut T {
    self.registers.entry(index).or_default()
  }
  pub fn first(&self) -> &T {
    self
      .registers
      .get(&0)
      .expect("Registers: first element should be initialized on creation. Invariant is broken.")
  }
  pub fn first_mut(&mut self) -> &mut T {
    self.registers.entry(0).or_default()
  }
}

impl<T: Default> From<Vec<T>> for Registers<T> {
  fn from(registers: Vec<T>) -> Self {
    let mut r = Registers {
      registers: registers.into_iter().enumerate().collect(),
    };
    if r.registers.is_empty() {
      r.registers.insert(0, T::default());
    }
    r
  }
}

impl<T: Default> Default for Registers<T> {
  fn default() -> Self {
    vec![].into()
  }
}
