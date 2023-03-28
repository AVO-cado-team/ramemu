use crate::op::Op;


struct Ram {
  program: Vec<Op>,
  registers: Vec<i32>,
  pc: usize,
  halt: bool,
}

impl Ram {
  fn new(program: Vec<Op>) -> Self {
    Ram {
      program,
      registers: vec![0; 10],
      pc: 0,
      halt: false,
    }
  }
  fn run(&mut self) {
    while !self.halt {
      self.step();
    }
  }
  fn step(&mut self) {
    if self.halt || self.pc >= self.program.len() {
      return;
    }

    match self.program[self.pc] {
      Op::Load(r) => self.registers[0] = self.registers[r],
      Op::Store(r) => self.registers[r] = self.registers[0],
      Op::Add(r) => self.registers[0] += self.registers[r],
      Op::Sub(r) => self.registers[0] -= self.registers[r],
      Op::Mult(r) => self.registers[0] *= self.registers[r],
      Op::Div(r) => self.registers[0] /= self.registers[r],
      Op::Halt => self.halt = true,
      Op::Jump(addr) => {
        self.pc = addr;
        return;
      }
      Op::JumpIfZero(addr) => {
        if self.registers[0] == 0 {
          self.pc = addr;
          return;
        }
      }
      Op::JumpIfNeg(addr) => {
        if self.registers[0] < 0 {
          self.pc = addr;
          return;
        }
      }
      Op::Input(r) => {
        let mut input = String::new();
        std::io::stdin()
          .read_line(&mut input)
          .expect("Failed to read input");
        self.registers[r] = input.trim().parse().expect("Invalid input");
      }
      Op::Output(r) => {
        println!("Output: {}", self.registers[r]);
      }
    }

    self.pc += 1;
  }
}
