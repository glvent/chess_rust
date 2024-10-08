// src/messages.rs

use actix::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::bitboard::MoveData;

pub struct Connect {
    pub addr: Addr<crate::websocket::MyWebSocket>,
}

impl Message for Connect {
    type Result = usize;
}

pub struct Disconnect {
    pub id: usize,
}

impl Message for Disconnect {
    type Result = ();
}

pub struct CreateRoom {
    pub id: usize,
}

impl Message for CreateRoom {
    type Result = ();
}

pub struct JoinRoom {
    pub id: usize,
    pub room_id: Uuid,
}

impl Message for JoinRoom {
    type Result = ();
}

pub struct JoinQueue {
    pub id: usize,
}

impl Message for JoinQueue {
    type Result = ();
}

pub struct ClientMove {
    pub id: usize,
    pub room_id: Uuid,
    pub move_data: MoveData,
}

impl Message for ClientMove {
    type Result = ();
}

pub struct RoomJoined {
    pub room_id: Uuid,
    pub color: String, // "w" or "b"
}

impl Message for RoomJoined {
    type Result = ();
}

#[derive(Clone)]
pub struct UpdateClient {
    pub pieces: Vec<Value>,
    pub turn: String,
}

impl Message for UpdateClient {
    type Result = ();
}

pub struct ErrorMessage {
    pub error: String,
}

impl Message for ErrorMessage {
    type Result = ();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientMessage {
    pub r#type: String,
    pub data: Value,
}
