// src/main.rs

use actix::Actor;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::sync::Arc;

mod bitboard;
mod game_room;
mod messages;
mod server;
mod websocket;

use crate::server::Server;
use crate::websocket::MyWebSocket;

async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Arc<actix::Addr<Server>>>,
) -> Result<HttpResponse, Error> {
    let ws = MyWebSocket::new((**srv.get_ref()).clone());
    ws::start(ws, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = Server::new().start();
    let server_addr = Arc::new(server);

    println!("Starting WebSocket server at ws://127.0.0.1:8080/ws/");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_addr.clone()))
            .route("/ws/", web::get().to(ws_index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
