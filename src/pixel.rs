use coord::Coord;
use image::Luma;

/// Represetns a coloured pixel in the cartesian plane.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Pixel {
    coord: Coord,
    color: u8,
}
impl Pixel {
    /// Access the coordiante
    pub fn coord(&self) -> Coord {
        self.coord
    }
    /// Access the color as a Luma
    pub fn luma(&self) -> Luma<u8> {
        Luma { data: [self.color] }
    }
    /// Create a new black pixel
    pub fn black(coord: Coord) -> Pixel {
        Pixel {
            coord: coord,
            color: 0,
        }
    }
}
