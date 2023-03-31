use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Default, Clone, PartialEq, Eq)]
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
  pub(crate) fn set(&mut self, index: usize, value: T) {
    self.registers.get_mut().insert(index, value);
  }
}

impl<T: Clone + Default + Debug> Debug for Registers<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let max_index = *self.registers.borrow().keys().max().unwrap_or(&0);
    let vec: Vec<T> = (0..=max_index)
      .map(|i| self.registers.borrow().get(&i).cloned().unwrap_or_default())
      .collect();
    Debug::fmt(&vec, f)
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

impl<T> FromIterator<(usize, T)> for Registers<T> {
  #[inline]
  fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
    Registers {
      registers: RefCell::new(iter.into_iter().collect()),
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
