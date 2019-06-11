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
use drawing::{ColorSpec, PositionSpec, HexagonSpec};

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

fn draw_game_board_space(gl: &gl::Gl, shader_program: &render_gl::Program, space_type: GameBoardSpaceType, x_pos: u8, y_pos: u8) {
    //    let hexagon_width = 1_f32/10_f32;
    //    let hexagon_height = 3_f32.sqrt()/20_f32;

    let x_pos_translated = (x_pos as f32 - 7.0) * 0.75 / 10.0;
    let mut y_pos_translated = (y_pos as f32 - 7.0) * 3_f32.sqrt() / 20.0;

    if x_pos % 2 == 0 {
        y_pos_translated += 3_f32.sqrt() / 40_f32;
    }

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

    drawing::draw_hexagon(&gl, &shader_program, HexagonSpec {
        color: ColorSpec { r: r_color, g: g_color, b: b_color },
        pos: PositionSpec { x: x_pos_translated, y: y_pos_translated } } );
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
    // sets the window to be resizable, builds the window, and checks for errors.
    // The Window allows you to get and set many of the SDL_Window properties (i.e., border, size, PixelFormat, etc)
    // However, you cannot directly access the pixels of the Window without a context.

    let window_width = 800_u32;
    let window_height = 800_u32;
    let aspect_ratio = window_width as f32 / window_height as f32;

    let window = video_subsystem
        .window("Game", window_width, window_height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // SDL_GL_CreateContext
    // Creates an OpenGL context for use with an OpenGL window, and makes it the current context.

    // NOTE: Prefixing this variable with an underscore is necessary to avoid an unused variable warning.

    let _gl_context = window.gl_create_context().unwrap();

    // Load the OpenGL function pointers from SDL.
    let gl =
        gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

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

        // For now, this just generates a 440 Hz square wave beep (100 ms long) every second.
        fn callback(&mut self, out: &mut [f32]) {
            for x in out.iter_mut() {
                if self.sample_number % 44100 < 4410 {
                    if (self.sample_number % 44100) % (44100/440) <= (44100/880) {
                        *x = 0.05;
                    } else {
                        *x = -0.05;
                    }
                } else {
                    *x = 0.0;
                }
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

    let mut board_state: [[GameBoardSpaceType; 15]; 15] = [[GameBoardSpaceType::Void; 15]; 15];
    for x in 0..15 {
        for y in 0..15 {
            let space_type: GameBoardSpaceType = rand::random();
            board_state[x][y] = space_type;
        }
    }

    // Loop with label 'main (exited by the break 'main statement)
    'main: loop {
        // Catch up on every event in the event_pump
        // See documentation for SDL_Event.
        for event in event_pump.poll_iter() {
            match event {
                // SDL_QuitEvent
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // No more events to handle

        // Clear the color buffer.
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw
        for x in 0..15 {
            for y in 0..15 {
                draw_game_board_space(&gl, &shader_program, board_state[x][y], x as u8, y as u8);
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
