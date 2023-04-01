pub mod parser;
pub mod program;
pub mod ram;
pub mod registers;
pub mod stmt;
pub mod errors;

#[cfg(feature = "wasm")]
pub mod wasm_bindings;


// TODO: Serde feature
