//! The module provides an efficient way to manage registers in the
//! context of Random Access Machines (RAM).
//! The [`Registers`] struct has methods to get and set register values, allowing
//! for dynamic growth of the register set as needed.
//!
//! # Examples
//!
//! ```
//! use ramemu::registers::Registers;
//!
//! let mut registers = Registers::default();
//! registers.set(0, 42);
//! registers.set(1, 24);
//!
//! assert_eq!(registers.get(0), 42);
//! assert_eq!(registers.get(1), 24);
//! ```
//!
//! This module is typically used in combination with other components of an
//! assembly language interpreter or compiler.

use rustc_hash::FxHashMap as HashMap;
use std::cell::RefCell;
use std::fmt::Debug;
use std::iter::FromIterator;

/// Represents an infinite set of registers.
///
/// The `Registers` struct provides a convenient way to manage an infinite number
/// of registers. This allows for efficient access to the registers and dynamic
/// growth of the register set.
///
/// # Examples
///
/// ```
/// use ramemu::registers::Registers;
///
/// let mut registers = Registers::default();
/// registers.set(0, 42);
/// registers.set(1, 24);
///
/// assert_eq!(registers.get(0), 42);
/// assert_eq!(registers.get(1), 24);
/// ```
#[derive(Default, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Registers<T> {
  registers: RefCell<HashMap<usize, T>>,
}

impl<T: Clone + Default> Registers<T> {
  /// Returns the value of the register at the given index.
  ///
  /// If the register has not been set, the default value for the value type `T` is returned.
  ///
  /// # Examples
  ///
  /// ```
  /// use ramemu::registers::Registers;
  ///
  /// let registers: Registers<u8> = Registers::default();
  /// assert_eq!(registers.get(4), 0);
  /// ```
  #[inline]
  pub fn get(&self, index: usize) -> T {
    let value = {
      let mut map = self.registers.borrow_mut();
      let value = map.entry(index).or_insert_with(T::default);
      value.clone()
    };
    value
  }
  /// Sets the value of the register at the given index.
  ///
  /// # Examples
  ///
  /// ```
  /// use ramemu::registers::Registers;
  ///
  /// let mut registers = Registers::default();
  /// registers.set(0, 42);
  /// assert_eq!(registers.get(0), 42);
  /// ```
  #[inline]
  pub fn set(&mut self, index: usize, value: T) {
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
