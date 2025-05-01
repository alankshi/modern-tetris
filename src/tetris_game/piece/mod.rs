pub mod util;

use std::fmt::{Display, Error, Formatter};

use super::board::Board;
use util::{Orientation, Position, TetrisError};
pub use util::{PieceType, UNIQUE_PIECE_TYPES};

pub struct Piece {
    kind: PieceType,
    orientation: Orientation,
    position: Position,
}

impl Piece {
    #[must_use]
    /// Creates a new piece of kind `kind` with a north orientation at (0, 0)
    pub fn new(kind: PieceType) -> Piece {
        Piece {
            kind,
            orientation: Orientation::North,
            position: Position::new(),
        }
    }

    /// Returns the positions of the piece's filled cells. The positions are
    /// encoded as a size-4 array of unsigned bytes, with values 0-15
    /// symbolizing the following positions:
    ///
    /// | | | | |
    /// |-|-|-|-|
    /// |0|1|2|3|
    /// |4|5|6|7|
    /// |8|9|10|11|
    /// |12|13|14|15|
    ///
    /// where the position of the piece is at the top left corner, at position
    /// 0. The array of bytes is sorted in ascending order.
    #[must_use]
    pub fn get_mask(&self) -> [u8; 4] {
        match self.kind {
            PieceType::I => match self.orientation {
                Orientation::North => [4, 5, 6, 7],
                Orientation::East => [2, 6, 10, 14],
                Orientation::South => [8, 9, 10, 11],
                Orientation::West => [1, 5, 9, 13],
            },
            PieceType::L => match self.orientation {
                Orientation::North => [2, 4, 5, 6],
                Orientation::East => [1, 5, 9, 10],
                Orientation::South => [4, 5, 6, 8],
                Orientation::West => [0, 1, 5, 9],
            },
            PieceType::J => match self.orientation {
                Orientation::North => [0, 4, 5, 6],
                Orientation::East => [1, 2, 5, 9],
                Orientation::South => [4, 5, 6, 10],
                Orientation::West => [1, 5, 9, 8],
            },
            PieceType::S => match self.orientation {
                Orientation::North => [1, 2, 4, 5],
                Orientation::East => [1, 5, 6, 10],
                Orientation::South => [5, 6, 8, 9],
                Orientation::West => [0, 4, 5, 9],
            },
            PieceType::Z => match self.orientation {
                Orientation::North => [0, 1, 5, 6],
                Orientation::East => [2, 6, 5, 9],
                Orientation::South => [4, 5, 9, 10],
                Orientation::West => [1, 4, 5, 8],
            },
            PieceType::T => match self.orientation {
                Orientation::North => [1, 4, 5, 6],
                Orientation::East => [1, 5, 6, 9],
                Orientation::South => [4, 5, 6, 9],
                Orientation::West => [1, 4, 5, 9],
            },
            PieceType::O => [1, 2, 5, 6],
        }
    }

    pub fn kind(&self) -> PieceType {
        self.kind
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    /// Rotate the piece clockwise, changing its orientation accordingly
    pub fn rotate_cw(&mut self) {
        self.orientation = self.orientation.clockwise();
    }

    /// Rotate the piece counterclockwise, changing its orientation accordingly
    pub fn rotate_ccw(&mut self) {
        self.orientation = self.orientation.counterclockwise();
    }

    /// Moves the piece to the right, returning `Ok` if the move is executed,
    /// else a `Err(TetrisError::InvalidMove)` if the piece cannot move due to
    /// an obstruction (an already placed piece or the right wall).
    pub fn move_right(&mut self, board: &Board) -> Result<(), TetrisError> {
        let mask = self.get_mask();

        // calculate the right edge to check whether the piece runs into anything on its right
        let mut right_edge = [None; 4];
        for cell_idx in mask {
            right_edge[(cell_idx / 4) as usize] = Some(cell_idx % 4);
        }

        for (y_offset, x_offset) in right_edge.iter().enumerate() {
            match x_offset {
                None => continue,
                Some(x_offset) => {
                    let row = self.position.y as usize + y_offset;
                    let col_to_move_to = self.position.x + *x_offset as i32 + 1;

                    if col_to_move_to >= 10 || board[row][col_to_move_to as usize].is_some() {
                        return Err(TetrisError::InvalidMove(
                            "Move right failed due to an obstruction.",
                        ));
                    }
                }
            }
        }

        self.position.x += 1;
        Ok(())
    }

    /// Moves the piece to the left, returning `Ok` if the move is executed,
    /// else a `Err(TetrisError::InvalidMove)` if the piece cannot move due to
    /// an obstruction (an already placed piece or the left wall).
    pub fn move_left(&mut self, board: &Board) -> Result<(), TetrisError> {
        let mask = self.get_mask();

        // calculate the left edge to check whether the piece runs into anything on its left
        let mut left_edge = [None; 4];
        for cell_idx in mask.iter().rev() {
            left_edge[(cell_idx / 4) as usize] = Some(cell_idx % 4);
        }

        for (y_offset, x_offset) in left_edge.iter().enumerate() {
            match x_offset {
                None => continue,
                Some(x_offset) => {
                    let row = self.position.y as usize + y_offset;
                    let curr_col = self.position.x + *x_offset as i32;

                    if curr_col <= 0 || board[row][(curr_col - 1) as usize].is_some() {
                        return Err(TetrisError::InvalidMove(
                            "Move left failed due to an obstruction.",
                        ));
                    }
                }
            }
        }

        self.position.x -= 1;
        Ok(())
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut curr_cell = 0;
        let mask = self.get_mask();

        for i in 0..16 {
            if i % 4 == 0 && i > 0 {
                writeln!(f)?;
            }

            write!(
                f,
                "{}",
                if curr_cell < 4 && i == mask[curr_cell] {
                    curr_cell += 1;
                    "■"
                } else {
                    " "
                }
            )?;
        }
        Ok(())
    }
}
