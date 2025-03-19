use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::sync::broadcast;

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let (tx, mut rx) = broadcast::channel::<String>(10);

    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            tx.send(text.clone()).unwrap();
            socket.send(Message::Text(format!("Received: {}", text))).await.unwrap();
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
