/// Represents a single cartesian coordinate.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Coord {
    x: u32,
    y: u32,
}
impl Coord {
    /// Access the X coordinate.
    pub fn x(&self) -> u32 {
        self.x
    }
    /// Access the y coordinate.
    pub fn y(&self) -> u32 {
        self.y
    }

    /// Create a new Coordinate pair.
    pub fn new(x: u32, y: u32) -> Coord {
        Coord { x: x, y: y }
    }
}
