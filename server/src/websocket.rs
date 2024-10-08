// src/websocket.rs

use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::bitboard::MoveData;
use crate::messages::{
    ClientMessage, ClientMove, CreateRoom, ErrorMessage, JoinQueue, JoinRoom, RoomJoined,
    UpdateClient,
};
use crate::server::Server;

pub struct MyWebSocket {
    pub hb: Instant,
    pub id: usize,
    pub server_addr: Addr<Server>,
    pub room_id: Option<Uuid>,
    pub color: Option<String>,
}

impl MyWebSocket {
    pub fn new(server_addr: Addr<Server>) -> Self {
        Self {
            hb: Instant::now(),
            id: 0,
            server_addr,
            room_id: None,
            color: None,
        }
    }

    pub fn send_message(&self, ctx: &mut ws::WebsocketContext<Self>, msg: serde_json::Value) {
        ctx.text(msg.to_string());
    }

    fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::new(5, 0), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::new(10, 0) {
                println!("WebSocket Client heartbeat failed, disconnecting!");
                act.server_addr.do_send(crate::messages::Disconnect { id: act.id });
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Register with the server
        let addr = ctx.address();
        self.server_addr
            .send(crate::messages::Connect { addr: addr.clone() })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(id) => {
                        act.id = id;
                        println!("WebSocket started with session id: {}", id);
                        act.start_heartbeat(ctx);
                    }
                    _ => {
                        println!("Failed to connect to server");
                        ctx.stop();
                    }
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.server_addr.do_send(crate::messages::Disconnect { id: self.id });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(client_msg) => {
                        match client_msg.r#type.as_str() {
                            "create_room" => {
                                self.server_addr.do_send(CreateRoom { id: self.id });
                            }
                            "join_room" => {
                                if let Some(room_id_str) = client_msg
                                    .data
                                    .get("room_id")
                                    .and_then(|v| v.as_str())
                                {
                                    if let Ok(room_id) = Uuid::parse_str(room_id_str) {
                                        self.server_addr.do_send(JoinRoom {
                                            id: self.id,
                                            room_id,
                                        });
                                    } else {
                                        let response = serde_json::json!({
                                            "type": "error",
                                            "data": "Invalid room ID format",
                                        });
                                        self.send_message(ctx, response);
                                    }
                                }
                            }
                            "join_queue" => {
                                self.server_addr.do_send(JoinQueue { id: self.id });
                            }
                            "move" => {
                                if let Some(room_id) = self.room_id {
                                    match serde_json::from_value::<MoveData>(
                                        client_msg.data.clone(),
                                    ) {
                                        Ok(move_data) => {
                                            self.server_addr.do_send(ClientMove {
                                                id: self.id,
                                                room_id,
                                                move_data,
                                            });
                                        }
                                        Err(err) => {
                                            println!("Error parsing move data: {:?}", err);
                                            let response = serde_json::json!({
                                                "type": "error",
                                                "data": "Invalid move data format",
                                            });
                                            self.send_message(ctx, response);
                                        }
                                    }
                                } else {
                                    let response = serde_json::json!({
                                        "type": "error",
                                        "data": "You are not in a room",
                                    });
                                    self.send_message(ctx, response);
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(err) => {
                        println!("Error parsing message: {:?}", err);
                        let response = serde_json::json!({
                            "type": "error",
                            "data": "Invalid message format",
                        });
                        self.send_message(ctx, response);
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                println!("WebSocket connection closed: {:?}", reason);
                self.server_addr.do_send(crate::messages::Disconnect { id: self.id });
                ctx.close(reason);
                ctx.stop();
            }
            Ok(_) => (),
            Err(e) => {
                println!("WebSocket error: {:?}", e);
                self.server_addr.do_send(crate::messages::Disconnect { id: self.id });
                ctx.stop();
            }
        }
    }
}

impl Handler<RoomJoined> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: RoomJoined, ctx: &mut Self::Context) {
        self.room_id = Some(msg.room_id);
        self.color = Some(msg.color.clone());

        let response = serde_json::json!({
            "type": "room_joined",
            "data": {
                "room_id": msg.room_id.to_string(),
                "color": msg.color,
            }
        });
        self.send_message(ctx, response);
    }
}

impl Handler<UpdateClient> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: UpdateClient, ctx: &mut Self::Context) {
        let response = serde_json::json!({
            "type": "update",
            "data": {
                "pieces": msg.pieces,
                "turn": msg.turn,
            }
        });
        self.send_message(ctx, response);
    }
}

impl Handler<ErrorMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: ErrorMessage, ctx: &mut Self::Context) {
        let response = serde_json::json!({
            "type": "error",
            "data": msg.error,
        });
        self.send_message(ctx, response);
    }
}
