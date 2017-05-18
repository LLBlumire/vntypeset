use character::Character;
use coord::Coord;
use image::GenericImage;
use image::Luma;
use stroke::Stroke;


/// Draws the `kah` character.
pub struct LogoK;
impl Character for LogoK {
    fn draw_character<G, S>(&self, region: &mut G, stroke: &S)
        where G: GenericImage<Pixel = Luma<u8>>,
              S: Stroke
    {

        let (width, height) = region.dimensions();
        let origin_x = stroke.pad_width();
        let origin_y = stroke.pad_width();
        let width = width - 2 * stroke.pad_width();
        let height = height - 2 * stroke.pad_width();
        let end_x = origin_x + width - 1;
        let end_y = origin_y + height - 1;

        for pixel in stroke.stroke(Coord::new(origin_x, origin_y), Coord::new(end_x, origin_y))
            .iter()
            .chain(stroke.stroke_via(Coord::new(origin_x, origin_y),
                            Coord::new(origin_x + (width / 2), end_y),
                            &[Coord::new(origin_x, end_y)])
                .iter())
            .chain(stroke.stroke(Coord::new(origin_x + (width / 2), origin_y),
                        Coord::new(origin_x + (width / 2), end_y))
                .iter()) {

            region.put_pixel(pixel.coord().x(), pixel.coord().y(), pixel.luma());
        }
    }
}
