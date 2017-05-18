extern crate image;

mod coord;
mod pixel;
mod stroke;
mod character;

use stroke::LinearStroke;
use image::GenericImage;
use image::ImageBuffer;
use image::Luma;
use character::Character;
use character::LogoK;

fn main() {
    let mut img = ImageBuffer::from_pixel(40, 40, Luma { data: [255] });

    LogoK.draw_character(&mut img.sub_image(0, 0, 40, 40), &LinearStroke::new(2));

    img.save("foo.png").unwrap();
}
