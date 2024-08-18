use tree_sitter::Parser;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub mod wasm_libc;

#[wasm_bindgen]
pub fn foo() -> u64 {
    Parser::new().timeout_micros()
}
