use crate::traits::Controllable;
use crate::{Bag, Board, Piece, PieceType, TetrisError};

use std::collections::VecDeque;
use std::mem;

pub struct SprintGame {
    board: Board,
    hold_piece: Option<PieceType>,
    active_piece: Option<Piece>,
    piece_queue: VecDeque<PieceType>,
    bag: Bag,

    can_hold: bool,
    queue_size: usize,
}

impl SprintGame {
    /// Creates a new sprint game with a specified bag and queue size
    #[must_use]
    pub fn new(bag_size: u32, queue_size: usize) -> Self {
        SprintGame {
            board: Board::new(),
            hold_piece: None,
            active_piece: None,
            piece_queue: VecDeque::new(),
            bag: Bag::new(bag_size),

            can_hold: true,
            queue_size,
        }
    }

    /// Creates a new TETR.IO sprint game with a bag size of 7 and queue size of
    /// 5
    #[must_use]
    pub fn new_tetrio() -> Self {
        SprintGame {
            board: Board::new(),
            hold_piece: None,
            active_piece: None,
            piece_queue: VecDeque::new(),

            bag: Bag::new(7),
            can_hold: true,
            queue_size: 5,
        }
    }

    /// Fills the queue by drawing pieces from the game's bag until
    /// `self.piece_queue.len() == self.queue_size`
    pub fn fill_queue(&mut self) {
        while self.piece_queue.len() < self.queue_size {
            self.piece_queue.push_back(self.bag.draw());
        }
    }

    /// Returns a reference to the game's board, `self.board`
    #[must_use]
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Returns the type of the current hold piece if there is one, else `None`
    #[must_use]
    pub fn hold_piece(&self) -> Option<PieceType> {
        self.hold_piece
    }

    /// Returns a reference to the current active piece if there is one, else
    /// `None`
    #[must_use]
    pub fn active_piece(&self) -> Option<&Piece> {
        self.active_piece.as_ref()
    }

    /// Returns a reference to the upcoming piece queue
    #[must_use]
    pub fn piece_queue(&self) -> &VecDeque<PieceType> {
        &self.piece_queue
    }

    fn load_next_piece(&mut self) {
        if self.active_piece.is_some() {
            panic!(
                "Cannot load next piece while current piece, {}, still exists.",
                self.active_piece().unwrap().kind()
            )
        }

        self.active_piece = Some(Piece::new(self.piece_queue.pop_front().unwrap()));
        self.fill_queue();
    }

    pub fn start(&mut self) {
        self.fill_queue();
        self.load_next_piece();
    }
}

impl Default for SprintGame {
    fn default() -> Self {
        Self::new(7, 5)
    }
}

impl Controllable for SprintGame {
    fn move_left(&mut self) {
        let _ = self.active_piece.as_mut().unwrap().move_left(&self.board);
    }

    fn move_right(&mut self) {
        let _ = self.active_piece.as_mut().unwrap().move_right(&self.board);
    }

    fn rotate_cw(&mut self) {
        self.active_piece.as_mut().unwrap().rotate_cw();
    }

    fn rotate_ccw(&mut self) {
        self.active_piece.as_mut().unwrap().rotate_ccw();
    }

    fn hard_drop(&mut self) {
        let mut piece_to_drop = None;
        mem::swap(&mut self.active_piece, &mut piece_to_drop);

        self.board.hard_drop(piece_to_drop.unwrap());
        self.can_hold = true;
        self.load_next_piece();
    }

    fn hold(&mut self) -> Result<(), TetrisError> {
        if !self.can_hold {
            return Err(TetrisError::InvalidHold);
        }

        let mut prev_hold_piece = match self.hold_piece {
            Some(piece_type) => Some(piece_type.into()),
            None => None,
        };
        mem::swap(&mut self.active_piece, &mut prev_hold_piece);

        if self.active_piece().is_none() {
            self.load_next_piece();
        }

        let prev_active_piece = prev_hold_piece;
        self.hold_piece = match prev_active_piece {
            Some(piece) => Some(piece.into()),
            None => None,
        };

        self.can_hold = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_hold() {
        let mut game = SprintGame::new_tetrio();
        game.start();

        let active_kind = game
            .active_piece()
            .expect("Active piece should not be None upon game start.")
            .kind();

        assert_eq!(
            Ok(()),
            game.hold(),
            "First hold after game start should be valid."
        );
        assert_eq!(
            active_kind,
            game.hold_piece()
                .expect("Hold piece should never be None after the first hold."),
            "Hold piece was not the same kind as the active piece after switching."
        );

        game.hard_drop();
        let hold_kind = game.hold_piece().unwrap();
        let active_kind = game
            .active_piece()
            .expect("Active piece should never be None after game start.")
            .kind();

        assert_eq!(
            Ok(()),
            game.hold(),
            "First hold after piece placement should be valid."
        );
        assert_eq!(
            active_kind,
            game.hold_piece()
                .expect("Hold piece should never be None after the first hold."),
            "Hold piece was not the same kind as the previous active piece after switching."
        );
        assert_eq!(
            hold_kind,
            game.active_piece()
                .expect("Active piece should never be None after game start.")
                .kind(),
            "Active piece was not the same kind as the previous hold piece after switching."
        );
    }

    #[test]
    fn invalid_hold() {
        let mut game = SprintGame::new_tetrio();
        game.start();

        assert_eq!(
            Ok(()),
            game.hold(),
            "First hold after game start should be valid."
        );
        assert_eq!(
            Err(TetrisError::InvalidHold),
            game.hold(),
            "Second hold after game start without placing a piece should be invalid."
        );
        assert_eq!(
            Err(TetrisError::InvalidHold),
            game.hold(),
            "Third hold after game start without placing a piece should be invalid."
        );

        game.hard_drop();
        assert_eq!(
            Ok(()),
            game.hold(),
            "First hold after piece placement should be valid."
        );
        assert_eq!(
            Err(TetrisError::InvalidHold),
            game.hold(),
            "Second hold after a piece placement without placing another piece should be invalid."
        );
        assert_eq!(
            Err(TetrisError::InvalidHold),
            game.hold(),
            "Third hold after a piece placement without placing another piece should be invalid."
        );
    }
}
