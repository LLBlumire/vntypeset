pub mod linear_stroke;

pub use stroke::linear_stroke::LinearStroke;

use pixel::Pixel;
use coord::Coord;

/// Models a single type of stroke that can be done.
pub trait Stroke {
    /// Return all the pixels that need to be added between two points.
    fn stroke(&self, start: Coord, end: Coord) -> Vec<Pixel>;

    /// Return all the pixels that need to be added between two points, curving through more
    /// points. Default implementation draws straight connecting lines.
    fn stroke_via(&self, start: Coord, end: Coord, via: &[Coord]) -> Vec<Pixel> {
        let mut output: Vec<Pixel> = Vec::new();
        let mut first = start;
        let mut iter = via.iter();
        while let Some(second) = iter.next() {
            output.append(&mut Stroke::stroke(self, first, *second));
            first = *second;
        }
        output.append(&mut Stroke::stroke(self, first, end));
        output
    }

    /// The width that needs to be padded by so that strokes of this type do not overflow the page.
    fn pad_width(&self) -> u32 {
        0
    }
    /// The height that needs to be padded by so that strokes of this type do not overflow the
    /// page.
    fn pad_height(&self) -> u32 {
        0
    }
}
