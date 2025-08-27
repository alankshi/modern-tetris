use crate::{Piece, TetrisError, piece::Position};

enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

impl Piece {
    /// Rotate the piece clockwise, changing its orientation accordingly
    pub fn rotate_cw(&mut self) -> Result<(), TetrisError> {
        self.orientation = self.orientation.clockwise();
        self.mask = self.kind.mask(self.orientation);
        Ok(())
    }

    /// Rotate the piece counterclockwise, changing its orientation accordingly
    pub fn rotate_ccw(&mut self) -> Result<(), TetrisError> {
        self.orientation = self.orientation.counterclockwise();
        self.mask = self.kind.mask(self.orientation);
        Ok(())
    }

    /// Rotate the piece 180 degrees, changing its orientation accordingly
    pub fn rotate_180(&mut self) -> Result<(), TetrisError> {
        self.orientation = self.orientation.opposite();
        self.mask = self.kind.mask(self.orientation);
        Ok(())
    }
}

fn get_offsets(piece: &Piece, direction: RotationDirection) -> impl Iterator<Item = Position> {
    [
        Position::at(0, 0),
        Position::at(0, 0),
        Position::at(0, 0),
        Position::at(0, 0),
        Position::at(0, 0),
    ]
    .into_iter()
}
