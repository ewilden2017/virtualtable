//! Represent each scene, a single continious area to display and move in.
//!

use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Deserialize)]
pub struct SceneTokenData {
    pub x: u32,
    pub y: u32,
}

#[wasm_bindgen]
#[derive(Deserialize)]
pub struct Scene {
    id: u32,
    name: String,
    tokens: HashMap<u32, SceneTokenData>,
}

#[wasm_bindgen]
impl Scene {
    pub fn from_json(s: &str) -> Option<Scene> {
        serde_json::from_str(s).ok()
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }
}
