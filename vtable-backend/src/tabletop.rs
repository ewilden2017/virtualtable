//! Represent elements on the client's game board.
//!

use serde::Serialize;
use std::collections::HashMap;

/// Store all objects and their metadata.
/// TODO replace with a database interface.
pub struct GameData {
    tokens: HashMap<u32, Token>,
    scenes: HashMap<u32, Scene>,
}

impl GameData {
    pub fn new() -> GameData {
        GameData {
            tokens: HashMap::new(),
            scenes: HashMap::new(),
        }
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.insert(token.id, token);
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.insert(scene.id, scene);
    }

    pub fn get_token(&mut self, id: u32) -> Option<&mut Token> {
        self.tokens.get_mut(&id)
    }

    pub fn get_scene(&mut self, id: u32) -> Option<&mut Scene> {
        self.scenes.get_mut(&id)
    }
}

/// Represent a token on the board.
#[derive(Debug)]
pub struct Token {
    id: u32,
}

impl Token {
    pub fn new(id: u32) -> Token {
        Token { id }
    }
}

/// Represent board-related token data.
#[derive(Debug, Serialize)]
pub struct SceneTokenData {
    pub x: u32,
    pub y: u32,
}

impl SceneTokenData {
    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

/// Represent a board, with tokens and background objects.
#[derive(Debug, Serialize)]
pub struct Scene {
    id: u32,
    name: String,
    tokens: HashMap<u32, SceneTokenData>,
}

impl Scene {
    pub fn new(id: u32, name: &str) -> Scene {
        Scene {
            id,
            name: String::from(name),
            tokens: HashMap::new(),
        }
    }

    pub fn add_token(&mut self, token_id: u32, x: u32, y: u32) {
        self.tokens.insert(token_id, SceneTokenData { x, y });
    }

    pub fn move_token(&mut self, token_id: u32, x: u32, y: u32) -> bool {
        match self.tokens.get_mut(&token_id) {
            Some(tokendata) => {
                tokendata.set_pos(x, y);
                true
            }
            None => false,
        }
    }
}
