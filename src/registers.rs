//! The module provides an efficient way to manage registers in the
//! context of Random Access Machines (RAM).
//! The [`Registers`] struct has methods to get and set register values, allowing
//! for dynamic growth of the register set as needed.
//!
//! # Examples
//!
//! ```
//! use ramemu::registers::Registers;
//! use ramemu::registers::RegisterId;
//!
//! let mut registers = Registers::default();
//! registers.set(RegisterId(0), 42);
//! registers.set(RegisterId(1), 24);
//!
//! assert_eq!(registers.get(RegisterId(0)), 42);
//! assert_eq!(registers.get(RegisterId(1)), 24);
//! ```
//!
//! This module is typically used in combination with other components of an
//! assembly language interpreter or compiler.

use rustc_hash::FxHashMap as HashMap;
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
/// use ramemu::registers::RegisterId;
///
/// let mut registers = Registers::default();
/// registers.set(RegisterId(0), 42);
/// registers.set(RegisterId(1), 24);
///
/// assert_eq!(registers.get(RegisterId(0)), 42);
/// assert_eq!(registers.get(RegisterId(1)), 24);
/// ```
#[derive(Default, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Registers<T> {
    registers: HashMap<RegisterId, T>,
}

/// Represents a register id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegisterId(pub usize);

impl From<usize> for RegisterId {
    fn from(value: usize) -> Self {
        Self(value)
    }
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
    /// use ramemu::registers::RegisterId;
    ///
    /// let registers: Registers<u8> = Registers::default();
    /// assert_eq!(registers.get(RegisterId(4)), 0);
    /// ```
    #[inline]
    pub fn get(&self, index: impl Into<RegisterId>) -> T {
        self.registers
            .get(&index.into())
            .cloned()
            .unwrap_or_default()
    }
    /// Sets the value of the register at the given index.
    ///
    /// # Examples
    ///
    /// ```
    /// use ramemu::registers::Registers;
    /// use ramemu::registers::RegisterId;
    ///
    /// let mut registers = Registers::default();
    /// registers.set(RegisterId(0), 42);
    /// assert_eq!(registers.get(RegisterId(0)), 42);
    /// ```
    #[inline]
    pub fn set(&mut self, index: impl Into<RegisterId>, value: T) {
        self.registers.insert(index.into(), value);
    }
}

impl<T: Clone + Default + Debug> Debug for Registers<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_index = self.registers.keys().max().unwrap_or(&0.into()).0;
        let vec: Vec<T> = (0..=max_index)
            .map(|i| self.registers.get(&i.into()).cloned().unwrap_or_default())
            .collect();
        Debug::fmt(&vec, f)
    }
}

impl<T> FromIterator<T> for Registers<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            registers: iter
                .into_iter()
                .enumerate()
                .map(|(i, v)| (RegisterId(i), v))
                .collect(),
        }
    }
}

impl<T> FromIterator<(RegisterId, T)> for Registers<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = (RegisterId, T)>>(iter: I) -> Self {
        Self {
            registers: iter.into_iter().collect(),
        }
    }
}

impl<T, const N: usize> From<[T; N]> for Registers<T> {
    #[inline]
    fn from(value: [T; N]) -> Self {
        Self::from_iter(value)
    }
}

impl<T: Clone> From<&[T]> for Registers<T> {
    fn from(value: &[T]) -> Self {
        value.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Registers;

    #[test]
    fn test_set_and_get() {
        let mut registers: Registers<u32> = Registers::default();

        registers.set(0, 42);
        registers.set(1, 24);

        assert_eq!(registers.get(0), 42);
        assert_eq!(registers.get(1), 24);
    }

    #[test]
    fn test_get_default() {
        let registers: Registers<i32> = Registers::default();

        assert_eq!(registers.get(2), 0);
    }

    #[test]
    fn test_from_iter() {
        let input = vec![10, 20, 30];
        let registers: Registers<_> = input.into_iter().collect();

        assert_eq!(registers.get(0), 10);
        assert_eq!(registers.get(1), 20);
        assert_eq!(registers.get(2), 30);
    }

    #[test]
    fn test_from_iter_tuple() {
        let input = vec![(0.into(), 10), (1.into(), 20), (2.into(), 30)];
        let registers: Registers<i32> = input.into_iter().collect();

        assert_eq!(registers.get(0), 10);
        assert_eq!(registers.get(1), 20);
        assert_eq!(registers.get(2), 30);
    }

    #[test]
    fn test_from_array() {
        let input = [10, 20, 30];
        let registers: Registers<_> = input.into();

        assert_eq!(registers.get(0), 10);
        assert_eq!(registers.get(1), 20);
        assert_eq!(registers.get(2), 30);
    }

    #[test]
    fn test_from_slice() {
        let input: &[i32] = &[10, 20, 30];
        let registers: Registers<i32> = input.into();

        assert_eq!(registers.get(0), 10);
        assert_eq!(registers.get(1), 20);
        assert_eq!(registers.get(2), 30);
    }

    #[test]
    fn test_debug_fmt() {
        let mut registers: Registers<u32> = Registers::default();

        registers.set(0, 42);
        registers.set(1, 24);
        registers.set(2, 10);

        let debug_output = format!("{registers:?}");
        assert_eq!(debug_output, "[42, 24, 10]");
    }
}
