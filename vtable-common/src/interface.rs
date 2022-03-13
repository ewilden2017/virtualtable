//! Represent objects and methods for interacting with the client.
//!

use crate::tabletop::Scene;
use serde::{Deserialize, Serialize};

// Types of messages.
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenUpdateMessage {
    pub token_id: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchSceneMessage {
    scene_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneResponseMessage {
    success: bool,
    scene: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SocketMessageType {
    TokenUpdate(TokenUpdateMessage),
    FetchScene(FetchSceneMessage),
    Response(ResponseMessage),
    SceneResponse(SceneResponseMessage),
}

// TODO add timestamp.
#[derive(Serialize, Deserialize, Debug)]
pub struct SocketMessage {
    msg: SocketMessageType,
}

impl SocketMessage {
    pub fn get_content(&self) -> &SocketMessageType {
        &self.msg
    }

    pub fn response(success: bool) -> SocketMessage {
        let resp = ResponseMessage { success };
        SocketMessage {
            msg: SocketMessageType::Response(resp),
        }
    }

    pub fn scene_response(scene: Option<&Scene>) -> SocketMessage {
        let resp = match scene {
            Some(scene) => SceneResponseMessage {
                success: true,
                scene: serde_json::to_value(scene).ok(),
            },

            None => SceneResponseMessage {
                success: false,
                scene: None,
            },
        };

        SocketMessage {
            msg: SocketMessageType::SceneResponse(resp),
        }
    }
}
