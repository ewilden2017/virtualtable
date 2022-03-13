//! Represent objects and methods for interacting with the client.
//!

use crate::tabletop::Scene;
pub use vtable_common::interface::*;

pub fn scene_response(scene: Option<&Scene>) -> SocketMessage {
    let data = scene.and_then(|scene| serde_json::to_value(scene).ok());
    SocketMessage::data_response(DataType::Scene, data.as_ref())
}
