use std::io::{stdin, stdout, BufReader};

use ram::{program::Program, ram::Ram};

fn main() {
  let source = r#"
      # Your RAM assembly code here 
      HALT
    "#;

  let program = Program::from_source(source).unwrap();
  let mut ram = Ram::new(
    program,
    Box::new(BufReader::new(stdin())),
    Box::new(stdout()),
  );

  match ram.run() {
    Ok(_) => println!("Program executed successfully"),
    Err(e) => println!("Error during execution: {:?}", e),
  }
}
