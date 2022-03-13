//! Representation and actions on a token on the table.
//!

use core::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

// For now, implement as circles. Later, change to be rectangle bounding box.
#[wasm_bindgen]
pub struct Token {
    pub x: u32,
    pub y: u32,
    pub radius: u32,
    pub style: u32,
}

#[wasm_bindgen]
impl Token {
    pub fn new(x: u32, y: u32, radius: u32, style: u32) -> Token {
        // Copy the style string.
        Token {
            x,
            y,
            radius,
            style,
        }
    }

    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}
