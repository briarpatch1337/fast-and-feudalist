// This file is going to be heavy on the comments, because I am still learning Rust and OpenGL.

//
// extern crates
//

// gl is an OpenGL function pointer loader for the Rust Programming Language.
// Its load_with function is important.  You pass it a loader function from the context library; in this case, sdl2.
// gl will forward OpenGL function calls to SDL.
extern crate gl;

// sdl2 is a library that allows you low level access to a video framebuffer, audio output,
// mouse, keyboard, and joysticks across a wide variety of operating systems.
// You can use the OpenGL API in combination with SDL, for 2D and 3D graphics.
// SDL provides exported functions that implement the OpenGL spec.
extern crate sdl2;

extern crate freetype;
extern crate glm;
extern crate nsvg;
extern crate rand;

// Working around what seems like a bug in one of our dependencies (or build toolchain)
#[link(name = "shell32")]
extern { }


//
// Other source files
//

// This tells the Rust compiler to compile render_gl.rs.  This module has some of the helper functions for rendering our game.
pub mod render_gl;

// Also there's resources.rs
pub mod resources;

// And some more
pub mod drawing;
pub mod colors;
pub mod gameboard;
pub mod mouse_position;

use colors::Color;
use mouse_position::{MousePos, mouse_pos_to_game_board_pos, mouse_pos_to_board_piece_destination};
use rand::Rng;
use resources::Resources;
use gameboard::{BoardPiece,GameBoardSpaceType,GameBoardSpacePos,game_board_pos_to_drawing_pos};
use std::path::Path;

#[derive(Clone,PartialEq)]
enum PlayerColor
{
    Red,
    Blue,
    Green,
    Yellow
}

mod drawing_constants {
    use game_constants;

    pub const HEXAGON_WIDTH: f32 = 0.2;

    // Because of the way the hexagons are staggered, the x spacing of columns is 3/4 of a hexagon width.
    pub const HEXAGON_X_SPACING: f32 = HEXAGON_WIDTH * 0.75;
    pub const GAME_BOARD_ORIGIN_X: f32 = -1.0 * (game_constants::MAX_BOARD_WIDTH / 2) as f32 * HEXAGON_X_SPACING - (HEXAGON_WIDTH / 2.0);

    // The height of a hexagon (turned with the points to the side) is width * sqrt(3) / 2.
    // sqrt(3) / 2 = 0.8660254
    pub const HEXAGON_HEIGHT: f32 =  HEXAGON_WIDTH * 0.8660254_f32;
    pub const HEXAGON_Y_SPACING: f32 = HEXAGON_HEIGHT;
    pub const GAME_BOARD_ORIGIN_Y: f32 = -1.0 * (game_constants::MAX_BOARD_HEIGHT / 2) as f32 * HEXAGON_Y_SPACING - (HEXAGON_HEIGHT * 3.0 / 4.0);
}


fn scaling_for_board(drawable_size: (u32, u32)) -> (f32, f32) {
    let (window_width, window_height) = drawable_size;
    let aspect_ratio = window_width as f32 / window_height as f32;

    // aspect_ratio is W/H
    let mut x_scale = 1_f32;
    let mut y_scale = 1_f32;
    if aspect_ratio >= 1.0 {
        x_scale = 1.0 / aspect_ratio;
    } else {
        y_scale = aspect_ratio;
    }

    (x_scale, y_scale)
}


fn draw_game_board_space(gl: &gl::Gl, shader_program: &render_gl::Program, space_type: GameBoardSpaceType, position: GameBoardSpacePos) {
    match space_type {
        GameBoardSpaceType::Void => {},
        _ => {
            drawing::draw_hexagon(&gl, &shader_program, drawing::HexagonSpec {
            color: space_type.color(),
            pos: game_board_pos_to_drawing_pos(position),
            width: drawing_constants::HEXAGON_WIDTH } );
        }
    }
}


fn highlight_space_for_board_setup(gl: &gl::Gl, shader_program: &render_gl::Program, space_type: GameBoardSpaceType, position: GameBoardSpacePos) {
    match space_type {
        GameBoardSpaceType::Void => {
            drawing::draw_hexagon(&gl, &shader_program, drawing::HexagonSpec {
                color: drawing::ColorSpec { r: 0xFF, g: 0xFF, b: 0xFF },
                pos: game_board_pos_to_drawing_pos(position),
                width: drawing_constants::HEXAGON_WIDTH } );
            drawing::draw_hexagon_outline(
                &gl,
                &shader_program,
                drawing::HexagonSpec {
                    color: drawing::ColorSpec { r: 0x00, g: 0x00, b: 0x00 },
                    pos: game_board_pos_to_drawing_pos(position),
                    width: drawing_constants::HEXAGON_WIDTH },
                3.0);
        },
        _ => {
            drawing::draw_hexagon_outline(
                &gl,
                &shader_program,
                drawing::HexagonSpec {
                    color: drawing::ColorSpec { r: 0xFF, g: 0x00, b: 0x00 },
                    pos: game_board_pos_to_drawing_pos(position),
                    width: drawing_constants::HEXAGON_WIDTH },
                3.0);
        }
    }
}


