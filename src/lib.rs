#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

mod bag;
mod board;
mod game;
mod piece;

use std::collections::VecDeque;

// private re-exports for modules
use bag::Bag;
use board::Board;
use piece::{Piece, PieceType};

pub use game::SprintGame;

#[derive(Debug, PartialEq)]
pub enum TetrisError {
    InvalidRightMove,
    InvalidLeftMove,
    InvalidCWRotation,
    InvalidCCWRotation,
    InvalidHold,
    GameOver,
}

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
}

trait Controllable {
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
        }
    }
}

pub trait Game: Controllable {
    fn start(&mut self);
    fn next_frame(&mut self, inputs: VecDeque<TetrisInput>);
}

#[cfg(test)]
mod tests {
    use crate::piece::Piece;
    use crate::{Controllable, Game, SprintGame, piece};

    #[test]
    // not an actual test
    fn game() {
        let mut game = SprintGame::new_tetrio();

        game.start();
        println!("{}", game);

        game.hold();
        game.hard_drop();
        println!("{}", game);

        game.move_right();
        game.move_right();
        game.move_right();
        game.move_right();
        game.rotate_cw();
        game.hard_drop();
        println!("{}", game);

        game.move_left();
        game.move_left();
        game.move_left();
        game.move_left();
        game.rotate_ccw();
        game.hard_drop();
        println!("{}", game);

        game.hard_drop();
        game.hard_drop();
        game.hard_drop();
        println!("{}", game);
    }

    #[test]
    // not an actual test
    fn board_state() {
        let mut game = SprintGame::new_tetrio();
        println!("{}", game.board());

        game.fill_queue();
        println!("Initial queue: {:?}", game.piece_queue());
    }

    #[test]
    // not an actual test
    fn display_all_piece_orientations() {
        for kind in piece::UNIQUE_TYPES {
            let mut piece = Piece::new(kind);

            for _ in 0..4 {
                println!("{}", piece);
                piece.rotate_cw();
            }
        }
    }
}
