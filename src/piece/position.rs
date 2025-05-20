#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    #[must_use]
    /// Constructs a new position at the origin
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    #[must_use]
    /// Constructs a new position at position (x, y)
    pub const fn at(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    #[must_use]
    /// Returns the x-coordinate of the position
    pub fn x(&self) -> i32 {
        self.x
    }

    #[must_use]
    /// Returns the y-coordinate of the position
    pub fn y(&self) -> i32 {
        self.y
    }

    #[must_use]
    /// Sets the y-coordinate of the position
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    #[must_use]
    /// Sets the x-coordinate of the position
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    #[must_use]
    /// Increments the x-coordinate of the position by 1
    pub fn move_right(&mut self) {
        self.x += 1;
    }

    #[must_use]
    /// Decrements the x-coordinate of the position by 1
    pub fn move_left(&mut self) {
        self.x -= 1;
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}
