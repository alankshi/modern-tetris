#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

mod bag;
mod board;
mod game;
mod piece;
mod traits;

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

#[cfg(test)]
mod tests {
    use crate::SprintGame;
    use crate::piece;
    use crate::piece::Piece;
    use crate::traits::Controllable;

    #[test]
    // not an actual test
    fn game() {
        let mut game = SprintGame::new_tetrio();

        game.start();
        println!("{}", game.board());

        game.hard_drop();
        println!("{}", game.board());

        game.move_right();
        game.move_right();
        game.move_right();
        game.move_right();
        game.rotate_cw();
        game.hard_drop();
        println!("{}", game.board());

        game.move_left();
        game.move_left();
        game.move_left();
        game.move_left();
        game.rotate_ccw();
        game.hard_drop();
        println!("{}", game.board());
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
