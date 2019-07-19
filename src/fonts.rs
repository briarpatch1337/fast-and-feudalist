
use freetype;
use drawing;
use std::path::Path;

pub struct FontResources {
    _freetype_lib: freetype::library::Library,
    pub cardinal_font_face: freetype::face::Face,
    pub text_cache: drawing::TextCache,
}

impl FontResources {
    pub fn new() -> FontResources {
        let freetype_lib = freetype::Library::init().unwrap();
        let cardinal_font_face = freetype_lib.new_face(Path::new("assets/fonts/Cardinal.ttf"), 0).unwrap();
        FontResources {
            _freetype_lib: freetype_lib,
            cardinal_font_face: cardinal_font_face,
            text_cache: drawing::TextCache::new()
        }
    }
}
