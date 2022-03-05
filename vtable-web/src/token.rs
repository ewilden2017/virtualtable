//! Representation and actions on a token on the table.
//!

use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::ops::Deref;
use core::cell::RefCell;


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
    pub fn new(x: u32, y:u32, radius: u32, style: u32) -> Token {
        // Copy the style string.
        Token { x, y, radius, style }
    }

    pub fn set_pos(&mut self, x: u32, y:u32) {
        self.x = x;
        self.y = y;
    }

}

// Reference wrapper for storing a token reference to interact with javascript.
#[wasm_bindgen]
pub struct TokenReference {
    token: Rc<RefCell<Token>>,
}

// Can't be created from reference by javascript.
impl TokenReference {
    pub fn from_ref(token: Rc<RefCell<Token>>) -> TokenReference {
        TokenReference { token }
    }
}

impl Deref for TokenReference {
    type Target = Rc<RefCell<Token>>;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

impl Clone for TokenReference {
    fn clone(&self) -> TokenReference {
        TokenReference { token: self.token.clone() }
    }
}

#[wasm_bindgen]
impl TokenReference {
    // Create a TokenReference with a new Token.
    pub fn new(x: u32, y:u32, radius: u32, style: u32) -> TokenReference {
        let token = Token::new(x, y, radius, style);
        let token = Rc::new(RefCell::new(token));
        TokenReference { token }
    }

    pub fn get_x(&self) -> u32 {
        self.token.borrow().x
    }

    pub fn get_y(&self) -> u32 {
        self.token.borrow().y
    }

    pub fn get_radius(&self) -> u32 {
        self.token.borrow().radius
    }

    pub fn get_style(&self) -> u32 {
        self.token.borrow().style
    }

    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.token.borrow_mut().set_pos(x, y);
    }
}
