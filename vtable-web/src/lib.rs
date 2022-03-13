mod interface;
mod scene;
mod token;
use interface::{SocketMessage, SocketMessageType};
use js_sys::Function;
use scene::Scene;
use std::collections::HashMap;
use token::Token;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Error {
    data: String,
}

#[wasm_bindgen]
impl Error {
    pub fn message(&self) -> String {
        self.data.to_owned()
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error {
            data: error.to_string().to_owned(),
        }
    }
}

#[wasm_bindgen]
pub struct State {
    current_scene: Option<Scene>,
    token_cache: HashMap<u32, Token>,
    send_func: Option<Function>,
}

impl State {
    fn send_msg(&self, msg: &str) {
        if let Some(f) = &self.send_func {
            let this = JsValue::null();
            let msg = JsValue::from(msg);
            let _ = f.call1(&this, &msg);
        }
    }
}

#[wasm_bindgen]
impl State {
    pub fn new() -> State {
        State {
            current_scene: None,
            token_cache: HashMap::new(),
            send_func: None,
        }
    }

    pub fn get_token(&self, id: u32) -> Option<Token> {
        // Cheat for now, eventually need to fetch from server.
        Some(Token::new(20, 0))
    }

    pub fn token_at_pos(&self, x: u32, y: u32) -> Option<u32> {
        let current_scene = self.current_scene.as_ref()?;

        for (&id, pos) in current_scene.tokens().iter() {
            let token = self.get_token(id);
            if let Some(t) = token {
                let rad_sq = t.radius * t.radius;

                // Compute squared distance to center
                let delta_x = x - pos.x;
                let delta_y = y - pos.y;
                let dist_sq = delta_x * delta_x + delta_y * delta_y;

                if dist_sq <= rad_sq {
                    return Some(id);
                };
            }
        }
        None
    }

    pub fn move_token(&mut self, id: u32, x: u32, y: u32) {
        if let Some(scene) = self.current_scene.as_mut() {
            scene.move_token(id, x, y);

            // Update server.
            let msg = SocketMessage::update_msg(id, x, y);
            let msg = serde_json::to_string(&msg).unwrap_or(String::from("{error=true}"));
            self.send_msg(&msg);
        }
    }

    pub fn each_token(&mut self, f: Function) {
        let this = JsValue::null();
        if let Some(current_scene) = &self.current_scene {
            for (id, data) in current_scene.tokens().iter() {
                let token = self.get_token(*id);
                if let Some(token) = token {
                    let token = JsValue::from(token);
                    let x = JsValue::from(data.x);
                    let y = JsValue::from(data.y);
                    let _ = f.call3(&this, &token, &x, &y);
                }
            }
        }
    }

    pub fn check_ready(&self) -> bool {
        self.current_scene.is_some()
    }

    pub fn set_send(&mut self, f: Function) {
        self.send_func = Some(f);
    }

    pub fn handle_message(&mut self, data: &str) -> Result<(), Error> {
        let msg: SocketMessage = serde_json::from_str(data)?;

        match msg.get_content() {
            SocketMessageType::Response(_resp) => {
                // TODO handle based on what is pending?
            }

            SocketMessageType::SceneResponse(resp) => {
                // For now, can only mean that the Scene changed.
                if let Some(scene) = resp.scene.clone() {
                    let scene = serde_json::from_value(scene)?;
                    self.current_scene = Some(scene);
                }
                // TODO error handling.
            }

            _ => (),
        };

        Ok(())
    }
}
