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

extern crate glm;
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

use resources::Resources;
use std::path::Path;

#[derive(Copy,Clone)]
enum GameBoardSpaceType
{
    Void,
    Water,
    Mountain,
    Forest,
    Plains,
    Field
}


impl rand::distributions::Distribution<GameBoardSpaceType> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> GameBoardSpaceType {
        match rng.gen_range(0, 5) {
            0 => GameBoardSpaceType::Water,
            1 => GameBoardSpaceType::Mountain,
            2 => GameBoardSpaceType::Forest,
            3 => GameBoardSpaceType::Plains,
            _ => GameBoardSpaceType::Field,
        }
    }
}


struct GameBoardSpacePos {
    x_pos: u8,
    y_pos: u8
}

impl GameBoardSpacePos {
    // Return the position of the space which is above this space.
    fn up(&self) -> GameBoardSpacePos {
        GameBoardSpacePos {
            x_pos: self.x_pos,
            y_pos: self.y_pos + 1
        }
    }

    // Return the position of the space which is up and to the right of this space.
    fn up_right(&self) -> GameBoardSpacePos {
        GameBoardSpacePos {
            x_pos: self.x_pos + 1,
            y_pos: if self.x_pos % 2 == 0 {self.y_pos + 1} else {self.y_pos}
        }
    }

    // Return the position of the space which is down and to the right of this space.
    fn down_right(&self) -> GameBoardSpacePos {
        GameBoardSpacePos {
            x_pos: self.x_pos + 1,
            y_pos: if self.x_pos % 2 == 0 {self.y_pos} else {self.y_pos - 1}
        }
    }

    // Return the position of the space which is below this space.
    fn down(&self) -> GameBoardSpacePos {
        GameBoardSpacePos {
            x_pos: self.x_pos,
            y_pos: self.y_pos - 1
        }
    }

    // Return the position of the space which is down and to the left of this space.
    fn down_left(&self) -> GameBoardSpacePos {
        GameBoardSpacePos {
            x_pos: self.x_pos - 1,
            y_pos: if self.x_pos % 2 == 0 {self.y_pos} else {self.y_pos - 1}
        }
    }

    // Return the position of the space which is up and to the left of this space.
    fn up_left(&self) -> GameBoardSpacePos {
        GameBoardSpacePos {
            x_pos: self.x_pos - 1,
            y_pos: if self.x_pos % 2 == 0 {self.y_pos + 1} else {self.y_pos}
        }
    }
}


fn draw_game_board_space(gl: &gl::Gl, shader_program: &render_gl::Program, space_type: GameBoardSpaceType, position: GameBoardSpacePos) {
    // Specifying f32 (single-precision float) as the type initially
    // makes it that f32 is the type that will be used
    // to store results for the rest of the function.
    let hexagon_width: f32 = 0.10;

    // Because of the way the hexagons are staggered, the x spacing of columns is 3/4 of a hexagon width.
    let hexagon_x_spacing = hexagon_width * 0.75;
    let game_board_origin_x = -8.0 * hexagon_x_spacing;

    // Need to be explicit about the type of the number 3, in order to call sqrt().
    let hexagon_height =  hexagon_width * 3_f32.sqrt()/2.0;
    let hexagon_y_spacing = hexagon_height;
    let game_board_origin_y = -7.0 * hexagon_y_spacing;

    let x_pos_translated = game_board_origin_x + position.x_pos as f32 * hexagon_x_spacing;

    // This is like a ternary operator, but more verbose.  I think it's easier to read.
    // Even numbered columns will be half a hexagon height higher than odd numbered columns.

    let y_pos_translated = game_board_origin_y + position.y_pos as f32 * hexagon_y_spacing
        +
        if position.x_pos % 2 == 0 { hexagon_height / 2.0 }
        else { 0.0 };

    let r_color: u8;
    let g_color: u8;
    let b_color: u8;

    match space_type {
        GameBoardSpaceType::Void => {
            r_color = 0x00;
            g_color = 0x00;
            b_color = 0x00;
        }
        GameBoardSpaceType::Water => {
            r_color = 0x00;
            g_color = 0x00;
            b_color = 0x80;
        }
        GameBoardSpaceType::Mountain => {
            r_color = 0x80;
            g_color = 0x80;
            b_color = 0x80;
        }
        GameBoardSpaceType::Forest => {
            r_color = 0x22;
            g_color = 0x8B;
            b_color = 0x22;
        }
        GameBoardSpaceType::Plains => {
            r_color = 0xF4;
            g_color = 0xA4;
            b_color = 0x60;
        }
        GameBoardSpaceType::Field => {
            r_color = 0xFF;
            g_color = 0xD7;
            b_color = 0x00;
        }
    }

    drawing::draw_hexagon(&gl, &shader_program, drawing::HexagonSpec {
        color: drawing::ColorSpec { r: r_color, g: g_color, b: b_color },
        pos: drawing::PositionSpec { x: x_pos_translated, y: y_pos_translated },
        width: hexagon_width } );
}

