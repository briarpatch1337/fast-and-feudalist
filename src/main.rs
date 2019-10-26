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
pub mod hardware;
pub mod images;
pub mod mouse_position;

use colors::Color;
use mouse_position::{MousePos, mouse_pos_to_game_board_pos, mouse_pos_to_board_piece_destination};
use rand::Rng;
use filereader::FileReader;
use gameboard::gameboard::{BoardPiece,GameBoard,GameBoardSpaceType,GameBoardSpacePos,game_constants};
use gameboard::gameboard_drawing::{drawing_constants,highlight_space_for_city_setup,highlight_spaces_for_board_setup,scaling_for_board,Draw};
use hardware::HardwareResources;
use images::SVGImages;
use std::path::Path;

#[derive(Clone,PartialEq)]
pub enum PlayerColor
{
    Red,
    Blue,
    Green,
    Yellow
}


enum PlayerAction
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
    active_player_action: PlayerAction,
    pos_under_mouse_for_board_setup: Option<(GameBoardSpacePos, GameBoardSpacePos, GameBoardSpacePos)>,
    pos_under_mouse_for_city_setup: Option<GameBoardSpacePos>,
}

impl GameUIData {
    fn defaults() -> GameUIData {
        GameUIData {
            game_board: GameBoard::new(),
            unplaced_board_pieces: game_constants::BOARD_PIECES.to_vec(),
            active_player_action: PlayerAction::SetupBoard,
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
                self.active_player_action = PlayerAction::SetupCities;
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
                            self.active_player_action = PlayerAction::Play;
                        }
                    }
                }
            }
        }
    }
}

struct EventFeedbackRunData
{
    pub mouse_clicked: bool,
    pub mouse_moved: bool,
    pub current_mouse_pos: MousePos,
    pub key_pressed: bool,
    pub last_key_pressed_scancode: Option<sdl2::keyboard::Scancode>
}

enum EventFeedback
{
    Quit,
    Run(EventFeedbackRunData)
}

impl EventFeedback {
    fn consume_pending_events(event_pump: &mut sdl2::EventPump) -> EventFeedback {
        let mut mouse_clicked = false;
        let mut mouse_moved = false;
        let mut key_pressed = false;
        let mut last_key_pressed_scancode: Option<sdl2::keyboard::Scancode> = None;
        let mut current_mouse_pos = MousePos { x_pos: 0, y_pos: 0 };

        // Catch up on every event in the event_pump
        // See documentation for SDL_Event.
        for event in event_pump.poll_iter() {
            match event {
                // SDL_QuitEvent
                sdl2::event::Event::Quit { .. } => { return EventFeedback::Quit }
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
                                sdl2::keyboard::Scancode::Escape => { return EventFeedback::Quit }
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

        EventFeedback::Run(EventFeedbackRunData {
            mouse_clicked: mouse_clicked,
            mouse_moved: mouse_moved,
            current_mouse_pos: current_mouse_pos,
            key_pressed: key_pressed,
            last_key_pressed_scancode: last_key_pressed_scancode
        })
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
    let player_color_spec = player_color.color();

    let (window_width, window_height) = hw.drawable_size;
    let (ddpi, hdpi, vdpi) = hw.display_dpi;
    let aspect_ratio = window_width as f32 / window_height as f32;

    // SVG images
    let svg_images = SVGImages::new(ddpi, window_width, player_color_spec.clone());

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

    // Loop with label 'main (exited by the break 'main statement)
    'main: loop {
        let event_feedback =
            match EventFeedback::consume_pending_events(&mut event_pump) {
                EventFeedback::Quit => { break 'main; }
                EventFeedback::Run(event_feedback_run_data) => { event_feedback_run_data }
            };


        if event_feedback.mouse_moved {
            match game_ui_data.active_player_action {
                PlayerAction::SetupBoard => {
                    game_ui_data.pos_under_mouse_for_board_setup = mouse_pos_to_board_piece_destination(event_feedback.current_mouse_pos, (window_width, window_height));
                }
                PlayerAction::SetupCities => {
                    game_ui_data.pos_under_mouse_for_city_setup = mouse_pos_to_game_board_pos(event_feedback.current_mouse_pos, (window_width, window_height));
                }
                _ => {}
            }
        }

        if event_feedback.mouse_clicked {
            match game_ui_data.active_player_action {
                PlayerAction::SetupBoard => {
                    game_ui_data.drop_board_piece();
                }
                PlayerAction::SetupCities => {
                    game_ui_data.drop_city(player_color.clone());
                }
                _ => {}
            }
        }

        if event_feedback.key_pressed {
            use sdl2::keyboard::Scancode::*;
            match event_feedback.last_key_pressed_scancode.unwrap() {
                F2 => {
                    // Reset board
                    game_ui_data.game_board = GameBoard::new();
                    game_ui_data.unplaced_board_pieces = game_constants::BOARD_PIECES.to_vec();
                    game_ui_data.active_player_action = PlayerAction::SetupBoard;
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

        match game_ui_data.active_player_action {
            PlayerAction::SetupBoard => {
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
            PlayerAction::SetupCities => {
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

            match game_ui_data.active_player_action {
                PlayerAction::SetupBoard => {
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.08, y: 0.90 }, 24, "Game Setup".to_string(),                              drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.18, y: 0.82 }, 18, "Lay board game pieces to build the map.".to_string(), drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                }
                PlayerAction::SetupCities => {
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.08, y: 0.90 }, 24, "City Setup".to_string(),                              drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.22, y: 0.82 }, 18, "Place cities to determine your starting positions.".to_string(), drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                }
                PlayerAction::Play => {
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.08, y: 0.90 }, 24, "Game Play".to_string(),                              drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                    drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.09, y: 0.82 }, 18, "Choose an action.".to_string(), drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
                }
                PlayerAction::End => {}
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
