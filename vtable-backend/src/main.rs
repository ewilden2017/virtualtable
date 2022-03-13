use async_std::prelude::*;
use async_std::sync::Mutex;
use lazy_static::lazy_static;
use tide::Request;
use tide_websockets::{Message, WebSocket, WebSocketConnection};

mod config;
use config::TomlConfig;

use vtable_common::interface::{SocketMessage, SocketMessageType};
use vtable_common::tabletop::*;

const CONFIG_FILE: &str = "vtable.toml";

lazy_static! {
// Global for storing all game data. Will be replaced with database.
static ref GAME_DATA: Mutex<GameData> = Mutex::new(GameData::new());
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let config = match TomlConfig::from_file(CONFIG_FILE) {
        Ok(x) => x,
        Err(e) => {
            println!("Warning: {}", e);
            TomlConfig::empty()
        }
    };

    let config = config.parse();

    // Set up game stuff.
    let mut game_data = GAME_DATA.lock().await;
    game_data.add_token(Token::new(0));
    game_data.add_scene(Scene::new(0, "Main Scene"));
    let test_scene = game_data.get_scene(0).unwrap();
    test_scene.add_token(0, 0, 0);

    // Make sure the mutex is released.
    drop(game_data);

    // Start HTTP server.
    println!("Server starting.");
    let mut app = tide::new();

    app.at("/session")
        .with(WebSocket::new(handle_socket))
        .get(|_req: Request<()>| async move {
            println!("Not a websocket request.");
            Ok("Not a websocket request\n")
        });

    app.listen(format!("{}:{}", config.bind_ip, config.port))
        .await?;

    Ok(())
}

async fn handle_socket(_req: Request<()>, mut stream: WebSocketConnection) -> tide::Result<()> {
    println!("Opening socket");
    while let Some(Ok(Message::Text(input))) = stream.next().await {
        println!("recieved: {}", input);
        let msg: SocketMessage = serde_json::from_str(&input)?;

        // I believe this keeps the mutex locked until test_scene goes out of scope.
        let mut game_data = GAME_DATA.lock().await;
        let test_scene = game_data.get_scene(0).unwrap();

        let resp = match msg.get_content() {
            SocketMessageType::TokenUpdate(data) => {
                if test_scene.move_token(data.token_id, data.x, data.y) {
                    SocketMessage::response(true)
                } else {
                    SocketMessage::response(false)
                }
            }

            SocketMessageType::FetchScene(_data) => {
                // TODO lookup scene by id
                let scene = &test_scene;

                SocketMessage::scene_response(Some(scene))
            }

            // Send false response on invalid data.
            _ => SocketMessage::response(false),
        };

        stream.send_json(&resp).await?;
    }

    Ok(())
}
