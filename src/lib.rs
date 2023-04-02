pub mod errors;
pub mod parser;
pub mod program;
pub mod ram;
pub mod registers;
pub mod stmt;

#[cfg(feature = "wasm")]
pub mod wasm_bindings;

// TODO: Serde feature
