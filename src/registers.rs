use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Registers<T> {
  registers: RefCell<HashMap<usize, T>>,
}

impl<T: Clone + Default> Registers<T> {
  #[inline]
  pub(crate) fn get(&self, index: usize) -> T {
    let mut map = self.registers.borrow_mut();
    map.entry(index).or_default().clone()
  }
  #[inline]
  pub(crate) fn get_mut(&mut self, index: usize) -> &mut T {
    self.registers.get_mut().entry(index).or_default()
  }
  #[inline]
  pub fn first(&self) -> T {
    self.get(0)
  }
  #[inline]
  pub fn first_mut(&mut self) -> &mut T {
    self.get_mut(0)
  }
}

impl<T> FromIterator<T> for Registers<T> {
  #[inline]
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    Registers {
      registers: RefCell::new(iter.into_iter().enumerate().map(|(i, v)| (i, v)).collect()),
    }
  }
}

impl<T, const N: usize> From<[T; N]> for Registers<T> {
  #[inline]
  fn from(value: [T; N]) -> Self {
    Self::from_iter(value.into_iter())
  }
}

impl<T: Clone> From<&[T]> for Registers<T> {
  fn from(value: &[T]) -> Self {
    Self::from_iter(value.iter().cloned())
  }
}
