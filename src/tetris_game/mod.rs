pub mod bag;
pub mod board;
pub mod piece;

use std::collections::VecDeque;

use self::board::Board;
use self::piece::{Piece, PieceType};

pub struct SprintGame {
    pub board: Board,
    hold_piece: Option<Piece>,
    active_piece: Option<Piece>,
    piece_queue: VecDeque<PieceType>,
}

impl SprintGame {
    #[must_use]
    pub fn new() -> Self {
        SprintGame {
            board: Board::new(),
            hold_piece: None,
            active_piece: None,
            piece_queue: VecDeque::new(),
        }
    }
}

impl Default for SprintGame {
    fn default() -> Self {
        Self::new()
    }
}
