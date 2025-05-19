use std::fmt::{Display, Error, Formatter};
use std::ops::Index;

pub struct Board {
    board: [[Option<()>; 10]; 24],
}

impl Board {
    #[must_use]
    /// Creates a new board with all cells empty
    pub fn new() -> Board {
        Board {
            board: [[None; 10]; 24],
        }
    }

    #[must_use]
    /// Returns the string representation of a row of the board
    ///
    /// # Panics
    ///
    /// Panics if `row` is out of bounds, that is if `row >= 20`, the height
    /// of a Tetris board.
    fn row_to_string(&self, row: usize) -> String {
        assert!(
            row < self.height() as usize,
            "Index out of bounds: attempted to access row {row}, but only {} exist",
            self.height(),
        );

        let mut row_string = String::new();
        row_string.push('|');

        for col in 0..self.width() as usize {
            row_string.push(if self.board[row][col].is_some() {
                '■'
            } else {
                ' '
            });
        }

        row_string.push_str("|\n");
        row_string
    }

    pub const fn width(&self) -> u8 {
        10
    }

    pub const fn height(&self) -> u8 {
        24
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "+----------+")?;
        for row in 0..self.height() as usize {
            write!(f, "{}", self.row_to_string(row))?;
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
