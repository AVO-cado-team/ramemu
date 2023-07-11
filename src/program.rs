//! The [`Program`] module represents a program in the assembly language. It contains
//! the instructions and labels of the program, and provides methods for creating,
//! modifying, and querying the program structure.

use rustc_hash::FxHashMap as HashMap;

use crate::{errors::ParseError, parser::parse, stmt::Stmt};

/// Represents a label id.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LabelId(pub usize);

/// Represents a code address.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CodeAddress(pub usize);

impl std::ops::Add<usize> for CodeAddress {
    type Output = Self;
    fn add(self, other: usize) -> Self::Output {
        Self(self.0 + other)
    }
}

impl From<usize> for CodeAddress {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<usize> for LabelId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

/// Represents a program code.
///
/// The [`Program`] struct contains the instructions and labels of a program,
/// and provides methods for creating, modifying, and querying the program
/// structure.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Program {
    /// Instructions of the program.
    pub instructions: Vec<Stmt>,
    /// Label id -> Code Address mapping.
    /// Should not have equal elements.
    pub labels: HashMap<LabelId, CodeAddress>,
}

impl Program {
    /// Creates a new [`Program`] from an iterable of [`Stmt`].
    ///
    /// This method initializes the labels in the program. Labels are assigned a
    /// unique name based on their position in the iterable, using the format "L{index}",
    /// where "{index}" is the position of the label in the input iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use ramemu::ram::Ram;
    /// use ramemu::program::Program;
    /// use ramemu::stmt::{Op::*, Stmt, Value};
    /// use std::io::BufReader;
    /// use std::io::BufWriter;

    /// let instructions = vec![
    ///     Stmt::new(Load(Value::Pure(2)), 1),
    ///     Stmt::new(Add(Value::Pure(2)), 3),
    ///     Stmt::new(Output(Value::Pure(0)), 4),
    ///     Stmt::new(Halt, 5),
    /// ];
    /// let labels = Default::default();
    /// let program = Program::from(instructions, labels);
    /// ```
    ///
    /// # Errors
    ///
    pub fn from<T>(instructions: T, labels: HashMap<LabelId, CodeAddress>) -> Self
    where
        T: IntoIterator<Item = Stmt>,
    {
        Self {
            instructions: instructions.into_iter().collect(),
            labels,
        }
    }

    /// Creates a new [`Program`] from the source code.
    ///
    /// This method parses the source code, creating a [`Program`] with the resulting
    /// instructions and labels.
    /// # Errors
    /// If the source code is invalid, returns a [`ParseError`].
    #[allow(clippy::missing_panics_doc)]
    pub fn from_source(source: &str) -> Result<Self, ParseError> {
        parse(source).map_err(|e| {
            e.into_iter()
                .next()
                .expect("Allways has at least one error.")
        })
    }

    /// Returns the instruction at the given index.
    ///
    /// If the index is out of bounds, returns `None`.
    #[inline]
    pub fn get(&self, index: impl Into<CodeAddress>) -> Option<&Stmt> {
        self.instructions.get(index.into().0)
    }

    /// Decodes the label into the instruction index.
    ///
    /// If the label is not found, returns `None`.
    #[inline]
    pub fn decode_label(&self, label: impl Into<LabelId>) -> Option<CodeAddress> {
        self.labels.get(&label.into()).copied()
    }
}

impl AsRef<Self> for Program {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for Program {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stmt::{Op::*, RegisterValue, Stmt, Value};

    fn get_test_program() -> Program {
        let instructions = vec![
            Stmt::new(Load(Value::Pure(42)), 1),
            Stmt::new(Add(Value::Register(RegisterValue::Direct(0))), 2),
            Stmt::new(Sub(Value::Register(RegisterValue::Direct(1))), 3),
            Stmt::new(Jump(LabelId(0)), 4),
        ];
        let labels = [(LabelId(0), CodeAddress(1))].into_iter().collect();
        Program::from(instructions, labels)
    }

    #[test]
    fn init_labels_test() {
        let program = get_test_program();
        assert_eq!(program.labels.get(&LabelId(0)), Some(&CodeAddress(1)));
    }

    #[test]
    fn get_instruction_test() {
        let program = get_test_program();
        assert_eq!(program.get(0), Some(&Stmt::new(Load(Value::Pure(42)), 1)));
        assert_eq!(program.get(3), Some(&Stmt::new(Jump(LabelId(0)), 4)));
        assert_eq!(program.get(6), None);
    }

    #[test]
    fn decode_label_test() {
        let program = get_test_program();
        assert_eq!(program.decode_label(0), Some(CodeAddress(1)));
        assert_eq!(program.decode_label(1), None);
    }
}
