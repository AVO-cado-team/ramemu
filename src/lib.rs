use op::Op;
use parser::ParseError;
use program::Program;

pub mod op;
pub mod parser;
pub mod program;
pub mod ram;
pub mod registers;

pub fn create_program(source: &str) -> Result<Program, ParseError> {
  let stmts: Result<Vec<Op>, ParseError> = parser::parse(source).collect();
  let stmts = stmts?;

  Ok(Program::new(stmts))
}
