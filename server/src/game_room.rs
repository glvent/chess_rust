// src/game_room.rs

use actix::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::bitboard::{apply_move, bitboard_to_pieces, Bitboard, MoveData};
use crate::messages::{RoomJoined, UpdateClient};
use crate::websocket::MyWebSocket;

pub struct GameRoom {
    pub id: Uuid,
    pub players: Vec<(usize, Addr<MyWebSocket>)>, // (session_id, address)
    pub bitboard: Bitboard,
    pub turn: String, // "w" or "b"
    pub player_colors: HashMap<usize, String>, // session_id -> color
}

impl GameRoom {
    pub fn new(id: Uuid, bitboard: Bitboard) -> Self {
        GameRoom {
            id,
            players: Vec::new(),
            bitboard,
            turn: "w".to_string(),
            player_colors: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, session_id: usize, addr: Addr<MyWebSocket>) {
        let color = if self.players.is_empty() { "w" } else { "b" };
        self.player_colors.insert(session_id, color.to_string());
        self.players.push((session_id, addr.clone()));

        // Send RoomJoined message to player
        addr.do_send(RoomJoined {
            room_id: self.id,
            color: color.to_string(),
        });

        println!("Player with session id {} joined room {}", session_id, self.id);
    }

    pub fn get_player_color(&self, session_id: usize) -> Option<String> {
        self.player_colors.get(&session_id).cloned()
    }

    pub fn apply_move(&mut self, move_data: &MoveData) -> bool {
        apply_move(&mut self.bitboard, move_data)
    }

    pub fn switch_turn(&mut self) {
        self.turn = if self.turn == "w" {
            "b".to_string()
        } else {
            "w".to_string()
        };
    }

    pub fn broadcast_update(&self) {
        let pieces = bitboard_to_pieces(&self.bitboard);

        let update_msg = UpdateClient {
            pieces,
            turn: self.turn.clone(),
        };

        for (_session_id, player) in &self.players {
            player.do_send(update_msg.clone());
        }
    }
}
