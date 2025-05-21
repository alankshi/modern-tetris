use crate::{TetrisError, TetrisInput};

pub trait Controllable {
    fn rotate_ccw(&mut self) -> Result<(), TetrisError>;
    fn rotate_cw(&mut self) -> Result<(), TetrisError>;
    fn rotate_180(&mut self) -> Result<(), TetrisError>;

    fn move_left(&mut self) -> Result<(), TetrisError>;
    fn move_right(&mut self) -> Result<(), TetrisError>;

    fn hold(&mut self) -> Result<(), TetrisError>;

    fn hard_drop(&mut self) -> Result<(), TetrisError>;

    fn execute_input(&mut self, input: TetrisInput) -> Result<(), TetrisError> {
        match input {
            TetrisInput::SoftDrop => {
                todo!()
            }
            TetrisInput::HardDrop => self.hard_drop(),
            TetrisInput::MoveLeft => self.move_left(),
            TetrisInput::MoveRight => self.move_right(),
            TetrisInput::SnapRight => {
                todo!()
            }
            TetrisInput::SnapLeft => {
                todo!()
            }
            TetrisInput::RotateCW => self.rotate_cw(),
            TetrisInput::RotateCCW => self.rotate_ccw(),
            TetrisInput::Rotate180 => self.rotate_180(),
            TetrisInput::Hold => self.hold(),
        }
    }
}

pub trait Game {
    fn start(&mut self);
    fn step(&mut self, dt: f64);
    fn next_frame(&mut self);
}