mod game_constants {
    use GameBoardSpaceType;
    use BoardPiece;

    pub const BOARD_PIECES: [BoardPiece; 36] = [
    // Mostly Mountain (6)
        BoardPiece { a: GameBoardSpaceType::Mountain, b: GameBoardSpaceType::Mountain, c: GameBoardSpaceType::Mountain },
        BoardPiece { a: GameBoardSpaceType::Water, b: GameBoardSpaceType::Mountain, c: GameBoardSpaceType::Mountain },
        BoardPiece { a: GameBoardSpaceType::Water, b: GameBoardSpaceType::Mountain, c: GameBoardSpaceType::Mountain },
        BoardPiece { a: GameBoardSpaceType::Forest, b: GameBoardSpaceType::Mountain, c: GameBoardSpaceType::Mountain },
        BoardPiece { a: GameBoardSpaceType::Plains, b: GameBoardSpaceType::Mountain, c: GameBoardSpaceType::Mountain },
        BoardPiece { a: GameBoardSpaceType::Field, b: GameBoardSpaceType::Mountain, c: GameBoardSpaceType::Mountain },
    // Mostly Field (6)
        BoardPiece { a: GameBoardSpaceType::Field, b: GameBoardSpaceType::Field, c: GameBoardSpaceType::Field },
        BoardPiece { a: GameBoardSpaceType::Water, b: GameBoardSpaceType::Field, c: GameBoardSpaceType::Field },
        BoardPiece { a: GameBoardSpaceType::Water, b: GameBoardSpaceType::Field, c: GameBoardSpaceType::Field },
        BoardPiece { a: GameBoardSpaceType::Mountain, b: GameBoardSpaceType::Field, c: GameBoardSpaceType::Field },
        BoardPiece { a: GameBoardSpaceType::Forest, b: GameBoardSpaceType::Field, c: GameBoardSpaceType::Field },
        BoardPiece { a: GameBoardSpaceType::Plains, b: GameBoardSpaceType::Field, c: GameBoardSpaceType::Field },
    // Mostly Plains (7)
        BoardPiece { a: GameBoardSpaceType::Plains, b: GameBoardSpaceType::Plains, c: GameBoardSpaceType::Plains },
        BoardPiece { a: GameBoardSpaceType::Plains, b: GameBoardSpaceType::Plains, c: GameBoardSpaceType::Plains },
        BoardPiece { a: GameBoardSpaceType::Water, b: GameBoardSpaceType::Plains, c: GameBoardSpaceType::Plains },
        BoardPiece { a: GameBoardSpaceType::Water, b: GameBoardSpaceType::Plains, c: GameBoardSpaceType::Plains },
        BoardPiece { a: GameBoardSpaceType::Mountain, b: GameBoardSpaceType::Plains, c: GameBoardSpaceType::Plains },
        BoardPiece { a: GameBoardSpaceType::Forest, b: GameBoardSpaceType::Plains, c: GameBoardSpaceType::Plains },
        BoardPiece { a: GameBoardSpaceType::Field, b: GameBoardSpaceType::Plains, c: GameBoardSpaceType::Plains },
    // Mostly Forest (8)
        BoardPiece { a: GameBoardSpaceType::Forest, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Forest },
        BoardPiece { a: GameBoardSpaceType::Forest, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Forest },
        BoardPiece { a: GameBoardSpaceType::Water, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Forest },
        BoardPiece { a: GameBoardSpaceType::Water, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Forest },
        BoardPiece { a: GameBoardSpaceType::Mountain, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Forest },
        BoardPiece { a: GameBoardSpaceType::Plains, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Forest },
        BoardPiece { a: GameBoardSpaceType::Field, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Forest },
        BoardPiece { a: GameBoardSpaceType::Plains, b: GameBoardSpaceType::Field, c: GameBoardSpaceType::Forest },
    // Mixed (9)
        BoardPiece { a: GameBoardSpaceType::Field, b: GameBoardSpaceType::Plains, c: GameBoardSpaceType::Mountain },
        BoardPiece { a: GameBoardSpaceType::Water, b: GameBoardSpaceType::Plains, c: GameBoardSpaceType::Mountain },
        BoardPiece { a: GameBoardSpaceType::Field, b: GameBoardSpaceType::Mountain, c: GameBoardSpaceType::Water },
        BoardPiece { a: GameBoardSpaceType::Plains, b: GameBoardSpaceType::Field, c: GameBoardSpaceType::Water },
        BoardPiece { a: GameBoardSpaceType::Plains, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Mountain },
        BoardPiece { a: GameBoardSpaceType::Field, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Mountain },
        BoardPiece { a: GameBoardSpaceType::Mountain, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Water },
        BoardPiece { a: GameBoardSpaceType::Plains, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Water },
        BoardPiece { a: GameBoardSpaceType::Field, b: GameBoardSpaceType::Forest, c: GameBoardSpaceType::Water },
    ];

