use ram::{program::Program, ram::Ram};

fn main() {
  let source = r#"
      # Your RAM assembly code here 
      HALT
    "#;

  let program = Program::from_source(source).unwrap();
  let mut ram = Ram::new(program);

  match ram.run() {
    Ok(_) => println!("Program executed successfully"),
    Err(e) => println!("Error during execution: {:?}", e),
  }
}

