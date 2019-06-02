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


use resources::Resources;
use std::path::Path;

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

    let window_width = 1920_u32;
    let window_height = 1080_u32;
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

    // Obtains the SDL event pump.
    // At most one EventPump is allowed to be alive during the program's execution. If this function is called while an EventPump instance is alive, the function will return an error.
    
    let mut event_pump = sdl.event_pump().unwrap();

    // render_gl is a different module in this project with helper functions.  See render_gl.rs .

    // include_str will embed the contents of a file in our program (at compile time)

    // Compile and link a program with shaders that match this file name
    let shader_program = render_gl::Program::from_res(&gl, &res, "shaders/triangle").unwrap();

    // Activate this program
    shader_program.set_used();

    let triangle_width = 2_f32 / aspect_ratio;
    let triangle_height = 3_f32.sqrt();

    // Some vertices for our triangle
    let vertices: Vec<f32> = vec![
        // positions                                    colors
	//                  x                        y     z      r    g    b
         triangle_width/2_f32,  -triangle_height/2_f32,  0.0,   1.0, 0.0, 0.0,   // bottom right
        -triangle_width/2_f32,  -triangle_height/2_f32,  0.0,   0.0, 1.0, 0.0,   // bottom left
                          0.0,   triangle_height/2_f32,  0.0,   0.0, 0.0, 1.0    // top
    ];

    // Vertex buffer object (VBO)
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }

    // bind the newly created buffer to the GL_ARRAY_BUFFER target
    // Copy data to it
    // This data is accessible to the shader program
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
    }

    // Vertex array object (VAO)
    // Describes how to interpret the data in vertices and converts it to inputs for our vertex shader
    // See vertex shader source to see how this is used by the program

    let mut vao: gl::types::GLuint = 0;

    unsafe {
        gl.GenVertexArrays(1, &mut vao);
	gl.BindVertexArray(vao);

	// position attribute values
        gl.VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // not normalized (doesn't apply to floats anyways, only ints and bytes)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attribute values)
            std::ptr::null() // offset of the first component
        );
        gl.EnableVertexAttribArray(0); // enable attribute at index/location 0

        // color attribute values
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );
        gl.EnableVertexAttribArray(1);


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

        // No more events to handle.

        // Clear the color buffer.
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

	// Draw
	unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
	}   

        // Swap the window pixels with what we have just rendered
        window.gl_swap_window();
    }
}
