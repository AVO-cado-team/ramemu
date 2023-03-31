use std::error::Error;

use ram::{
  create_program,
  // op::Op,
  // parser::{parse, ParseError},
  // program::Program,
  ram::{InterpretError, Ram},
};

const SOURCE: &str = include_str!("../program.txt");

fn main() -> Result<(), Box<dyn Error>> {
  let program = create_program(SOURCE)?;

  // let stmts: Result<Vec<Op>, ParseError> = parse(SOURCE).collect();
  // let stmts = stmts?;
  // println!("{:?}", stmts);
  // let program = Program::new(stmts);

  let mut ram = Ram::new(program);
  let result: Vec<_> = ram.collect();
  // let result: Result<(), InterpretError> = ram.try_for_each(|res| {
  //   let res = res?;
  //   let current_instruction = res.program.get(res.pc);
  //   let registers = res.registers.clone();

  //   println!("{:?}", current_instruction.clone());
  //   println!("{:?}", registers);

  //   Ok(())
  // });
  println!("{:?}", result.last());
  Ok(())
}
