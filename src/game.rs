mod controller;

use crate::piece::DEFAULT_ORIENTATION;
use crate::{Bag, Board, Piece, PieceType};

use std::collections::VecDeque;
use std::fmt::{Display, Error, Formatter};

pub struct SprintGame {
    board: Board,
    hold_piece: Option<PieceType>,
    active_piece: Option<Piece>,
    piece_queue: VecDeque<PieceType>,
    bag: Bag,

    can_hold: bool,
    queue_size: usize,
}

impl SprintGame {
    /// Creates a new sprint game with a specified bag and queue size
    #[must_use]
    pub fn new(bag_size: u32, queue_size: usize) -> Self {
        SprintGame {
            board: Board::new(),
            hold_piece: None,
            active_piece: None,
            piece_queue: VecDeque::new(),
            bag: Bag::new(bag_size),

            can_hold: true,
            queue_size,
        }
    }

    /// Creates a new TETR.IO sprint game with a bag size of 7 and queue size of
    /// 5
    #[must_use]
    pub fn new_tetrio() -> Self {
        SprintGame {
            board: Board::new(),
            hold_piece: None,
            active_piece: None,
            piece_queue: VecDeque::new(),

            bag: Bag::new(7),
            can_hold: true,
            queue_size: 5,
        }
    }

    /// Fills the queue by drawing pieces from the game's bag until
    /// `self.piece_queue.len() == self.queue_size`
    pub fn fill_queue(&mut self) {
        while self.piece_queue.len() < self.queue_size {
            self.piece_queue.push_back(self.bag.draw());
        }
    }

    /// Returns a reference to the game's board, `self.board`
    #[must_use]
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Returns the type of the current hold piece if there is one, else `None`
    #[must_use]
    pub fn hold_piece(&self) -> Option<PieceType> {
        self.hold_piece
    }

    /// Returns a reference to the current active piece if there is one, else
    /// `None`
    #[must_use]
    pub fn active_piece(&self) -> Option<&Piece> {
        self.active_piece.as_ref()
    }

    /// Returns a reference to the upcoming piece queue
    #[must_use]
    pub fn piece_queue(&self) -> &VecDeque<PieceType> {
        &self.piece_queue
    }

    fn load_next_piece(&mut self) {
        if self.active_piece.is_some() {
            panic!(
                "Cannot load next piece while current piece, {}, still exists.",
                self.active_piece().unwrap().kind()
            )
        }

        self.active_piece = Some(Piece::new(self.piece_queue.pop_front().unwrap()));
        self.fill_queue();
    }

    pub fn start(&mut self) {
        self.fill_queue();
        self.load_next_piece();
    }
}

impl Default for SprintGame {
    fn default() -> Self {
        Self::new(7, 5)
    }
}

impl Display for SprintGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut board_display = [[' '; 12]; 24];

        for row in 0..24 {
            if row >= 4 {
                board_display[row][0] = '|';
                board_display[row][11] = '|';
            }

            for (col, char) in self.board.row_to_string(row).chars().enumerate() {
                board_display[row][col + 1] = char;
            }
        }
        // Overlay active piece
        if let Some(piece) = &self.active_piece {
            let pos_mask = piece.get_pos_mask();
            for pos in pos_mask {
                board_display[pos.y() as usize][pos.x() as usize + 1] = '■';
            }
        }

        let mut hold_display = [
            [' ', 'H', 'o', 'l', 'd', ' ', ' '],
            ['+', '-', '-', '-', '-', '+', ' '],
            ['|', ' ', ' ', ' ', ' ', '|', ' '],
            ['|', ' ', ' ', ' ', ' ', '|', ' '],
            ['+', '-', '-', '-', '-', '+', ' '],
        ];
        if let Some(piece_type) = self.hold_piece {
            for pos in piece_type.mask(DEFAULT_ORIENTATION) {
                let row = (pos / 4) as usize;
                let col = (pos % 4) as usize;
                hold_display[row + 2][col + 1] = '■';
            }
        }

        let mut next_display = [
            [' ', ' ', 'N', 'e', 'x', 't', ' '],
            [' ', '+', '-', '-', '-', '-', '+'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '|', ' ', ' ', ' ', ' ', '|'],
            [' ', '+', '-', '-', '-', '-', '+'],
        ];

        for (i, piece_type) in self.piece_queue().iter().enumerate() {
            for pos in piece_type.mask(DEFAULT_ORIENTATION) {
                let row = (pos / 4) as usize;
                let col = (pos % 4) as usize;
                next_display[row + 2 + 3 * i][col + 2] = '■';
            }
        }

        for row in 0..24 {
            if 4 <= row && row <= 8 {
                for col in 0..7 {
                    write!(f, "{}", hold_display[row - 4][col])?;
                }
            } else {
                write!(f, "       ")?;
            }

            for col in 0..12 {
                write!(f, "{}", board_display[row][col])?;
            }

            if 4 <= row && row <= 21 {
                for col in 0..7 {
                    write!(f, "{}", next_display[row - 4][col])?;
                }
            }

            writeln!(f, "")?;
        }
        writeln!(f, "       +----------+")?;

        Ok(())
    }
}
