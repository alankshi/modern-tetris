use crate::{Piece, TetrisError};

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
