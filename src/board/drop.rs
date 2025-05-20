use crate::{Board, Piece};

impl Board {
    pub fn hard_drop(&mut self, piece: Piece) {
        let mask = piece.get_mask();

        let mut lower_edge = [None; 4];
        for cell_idx in mask {
            let i = (cell_idx % 4) as usize;
            lower_edge[i] = Some((i, cell_idx / 4));
        }

        let mut drop_y = 0;

        for (i, y) in lower_edge.into_iter().flatten() {
            // `piece.x() + i` will always be positive because the rules of
            // piece movement (no cells can be outside the board)
            let new_drop_y = y + self.column_heights[(piece.x() + i as i32) as usize];

            if new_drop_y > drop_y {
                drop_y = new_drop_y;
            }
        }
        println!("{drop_y}");

        // set the cells on the board at the specified location to Some
        for pos in mask {
            // TODO: Make a function for piece that returns 4 Positions for its
            // cell positions
            let rel_x = pos % 4;
            let rel_y = pos / 4;

            let abs_x = (piece.x() + rel_x as i32) as usize;
            let abs_y = (23 - drop_y + rel_y) as usize;

            self.board[abs_y][abs_x] = Some(());
        }

        // update column_heights
        let mut upper_edge = [None; 4];
        for cell_idx in mask.iter().rev() {
            let i = (cell_idx % 4) as usize;
            upper_edge[i] = Some((i, cell_idx / 4));
        }

        for (i, y_offset) in upper_edge.into_iter().flatten() {
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
