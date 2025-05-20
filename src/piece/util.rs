use std::fmt::{Display, Error, Formatter};

use super::Piece;
use super::position::Position;

pub const DEFAULT_POSITION: Position = Position::at(3, 1);
pub const UNIQUE_TYPES: [PieceType; 7] = [
    PieceType::I,
    PieceType::L,
    PieceType::J,
    PieceType::S,
    PieceType::Z,
    PieceType::T,
    PieceType::O,
];

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

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{self:?}")
    }
}

impl Orientation {
    /// Returns the direction clockwise of `self`.
    #[must_use]
    pub fn clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    /// Returns the direction clockwise of `self`.
    #[must_use]
    pub fn counterclockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
}

#[derive(Debug)]
pub enum TetrisError<'a> {
    InvalidMove(&'a str),
    InvalidRotation(&'a str),
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

impl Piece {
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

    #[must_use]
    pub fn get_pos_mask(&self) -> [Position; 4] {
        let mut pos_mask = [Position::new(); 4];

        for (i, pos) in self.get_mask().into_iter().enumerate() {
            let cell_x_offset = (pos % 4) as i32;
            let cell_y_offset = (pos / 4) as i32;

            let cell_x = self.x() + cell_x_offset;
            let cell_y = 23 - self.y() + cell_y_offset;

            pos_mask[i] = Position::at(cell_x, cell_y);
        }

        pos_mask
    }

    pub fn left_edge(&self) -> [Option<(usize, u8)>; 4] {
        let mut left_edge = [None; 4];

        for cell_idx in self.get_mask().iter().rev() {
            let i = (cell_idx / 4) as usize;
            left_edge[i] = Some((i, cell_idx % 4));
        }

        left_edge
    }

    pub fn right_edge(&self) -> [Option<(usize, u8)>; 4] {
        let mut right_edge = [None; 4];

        for cell_idx in self.get_mask() {
            let i = (cell_idx / 4) as usize;
            right_edge[i] = Some((i, cell_idx % 4));
        }

        right_edge
    }

    pub fn lower_edge(&self) -> [Option<(usize, u8)>; 4] {
        let mut lower_edge = [None; 4];

        for cell_idx in self.get_mask() {
            let i = (cell_idx % 4) as usize;
            lower_edge[i] = Some((i, cell_idx / 4));
        }

        lower_edge
    }

    pub fn upper_edge(&self) -> [Option<(usize, u8)>; 4] {
        let mut upper_edge = [None; 4];

        for cell_idx in self.get_mask().iter().rev() {
            let i = (cell_idx % 4) as usize;
            upper_edge[i] = Some((i, cell_idx / 4));
        }

        upper_edge
    }
}
