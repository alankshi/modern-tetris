use crate::Controllable;
use crate::{Game, TetrisError, TetrisGame};

use std::mem;

use tracing::{Level, event};

impl Controllable for TetrisGame {
    fn move_left(&mut self) -> Result<(), TetrisError> {
        self.active_piece.as_mut().unwrap().move_left(&self.board)
    }

    fn move_right(&mut self) -> Result<(), TetrisError> {
        self.active_piece.as_mut().unwrap().move_right(&self.board)
    }

    fn rotate_cw(&mut self) -> Result<(), TetrisError> {
        self.active_piece.as_mut().unwrap().rotate_cw()
    }

    fn rotate_ccw(&mut self) -> Result<(), TetrisError> {
        self.active_piece.as_mut().unwrap().rotate_ccw()
    }

    fn rotate_180(&mut self) -> Result<(), TetrisError> {
        self.active_piece.as_mut().unwrap().rotate_180()
    }

    fn hard_drop(&mut self) -> Result<(), TetrisError> {
        event!(Level::INFO, "Hard drop initiated");

        let mut piece_to_drop = None;
        mem::swap(&mut self.active_piece, &mut piece_to_drop);

        self.board.hard_drop(piece_to_drop.unwrap())?;
        self.can_hold = true;
        self.load_next_piece().unwrap();

        Ok(())
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
            self.load_next_piece().unwrap();
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
        let mut game = TetrisGame::new_tetrio();
        game.start().unwrap();

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

        let _ = game.hard_drop();
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
    fn unsuccessful_hold() {
        let mut game = TetrisGame::new_tetrio();
        game.start().unwrap();

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

        let _ = game.hard_drop();
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
