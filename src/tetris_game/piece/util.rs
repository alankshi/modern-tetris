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

pub const UNIQUE_PIECE_TYPES: [PieceType; 7] = [
    PieceType::I,
    PieceType::L,
    PieceType::J,
    PieceType::S,
    PieceType::Z,
    PieceType::T,
    PieceType::O,
];

#[derive(Debug)]
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
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    #[must_use]
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}
