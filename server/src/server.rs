// src/server.rs

use actix::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::bitboard::init_bitboard;
use crate::game_room::GameRoom;
use crate::messages::*;
use crate::websocket::MyWebSocket;

pub struct Server {
    pub sessions: HashMap<usize, Addr<MyWebSocket>>,
    pub rooms: HashMap<Uuid, GameRoom>,
    pub waiting_players: Vec<(usize, Addr<MyWebSocket>)>,
    pub session_id_counter: usize,
}

impl Server {
    pub fn new() -> Self {
        Server {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            waiting_players: Vec::new(),
            session_id_counter: 0,
        }
    }

    fn generate_session_id(&mut self) -> usize {
        self.session_id_counter += 1;
        self.session_id_counter
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.generate_session_id();
        self.sessions.insert(id, msg.addr);
        println!("Client connected with session id: {}", id);
        id
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
        println!("Client disconnected with session id: {}", msg.id);
    }
}

impl Handler<CreateRoom> for Server {
    type Result = ();

    fn handle(&mut self, msg: CreateRoom, _: &mut Context<Self>) {
        let room_id = Uuid::new_v4();
        let bitboard = init_bitboard();
        let mut room = GameRoom::new(room_id, bitboard);

        if let Some(addr) = self.sessions.get(&msg.id) {
            room.add_player(msg.id, addr.clone());
        }

        self.rooms.insert(room_id, room);

        println!("Room created with id: {}", room_id);
    }
}

impl Handler<JoinRoom> for Server {
    type Result = ();

    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) {
        if let Some(room) = self.rooms.get_mut(&msg.room_id) {
            if room.players.len() < 2 {
                if let Some(addr) = self.sessions.get(&msg.id) {
                    room.add_player(msg.id, addr.clone());
                    println!("Client {} joined room {}", msg.id, msg.room_id);
                }
            } else {
                // Room is full
                if let Some(addr) = self.sessions.get(&msg.id) {
                    addr.do_send(ErrorMessage {
                        error: "Room is full".to_string(),
                    });
                }
            }
        } else {
            // Room not found
            if let Some(addr) = self.sessions.get(&msg.id) {
                addr.do_send(ErrorMessage {
                    error: "Room not found".to_string(),
                });
            }
        }
    }
}

impl Handler<JoinQueue> for Server {
    type Result = ();

    fn handle(&mut self, msg: JoinQueue, _: &mut Context<Self>) {
        if let Some(addr) = self.sessions.get(&msg.id) {
            self.waiting_players.push((msg.id, addr.clone()));
            println!("Client {} joined the queue", msg.id);

            if self.waiting_players.len() >= 2 {
                let (id1, player1) = self.waiting_players.remove(0);
                let (id2, player2) = self.waiting_players.remove(0);

                let room_id = Uuid::new_v4();
                let bitboard = init_bitboard();
                let mut room = GameRoom::new(room_id, bitboard);

                room.add_player(id1, player1.clone());
                room.add_player(id2, player2.clone());

                self.rooms.insert(room_id, room);

                println!("Room {} created with players {} and {}", room_id, id1, id2);
            }
        }
    }
}

impl Handler<ClientMove> for Server {
    type Result = ();

    fn handle(&mut self, msg: ClientMove, _: &mut Context<Self>) {
        if let Some(room) = self.rooms.get_mut(&msg.room_id) {
            let player_color = room.get_player_color(msg.id);

            if let Some(color) = player_color {
                if color == room.turn {
                    let valid_move = room.apply_move(&msg.move_data);

                    if valid_move {
                        room.switch_turn();
                        room.broadcast_update();
                    } else {
                        // Invalid move
                        if let Some(addr) = self.sessions.get(&msg.id) {
                            addr.do_send(ErrorMessage {
                                error: "Invalid move".to_string(),
                            });
                        }
                    }
                } else {
                    // Not your turn
                    if let Some(addr) = self.sessions.get(&msg.id) {
                        addr.do_send(ErrorMessage {
                            error: "Not your turn".to_string(),
                        });
                    }
                }
            } else {
                // Player not found in room
                if let Some(addr) = self.sessions.get(&msg.id) {
                    addr.do_send(ErrorMessage {
                        error: "Player not found in room".to_string(),
                    });
                }
            }
        } else {
            // Room not found
            if let Some(addr) = self.sessions.get(&msg.id) {
                addr.do_send(ErrorMessage {
                    error: "Room not found".to_string(),
                });
            }
        }
    }
}
