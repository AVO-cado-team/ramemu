//! The [`Program`] module represents a program in the assembly language. It contains
//! the instructions and labels of the program, and provides methods for creating,
//! modifying, and querying the program structure.

use rustc_hash::FxHashMap as HashMap;

use crate::{
    errors::{InterpretError, ParseError, ParseErrorKind},
    parser,
    stmt::{
        Op::{self, *},
        Stmt,
    },
};

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
        CodeAddress(value)
    }
}

impl From<usize> for LabelId {
    fn from(value: usize) -> Self {
        LabelId(value)
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
    /// use ramemu::program::Program;
    /// use ramemu::program::{LabelId, CodeAddress};
    /// use ramemu::stmt::Stmt;
    /// use ramemu::stmt::Op::*;
    ///
    /// let instructions = vec![
    ///     Stmt::new(Label(LabelId(0)), 1),
    ///     Stmt::new(Label(LabelId(1)), 2),
    /// ];
    ///
    /// let program = Program::from(instructions).unwrap();
    ///
    /// assert_eq!(program.labels.get(&LabelId(0)), Some(&CodeAddress(0)));
    /// assert_eq!(program.labels.get(&LabelId(1)), Some(&CodeAddress(1)));
    /// ```
    pub fn from<T>(instructions: T) -> Result<Program, InterpretError>
    where
        T: IntoIterator<Item = Stmt>,
    {
        let instructions: Vec<Stmt> = instructions.into_iter().collect();

        // TODO: Panic if duplicate label ids.
        let labels: HashMap<LabelId, CodeAddress> = instructions
            .iter()
            .enumerate()
            .map(|(i, stmt)| (CodeAddress(i), stmt))
            .filter_map(|(addr, stmt)| match stmt.op {
                Op::Label(id) => Some((id, addr)),
                _ => None,
            })
            .collect();

        Ok(Program {
            instructions,
            labels,
        })
    }

    /// Creates a new [`Program`] from the source code.
    ///
    /// This method parses the source code, creating a [`Program`] with the resulting
    /// instructions and labels.
    pub fn from_source(source: &str) -> Result<Program, ParseError> {
        let stmts: Result<Vec<Stmt>, ParseError> = parser::parse(source).collect();
        let instructions = stmts?;
        let mut labels = HashMap::default();

        // add labels to the label map
        for (pc, stmt) in instructions.iter().enumerate() {
            if let Op::Label(id) = stmt.op {
                if labels.insert(id, CodeAddress(pc)).is_some() {
                    return Err(ParseError {
                        kind: ParseErrorKind::LabelIsNotValid,
                        line: stmt.line,
                    });
                }
            }
        }

        //  check if the jump labels are valid
        for stmt in instructions.iter() {
            if let Jump(id) | JumpIfZero(id) | JumpGreatherZero(id) = stmt.op {
                if !labels.contains_key(&id) {
                    return Err(ParseError {
                        kind: ParseErrorKind::LabelIsNotValid,
                        line: stmt.line,
                    });
                }
            }
        }

        Ok(Program {
            instructions,
            labels,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stmt::{RegisterValue, Stmt, Value};

    fn get_test_program() -> Program {
        let instructions = vec![
            Stmt::new(Load(Value::Pure(42)), 1),
            Stmt::new(Op::Label(0.into()), 2),
            Stmt::new(Add(Value::Register(RegisterValue::Direct(0))), 3),
            Stmt::new(Sub(Value::Register(RegisterValue::Direct(1))), 4),
            Stmt::new(Jump(0.into()), 5),
        ];
        Program::from(instructions).unwrap()
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
        assert_eq!(program.get(1), Some(&Stmt::new(Op::Label(0.into()), 2)));
        assert_eq!(program.get(6), None);
    }

    #[test]
    fn decode_label_test() {
        let program = get_test_program();
        assert_eq!(program.decode_label(0), Some(CodeAddress(1)));
        assert_eq!(program.decode_label(1), None);
    }
}
