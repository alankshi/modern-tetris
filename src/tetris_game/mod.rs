pub mod bag;
pub mod board_state;
pub mod piece;

use std::collections::VecDeque;

use board_state::Board;
use piece::{Piece, PieceType};

pub struct SprintGame {
    pub board: Board,
    hold_piece: Option<Piece>,
    active_piece: Option<Piece>,
    piece_queue: VecDeque<PieceType>,
}

impl SprintGame {
    pub fn new() -> SprintGame {
        SprintGame {
            board: Board::new(),
            hold_piece: None,
            active_piece: None,
            piece_queue: VecDeque::new(),
        }
    }
}
