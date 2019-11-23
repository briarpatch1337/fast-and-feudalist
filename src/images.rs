use colors::Color;
use std::collections::HashMap;
use std::path::Path;
use PlayerColor;


pub struct SVGImages {
    city_images: HashMap<PlayerColor, nsvg::image::RgbaImage>,
    village_images: HashMap<PlayerColor, nsvg::image::RgbaImage>,
    stronghold_images: HashMap<PlayerColor, nsvg::image::RgbaImage>,
    knight_images: HashMap<PlayerColor, nsvg::image::RgbaImage>,
    pub scroll_image: nsvg::image::RgbaImage
}

impl SVGImages {
    pub fn new(ddpi: f32, window_width: u32) -> SVGImages {
        const PLAYER_COLORS: [PlayerColor; 4] = [PlayerColor::Red, PlayerColor::Blue, PlayerColor::Green, PlayerColor::Yellow];
        let mut city_images = HashMap::new();
        let city_image = {
            let svg = nsvg::parse_file(Path::new("assets/svg/city.svg"), nsvg::Units::Pixel, ddpi).unwrap();
            let svg_scaling = svg.width() * 2.0 / window_width as f32;
            svg.rasterize(svg_scaling).unwrap()
        };
        for player_color in PLAYER_COLORS.iter() {
            let player_color_spec = player_color.color();
            let mut new_city_image = city_image.clone();
            for pixel in new_city_image.pixels_mut() {
                let old_pixel = pixel.clone();
                pixel.data = [player_color_spec.r, player_color_spec.g, player_color_spec.b, old_pixel.data[3]];
            }
            city_images.insert(*player_color, new_city_image);
        }

        let mut village_images = HashMap::new();
        let village_image = {
            let svg = nsvg::parse_file(Path::new("assets/svg/village.svg"), nsvg::Units::Pixel, ddpi).unwrap();
            let svg_scaling = svg.width() * 2.0 / window_width as f32;
            svg.rasterize(svg_scaling).unwrap()
        };
        for player_color in PLAYER_COLORS.iter() {
            let player_color_spec = player_color.color();
            let mut new_village_image = village_image.clone();
            for pixel in new_village_image.pixels_mut() {
                let old_pixel = pixel.clone();
                pixel.data = [player_color_spec.r, player_color_spec.g, player_color_spec.b, old_pixel.data[3]];
            }
            village_images.insert(*player_color, new_village_image);
        }

        let mut stronghold_images = HashMap::new();
        let stronghold_image = {
            let svg = nsvg::parse_file(Path::new("assets/svg/stronghold.svg"), nsvg::Units::Pixel, ddpi).unwrap();
            let svg_scaling = svg.width() * 2.0 / window_width as f32;
            svg.rasterize(svg_scaling).unwrap()
        };
        for player_color in PLAYER_COLORS.iter() {
            let player_color_spec = player_color.color();
            let mut new_stronghold_image = stronghold_image.clone();
            for pixel in new_stronghold_image.pixels_mut() {
                let old_pixel = pixel.clone();
                pixel.data = [player_color_spec.r, player_color_spec.g, player_color_spec.b, old_pixel.data[3]];
            }
            stronghold_images.insert(*player_color, new_stronghold_image);
        }

        let mut knight_images = HashMap::new();
        let knight_image = {
            let svg = nsvg::parse_file(Path::new("assets/svg/knight.svg"), nsvg::Units::Pixel, ddpi).unwrap();
            let svg_scaling = svg.width() * 2.0 / window_width as f32;
            svg.rasterize(svg_scaling).unwrap()
        };
        for player_color in PLAYER_COLORS.iter() {
            let player_color_spec = player_color.color();
            let mut new_knight_image = knight_image.clone();
            for pixel in new_knight_image.pixels_mut() {
                let old_pixel = pixel.clone();
                pixel.data = [player_color_spec.r, player_color_spec.g, player_color_spec.b, old_pixel.data[3]];
            }
            knight_images.insert(*player_color, new_knight_image);
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
            city_images: city_images,
            village_images: village_images,
            stronghold_images: stronghold_images,
            knight_images: knight_images,
            scroll_image: scroll_image
        }
    }

    pub fn get_city_image(&self, player_color_spec: &PlayerColor) -> &nsvg::image::RgbaImage {
        &self.city_images[player_color_spec]
    }

    pub fn get_village_image(&self, player_color_spec: &PlayerColor) -> &nsvg::image::RgbaImage {
        &self.village_images[player_color_spec]
    }

    pub fn get_stronghold_image(&self, player_color_spec: &PlayerColor) -> &nsvg::image::RgbaImage {
        &self.stronghold_images[player_color_spec]
    }

    pub fn get_knight_image(&self, player_color_spec: &PlayerColor) -> &nsvg::image::RgbaImage {
        &self.knight_images[player_color_spec]
    }
}
