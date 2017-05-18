use coord::Coord;
use pixel::Pixel;
use stroke::Stroke;

use std::cmp::max;
use std::mem::swap;

/// A single linear stroke, filling in a square grid of pixles of width radius around the center.
pub struct LinearStroke {
    width: u32,
}
impl LinearStroke {
    pub fn new(width: u32) -> LinearStroke {
        LinearStroke { width: width }
    }
}
impl Stroke for LinearStroke {
    fn pad_width(&self) -> u32 {
        self.width
    }
    fn stroke(&self, start: Coord, end: Coord) -> Vec<Pixel> {
        let mut output = Vec::new();
        let mut lesser_x = start;
        let mut greater_x = end;
        if lesser_x.x() > greater_x.x() {
            swap(&mut lesser_x, &mut greater_x)
        };
        let gradient_x = (greater_x.y() as f64 - lesser_x.y() as f64) /
                         (greater_x.x() as f64 - lesser_x.x() as f64);
        let mut lesser_y = start;
        let mut greater_y = end;
        if lesser_y.y() > greater_y.y() {
            swap(&mut lesser_y, &mut greater_y)
        };
        let gradient_y = (greater_y.x() as f64 - lesser_y.x() as f64) /
                         (greater_y.y() as f64 - lesser_y.y() as f64);
        output.push(Pixel::black(start));
        output.push(Pixel::black(end));
        let mut current_x;
        let mut current_y;
        let greater_dominant;
        let dx;
        let dy;
        if gradient_x > 1.0 {
            current_x = lesser_x.x() as f64;
            current_y = lesser_x.y() as f64;
            greater_dominant = greater_y.y();
            dx = gradient_y;
            dy = 1.0;
        } else {
            current_x = lesser_y.x() as f64;
            current_y = lesser_y.y() as f64;
            greater_dominant = greater_x.x();
            dx = 1.0;
            dy = gradient_x;
        }
        while greater_dominant !=
              if gradient_x > 1.0 {
                current_y
            } else {
                current_x
            }
            .floor() as u32 {
            current_x += dx;
            current_y += dy;
            for y in (max(0,
                          current_y.floor() as i64 -
                          (self.width as i64 -
                           1)) as u32)..(current_y.floor() as u32 +
                                                              (self.width - 1)) {
                for x in (max(0,
                              current_x.floor() as i64 -
                              (self.width as i64 -
                               1)) as u32)..(current_x.floor() as u32 +
                                                                      (self.width - 1)) {
                    output.push(Pixel::black(Coord::new(x, y)));
                }
            }
        }
        output
    }
}
