use ram::{create_program, ram::Ram};

fn main() {
  let source = r#"
      # Your RAM assembly code here 
    "#;

  let program = create_program(source).unwrap();
  let mut ram = Ram::new(program);

  match ram.run() {
    Ok(_) => println!("Program executed successfully"),
    Err(e) => println!("Error during execution: {:?}", e),
  }
}
