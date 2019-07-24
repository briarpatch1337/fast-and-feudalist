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

// Also there's filereader.rs
pub mod filereader;

// And some more
pub mod drawing;
pub mod colors;
pub mod fonts;
pub mod gameboard;
pub mod mouse_position;

use colors::Color;
use mouse_position::{MousePos, mouse_pos_to_game_board_pos, mouse_pos_to_board_piece_destination};
use rand::Rng;
use filereader::FileReader;
use gameboard::gameboard::{BoardPiece,GameBoard,GameBoardSpaceType,GameBoardSpacePos,game_constants};
use gameboard::gameboard_drawing::{drawing_constants,highlight_space_for_city_setup,highlight_spaces_for_board_setup,scaling_for_board,Draw};
use std::path::Path;

#[derive(Clone,PartialEq)]
pub enum PlayerColor
{
    Red,
    Blue,
    Green,
    Yellow
}


enum GameStage
{
    SetupBoard,
    SetupCities,
    Play,
    End
}


// UI data, for now, will be constructed in the main function, and passed by reference where needed.
struct GameUIData {
    game_board: GameBoard,
    unplaced_board_pieces: std::vec::Vec<BoardPiece>,
    game_stage: GameStage,
    pos_under_mouse_for_board_setup: Option<(GameBoardSpacePos, GameBoardSpacePos, GameBoardSpacePos)>,
    pos_under_mouse_for_city_setup: Option<GameBoardSpacePos>,
}

impl GameUIData {
    fn defaults() -> GameUIData {
        GameUIData {
            game_board: GameBoard::new(),
            unplaced_board_pieces: game_constants::BOARD_PIECES.to_vec(),
            game_stage: GameStage::SetupBoard,
            pos_under_mouse_for_board_setup: None,
            pos_under_mouse_for_city_setup: None
        }
    }

    fn drop_board_piece(&mut self) {
        if self.pos_under_mouse_for_board_setup.is_some() {
            let (pos_under_mouse_a, pos_under_mouse_b, pos_under_mouse_c) = self.pos_under_mouse_for_board_setup.unwrap();
            let game_board = &mut self.game_board;

            if
                game_board.get_board_space_type(pos_under_mouse_a) == GameBoardSpaceType::Void &&
                game_board.get_board_space_type(pos_under_mouse_b) == GameBoardSpaceType::Void &&
                game_board.get_board_space_type(pos_under_mouse_c) == GameBoardSpaceType::Void
            {
                // pick a card, any card.
                let old_len = self.unplaced_board_pieces.len();
                let new_game_piece = self.unplaced_board_pieces.remove(rand::thread_rng().gen_range(0, old_len));

                // randomize the orientation
                let (new_a, new_b, new_c) =
                    match rand::thread_rng().gen_range(0, 3) {
                        0 => (new_game_piece.a, new_game_piece.b, new_game_piece.c),
                        1 => (new_game_piece.b, new_game_piece.c, new_game_piece.a),
                        _ => (new_game_piece.c, new_game_piece.a, new_game_piece.b)
                    };
                game_board.set_board_space_type(pos_under_mouse_a, new_a);
                game_board.set_board_space_type(pos_under_mouse_b, new_b);
                game_board.set_board_space_type(pos_under_mouse_c, new_c);
            }

            const PIECES_PER_PLAYER: usize = 9;
            let num_players = 1;

            if self.unplaced_board_pieces.len() <= game_constants::BOARD_PIECES.len() - PIECES_PER_PLAYER * num_players {
                self.game_stage = GameStage::SetupCities;
            }
        }
    }

    fn drop_city(&mut self, player_color: PlayerColor) {
        if self.pos_under_mouse_for_city_setup.is_some() {
            let pos_under_mouse = self.pos_under_mouse_for_city_setup.unwrap();
            match self.game_board.get_board_space_type(pos_under_mouse) {
                GameBoardSpaceType::Void => {}
                _ => {
                    if self.game_board.space_ok_for_city(pos_under_mouse) {
                        self.game_board.add_city(pos_under_mouse, player_color);
                        if self.game_board.num_cities() >= 3 {
                            self.game_stage = GameStage::Play;
                        }
                    }
                }
            }
        }
    }
}

// NOTE: Prefixing these fields with an underscore is necessary to avoid an unused variable warning.
struct HardwareResources
{
    sdl: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    drawable_size: (u32, u32),
    display_dpi: (f32, f32, f32),
    _gl_context: sdl2::video::GLContext,
    gl: gl::Gl,
    timer_subsystem: sdl2::TimerSubsystem,
    _audio_subsystem: sdl2::AudioSubsystem
}

