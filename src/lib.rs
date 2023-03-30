use stmt::Stmt;
use parser::ParseError;
use program::Program;

pub mod stmt;
pub mod parser;
pub mod program;
pub mod ram;
pub mod registers;

pub fn create_program(source: &str) -> Result<Program, ParseError> {
  let stmts: Result<Vec<Stmt>, ParseError> = parser::parse(source).collect();
  let stmts = stmts?;

  Ok(Program::new(stmts))
}
