use std::fmt::{Display, Error, Formatter};

pub struct Board {
    board: [[bool; 10]; 20],
}

impl Board {
    #[must_use]
    pub fn new() -> Board {
        Board {
            board: [[false; 10]; 20],
        }
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
            row < 20,
            "Index out of bounds: attempted to access row {row} of 20",
        );

        let mut row_string = String::new();
        row_string.push('|');

        for col in 0..10 {
            row_string.push(if self.board[row][col] { '■' } else { ' ' });
        }

        row_string.push_str("|\n");
        row_string
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "+----------+")?;
        for row in 0..20 {
            write!(f, "{}", self.row_to_string(row))?;
        }
        writeln!(f, "+----------+")
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