impl HardwareResources {
    fn init() -> HardwareResources {
        // SDL_Init
        // Use this function to initialize the SDL library. This must be called before using most other SDL functions.
        // The return type of init() is Result<Sdl, String>
        // We call unwrap() on the Result.  This checks for errors and will terminate the program and
        // print the "String" part of the result if there was an error.  On success, the "Sdl" struct is returned.

        let sdl = sdl2::init().unwrap();

        // SDL_VideoInit
        // Initializes the video subsystem of SDL

        let video_subsystem = sdl.video().unwrap();

        {
            // SDL_GL_SetAttribute
            // Obtains access to the OpenGL window attributes.

            let gl_attr = video_subsystem.gl_attr();

            // Set OpenGL Profile and Version.  This will help ensure that libraries that implement future versions of
            // the OpenGL standard will still always work with this code.

            // SDL_GL_CONTEXT_PROFILE_MASK
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

            // SDL_GL_CONTEXT_MAJOR_VERSION, SDL_GL_CONTEXT_MINOR_VERSION
            gl_attr.set_context_version(4, 5);
        }

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

        // SDL_GL_CreateContext
        // Creates an OpenGL context for use with an OpenGL window, and makes it the current context.

        let gl_context = window.gl_create_context().unwrap();

        // Load the OpenGL function pointers from SDL.

        let gl = {
            // Use a closure (lambda function), to add a cast to a C-style void pointer (which must be the return type of the function passed to load_with)
            let gl_get_proc_address_function = |procname| {
                video_subsystem.gl_get_proc_address(procname) as *const std::os::raw::c_void
            };
            gl::Gl::load_with(gl_get_proc_address_function)
        };

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
        let timer_subsystem = sdl.timer().unwrap();

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

        HardwareResources {
            sdl: sdl,
            _video_subsystem: video_subsystem,
            window: window,
            drawable_size: (window_width, window_height),
            display_dpi: (ddpi, hdpi, vdpi),
            _gl_context: gl_context,
            gl: gl,
            timer_subsystem: timer_subsystem,
            _audio_subsystem: audio_subsystem
        }
    }
}

struct SVGImages {
    city_image: nsvg::image::RgbaImage,
    village_image: nsvg::image::RgbaImage,
    stronghold_image: nsvg::image::RgbaImage,
    knight_image: nsvg::image::RgbaImage,
    scroll_image: nsvg::image::RgbaImage
}

