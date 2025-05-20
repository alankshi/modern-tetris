use super::Orientation;

use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PieceType {
    S,
    Z,
    L,
    J,
    T,
    I,
    O,
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{self:?}")
    }
}

impl PieceType {
    #[must_use]
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
    /// where the position of the piece is at the top left corner, labeled 0
    /// The array of bytes is sorted in ascending order.
    pub fn mask(&self, orientation: Orientation) -> [u8; 4] {
        match self {
            PieceType::I => match orientation {
                Orientation::North => [4, 5, 6, 7],
                Orientation::East => [2, 6, 10, 14],
                Orientation::South => [8, 9, 10, 11],
                Orientation::West => [1, 5, 9, 13],
            },
            PieceType::L => match orientation {
                Orientation::North => [2, 4, 5, 6],
                Orientation::East => [1, 5, 9, 10],
                Orientation::South => [4, 5, 6, 8],
                Orientation::West => [0, 1, 5, 9],
            },
            PieceType::J => match orientation {
                Orientation::North => [0, 4, 5, 6],
                Orientation::East => [1, 2, 5, 9],
                Orientation::South => [4, 5, 6, 10],
                Orientation::West => [1, 5, 9, 8],
            },
            PieceType::S => match orientation {
                Orientation::North => [1, 2, 4, 5],
                Orientation::East => [1, 5, 6, 10],
                Orientation::South => [5, 6, 8, 9],
                Orientation::West => [0, 4, 5, 9],
            },
            PieceType::Z => match orientation {
                Orientation::North => [0, 1, 5, 6],
                Orientation::East => [2, 6, 5, 9],
                Orientation::South => [4, 5, 9, 10],
                Orientation::West => [1, 4, 5, 8],
            },
            PieceType::T => match orientation {
                Orientation::North => [1, 4, 5, 6],
                Orientation::East => [1, 5, 6, 9],
                Orientation::South => [4, 5, 6, 9],
                Orientation::West => [1, 4, 5, 9],
            },
            PieceType::O => [1, 2, 5, 6],
        }
    }
}
