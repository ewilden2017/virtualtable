//! Representation and actions on a token on the table.
//!

use wasm_bindgen::prelude::*;

// For now, implement as circles. Later, change to be rectangle bounding box.
#[wasm_bindgen]
pub struct Token {
    pub radius: u32,
    pub style: u32,
}

#[wasm_bindgen]
impl Token {
    pub fn new(radius: u32, style: u32) -> Token {
        // Copy the style string.
        Token { radius, style }
    }
}
