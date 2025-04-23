use std::fmt::{Display, Error, Formatter};

pub struct Board {
    board: [[bool; 10]; 20],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[false; 10]; 20],
        }
    }

    pub fn row_to_string(&self, row: usize) -> String {
        if row >= 20 {
            panic!("Index out of bounds: attempted to access row {} of 20", row)
        }

        let mut row_string = String::new();
        row_string.push('|');

        for col in 0..10 {
            row_string.push(if self.board[row][col] { '■' } else { ' ' });
        }

        row_string.push_str("|\n");
        return row_string;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "+----------+\n")?;
        for row in 0..20 {
            write!(f, "{}", self.row_to_string(row))?;
        }
        write!(f, "+----------+\n")
    }
}
