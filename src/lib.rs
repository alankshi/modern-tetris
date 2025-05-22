#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

mod bag;
mod board;
mod game;
mod piece;

use std::{cell::RefMut, collections::VecDeque};

// private re-exports for modules
use bag::Bag;
use board::Board;
use piece::{Piece, PieceType};

pub use game::TetrisGame;

#[derive(Debug, PartialEq)]
pub enum TetrisError {
    InvalidRightMove,
    InvalidLeftMove,
    InvalidCWRotation,
    InvalidCCWRotation,
    InvalidHold,
    FailedToLoadPiece,
    GameOver,
    GameNotStarted,
}

#[derive(Clone)]
pub enum TetrisInput {
    SoftDrop,
    HardDrop,
    MoveLeft,
    MoveRight,
    SnapRight,
    SnapLeft,
    RotateCW,
    RotateCCW,
    Rotate180,
    Hold,
    Forfeit,
}

trait Controllable: Game {
    fn rotate_ccw(&mut self) -> Result<(), TetrisError>;
    fn rotate_cw(&mut self) -> Result<(), TetrisError>;
    fn rotate_180(&mut self) -> Result<(), TetrisError>;

    fn move_left(&mut self) -> Result<(), TetrisError>;
    fn move_right(&mut self) -> Result<(), TetrisError>;

    fn hold(&mut self) -> Result<(), TetrisError>;

    fn hard_drop(&mut self) -> Result<(), TetrisError>;

    fn execute_input(&mut self, input: TetrisInput) -> Result<(), TetrisError> {
        match input {
            TetrisInput::SoftDrop => {
                todo!()
            }
            TetrisInput::HardDrop => self.hard_drop(),
            TetrisInput::MoveLeft => self.move_left(),
            TetrisInput::MoveRight => self.move_right(),
            TetrisInput::SnapRight => {
                todo!()
            }
            TetrisInput::SnapLeft => {
                todo!()
            }
            TetrisInput::RotateCW => self.rotate_cw(),
            TetrisInput::RotateCCW => self.rotate_ccw(),
            TetrisInput::Rotate180 => self.rotate_180(),
            TetrisInput::Hold => self.hold(),
            TetrisInput::Forfeit => Ok(self.end_game()),
        }
    }
}

pub trait Game {
    fn start(&mut self) -> Result<(), TetrisError>;
    fn next_frame(&mut self, inputs: &mut VecDeque<TetrisInput>) -> Result<(), TetrisError>;
    fn end_game(&mut self);
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::TetrisError;
    use crate::{Controllable, Game, TetrisGame, piece};

    #[test]
    fn game_lifetime() {
        let mut game = TetrisGame::new_tetrio();

        assert_eq!(
            Err(TetrisError::GameNotStarted),
            game.next_frame(&mut VecDeque::new()),
            "Game should throw GameNotStarted if trying to advance frames before it starts."
        );

        game.start()
            .expect("Game should be able to be started before game over.");

        assert_eq!(
            Err(TetrisError::FailedToLoadPiece),
            game.start(),
            "Game should throw a FailedToLoadPiece if started after already being started but before game over."
        );

        game.next_frame(&mut VecDeque::new())
            .expect("Game should be able to advance frames after game start and before game end.");

        game.end_game();

        assert_eq!(
            Err(TetrisError::GameOver),
            game.start(),
            "Attempting to start the game after game over should throw a GameOver error"
        );

        assert_eq!(
            Err(TetrisError::GameOver),
            game.next_frame(&mut VecDeque::new()),
            "Attempting to advance frames after game over should throw a GameOver error"
        );
    }
}
