#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

pub mod tetris_game;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::tetris_game::SprintGame;
    use crate::tetris_game::bag::Bag;
    use crate::tetris_game::board::Board;
    use crate::tetris_game::piece;
    use crate::tetris_game::piece::{Piece, PieceType};

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
        for kind in piece::UNIQUE_TYPES {
            let mut piece = Piece::new(kind);

            for _ in 0..4 {
                println!("{}", piece);
                piece.rotate_cw();
            }
        }
    }

    #[test]
    fn default_piece_position() {
        for piece_type in piece::UNIQUE_TYPES {
            let piece = Piece::new(piece_type);
            assert_eq!(
                piece::DEFAULT_POSITION.x,
                piece.position().x,
                "Incorrect initial {} piece x, Expected {}, was {}",
                piece_type,
                piece::DEFAULT_POSITION.x,
                piece.position().x
            );
            assert_eq!(
                piece::DEFAULT_POSITION.y,
                piece.position().y,
                "Incorrect initial {} piece y, Expected {}, was {}",
                piece_type,
                piece::DEFAULT_POSITION.y,
                piece.position().y
            );
        }
    }

    #[test]
    fn north_facing_empty_board_move_right_from_default() {
        let expected_x_positions_right_from_default = HashMap::from([
            (PieceType::I, [4, 5, 6, 6, 6, 6, 6, 6, 6, 6]),
            (PieceType::L, [4, 5, 6, 7, 7, 7, 7, 7, 7, 7]),
            (PieceType::J, [4, 5, 6, 7, 7, 7, 7, 7, 7, 7]),
            (PieceType::S, [4, 5, 6, 7, 7, 7, 7, 7, 7, 7]),
            (PieceType::Z, [4, 5, 6, 7, 7, 7, 7, 7, 7, 7]),
            (PieceType::T, [4, 5, 6, 7, 7, 7, 7, 7, 7, 7]),
            (PieceType::O, [4, 5, 6, 7, 7, 7, 7, 7, 7, 7]),
        ]);

        for piece_type in piece::UNIQUE_TYPES {
            let mut piece = Piece::new(piece_type);
            let board = Board::new();

            for i in 0..10 {
                let _ = piece.move_right(&board);
                assert_eq!(
                    expected_x_positions_right_from_default[&piece_type][i],
                    piece.position().x,
                    "Incorrect {piece_type} piece x-position after {} right \
                    moves starting from default position. Expected {}, was {}",
                    i + 1,
                    expected_x_positions_right_from_default[&piece_type][i],
                    piece.position().x
                )
            }
        }
    }

    #[test]
    fn north_facing_empty_board_move_left_from_default() {
        let expected_x_positions_left_from_default = HashMap::from([
            (PieceType::I, [2, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
            (PieceType::L, [2, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
            (PieceType::J, [2, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
            (PieceType::S, [2, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
            (PieceType::Z, [2, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
            (PieceType::T, [2, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
            (PieceType::O, [2, 1, 0, -1, -1, -1, -1, -1, -1, -1]),
        ]);

        for piece_type in piece::UNIQUE_TYPES {
            let mut piece = Piece::new(piece_type);
            let board = Board::new();

            for i in 0..10 {
                let _ = piece.move_left(&board);
                assert_eq!(
                    expected_x_positions_left_from_default[&piece_type][i],
                    piece.position().x,
                    "Incorrect {piece_type} piece x-position after {} left \
                    moves starting from default position. Expected {}, was {}",
                    i + 1,
                    expected_x_positions_left_from_default[&piece_type][i],
                    piece.position().x
                )
            }
        }
    }

    #[test]
    fn north_facing_empty_board_move_right_from_left() {
        let expected_x_positions_right_from_left = HashMap::from([
            (PieceType::I, [1, 2, 3, 4, 5, 6, 6, 6, 6, 6]),
            (PieceType::L, [1, 2, 3, 4, 5, 6, 7, 7, 7, 7]),
            (PieceType::J, [1, 2, 3, 4, 5, 6, 7, 7, 7, 7]),
            (PieceType::S, [1, 2, 3, 4, 5, 6, 7, 7, 7, 7]),
            (PieceType::Z, [1, 2, 3, 4, 5, 6, 7, 7, 7, 7]),
            (PieceType::T, [1, 2, 3, 4, 5, 6, 7, 7, 7, 7]),
            (PieceType::O, [0, 1, 2, 3, 4, 5, 6, 7, 7, 7]),
        ]);

        for piece_type in piece::UNIQUE_TYPES {
            let mut piece = Piece::new(piece_type);
            let board = Board::new();

            for _ in 0..10 {
                // move piece to the left edge
                let _ = piece.move_left(&board);
            }

            for i in 0..10 {
                let _ = piece.move_right(&board);
                assert_eq!(
                    expected_x_positions_right_from_left[&piece_type][i],
                    piece.position().x,
                    "Incorrect {piece_type} piece x-position after {} right \
                    moves starting from the left edge. Expected {}, was {}",
                    i + 1,
                    expected_x_positions_right_from_left[&piece_type][i],
                    piece.position().x
                )
            }
        }
    }

    #[test]
    fn north_facing_empty_board_move_left_from_right() {
        let expected_x_positions_left_from_right = HashMap::from([
            (PieceType::I, [5, 4, 3, 2, 1, 0, 0, 0, 0, 0]),
            (PieceType::L, [6, 5, 4, 3, 2, 1, 0, 0, 0, 0]),
            (PieceType::J, [6, 5, 4, 3, 2, 1, 0, 0, 0, 0]),
            (PieceType::S, [6, 5, 4, 3, 2, 1, 0, 0, 0, 0]),
            (PieceType::Z, [6, 5, 4, 3, 2, 1, 0, 0, 0, 0]),
            (PieceType::T, [6, 5, 4, 3, 2, 1, 0, 0, 0, 0]),
            (PieceType::O, [6, 5, 4, 3, 2, 1, 0, -1, -1, -1]),
        ]);

        for piece_type in piece::UNIQUE_TYPES {
            let mut piece = Piece::new(piece_type);
            let board = Board::new();

            for _ in 0..10 {
                // move piece to the right edge
                let _ = piece.move_right(&board);
            }

            for i in 0..10 {
                let _ = piece.move_left(&board);
                assert_eq!(
                    expected_x_positions_left_from_right[&piece_type][i],
                    piece.position().x,
                    "Incorrect {piece_type} piece x-position after {} left \
                    moves starting from the right edge. Expected {}, was {}",
                    i + 1,
                    expected_x_positions_left_from_right[&piece_type][i],
                    piece.position().x
                )
            }
        }
    }
}
