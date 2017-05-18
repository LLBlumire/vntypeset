pub mod logo_k;

pub use character::logo_k::LogoK;

use image::GenericImage;
use image::Luma;
use stroke::Stroke;

pub trait Character {
    fn draw_character<G, S>(&self, region: &mut G, stroke: &S)
        where G: GenericImage<Pixel = Luma<u8>>,
              S: Stroke;
}
