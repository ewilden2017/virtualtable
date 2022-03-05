//! Represent each scene, a single continious area to display and move in.
//!

use wasm_bindgen::prelude::*;
use crate::token::*;
use js_sys::{Array, Function};

#[wasm_bindgen]
pub struct Scene {
    tokens: Vec<TokenReference>,
}

#[wasm_bindgen]
impl Scene {
    pub fn new() -> Scene {
        Scene { tokens: Vec::new() }
    }

    pub fn add_token(&mut self, token: &TokenReference) {
        self.tokens.push(token.clone());
    }

    // Find a token by world position. Returns a context object that can be
    // used from js to interface with the token, while preserving borrow checking.
    pub fn find_token(&self, x: u32, y: u32) -> Option<TokenReference> {
        for tok_ref in &self.tokens {
            let t = tok_ref.borrow();
            let rad_sq = t.radius * t.radius;

            // Compute squared distance to center
            let delta_x = x - t.x;
            let delta_y = y - t.y;
            let dist_sq = delta_x * delta_x + delta_y * delta_y;

            if dist_sq <= rad_sq {
                return Some(tok_ref.clone());
            };
        }
        None
    }

    pub fn get_tokens(&self) -> Array {
        // Make a clone of each reference.
        // TODO better way to do this without cloning everything?
        self.tokens.iter().map(|x| JsValue::from(x.clone())).collect()
    }

    /// Call a JS method on each token in the scene.
    pub fn each_token(&self, f: Function) {
        let this = JsValue::null();
        for tok_ref in &self.tokens {
            let tok_ref = JsValue::from(tok_ref.clone());
            let _ = f.call1(&this, &tok_ref);
        }
    }
}

