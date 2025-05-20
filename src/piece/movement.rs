use super::Piece;
use super::util::TetrisError;
use crate::board::Board;

impl Piece {
    /// Moves the piece to the right, returning `Ok` if the move is executed,
    /// else a `Err(TetrisError::InvalidMove)` if the piece cannot move due to
    /// an obstruction (an already placed piece or the right wall).
    pub fn move_right(&mut self, board: &Board) -> Result<(), TetrisError> {
        for (y_offset, x_offset) in self.right_edge().into_iter().flatten() {
            let row = self.y() as usize + y_offset;
            let col_to_move_to = self.x() + x_offset as i32 + 1;

            if col_to_move_to >= board.width() as i32
                || board[row][col_to_move_to as usize].is_some()
            {
                return Err(TetrisError::InvalidMove(
                    "Move right failed due to an obstruction.",
                ));
            }
        }

        self.position.move_right();
        Ok(())
    }

    /// Moves the piece to the left, returning `Ok` if the move is executed,
    /// else a `Err(TetrisError::InvalidMove)` if the piece cannot move due to
    /// an obstruction (an already placed piece or the left wall).
    pub fn move_left(&mut self, board: &Board) -> Result<(), TetrisError> {
        for (y_offset, x_offset) in self.left_edge().into_iter().flatten() {
            let row = self.y() as usize + y_offset;
            let curr_col = self.x() + x_offset as i32;

            if curr_col <= 0 || board[row][(curr_col - 1) as usize].is_some() {
                return Err(TetrisError::InvalidMove(
                    "Move left failed due to an obstruction.",
                ));
            }
        }

        self.position.move_left();
        Ok(())
    }

    pub fn hard_drop(&mut self, column_heights: [u8; 10]) -> u8 {
        let mut drop_y = 0;

        for (i, y) in self.lower_edge().into_iter().flatten() {
            // `piece.x() + i` will always be positive because the rules of
            // piece movement (no cells can be outside the board)
            let new_drop_y = y + column_heights[(self.x() + i as i32) as usize];

            if new_drop_y > drop_y {
                drop_y = new_drop_y;
            }
        }

        self.position.set_y(drop_y as i32);
        drop_y
    }
}

#[cfg(test)]
mod tests {
    use super::super::{DEFAULT_POSITION, PieceType, UNIQUE_TYPES};
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn default_piece_position() {
        for piece_type in UNIQUE_TYPES {
            let piece = Piece::new(piece_type);
            assert_eq!(
                DEFAULT_POSITION.x(),
                piece.x(),
                "Incorrect initial {} piece x, Expected {}, was {}",
                piece_type,
                DEFAULT_POSITION.x(),
                piece.x()
            );
            assert_eq!(
                DEFAULT_POSITION.y(),
                piece.y(),
                "Incorrect initial {} piece y, Expected {}, was {}",
                piece_type,
                DEFAULT_POSITION.y(),
                piece.y()
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

        for piece_type in UNIQUE_TYPES {
            let mut piece = Piece::new(piece_type);
            let board = Board::new();

            for i in 0..10 {
                let _ = piece.move_right(&board);
                assert_eq!(
                    expected_x_positions_right_from_default[&piece_type][i],
                    piece.x(),
                    "Incorrect {piece_type} piece x-position after {} right \
                    moves starting from default position. Expected {}, was {}",
                    i + 1,
                    expected_x_positions_right_from_default[&piece_type][i],
                    piece.x()
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

        for piece_type in UNIQUE_TYPES {
            let mut piece = Piece::new(piece_type);
            let board = Board::new();

            for i in 0..10 {
                let _ = piece.move_left(&board);
                assert_eq!(
                    expected_x_positions_left_from_default[&piece_type][i],
                    piece.x(),
                    "Incorrect {piece_type} piece x-position after {} left \
                    moves starting from default position. Expected {}, was {}",
                    i + 1,
                    expected_x_positions_left_from_default[&piece_type][i],
                    piece.x()
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

        for piece_type in UNIQUE_TYPES {
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
                    piece.x(),
                    "Incorrect {piece_type} piece x-position after {} right \
                    moves starting from the left edge. Expected {}, was {}",
                    i + 1,
                    expected_x_positions_right_from_left[&piece_type][i],
                    piece.x()
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

        for piece_type in UNIQUE_TYPES {
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
                    piece.x(),
                    "Incorrect {piece_type} piece x-position after {} left \
                    moves starting from the right edge. Expected {}, was {}",
                    i + 1,
                    expected_x_positions_left_from_right[&piece_type][i],
                    piece.x()
                )
            }
        }
    }
}
