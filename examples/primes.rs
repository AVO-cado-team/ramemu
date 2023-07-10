use std::io::{stdin, stdout, BufReader};

use ramemu::{program::Program, ram::Ram};

const SOURCE: &str = r#"
load =0

read  0
store 1 # nmber


# IF 1 or 2 or 3
sub =1
jz quit_1
load 1
sub =2
jz quit_1
load 1
sub =3
jz quit_1
#

load 1
div  =2
store 2 # num / 2
load =2
store 3 # num to div


load 1
div =2
mul =2
sub 1
jz quit_2

loop_1:
 load  1
 div   3
 mul   3
 sub   1
 jz quit_2
 load  3
 sub   2
 jz quit_1
 load  3
 add  =1
 store 3
 jmp loop_1



quit_1:
write =1
jmp quit

quit_2:
write =0
jmp quit

quit:

halt
"#;

fn main() {
    let program = Program::from_source(SOURCE).expect("Program is correct.");

    let mut ram = Ram::new(
        program,
        Box::new(BufReader::new(stdin())),
        Box::new(stdout()),
    );

    match ram.run() {
        Ok(_) => println!("\nProgram executed successfully"),
        Err(e) => println!("\nError during execution: {e:?}"),
    }
}
