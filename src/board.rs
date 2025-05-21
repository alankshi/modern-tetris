mod drop;

use std::fmt::{Display, Error, Formatter};
use std::ops::Index;

/// Represents a 10x20 Tetris game board, although the implementation height is
/// 24, the maximum height a piece can be placed.
pub struct Board {
    board: [[Option<()>; 10]; 24],
    column_heights: [u8; 10],
}

impl Board {
    #[must_use]
    /// Creates a new board with all cells empty
    pub fn new() -> Board {
        Board {
            board: [[None; 10]; 24],
            column_heights: [0; 10],
        }
    }

    /// Returns the width of the board, which is always 10
    pub const fn width(&self) -> u8 {
        10
    }

    /// Returns the height of the board, which is always 24
    pub const fn height(&self) -> u8 {
        24
    }

    #[must_use]
    /// Returns the string representation of a row of the board
    ///
    /// # Panics
    ///
    /// Panics if `row` is out of bounds, that is if `row >= 20`, the height
    /// of a Tetris board.
    pub fn row_to_string(&self, row: usize) -> String {
        assert!(
            row < self.height() as usize,
            "Index out of bounds: attempted to access row {row}, but only {} exist",
            self.height(),
        );

        let mut row_string = String::new();

        for col in 0..self.width() as usize {
            row_string.push(if self.board[row][col].is_some() {
                '■'
            } else {
                ' '
            });
        }

        row_string
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for row in 0..4 as usize {
            writeln!(f, " {} ", self.row_to_string(row))?;
        }
        for row in 4..self.height() as usize {
            writeln!(f, "|{}|", self.row_to_string(row))?;
        }
        writeln!(f, "+----------+")
    }
}

impl Index<usize> for Board {
    type Output = [Option<()>; 10];

    fn index(&self, row: usize) -> &Self::Output {
        &self.board[row]
    }
}
