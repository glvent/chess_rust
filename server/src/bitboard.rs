// src/bitboard.rs

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone)]
pub struct Bitboard {
    pub pawns: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub queens: u64,
    pub kings: u64,
    pub white_pieces: u64,
    pub black_pieces: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PieceInfo {
    pub piece_type: String,
    pub color: String, // "w" or "b"
    pub position: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveData {
    pub from: usize,
    pub to: usize,
}

pub fn init_bitboard() -> Bitboard {
    let mut bitboard = Bitboard {
        pawns: 0,
        knights: 0,
        bishops: 0,
        rooks: 0,
        queens: 0,
        kings: 0,
        white_pieces: 0,
        black_pieces: 0,
    };

    // Initialize white pieces
    bitboard.pawns |= 0x000000000000FF00;
    bitboard.rooks |= 0x0000000000000081;
    bitboard.knights |= 0x0000000000000042;
    bitboard.bishops |= 0x0000000000000024;
    bitboard.queens |= 0x0000000000000008;
    bitboard.kings |= 0x0000000000000010;
    bitboard.white_pieces |= 0x000000000000FFFF;

    // Initialize black pieces
    bitboard.pawns |= 0x00FF000000000000;
    bitboard.rooks |= 0x8100000000000000;
    bitboard.knights |= 0x4200000000000000;
    bitboard.bishops |= 0x2400000000000000;
    bitboard.queens |= 0x0800000000000000;
    bitboard.kings |= 0x1000000000000000;
    bitboard.black_pieces |= 0xFFFF000000000000;

    bitboard
}

pub fn apply_move(bitboard: &mut Bitboard, move_data: &MoveData) -> bool {
    let from_bb = 1u64 << move_data.from;
    let to_bb = 1u64 << move_data.to;

    let piece_masks = [
        ("pawns", bitboard.pawns),
        ("knights", bitboard.knights),
        ("bishops", bitboard.bishops),
        ("rooks", bitboard.rooks),
        ("queens", bitboard.queens),
        ("kings", bitboard.kings),
    ];

    let mut piece_type_moving = None;
    for (piece_type, mask) in &piece_masks {
        if mask & from_bb != 0 {
            piece_type_moving = Some(*piece_type);
            break;
        }
    }

    if let Some(piece_type) = piece_type_moving {
        match piece_type {
            "pawns" => bitboard.pawns &= !from_bb,
            "knights" => bitboard.knights &= !from_bb,
            "bishops" => bitboard.bishops &= !from_bb,
            "rooks" => bitboard.rooks &= !from_bb,
            "queens" => bitboard.queens &= !from_bb,
            "kings" => bitboard.kings &= !from_bb,
            _ => (),
        }

        let all_pieces_masks = [
            &mut bitboard.pawns,
            &mut bitboard.knights,
            &mut bitboard.bishops,
            &mut bitboard.rooks,
            &mut bitboard.queens,
            &mut bitboard.kings,
        ];

        for mask in all_pieces_masks {
            if *mask & to_bb != 0 {
                *mask &= !to_bb;
            }
        }

        match piece_type {
            "pawns" => bitboard.pawns |= to_bb,
            "knights" => bitboard.knights |= to_bb,
            "bishops" => bitboard.bishops |= to_bb,
            "rooks" => bitboard.rooks |= to_bb,
            "queens" => bitboard.queens |= to_bb,
            "kings" => bitboard.kings |= to_bb,
            _ => (),
        }

        update_occupancy(bitboard);

        true
    } else {
        false
    }
}

fn update_occupancy(bitboard: &mut Bitboard) {
    bitboard.white_pieces = (bitboard.pawns
        | bitboard.knights
        | bitboard.bishops
        | bitboard.rooks
        | bitboard.queens
        | bitboard.kings)
        & 0x000000000000FFFF;
    bitboard.black_pieces = (bitboard.pawns
        | bitboard.knights
        | bitboard.bishops
        | bitboard.rooks
        | bitboard.queens
        | bitboard.kings)
        & 0xFFFF000000000000;
}

pub fn bitboard_to_pieces(bitboard: &Bitboard) -> Vec<serde_json::Value> {
    let mut pieces = Vec::new();

    for position in 0..64 {
        let bb = 1u64 << position;

        let mut piece_type = None;
        if bitboard.pawns & bb != 0 {
            piece_type = Some("p");
        } else if bitboard.knights & bb != 0 {
            piece_type = Some("n");
        } else if bitboard.bishops & bb != 0 {
            piece_type = Some("b");
        } else if bitboard.rooks & bb != 0 {
            piece_type = Some("r");
        } else if bitboard.queens & bb != 0 {
            piece_type = Some("q");
        } else if bitboard.kings & bb != 0 {
            piece_type = Some("k");
        }

        if let Some(pt) = piece_type {
            let color = if bitboard.white_pieces & bb != 0 {
                "w"
            } else if bitboard.black_pieces & bb != 0 {
                "b"
            } else {
                continue;
            };

            pieces.push(json!({
                "piece_type": pt,
                "color": color,
                "position": position,
            }));
        }
    }

    pieces
}
