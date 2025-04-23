pub struct Piece {
    kind: PieceType,
    rotation: u8,
    position: Position,
}

pub enum PieceType {
    S,
    Z,
    L,
    J,
    T,
    I,
    O,
}

struct Position {
    x: u8,
    y: u8,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }
}

impl Piece {
    pub fn new(kind: PieceType) -> Piece {
        Piece {
            kind,
            rotation: 0,
            position: Position::new(),
        }
    }
}
