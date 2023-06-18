use ramemu::program::Program;
use ramemu::ram::Ram;
use ramemu::stmt::{Op::*, Stmt, Value};
use std::io::BufReader;
use std::io::BufWriter;

fn main() {
    let program = Program::from(vec![
        Stmt::new(Load(Value::Pure(2)), 1),
        Stmt::new(Add(Value::Pure(2)), 3),
        Stmt::new(Output(Value::Pure(0)), 4),
        Stmt::new(Halt, 5),
    ])
    .unwrap();

    let reader = BufReader::new(std::io::empty());
    let writer = BufWriter::new(std::io::sink());
    let mut ram = Ram::new(program, Box::new(reader), Box::new(writer));

    ram.run().unwrap();
    assert_eq!(ram.get_registers().get(0), 4);
}