    pub const MAX_BOARD_HEIGHT: usize = 7;
    pub const MAX_BOARD_WIDTH: usize = 13;
}


enum GameStage
{
    SetupBoard,
    SetupCities,
    Play,
    End
}


struct CityInfo {
    position: GameBoardSpacePos,
    owner: PlayerColor
}


// UI data, for now, will be constructed in the main function, and passed by reference where needed.
struct GameUIData {
    board_state: [[GameBoardSpaceType; game_constants::MAX_BOARD_WIDTH]; game_constants::MAX_BOARD_HEIGHT],
    unplaced_board_pieces: std::vec::Vec<BoardPiece>,
    game_stage: GameStage,
    pos_under_mouse_for_board_setup: Option<(GameBoardSpacePos, GameBoardSpacePos, GameBoardSpacePos)>,
    pos_under_mouse_for_city_setup: Option<GameBoardSpacePos>,
    cities: std::vec::Vec<CityInfo>
}

impl GameUIData {
    fn defaults() -> GameUIData {
        GameUIData {
            board_state: [[GameBoardSpaceType::Void; game_constants::MAX_BOARD_WIDTH]; game_constants::MAX_BOARD_HEIGHT],
            unplaced_board_pieces: game_constants::BOARD_PIECES.to_vec(),
            game_stage: GameStage::SetupBoard,
            pos_under_mouse_for_board_setup: None,
            pos_under_mouse_for_city_setup: None,
            cities: std::vec::Vec::<CityInfo>::new()
        }
    }
}


fn drop_board_piece(game_ui_data: &mut GameUIData, drawable_size: (u32, u32), last_mouse_click_pos: MousePos) {
    let result = mouse_pos_to_board_piece_destination(last_mouse_click_pos, drawable_size);
    match result {
        Some((pos_under_mouse_a, pos_under_mouse_b, pos_under_mouse_c)) => {
            let board_state = &mut game_ui_data.board_state;

            let x_a = pos_under_mouse_a.x_pos as usize;
            let y_a = pos_under_mouse_a.y_pos as usize;
            let x_b = pos_under_mouse_b.x_pos as usize;
            let y_b = pos_under_mouse_b.y_pos as usize;
            let x_c = pos_under_mouse_c.x_pos as usize;
            let y_c = pos_under_mouse_c.y_pos as usize;

            if
                board_state[y_a][x_a] == GameBoardSpaceType::Void &&
                board_state[y_b][x_b] == GameBoardSpaceType::Void &&
                board_state[y_c][x_c] == GameBoardSpaceType::Void
            {
                // pick a card, any card.
                let old_len = game_ui_data.unplaced_board_pieces.len();
                let new_game_piece = game_ui_data.unplaced_board_pieces.remove(rand::thread_rng().gen_range(0, old_len));

                // randomize the orientation
                let (new_a, new_b, new_c) =
                    match rand::thread_rng().gen_range(0, 3) {
                        0 => (new_game_piece.a, new_game_piece.b, new_game_piece.c),
                        1 => (new_game_piece.b, new_game_piece.c, new_game_piece.a),
                        _ => (new_game_piece.c, new_game_piece.a, new_game_piece.b)
                    };
                board_state[y_a][x_a] = new_a;
                board_state[y_b][x_b] = new_b;
                board_state[y_c][x_c] = new_c;
            }

            const PIECES_PER_PLAYER: usize = 9;
            let num_players = 1;

            if game_ui_data.unplaced_board_pieces.len() <= game_constants::BOARD_PIECES.len() - PIECES_PER_PLAYER * num_players {
                game_ui_data.game_stage = GameStage::SetupCities;
            }
        }
        None => {}
    }
}


