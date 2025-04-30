pub mod bag;
pub mod board;
pub mod piece;

use std::collections::VecDeque;

use bag::Bag;

use self::board::Board;
use self::piece::{Piece, PieceType};

pub struct SprintGame {
    board: Board,
    hold_piece: Option<PieceType>,
    active_piece: Option<Piece>,
    piece_queue: VecDeque<PieceType>,

    bag: Bag,
    queue_size: usize,
}

impl SprintGame {
    #[must_use]
    pub fn new(bag_size: u32, queue_size: usize) -> Self {
        SprintGame {
            board: Board::new(),
            hold_piece: None,
            active_piece: None,
            piece_queue: VecDeque::new(),

            bag: Bag::new(bag_size),
            queue_size,
        }
    }

    #[must_use]
    pub fn new_tetrio() -> Self {
        SprintGame {
            board: Board::new(),
            hold_piece: None,
            active_piece: None,
            piece_queue: VecDeque::new(),

            bag: Bag::new(7),
            queue_size: 5,
        }
    }

    pub fn fill_queue(&mut self) {
        while self.piece_queue.len() < self.queue_size {
            self.piece_queue.push_back(self.bag.draw());
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn hold_piece(&self) -> Option<PieceType> {
        self.hold_piece
    }

    pub fn active_piece(&self) -> Option<&Piece> {
        self.active_piece.as_ref()
    }

    pub fn piece_queue(&self) -> &VecDeque<PieceType> {
        &self.piece_queue
    }
}

impl Default for SprintGame {
    fn default() -> Self {
        Self::new(7, 5)
    }
}
