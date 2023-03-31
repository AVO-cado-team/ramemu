use parser::ParseError;
use program::Program;
use stmt::Stmt;

pub mod parser;
pub mod program;
pub mod ram;
pub mod registers;
pub mod stmt;

// TODO: Serde feature

pub fn create_program(source: &str) -> Result<Program, ParseError> {
  let stmts: Result<Vec<Stmt>, ParseError> = parser::parse(source).collect();
  let stmts = stmts?;

  Ok(Program::new(stmts))
}
