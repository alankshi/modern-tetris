use crate::{Board, Piece};

impl Board {
    pub fn hard_drop(&mut self, mut piece: Piece) {
        let drop_y = piece.hard_drop(self.column_heights);

        for pos in piece.get_pos_mask() {
            self.board[pos.y() as usize][pos.x() as usize] = Some(());
        }

        for (i, y_offset) in piece.upper_edge().into_iter().flatten() {
            self.column_heights[(piece.x() + i as i32) as usize] = drop_y - y_offset + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::PieceType;

    #[test]
    fn hard_drop() {
        let mut board = Board::new();

        board.hard_drop(Piece::new(PieceType::I));
        board.hard_drop(Piece::new(PieceType::S));
        board.hard_drop(Piece::new(PieceType::T));
        board.hard_drop(Piece::new(PieceType::L));
        board.hard_drop(Piece::new(PieceType::O));

        println!("{}", board);
        println!("{:?}", board.column_heights);
    }
}
