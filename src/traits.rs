pub trait Controllable {
    fn rotate_ccw(&mut self);
    fn rotate_cw(&mut self);

    fn move_left(&mut self);
    fn move_right(&mut self);

    fn hard_drop(&mut self);

    // fn hold(&mut self);
}
