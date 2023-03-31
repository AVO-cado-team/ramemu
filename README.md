# RAM Emulator

![Version 0.1.0](https://img.shields.io/badge/version-0.1.0-blue.svg)
[![License](https://img.shields.io/badge/license-GNU3-blue.svg)](./LICENSE)
[![Ddystopia](https://img.shields.io/badge/Author-Ddystopia-blue.svg?style=flat)](mailto:alexanderbabak@proton.me)
[![Ddystopia](https://img.shields.io/badge/Github-Ddystopia-green.svg?style=flat)](https://github.com/Ddystopia/)

A Rust-based library for emulating a Random Access Machine (RAM). This library
provides parsing, interpretation, and execution of RAM assembly code, as well as
support for mathematical operations, labels, jumps, and I/O operations.

## Features

- Parsing of RAM assembly code
- Mathematical operations: `ADD`, `SUB`, `MUL`, `DIV`
- Labels and jumps: `JUMP`, `JMP`, `JZ`, `JZERO`, `JGZ`, `JGTZ`
- I/O operations: `LOAD`, `STORE`, `INPUT`, `READ`, `WRITE`, `OUTPUT`
- Error handling and reporting: `ParseError`, `InterpretError`

## Installation and Usage

Add the library as a dependency to your Rust project by including the following
in your `Cargo.toml` file:

```toml
ram = { git = "https://github.com/AVO-cado-team/RamEmu.git", tag = "0.1.0"}
```

## Examples

Here's an example of how to use the library to create a RAM program and run it:

```rust

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

```

## Supported Syntax

The parser supports the following syntax:
- Comments: Start with `#`
- Labels: End with `:`
- Links: Types include explicit (`int`), without link (`=int`), and double link (`*int`)

## Limitations and Future Improvements

This library is a work in progress and may have limitations. Future improvements
may include better error handling, performance optimizations, and additional
features.

## Contributing

Contributions to the project are welcome. You can report bugs, request features,
or submit pull requests. Before submitting a pull request, make sure your
changes are well-tested and documented.

