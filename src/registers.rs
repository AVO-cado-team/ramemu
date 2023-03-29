#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Registers<T> {
  registers: Vec<T>,
}

impl<T: Clone + Default> Registers<T> {
  pub(crate) fn get_mut(&mut self, index: usize) -> &mut T {
    if index >= self.registers.len() {
      self.registers.resize(index + 1, T::default());
    }
    &mut self.registers[index]
  }
  pub fn first(&self) -> &T {
    &self.registers[0]
  }
  pub fn first_mut(&mut self) -> &mut T {
    &mut self.registers[0]
  }
}

impl<T: Default> From<Vec<T>> for Registers<T> {
  fn from(registers: Vec<T>) -> Self {
    let mut r = Registers { registers };
    if r.registers.is_empty() {
      r.registers.push(T::default());
    }
    r
  }
}