// a, b, c spaces are in clockwise order
pub struct BoardPiece {
    a: GameBoardSpaceType,
    b: GameBoardSpaceType,
    c: GameBoardSpaceType
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
}


// UI data, for now, will be constructed in the main function, and passed by reference where needed.
struct GameUIData {
    board_state: [[GameBoardSpaceType; 15]; 17]
}

impl GameUIData {
    fn defaults() -> GameUIData {
        GameUIData {
            board_state: [[GameBoardSpaceType::Void; 15]; 17]
        }
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
    // sets the window to be fullscreen at desktop resolution, builds the window, and checks for errors.
    // The Window allows you to get and set many of the SDL_Window properties (i.e., border, size, PixelFormat, etc)
    // However, you cannot directly access the pixels of the Window without a context.

    let window = video_subsystem
        .window("Game", 0, 0)
        .opengl()
        .fullscreen_desktop()
        .build()
        .unwrap();

    let display_mode = window.display_mode().unwrap();

    let window_width = display_mode.w as u32;
    let window_height = display_mode.h as u32;
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

    // Obtains the SDL event pump.
    // At most one EventPump is allowed to be alive during the program's execution. If this function is called while an EventPump instance is alive, the function will return an error.

    let mut event_pump = sdl.event_pump().unwrap();

    // render_gl is a different module in this project with helper functions.  See render_gl.rs .
    // Compile and link a program with shaders that match this file name
    let shader_program = render_gl::Program::from_res(&gl, &res, "shaders/basic").unwrap();
    drawing::write_scale_data(&gl, &shader_program, aspect_ratio);
    drawing::write_rotate_data(&gl, &shader_program, 0.0);

    let frames_per_second = 60;

    let mut frame_count: u32 = 0;
    let mut frame_time: u32;

    let mut game_ui_data = GameUIData::defaults();
    let board_state = &mut game_ui_data.board_state;

    let mut board_piece_idx = 0;
    for x in 0..6 {
        for y in 0..6 {
            let current_board_piece = &game_constants::BOARD_PIECES[board_piece_idx];
            if ((x % 2 == 0) && (y % 2 == 0)) || ((x % 2 == 1) && (y % 2 == 1)){
                // One space on the left, two spaces on the right
                let left_space_pos = GameBoardSpacePos { x_pos: x * 3, y_pos: (y * 5 + 1) / 2 };
                //let left_space_pos = GameBoardSpacePos { x_pos: x * 2, y_pos: (y * 3 + 1) / 2 };
                let up_right_space_pos = left_space_pos.up_right();
                let down_right_space_pos = left_space_pos.down_right();
                board_state[left_space_pos.x_pos as usize][left_space_pos.y_pos as usize] = current_board_piece.a;
                board_state[up_right_space_pos.x_pos as usize][up_right_space_pos.y_pos as usize] = current_board_piece.b;
                board_state[down_right_space_pos.x_pos as usize][down_right_space_pos.y_pos as usize] = current_board_piece.c;
            }
            else {
                // One space on the right, two spaces on the left
                let down_left_space_pos = GameBoardSpacePos { x_pos: x * 3, y_pos: y * 5 / 2 };
                //let down_left_space_pos = GameBoardSpacePos { x_pos: x * 2, y_pos: y * 3 / 2 };
                let up_left_space_pos = down_left_space_pos.up();
                let right_space_pos = down_left_space_pos.up_right();
                board_state[down_left_space_pos.x_pos as usize][down_left_space_pos.y_pos as usize] = current_board_piece.a;
                board_state[up_left_space_pos.x_pos as usize][up_left_space_pos.y_pos as usize] = current_board_piece.b;
                board_state[right_space_pos.x_pos as usize][right_space_pos.y_pos as usize] = current_board_piece.c;
            }
            board_piece_idx = board_piece_idx + 1;
        }
    }

    // Loop with label 'main (exited by the break 'main statement)
    'main: loop {
        let mut mouse_clicked = false;
        let mut last_mouse_click_pos = drawing::PositionSpec { x: 0.0, y: 0.0 };

        // Catch up on every event in the event_pump
        // See documentation for SDL_Event.
        for event in event_pump.poll_iter() {
            match event {
                // SDL_QuitEvent
                sdl2::event::Event::Quit { .. } => { break 'main }
                // SDL_MouseButtonEvent
                sdl2::event::Event::MouseButtonDown {timestamp: _, window_id: _, which: _, mouse_btn: _, clicks: _, x: x_mouse, y: y_mouse} => {
                    last_mouse_click_pos = drawing::PositionSpec { x: x_mouse as f32, y: y_mouse as f32};
                    mouse_clicked = true;
                }
                _ => {}
            }
        }

        // No more events to handle

        if mouse_clicked {
            println!("Mouse clicked on {}, {}", last_mouse_click_pos.x, last_mouse_click_pos.y);
        }

        // Clear the color buffer.
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw
        for x in 0..17 {
            for y in 0..15 {
                draw_game_board_space(&gl, &shader_program, board_state[x][y], GameBoardSpacePos {x_pos: x as u8, y_pos: y as u8});
            }
        }
        drawing::draw_point(&gl, &shader_program);

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
