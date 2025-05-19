pub mod movement;
pub mod util;

use crate::board::Board;
pub use util::{DEFAULT_POSITION, PieceType, UNIQUE_TYPES};
use util::{Orientation, Position, TetrisError};

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
            position: DEFAULT_POSITION,
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
    /// where the position of the piece is at the top left corner, labeled 0
    /// The array of bytes is sorted in ascending order.
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

    /// Returns the PieceType of the piece
    pub fn kind(&self) -> PieceType {
        self.kind
    }

    /// Returns the orientation of the piece
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Returns the position of the piece
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
}
