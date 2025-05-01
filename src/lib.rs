#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

pub mod tetris_game;

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::tetris_game::SprintGame;
    use super::tetris_game::bag::Bag;
    use super::tetris_game::board::Board;
    use super::tetris_game::piece::{Piece, PieceType, UNIQUE_PIECE_TYPES};

    #[test]
    fn board_state() {
        let mut game = SprintGame::new_tetrio();
        println!("{}", game.board());

        game.fill_queue();
        println!("Initial queue: {:?}", game.piece_queue());
    }

    #[test]
    fn bag() {
        let mut my_bag = Bag::new(7);
        assert!(my_bag.empty(), "bag should start out empty");
        for _ in 0..7 {
            my_bag.draw();
        }
        assert!(
            my_bag.empty(),
            "Bag should be empty after drawing size times"
        );

        let mut my_bag = Bag::new(21);
        let mut piece_counts = HashMap::new();
        for _ in 0..21 {
            *(piece_counts.entry(my_bag.draw()).or_insert(0)) += 1;
        }

        for (_, count) in piece_counts {
            assert!(
                count == 3,
                "Every piece should be seen exactly thrice when drawing 21 pieces from a 21 sized bag"
            );
        }
    }

    #[test]
    fn display_all_piece_orientations() {
        for kind in UNIQUE_PIECE_TYPES {
            let mut piece = Piece::new(kind);

            for _ in 0..4 {
                println!("{}", piece);
                piece.rotate_cw();
            }
        }
    }

    #[test]
    fn lateral_movement() {
        for piece_type in UNIQUE_PIECE_TYPES {
            let mut piece = Piece::new(piece_type);
            let board = Board::new();

            println!("{}, {:?}", piece, piece.position());

            for _ in 0..10 {
                println!("{:?}", piece.move_right(&board));
                println!("{}, {:?}", piece, piece.position());
            }

            for _ in 0..10 {
                println!("{:?}", piece.move_left(&board));
                println!("{}, {:?}", piece, piece.position());
            }
        }
    }
}