fn draw_game_board(gl: &gl::Gl, shader_program: &render_gl::Program, game_ui_data: &GameUIData) {
    for x in 0..game_constants::MAX_BOARD_WIDTH {
        for y in 0..game_constants::MAX_BOARD_HEIGHT {
            draw_game_board_space(&gl, &shader_program, game_ui_data.board_state[y][x], GameBoardSpacePos {x_pos: x as u8, y_pos: y as u8});
        }
    }
}


fn highlight_spaces_for_board_setup(gl: &gl::Gl, shader_program: &render_gl::Program, game_ui_data: &GameUIData) {
    match game_ui_data.pos_under_mouse_for_board_setup {
        Some((pos_under_mouse_a, pos_under_mouse_b, pos_under_mouse_c)) => {
            let board_state = &game_ui_data.board_state;

            let x_a = pos_under_mouse_a.x_pos as usize;
            let y_a = pos_under_mouse_a.y_pos as usize;
            let x_b = pos_under_mouse_b.x_pos as usize;
            let y_b = pos_under_mouse_b.y_pos as usize;
            let x_c = pos_under_mouse_c.x_pos as usize;
            let y_c = pos_under_mouse_c.y_pos as usize;
            highlight_space_for_board_setup(&gl, &shader_program, board_state[y_a][x_a], pos_under_mouse_a);
            highlight_space_for_board_setup(&gl, &shader_program, board_state[y_b][x_b], pos_under_mouse_b);
            highlight_space_for_board_setup(&gl, &shader_program, board_state[y_c][x_c], pos_under_mouse_c);
        }
        None => {}
    }
}


fn space_ok_for_city(game_ui_data: &GameUIData, position: GameBoardSpacePos) -> bool {
    let board_state = &game_ui_data.board_state;

    let x = position.x_pos as usize;
    let y = position.y_pos as usize;

    match board_state[y][x] {
        GameBoardSpaceType::Void | GameBoardSpaceType::Water | GameBoardSpaceType::Forest => {
            false
        }
        _ => {
            for city in game_ui_data.cities.iter() {
                if city.position == position {
                    return false;
                }
            }
            let neighbor_positions = [position.up(), position.down(), position.up_right(), position.up_left(), position.down_right(), position.down_left()];
            for position in &neighbor_positions {
                for city in game_ui_data.cities.iter() {
                    if position.is_some() && city.position == position.unwrap() {
                        return false;
                    }
                }
            }
            true
        }
    }
}


fn highlight_space_for_city_setup(gl: &gl::Gl, shader_program: &render_gl::Program, image_program: &render_gl::Program, city_image: &nsvg::image::RgbaImage, game_ui_data: &GameUIData, drawable_size: (u32, u32)) {
        let (x_scale, y_scale) = scaling_for_board(drawable_size);

    match game_ui_data.pos_under_mouse_for_city_setup {
        Some(pos_under_mouse) => {
            let board_state = &game_ui_data.board_state;

            let x = pos_under_mouse.x_pos as usize;
            let y = pos_under_mouse.y_pos as usize;
            match board_state[y][x] {
                GameBoardSpaceType::Void => {},
                _ => {
                    if space_ok_for_city(game_ui_data, pos_under_mouse) {
                        let drawing_pos = game_board_pos_to_drawing_pos(pos_under_mouse);
                        let x_margin = 0.25;
                        let y_margin = 0.25;
                        let x_offset = 0.0;
                        let y_offset = 0.5;

                        drawing::draw_image(
                            &gl,
                            &image_program,
                            &city_image,
                            drawing::PositionSpec{
                                x: drawing_pos.x * x_scale - 0.5 * drawing_constants::HEXAGON_WIDTH * x_scale + drawing_constants::HEXAGON_WIDTH * x_scale * (x_margin + x_offset),
                                y: drawing_pos.y * y_scale - 0.5 * drawing_constants::HEXAGON_HEIGHT * y_scale + drawing_constants::HEXAGON_WIDTH * x_scale * (y_margin + y_offset)},
                            drawing::SizeSpec{
                                x: drawing_constants::HEXAGON_WIDTH * x_scale * (1.0 - x_margin * 2.0),
                                y: drawing_constants::HEXAGON_HEIGHT * y_scale * (1.0 - y_margin * 2.0)});

                        drawing::draw_hexagon_outline(
                            &gl,
                            &shader_program,
                            drawing::HexagonSpec {
                                color: drawing::ColorSpec { r: 0xFF, g: 0xFF, b: 0xFF },
                                pos: drawing_pos,
                                width: drawing_constants::HEXAGON_WIDTH },
                            3.0);
                    } else {
                        let drawing_pos = game_board_pos_to_drawing_pos(pos_under_mouse);
                        drawing::draw_hexagon_outline(
                            &gl,
                            &shader_program,
                            drawing::HexagonSpec {
                                color: drawing::ColorSpec { r: 0xFF, g: 0x00, b: 0x00 },
                                pos: drawing_pos,
                                width: drawing_constants::HEXAGON_WIDTH },
                            3.0);
                    }
                }
            }
        }
        None => {}
    }
}



