use drawing;
use std::path::Path;


pub struct SVGImages {
    pub city_image: nsvg::image::RgbaImage,
    pub village_image: nsvg::image::RgbaImage,
    pub stronghold_image: nsvg::image::RgbaImage,
    pub knight_image: nsvg::image::RgbaImage,
    pub scroll_image: nsvg::image::RgbaImage
}

impl SVGImages {
    pub fn new(ddpi: f32, window_width: u32, player_color_spec: drawing::ColorSpec) -> SVGImages {
        let mut city_image = {
            let svg = nsvg::parse_file(Path::new("assets/svg/city.svg"), nsvg::Units::Pixel, ddpi).unwrap();
            let svg_scaling = svg.width() * 2.0 / window_width as f32;
            svg.rasterize(svg_scaling).unwrap()
        };
        for pixel in city_image.pixels_mut() {
            let old_pixel = pixel.clone();
            pixel.data = [player_color_spec.r, player_color_spec.g, player_color_spec.b, old_pixel.data[3]];
        }

        let mut village_image = {
            let svg = nsvg::parse_file(Path::new("assets/svg/village.svg"), nsvg::Units::Pixel, ddpi).unwrap();
            let svg_scaling = svg.width() * 2.0 / window_width as f32;
            svg.rasterize(svg_scaling).unwrap()
        };
        for pixel in village_image.pixels_mut() {
            let old_pixel = pixel.clone();
            pixel.data = [player_color_spec.r, player_color_spec.g, player_color_spec.b, old_pixel.data[3]];
        }

        let mut stronghold_image = {
            let svg = nsvg::parse_file(Path::new("assets/svg/stronghold.svg"), nsvg::Units::Pixel, ddpi).unwrap();
            let svg_scaling = svg.width() * 2.0 / window_width as f32;
            svg.rasterize(svg_scaling).unwrap()
        };
        for pixel in stronghold_image.pixels_mut() {
            let old_pixel = pixel.clone();
            pixel.data = [player_color_spec.r, player_color_spec.g, player_color_spec.b, old_pixel.data[3]];
        }

        let mut knight_image = {
            let svg = nsvg::parse_file(Path::new("assets/svg/knight.svg"), nsvg::Units::Pixel, ddpi).unwrap();
            let svg_scaling = svg.width() * 2.0 / window_width as f32;
            svg.rasterize(svg_scaling).unwrap()
        };
        for pixel in knight_image.pixels_mut() {
            let old_pixel = pixel.clone();
            pixel.data = [player_color_spec.r, player_color_spec.g, player_color_spec.b, old_pixel.data[3]];
        }

        let mut scroll_image = {
            let svg = nsvg::parse_file(Path::new("assets/svg/paper-scroll.svg"), nsvg::Units::Pixel, ddpi).unwrap();
            let svg_scaling = svg.width() * 2.0 / window_width as f32;
            svg.rasterize(svg_scaling).unwrap()
        };
        for pixel in scroll_image.pixels_mut() {
            let old_pixel = pixel.clone();
            pixel.data = [
                (old_pixel.data[0] as f32 * 0.75) as u8,
                (old_pixel.data[1] as f32 * 0.75) as u8,
                (old_pixel.data[2] as f32 * 0.75) as u8,
                old_pixel.data[3]];
        }

        SVGImages {
            city_image: city_image,
            village_image: village_image,
            stronghold_image: stronghold_image,
            knight_image: knight_image,
            scroll_image: scroll_image
        }
    }
}
