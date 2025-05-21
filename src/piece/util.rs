use std::fmt::{Display, Error, Formatter};

use super::Piece;
use super::piece_type::PieceType;
use super::position::Position;

pub const DEFAULT_POSITION: Position = Position::at(3, 1);
pub const DEFAULT_ORIENTATION: Orientation = Orientation::North;
pub const UNIQUE_TYPES: [PieceType; 7] = [
    PieceType::I,
    PieceType::L,
    PieceType::J,
    PieceType::S,
    PieceType::Z,
    PieceType::T,
    PieceType::O,
];

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    #[must_use]
    /// Returns the direction clockwise of `self`.
    pub fn clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    #[must_use]
    /// Returns the direction clockwise of `self`.
    pub fn counterclockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{self:?}")
    }
}

impl Piece {
    #[must_use]
    /// Returns the positions of the piece's filled cells.
    pub fn get_pos_mask(&self) -> [Position; 4] {
        let mut pos_mask = [Position::new(); 4];

        for (i, pos) in self.mask().into_iter().enumerate() {
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

        for cell_idx in self.mask().iter().rev() {
            let i = (cell_idx / 4) as usize;
            left_edge[i] = Some((i, cell_idx % 4));
        }

        left_edge
    }

    pub fn right_edge(&self) -> [Option<(usize, u8)>; 4] {
        let mut right_edge = [None; 4];

        for cell_idx in self.mask() {
            let i = (cell_idx / 4) as usize;
            right_edge[i] = Some((i, cell_idx % 4));
        }

        right_edge
    }

    pub fn lower_edge(&self) -> [Option<(usize, u8)>; 4] {
        let mut lower_edge = [None; 4];

        for cell_idx in self.mask() {
            let i = (cell_idx % 4) as usize;
            lower_edge[i] = Some((i, cell_idx / 4));
        }

        lower_edge
    }

    pub fn upper_edge(&self) -> [Option<(usize, u8)>; 4] {
        let mut upper_edge = [None; 4];

        for cell_idx in self.mask().iter().rev() {
            let i = (cell_idx % 4) as usize;
            upper_edge[i] = Some((i, cell_idx / 4));
        }

        upper_edge
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut curr_cell = 0;
        let mask = self.mask();

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

impl From<PieceType> for Piece {
    fn from(item: PieceType) -> Self {
        Self::new(item)
    }
}
