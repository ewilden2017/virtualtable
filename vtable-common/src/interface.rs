//! Represent objects and methods for interacting with the client.
//!

use serde::{Deserialize, Serialize};

// Types of data.
#[derive(Serialize, Deserialize, Debug)]
pub enum DataType {
    Scene,
    Token,
}

// Types of messages.
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenUpdateMessage {
    pub token_id: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchMessage {
    pub data_type: DataType,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataResponseMessage {
    pub success: bool,
    pub data_type: DataType,
    pub data: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SocketMessageType {
    TokenUpdate(TokenUpdateMessage),
    Fetch(FetchMessage),
    Response(ResponseMessage),
    DataResponse(DataResponseMessage),
}

// TODO add timestamp.
#[derive(Serialize, Deserialize, Debug)]
pub struct SocketMessage {
    msg: SocketMessageType,
}

impl SocketMessage {
    pub fn token_update(id: u32, x: u32, y: u32) -> Self {
        SocketMessage {
            msg: SocketMessageType::TokenUpdate(TokenUpdateMessage { token_id: id, x, y }),
        }
    }

    pub fn fetch(data_type: DataType, id: u32) -> Self {
        SocketMessage {
            msg: SocketMessageType::Fetch(FetchMessage { data_type, id }),
        }
    }

    pub fn response(success: bool) -> Self {
        SocketMessage {
            msg: SocketMessageType::Response(ResponseMessage { success }),
        }
    }

    pub fn data_response(data_type: DataType, data: Option<&serde_json::Value>) -> SocketMessage {
        let resp = DataResponseMessage {
            success: data.is_some(),
            data_type,
            data: data.cloned(),
        };

        SocketMessage {
            msg: SocketMessageType::DataResponse(resp),
        }
    }

    pub fn get_content(&self) -> &SocketMessageType {
        &self.msg
    }
}