//
// Main function
//

fn main() {
    // our game resources object
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    // SDL_Init
    // Use this function to initialize the SDL library. This must be called before using most other SDL functions.
    // The return type of init() is Result<Sdl, String>
    // We call unwrap() on the Result.  This checks for errors and will terminate the program and
    // print the "String" part of the result if there was an error.  On success, the "Sdl" struct is returned.

    let sdl = sdl2::init().unwrap();

    // SDL_VideoInit
    // Initializes the video subsystem of SDL

    let video_subsystem = sdl.video().unwrap();

    // SDL_GL_SetAttribute
    // Obtains access to the OpenGL window attributes.

    let gl_attr = video_subsystem.gl_attr();

    // Set OpenGL Profile and Version.  This will help ensure that libraries that implement future versions of
    // the OpenGL standard will still always work with this code.

    // SDL_GL_CONTEXT_PROFILE_MASK
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

    // SDL_GL_CONTEXT_MAJOR_VERSION, SDL_GL_CONTEXT_MINOR_VERSION
    gl_attr.set_context_version(4, 5);

    // Initializes a new WindowBuilder, sets the window to be usable with an OpenGL context,
    // sets the window to be fullscreen at 1080 HD, builds the window, and checks for errors.
    // The Window allows you to get and set many of the SDL_Window properties (i.e., border, size, PixelFormat, etc)
    // However, you cannot directly access the pixels of the Window without a context.

    let window = video_subsystem
        .window("Game", 1920, 1080)
        .opengl()
        .fullscreen()
        .build()
        .unwrap();

    let (window_width, window_height) = window.drawable_size();
    let (ddpi, hdpi, vdpi) = video_subsystem.display_dpi(0).unwrap();
    let aspect_ratio = window_width as f32 / window_height as f32;

    // SDL_GL_CreateContext
    // Creates an OpenGL context for use with an OpenGL window, and makes it the current context.

    // NOTE: Prefixing this variable with an underscore is necessary to avoid an unused variable warning.

    let _gl_context = window.gl_create_context().unwrap();

    // Load the OpenGL function pointers from SDL.
    // Use a closure (lambda function), to add a cast to a C-style void pointer (which must be the return type of the function passed to load_with)
    let gl_get_proc_address_function = |procname| {
        video_subsystem.gl_get_proc_address(procname) as *const std::os::raw::c_void
    };
    let gl = gl::Gl::load_with(gl_get_proc_address_function);

    unsafe {
        // glViewport
        // We have to tell OpenGL the size of the rendering window so OpenGL knows how we want to display the data and coordinates with respect to the window.
        // The first two parameters of glViewport set the location of the lower left corner of the window.
        // The third and fourth parameter set the width and height of the rendering window in pixels, which we set equal to SDL's window size.
        // We could actually set the viewport dimensions at values smaller than GLFW's dimensions;
        // then all the OpenGL rendering would be displayed in a smaller window and we could for example display other elements outside the OpenGL viewport.
        // The moment a user resizes the window the viewport should be adjusted as well.

        gl.Viewport(0, 0, window_width as i32, window_height as i32); // set viewport

        // glClearColor
        // Whenever we call glClear and clear the color buffer, the entire color buffer will be filled with the color as configured by glClearColor.

        gl.ClearColor(0.0, 0.0, 0.0, 1.0); // black, fully opaque

        gl.Enable(gl::BLEND);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    // SDL_GetTicks
    let mut timer_subsystem = sdl.timer().unwrap();

    struct AudioEngine {
        sample_number: u128
    }

    impl sdl2::audio::AudioCallback for AudioEngine {
        type Channel = f32;

        fn callback(&mut self, out: &mut [f32]) {
            for x in out.iter_mut() {
                // This is where audio output will go.
                *x = 0.0;
                self.sample_number = self.sample_number + 1;
            }
        }
    }

    // SDL_AudioInit
    let audio_subsystem = sdl.audio().unwrap();
    let desired_spec = sdl2::audio::AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), //mono
        samples: None // device default sample buffer size
    };

    let audio_device = audio_subsystem.open_playback(None, &desired_spec, |_spec| {
        AudioEngine {
            sample_number: 0
        }
    }).unwrap();

    println!("Audio device buffer size: {} samples", audio_device.spec().samples);

    audio_device.resume();

    // Fonts
    let freetype_lib = freetype::Library::init().unwrap();
    let cardinal_font_face = freetype_lib.new_face(Path::new("assets/fonts/Cardinal.ttf"), 0).unwrap();
    let mut text_cache = drawing::TextCache::new();

    let player_color = PlayerColor::Red;
    //let player_color = PlayerColor::Blue;
    //let player_color = PlayerColor::Green;
    //let player_color = PlayerColor::Yellow;
    let player_color_spec = player_color.color();

    // SVG images
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

    // Obtains the SDL event pump.
    // At most one EventPump is allowed to be alive during the program's execution. If this function is called while an EventPump instance is alive, the function will return an error.

    let mut event_pump = sdl.event_pump().unwrap();

    // render_gl is a different module in this project with helper functions.  See render_gl.rs .
    // Compile and link a program with shaders that match this file name
    let shader_program = render_gl::Program::from_res(&gl, &res, "shaders/basic").unwrap();
    let text_program = render_gl::Program::from_res(&gl, &res, "shaders/text").unwrap();
    let image_program = render_gl::Program::from_res(&gl, &res, "shaders/image").unwrap();
    drawing::write_scale_data(&gl, &shader_program, aspect_ratio);
    drawing::write_rotate_data(&gl, &shader_program, 0.0);

    let frames_per_second = 60;

    let mut frame_count: u32 = 0;
    let mut frame_time: u32;

    let mut game_ui_data = GameUIData::defaults();

    let mut current_mouse_pos = MousePos { x_pos: 0, y_pos: 0 };
    // Loop with label 'main (exited by the break 'main statement)
    'main: loop {
        let mut mouse_clicked = false;
        let mut mouse_moved = false;
        let mut last_mouse_click_pos = MousePos { x_pos: 0, y_pos: 0 };
        let mut key_pressed = false;
        let mut last_key_pressed_scancode: Option<sdl2::keyboard::Scancode> = None;

        // Catch up on every event in the event_pump
        // See documentation for SDL_Event.
        for event in event_pump.poll_iter() {
            match event {
                // SDL_QuitEvent
                sdl2::event::Event::Quit { .. } => { break 'main }
                // SDL_MouseButtonEvent
                sdl2::event::Event::MouseButtonDown {timestamp: _, window_id: _, which: _, mouse_btn: _, clicks: _, x: x_mouse, y: y_mouse} => {
                    last_mouse_click_pos = MousePos { x_pos: x_mouse, y_pos: y_mouse };
                    mouse_clicked = true;
                }
                // SDL_MouseMotionEvent
                sdl2::event::Event::MouseMotion {timestamp: _, window_id: _, which: _, mousestate: _, x: x_mouse, y: y_mouse, xrel: _, yrel: _} => {
                    current_mouse_pos = MousePos { x_pos: x_mouse, y_pos: y_mouse };
                    mouse_moved = true;
                }
                // SDL_KeyboardEvent
                sdl2::event::Event::KeyDown {timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _} => {
                    // This is tricky, but effective.
                    // The variable name 'scancode' is reused to mean something different at different scopes
                    // Here, scancode is an Option type
                    match scancode {
                        Some(scancode) => {
                            // Here, scancode is a sdl2::keyboard::Scancode type
                            match scancode {
                                sdl2::keyboard::Scancode::Escape => { break 'main }
                                _ => {
                                    key_pressed = true;
                                    last_key_pressed_scancode = Some(scancode);
                                }
                            }
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }

        // No more events to handle

        if mouse_clicked {
            match game_ui_data.game_stage {
                GameStage::SetupBoard => {
                    drop_board_piece(&mut game_ui_data, (window_width, window_height), last_mouse_click_pos);
                }
                GameStage::SetupCities => {
                    match game_ui_data.pos_under_mouse_for_city_setup {
                        Some(pos_under_mouse) => {
                            let x = pos_under_mouse.x_pos as usize;
                            let y = pos_under_mouse.y_pos as usize;
                            match game_ui_data.board_state[y][x] {
                                GameBoardSpaceType::Void => {}
                                _ => {
                                    if space_ok_for_city(&game_ui_data, pos_under_mouse) {
                                        game_ui_data.cities.push(CityInfo{ position: pos_under_mouse, owner: player_color.clone() });
                                        if game_ui_data.cities.len() >= 3 {
                                            game_ui_data.game_stage = GameStage::Play;
                                        }
                                    }
                                }
                            }

                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }

        if mouse_moved {
            match game_ui_data.game_stage {
                GameStage::SetupBoard => {
                    game_ui_data.pos_under_mouse_for_board_setup = mouse_pos_to_board_piece_destination(current_mouse_pos, (window_width, window_height));
                }
                GameStage::SetupCities => {
                    game_ui_data.pos_under_mouse_for_city_setup = mouse_pos_to_game_board_pos(current_mouse_pos, (window_width, window_height));
                }
                _ => {}
            }
        }

        if key_pressed {
            use sdl2::keyboard::Scancode::*;
            match last_key_pressed_scancode.unwrap() {
                F2 => {
                    // Reset board
                    game_ui_data.board_state = [[GameBoardSpaceType::Void; game_constants::MAX_BOARD_WIDTH]; game_constants::MAX_BOARD_HEIGHT];
                    game_ui_data.unplaced_board_pieces = game_constants::BOARD_PIECES.to_vec();
                    game_ui_data.game_stage = GameStage::SetupBoard;
                    game_ui_data.cities = std::vec::Vec::<CityInfo>::new();
                }
                _ => {}
            }
        }

        // Clear the color buffer.
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw
        draw_game_board(&gl, &shader_program, &game_ui_data);

        match game_ui_data.game_stage {
            GameStage::SetupBoard => {
                highlight_spaces_for_board_setup(&gl, &shader_program, &game_ui_data);
            }
            GameStage::SetupCities => {
                highlight_space_for_city_setup(&gl, &shader_program, &image_program, &city_image, &game_ui_data, (window_width, window_height));
            }
            _ => {}
        }

        // Draw rectangular border around the game board area.
        drawing::draw_rectangle_outline(
            &gl,
            &shader_program,
            drawing::RectangleSpec {
                color: drawing::ColorSpec { r: 0xFF, g: 0xFF, b: 0xFF },
                pos: drawing::PositionSpec {
                    x: drawing_constants::GAME_BOARD_ORIGIN_X,
                    y: drawing_constants::GAME_BOARD_ORIGIN_Y },
                size: drawing::SizeSpec {
                    x: drawing_constants::HEXAGON_X_SPACING * game_constants::MAX_BOARD_WIDTH as f32 + 0.25 * drawing_constants::HEXAGON_WIDTH,
                    y: drawing_constants::HEXAGON_Y_SPACING * game_constants::MAX_BOARD_HEIGHT as f32 + 0.5 * drawing_constants::HEXAGON_HEIGHT}},
            3.0);

        // Draw scroll image
        drawing::draw_image(
            &gl,
            &image_program,
            &scroll_image,
            drawing::PositionSpec{ x: -0.99, y: 0.28 },
            drawing::SizeSpec{ x: 0.18, y: 0.45 });

        // Draw text
        {
            let mut text_drawing_baggage = drawing::TextDrawingBaggage {
                gl: gl.clone(),
                shader_program: &text_program,
                drawable_size: (window_width, window_height),
                display_dpi: (ddpi, hdpi, vdpi),
                font_face: &cardinal_font_face,
                text_cache: &mut text_cache
            };

            drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.95, y: 0.85 }, 48, "Fast and Feudalist".to_string(), drawing::ColorSpec { r: 0xFF, g: 0xD7, b: 0x00 });

            match game_ui_data.game_stage {
                GameStage::SetupBoard => {
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.08, y: 0.90 }, 24, "Game Setup".to_string(),                              drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.18, y: 0.82 }, 18, "Lay board game pieces to build the map.".to_string(), drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                }
                GameStage::SetupCities => {
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.08, y: 0.90 }, 24, "City Setup".to_string(),                              drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.22, y: 0.82 }, 18, "Place cities to determine your starting positions.".to_string(), drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                }
                GameStage::Play => {
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.08, y: 0.90 }, 24, "Game Play".to_string(),                              drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.09, y: 0.82 }, 18, "Choose an action.".to_string(), drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                }
                GameStage::End => {}
            }


            {
                let (mut x_scale, mut y_scale) = scaling_for_board((window_width, window_height));
                x_scale = x_scale * drawing_constants::HEXAGON_WIDTH * 0.5;
                y_scale = y_scale * drawing_constants::HEXAGON_HEIGHT * 0.5;

                drawing::draw_image(
                    &gl,
                    &image_program,
                    &city_image,
                    drawing::PositionSpec{ x: -0.95, y: 0.60 },
                    drawing::SizeSpec{ x: x_scale, y: y_scale});

                drawing::draw_image(
                    &gl,
                    &image_program,
                    &stronghold_image,
                    drawing::PositionSpec{ x: -0.95, y: 0.51 },
                    drawing::SizeSpec{ x: x_scale, y: y_scale});

                drawing::draw_image(
                    &gl,
                    &image_program,
                    &village_image,
                    drawing::PositionSpec{ x: -0.92, y: 0.44 },
                    drawing::SizeSpec{ x: x_scale * 0.5, y: y_scale * 0.5});

                drawing::draw_image(
                    &gl,
                    &image_program,
                    &knight_image,
                    drawing::PositionSpec{ x: -0.92, y: 0.36 },
                    drawing::SizeSpec{ x: x_scale * 0.5, y: y_scale * 0.5});
            }
            drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.88, y: 0.60 }, 24, "5".to_string(),  player_color_spec.clone());
            drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.88, y: 0.52 }, 24, "2".to_string(),  player_color_spec.clone());
            drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.88, y: 0.44 }, 24, "14".to_string(), player_color_spec.clone());
            drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.88, y: 0.36 }, 24, "7".to_string(),  player_color_spec.clone());
        }

        // Draw cities
        for city in &game_ui_data.cities {
            let (x_scale, y_scale) = scaling_for_board((window_width, window_height));
            let drawing_pos = game_board_pos_to_drawing_pos((&city).position);
            {
                let x_margin = 0.25;
                let y_margin = 0.25;
                let x_offset = 0.0;
                let y_offset = 0.5;

                drawing::draw_image(
                    &gl,
                    &image_program,
                    &city_image,
                    drawing::PositionSpec{
                        x: drawing_pos.x * x_scale - 0.5 * drawing_constants::HEXAGON_WIDTH * x_scale + drawing_constants::HEXAGON_WIDTH * x_scale * (x_margin + x_offset),
                        y: drawing_pos.y * y_scale - 0.5 * drawing_constants::HEXAGON_HEIGHT * y_scale + drawing_constants::HEXAGON_WIDTH * x_scale * (y_margin + y_offset)},
                    drawing::SizeSpec{
                        x: drawing_constants::HEXAGON_WIDTH * x_scale * (1.0 - x_margin * 2.0),
                        y: drawing_constants::HEXAGON_HEIGHT * y_scale * (1.0 - y_margin * 2.0)});
            }
            {
                let x_margin = 3.0 / 8.0;
                let y_margin = 3.0 / 8.0;
                let x_offset = -0.2;
                let y_offset = -0.2;

                drawing::draw_image(
                    &gl,
                    &image_program,
                    &knight_image,
                    drawing::PositionSpec{
                        x: drawing_pos.x * x_scale - 0.5 * drawing_constants::HEXAGON_WIDTH * x_scale + drawing_constants::HEXAGON_WIDTH * x_scale * (x_margin + x_offset),
                        y: drawing_pos.y * y_scale - 0.5 * drawing_constants::HEXAGON_HEIGHT * y_scale + drawing_constants::HEXAGON_WIDTH * x_scale * (y_margin + y_offset)},
                    drawing::SizeSpec{
                        x: drawing_constants::HEXAGON_WIDTH * x_scale * (1.0 - x_margin * 2.0),
                        y: drawing_constants::HEXAGON_HEIGHT * y_scale * (1.0 - y_margin * 2.0)});
            }
        }

        // Swap the window pixels with what we have just rendered
        window.gl_swap_window();


        // Frame rate control
        let tick_count: u32 = timer_subsystem.ticks();
        let prev_frame_count = frame_count;
        frame_count = (tick_count as f32 * frames_per_second as f32 / 1000_f32) as u32 + 1;
        frame_time = frame_count * 1000 / frames_per_second;
        if frame_count - prev_frame_count > 1 {
            println!("{:?}: Dropped {} frame(s)", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), frame_count - 1 - prev_frame_count);
        }
        let time_left: i32 = frame_time as i32 - tick_count as i32;
        if time_left > 0 {
            let sleep_duration = std::time::Duration::from_millis(time_left as u64);
            std::thread::sleep(sleep_duration);
        }
    }
}
