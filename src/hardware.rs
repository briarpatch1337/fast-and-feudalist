
// NOTE: Prefixing these fields with an underscore is necessary to avoid an unused variable warning.
pub struct HardwareResources
{
    pub sdl: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
    pub drawable_size: (u32, u32),
    pub display_dpi: (f32, f32, f32),
    _gl_context: sdl2::video::GLContext,
    pub gl: gl::Gl,
    pub timer_subsystem: sdl2::TimerSubsystem,
    _audio_subsystem: sdl2::AudioSubsystem
}

impl HardwareResources {
    pub fn init() -> HardwareResources {
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
