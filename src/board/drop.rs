use crate::{Board, Piece, TetrisError};

impl Board {
    /// Hard drops the passed piece, consuming it, then updates the board to
    /// represent the placed piece.
    pub fn hard_drop(&mut self, mut piece: Piece) -> Result<(), TetrisError> {
        piece.hard_drop(self.column_heights)?;

        for pos in piece.get_pos_mask() {
            self.board[pos.y() as usize][pos.x() as usize] = Some(());
        }

        for (i, y_offset) in piece.upper_edge().into_iter().flatten() {
            // -piece.y() + 23 converts piece y-coordinate (where higher is
            // lower on the board because arrays) to height, actual y-coordinate
            self.column_heights[(piece.x() + i as i32) as usize] =
                (-piece.y() + 23) as u8 - y_offset + 1;
        }

        let lines_cleared = self.clear_lines(piece);

        Ok(())
    }

    fn clear_lines(&mut self, piece: Piece) -> u8 {
        let mut lines_cleared: u8 = 0;

        for (i, edge) in piece.left_edge().iter().rev().enumerate() {
            let row = (piece.y() + 3) as usize - i;

            if edge.is_some() && self.board[row].iter().all(|cell| cell.is_some()) {
                lines_cleared += 1;
                for cell in self.board[row].iter_mut() {
                    *cell = None;
                }
            } else if row < self.height() as usize && lines_cleared > 0 {
                for i in 0..self.width() as usize {
                    self.board[row + lines_cleared as usize][i] = self.board[row][i];
                }
            }
        }

        if lines_cleared > 0 {
            for row in (0..piece.y() as usize).rev() {
                for i in 0..self.width() as usize {
                    self.board[row + lines_cleared as usize][i] = self.board[row][i];
                }
            }

            for col in self.column_heights.iter_mut() {
                *col -= lines_cleared;
            }
        }
        if lines_cleared > 0 {
            println!("{lines_cleared}");
        }
        return lines_cleared;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::PieceType;

    #[test]
    fn empty_board_drop_flat_i_center() {
        let mut board = Board::new();
        let mut correct_col_heights = [0u8; 10];

        for i in 0..22 {
            let _ = board.hard_drop(Piece::new(PieceType::I));
            correct_col_heights[3] += 1;
            correct_col_heights[4] += 1;
            correct_col_heights[5] += 1;
            correct_col_heights[6] += 1;

            assert_eq!(
                correct_col_heights,
                board.column_heights,
                "Incorrect column heights after placing {} flat I-pieces. Expected {:?}, was {:?}",
                i + 1,
                correct_col_heights,
                board.column_heights,
            );
        }
    }
}
