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
