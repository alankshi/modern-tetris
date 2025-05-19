use std::fmt::{Display, Error, Formatter};

pub const DEFAULT_POSITION: Position = Position::at(3, 1);

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
    pub x: i32,
    pub y: i32,
}

impl Position {
    #[must_use]
    /// Constructs a new position at the origin
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    #[must_use]
    /// Constructs a new position at position (x, y)
    pub const fn at(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum TetrisError<'a> {
    InvalidMove(&'a str),
    InvalidRotation(&'a str),
}
