use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Mutex;
use lazy_static::lazy_static;
use chess::Game;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            let mut game = GAME.lock().unwrap();
            match game.make_move(&text) {
                Ok(_) => socket.send(Message::Text(game.get_board())).await.unwrap(),
                Err(err) => socket.send(Message::Text(err)).await.unwrap(),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(websocket_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