impl SVGImages {
    fn init(ddpi: f32, window_width: u32, player_color_spec: drawing::ColorSpec) -> SVGImages {
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

//
// Main function
//

fn main() {
    // file reader object for loading GLSL shader program source files
    let filereader = FileReader::from_relative_exe_path(Path::new("assets")).unwrap();

    let mut hw = HardwareResources::init();

    // Fonts
    let mut font_resources = fonts::FontResources::new();

    let player_color = PlayerColor::Red;
    //let player_color = PlayerColor::Blue;
    //let player_color = PlayerColor::Green;
    //let player_color = PlayerColor::Yellow;
    let player_color_spec = player_color.color();

    let (window_width, window_height) = hw.drawable_size;
    let (ddpi, hdpi, vdpi) = hw.display_dpi;
    let aspect_ratio = window_width as f32 / window_height as f32;

    // SVG images
    let svg_images = SVGImages::init(ddpi, window_width, player_color_spec.clone());

    // Obtains the SDL event pump.
    // At most one EventPump is allowed to be alive during the program's execution. If this function is called while an EventPump instance is alive, the function will return an error.

    let mut event_pump = hw.sdl.event_pump().unwrap();

    // render_gl is a different module in this project with helper functions.  See render_gl.rs .
    // Compile and link a program with shaders that match this file name
    let shader_program = render_gl::Program::from_file(&hw.gl, &filereader, "shaders/basic").unwrap();
    let text_program = render_gl::Program::from_file(&hw.gl, &filereader, "shaders/text").unwrap();
    let image_program = render_gl::Program::from_file(&hw.gl, &filereader, "shaders/image").unwrap();
    drawing::write_scale_data(&hw.gl, &shader_program, aspect_ratio);
    drawing::write_rotate_data(&hw.gl, &shader_program, 0.0);

    let frames_per_second = 60;

    let mut frame_count: u32 = 0;
    let mut frame_time: u32;

    let mut game_ui_data = GameUIData::defaults();

    let mut current_mouse_pos = MousePos { x_pos: 0, y_pos: 0 };
    // Loop with label 'main (exited by the break 'main statement)
    'main: loop {
        let mut mouse_clicked = false;
        let mut mouse_moved = false;
        let mut key_pressed = false;
        let mut last_key_pressed_scancode: Option<sdl2::keyboard::Scancode> = None;

        // Catch up on every event in the event_pump
        // See documentation for SDL_Event.
        for event in event_pump.poll_iter() {
            match event {
                // SDL_QuitEvent
                sdl2::event::Event::Quit { .. } => { break 'main }
                // SDL_MouseButtonEvent
                sdl2::event::Event::MouseButtonDown {timestamp: _, window_id: _, which: _, mouse_btn: _, clicks: _, x: _, y: _} => {
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

        if mouse_clicked {
            match game_ui_data.game_stage {
                GameStage::SetupBoard => {
                    game_ui_data.drop_board_piece();
                }
                GameStage::SetupCities => {
                    game_ui_data.drop_city(player_color.clone());
                }
                _ => {}
            }
        }

        if key_pressed {
            use sdl2::keyboard::Scancode::*;
            match last_key_pressed_scancode.unwrap() {
                F2 => {
                    // Reset board
                    game_ui_data.game_board = GameBoard::new();
                    game_ui_data.unplaced_board_pieces = game_constants::BOARD_PIECES.to_vec();
                    game_ui_data.game_stage = GameStage::SetupBoard;
                }
                _ => {}
            }
        }

        // Clear the color buffer.
        unsafe {
            hw.gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw
        game_ui_data.game_board.draw_board(&hw.gl, &shader_program);

        match game_ui_data.game_stage {
            GameStage::SetupBoard => {
                match game_ui_data.pos_under_mouse_for_board_setup {
                    Some((pos_under_mouse_a, pos_under_mouse_b, pos_under_mouse_c)) => {
                        highlight_spaces_for_board_setup(
                            &hw.gl,
                            &shader_program,
                            (pos_under_mouse_a, pos_under_mouse_b, pos_under_mouse_c),
                            &game_ui_data.game_board);
                    }
                    None => {}
                }
            }
            GameStage::SetupCities => {
                match game_ui_data.pos_under_mouse_for_city_setup {
                    Some(pos_under_mouse) => {
                        highlight_space_for_city_setup(
                            &hw.gl,
                            &shader_program,
                            &image_program,
                            &svg_images.city_image,
                            pos_under_mouse,
                            &game_ui_data.game_board,
                            (window_width, window_height));
                    }
                    None => {}
                }
            }
            _ => {}
        }

        // Draw rectangular border around the game board area.
        GameBoard::draw_border(&hw.gl, &shader_program);

        // Draw scroll image
        drawing::draw_image(
            &hw.gl,
            &image_program,
            &svg_images.scroll_image,
            drawing::PositionSpec{ x: -0.99, y: 0.28 },
            drawing::SizeSpec{ x: 0.18, y: 0.45 });

        // Draw text
        {
            let mut text_drawing_baggage = drawing::TextDrawingBaggage {
                gl: hw.gl.clone(),
                shader_program: &text_program,
                drawable_size: (window_width, window_height),
                display_dpi: (ddpi, hdpi, vdpi),
                font_face: &font_resources.cardinal_font_face,
                text_cache: &mut font_resources.text_cache
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
                    &hw.gl,
                    &image_program,
                    &svg_images.city_image,
                    drawing::PositionSpec{ x: -0.95, y: 0.60 },
                    drawing::SizeSpec{ x: x_scale, y: y_scale});

                drawing::draw_image(
                    &hw.gl,
                    &image_program,
                    &svg_images.stronghold_image,
                    drawing::PositionSpec{ x: -0.95, y: 0.51 },
                    drawing::SizeSpec{ x: x_scale, y: y_scale});

                drawing::draw_image(
                    &hw.gl,
                    &image_program,
                    &svg_images.village_image,
                    drawing::PositionSpec{ x: -0.92, y: 0.44 },
                    drawing::SizeSpec{ x: x_scale * 0.5, y: y_scale * 0.5});

                drawing::draw_image(
                    &hw.gl,
                    &image_program,
                    &svg_images.knight_image,
                    drawing::PositionSpec{ x: -0.92, y: 0.36 },
                    drawing::SizeSpec{ x: x_scale * 0.5, y: y_scale * 0.5});
            }
            drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.88, y: 0.60 }, 24, "5".to_string(),  player_color_spec.clone());
            drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.88, y: 0.52 }, 24, "2".to_string(),  player_color_spec.clone());
            drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.88, y: 0.44 }, 24, "14".to_string(), player_color_spec.clone());
            drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.88, y: 0.36 }, 24, "7".to_string(),  player_color_spec.clone());
        }

        // Draw cities
        game_ui_data.game_board.draw_cities(&hw.gl, &image_program, (window_width, window_height), &svg_images.city_image, &svg_images.knight_image);

        // Swap the window pixels with what we have just rendered
        hw.window.gl_swap_window();


        // Frame rate control
        let tick_count: u32 = hw.timer_subsystem.ticks();
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
