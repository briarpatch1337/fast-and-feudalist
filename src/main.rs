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

#[derive(Clone, Copy, PartialEq)]
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
            y_pos: if self.x_pos % 2 == 1 {self.y_pos + 1} else {self.y_pos}
        }
    }

    // Return the position of the space which is down and to the right of this space.
    fn down_right(&self) -> GameBoardSpacePos {
        GameBoardSpacePos {
            x_pos: self.x_pos + 1,
            y_pos: if self.x_pos % 2 == 1 {self.y_pos} else {self.y_pos - 1}
        }
    }

    // Return the position of the space which is below this space.
    fn _down(&self) -> GameBoardSpacePos {
        GameBoardSpacePos {
            x_pos: self.x_pos,
            y_pos: self.y_pos - 1
        }
    }

    // Return the position of the space which is down and to the left of this space.
    fn _down_left(&self) -> GameBoardSpacePos {
        GameBoardSpacePos {
            x_pos: self.x_pos - 1,
            y_pos: if self.x_pos % 2 == 1 {self.y_pos} else {self.y_pos - 1}
        }
    }

    // Return the position of the space which is up and to the left of this space.
    fn _up_left(&self) -> GameBoardSpacePos {
        GameBoardSpacePos {
            x_pos: self.x_pos - 1,
            y_pos: if self.x_pos % 2 == 1 {self.y_pos + 1} else {self.y_pos}
        }
    }
}

#[derive(Clone, Copy)]
struct MousePos {
    x_pos: i32,
    y_pos: i32
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
    pub const GAME_BOARD_ORIGIN_Y: f32 = -1.0 * (game_constants::MAX_BOARD_HEIGHT / 2) as f32 * HEXAGON_Y_SPACING - (HEXAGON_HEIGHT / 2.0);
}


fn mouse_pos_to_drawing_pos(mouse_position: MousePos, drawable_size: (u32, u32)) -> drawing::PositionSpec {
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

    let drawing_x = (mouse_position.x_pos - (window_width as i32/ 2)) as f32 / window_width as f32 * 2.0 / x_scale;
    let drawing_y = ((window_height as i32 / 2) - mouse_position.y_pos) as f32 / window_height as f32 * 2.0 / y_scale;

    drawing::PositionSpec { x: drawing_x, y: drawing_y }
}


fn mouse_pos_to_game_board_pos(mouse_position: MousePos, drawable_size: (u32, u32)) -> Option<GameBoardSpacePos> {
    let drawing_pos = mouse_pos_to_drawing_pos(mouse_position, drawable_size);

    let from_game_board_origin_x = drawing_pos.x - drawing_constants::GAME_BOARD_ORIGIN_X;
    let from_game_board_origin_y = drawing_pos.y - drawing_constants::GAME_BOARD_ORIGIN_Y;

    // Cut the hexagons into quarters on the x axis, and halves on the y axis.
    // The two center quarters form rectangles, and the two outter quarters form triangles.
    // It's easy to know which hexagon the mouse pos is in if it falls in a rectangle.
    // It's a little bit trickier if the mouse pos is in one of the triangles.

    let scaled_x = from_game_board_origin_x / drawing_constants::HEXAGON_WIDTH * 4.0;
    let scaled_y = from_game_board_origin_y / drawing_constants::HEXAGON_HEIGHT * 2.0;

    let rounded_x = scaled_x.floor() as i32;
    let rounded_y = scaled_y.floor() as i32;

    // Because of the way the hexagons are staggered, every three quarters is a new column.

    let x_pos_game = rounded_x / 3 -
        if rounded_x % 3 == 0 {
            // Mouse pos is in a triangle. Determine if it was to the left or right of the diagonal line.
            if (rounded_x % 6 == 0 && rounded_y % 2 == 1) || (rounded_x % 6 == 3 && rounded_y % 2 == 0) {
                // positive slope
                if scaled_y - scaled_y.floor() < scaled_x - scaled_x.floor() {
                    // right
                    0
                }
                else {
                    // left
                    1
                }
            } else {
                // negative slope
                if scaled_y - (scaled_y + 1.0).floor() < (scaled_x - scaled_x.floor()) * -1.0 {
                    // left
                    1
                } else {
                    // right
                    0
                }
            }
        } else {
            // Mouse pos is in a rectangle.
            0
        };

    let shifted_y = rounded_y - if x_pos_game % 2 == 1 { 1 } else { 0 };
    let y_pos_game = shifted_y / 2;

    if rounded_x < 0 || x_pos_game < 0 || x_pos_game >= game_constants::MAX_BOARD_WIDTH as i32 || shifted_y < 0 || y_pos_game >= game_constants::MAX_BOARD_HEIGHT as i32 {
        return None;
    }

    Some(GameBoardSpacePos { x_pos: x_pos_game as u8, y_pos: y_pos_game as u8})
}


fn mouse_pos_to_board_piece_destination(mouse_position: MousePos, drawable_size: (u32, u32)) -> Option<(GameBoardSpacePos, GameBoardSpacePos, GameBoardSpacePos)> {
    let drawing_pos = mouse_pos_to_drawing_pos(mouse_position, drawable_size);

    // Adjust the origin so it is at the center of the bottom-left-most hexagon
    let adjusted_game_board_origin_x = drawing_constants::GAME_BOARD_ORIGIN_X + drawing_constants::HEXAGON_WIDTH / 2.0;
    let adjusted_game_board_origin_y = drawing_constants::GAME_BOARD_ORIGIN_Y + drawing_constants::HEXAGON_HEIGHT / 2.0;

    let from_game_board_origin_x = drawing_pos.x - adjusted_game_board_origin_x;
    let from_game_board_origin_y = drawing_pos.y - adjusted_game_board_origin_y;

    // Cut the board vertically along the centers of the hexagon columns (3/4 width)
    // Cut the board horizontally along the centers of all hexagons (1/2 height)
    let scaled_x = from_game_board_origin_x / drawing_constants::HEXAGON_WIDTH * 4.0 / 3.0;
    let scaled_y = from_game_board_origin_y / drawing_constants::HEXAGON_HEIGHT * 2.0;

    let rounded_x = scaled_x.floor() as i32;
    let rounded_y = scaled_y.floor() as i32;

    // Imagine that there are lines drawn over the board connecting the centers of adjacent hexagons
    // These lines will form equalateral triangles, with one of the line segments of each triangle being parallel to the Y axis (i.e. vertical, along a fixed x-value).
    // This function returns the positions of the three game board spaces that all share the same space as the triangle that the mouse is positioned over.
    // Each triangle is cut in half horizontally, but the vertical grid lines do not cut into the triangles, they form the vertical lines.
    // Each triangle will have one vertical line segment along an integer value of x, and a point along another integer value of x.
    // The area of each rectangular space of the grid will contain exactly one diagonal triangle line segment.
    // Determine whether the point clicked was in a rectangular piece with a positively or negatively sloped the diagonal line.
    // Then, determine whether this point is above or below that line.
    // The line goes from either
    //  (floor(x), floor(y))     to (floor(x) + 1, floor(y) + 1) == positive slope
    //  or from
    //  (floor(x), floor(y) + 1) to (floor(x) + 1, floor(y))     == negative slope
    // x and y have already been scaled so that +1.0 y is half of the distance of one of the vertical triangle line segments,
    // and +1.0 x is the distance from a vertex to the midpoint on the opposite side.

    //      0     1     2     3     4
    //      |     |     |     |     |
    //
    // 4--  X     |     X     |     X  --4
    //      |  \  |  /  |  \  |  /  |
    // 3--  |     X     |     X     |  --3
    //      |  /  |  \  |  /  |  \  |
    // 2--  X     |     X     |     X  --2
    //      |  \  |  /  |  \  |  /  |
    // 1--  |     X     |     X     |  --1
    //      |  /     \  |  /     \  |
    // 0--  X           X           X  --0
    //
    //      |     |     |     |     |
    //      0     1     2     3     4

    // Describe the x position of a triangle using the column it falls in.
    // Moving one x position value to the right moves to a triangle in the next column.
    let x_pos_triangle = rounded_x;

    // Describe the y position of a triangle using the order that it sits in the vertical stack of staggered triangles.
    // The bottom-most point of two vertically adjacent triangles are separated by one y value,
    // even though each triangle is 2.0 y units tall.
    // It is trickier to determine the y order of the triangle, because the top and bottom of each triangle are diagonal lines.

    let y_pos_triangle = rounded_y +
        if rounded_y % 2 == rounded_x % 2 {
            // in a grid area that contains a positive slope triangle edge
            if scaled_y - scaled_y.floor() < scaled_x - scaled_x.floor() {
                // point is below the edge
                -1
            } else {
                // point is above the edge
                0
            }
        } else {
            // in a grid area that contains a negative slope triangle edge
            if scaled_y - (scaled_y + 1.0).floor() < (scaled_x - scaled_x.floor()) * -1.0 {
                // point is below the edge
                -1
            } else {
                // point is above the edge
                0
            }
        };

    // Divide y_pos_triangle by two to get the hexagaon y_pos, since a hexagon height is 2 y-values

    if x_pos_triangle < 0 || y_pos_triangle < 0 || x_pos_triangle >= game_constants::MAX_BOARD_WIDTH as i32 - 1 || y_pos_triangle >= (game_constants::MAX_BOARD_HEIGHT as i32 - 1) * 2 {
        None
    } else if x_pos_triangle % 2 == y_pos_triangle % 2 {

        // two pieces on the left, one on the right

        let lower_left_pos = GameBoardSpacePos {
            x_pos: x_pos_triangle as u8,
            y_pos: (y_pos_triangle / 2) as u8
        };
        let upper_left_pos = lower_left_pos.up();
        let right_pos = lower_left_pos.up_right();

        Some((
            lower_left_pos,
            upper_left_pos,
            right_pos
        ))
    } else {

        // two pieces on the right, one on the left
        // add one to the y pos for even numbered columns, because they are shifted lower.

        let left_pos = GameBoardSpacePos {
            x_pos: x_pos_triangle as u8,
            y_pos: (y_pos_triangle / 2) as u8 +
                if x_pos_triangle % 2 == 0 {
                    1
                } else {
                    0
                }
        };
        let lower_right_pos = left_pos.down_right();
        let upper_right_pos = left_pos.up_right();

        Some((
            left_pos,
            lower_right_pos,
            upper_right_pos
        ))
    }
}


fn game_board_pos_to_drawing_pos(position: GameBoardSpacePos) -> drawing::PositionSpec {
    let x_pos_translated = drawing_constants::GAME_BOARD_ORIGIN_X
        +
        (drawing_constants::HEXAGON_WIDTH / 2.0)
        +
        position.x_pos as f32 * drawing_constants::HEXAGON_X_SPACING;

    // This is like a ternary operator, but more verbose.  I think it's easier to read.
    // Even numbered columns will be half a hexagon height higher than odd numbered columns.

    let y_pos_translated = drawing_constants::GAME_BOARD_ORIGIN_Y
        +
        (drawing_constants::HEXAGON_HEIGHT / 2.0)
        +
        position.y_pos as f32 * drawing_constants::HEXAGON_Y_SPACING
        +
        if position.x_pos % 2 == 1 { drawing_constants::HEXAGON_HEIGHT / 2.0 }
        else { 0.0 };

    drawing::PositionSpec { x: x_pos_translated, y: y_pos_translated }
}


fn color_for_game_board_space_type(space_type: GameBoardSpaceType) -> drawing::ColorSpec {
    match space_type {
        GameBoardSpaceType::Void => drawing::ColorSpec {
            r: 0x00,
            g: 0x00,
            b: 0x00
        },
        GameBoardSpaceType::Water => drawing::ColorSpec {
            r: 0x00,
            g: 0x00,
            b: 0x80
        },
        GameBoardSpaceType::Mountain => drawing::ColorSpec {
            r: 0x80,
            g: 0x80,
            b: 0x80
        },
        GameBoardSpaceType::Forest => drawing::ColorSpec {
            r: 0x11,
            g: 0x46,
            b: 0x11,
        },
        GameBoardSpaceType::Plains => drawing::ColorSpec {
            r: 0x00,
            g: 0xFF,
            b: 0x7F
        },
        GameBoardSpaceType::Field => drawing::ColorSpec {
            r: 0xFF,
            g: 0xD7,
            b: 0x00
        }
    }
}


fn draw_game_board_space(gl: &gl::Gl, shader_program: &render_gl::Program, space_type: GameBoardSpaceType, position: GameBoardSpacePos) {
    match space_type {
        GameBoardSpaceType::Void => {
            drawing::draw_hexagon_outline(
                &gl,
                &shader_program,
                drawing::HexagonSpec {
                    color: drawing::ColorSpec { r: 0xFF, g: 0xFF, b: 0xFF },
                    pos: game_board_pos_to_drawing_pos(position),
                    width: drawing_constants::HEXAGON_WIDTH },
                2.0);
        },
        _ => {
            drawing::draw_hexagon(&gl, &shader_program, drawing::HexagonSpec {
            color: color_for_game_board_space_type(space_type),
            pos: game_board_pos_to_drawing_pos(position),
            width: drawing_constants::HEXAGON_WIDTH } );
        }
    }
}


fn highlight_space(gl: &gl::Gl, shader_program: &render_gl::Program, space_type: GameBoardSpaceType, position: GameBoardSpacePos) {
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


// a, b, c spaces are in clockwise order
pub struct BoardPiece {
    a: GameBoardSpaceType,
    b: GameBoardSpaceType,
    c: GameBoardSpaceType
}


mod game_constants {
    use GameBoardSpaceType;
    use BoardPiece;

    pub const _BOARD_PIECES: [BoardPiece; 36] = [
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


// UI data, for now, will be constructed in the main function, and passed by reference where needed.
struct GameUIData {
    board_state: [[GameBoardSpaceType; game_constants::MAX_BOARD_WIDTH]; game_constants::MAX_BOARD_HEIGHT],
    pos_under_mouse: Option<(GameBoardSpacePos, GameBoardSpacePos, GameBoardSpacePos)>
}

impl GameUIData {
    fn defaults() -> GameUIData {
        GameUIData {
            board_state: [[GameBoardSpaceType::Void; game_constants::MAX_BOARD_WIDTH]; game_constants::MAX_BOARD_HEIGHT],
            pos_under_mouse: None
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
    let text_cache = drawing::TextCache::new();

    // Obtains the SDL event pump.
    // At most one EventPump is allowed to be alive during the program's execution. If this function is called while an EventPump instance is alive, the function will return an error.

    let mut event_pump = sdl.event_pump().unwrap();

    // render_gl is a different module in this project with helper functions.  See render_gl.rs .
    // Compile and link a program with shaders that match this file name
    let shader_program = render_gl::Program::from_res(&gl, &res, "shaders/basic").unwrap();
    let text_program = render_gl::Program::from_res(&gl, &res, "shaders/text").unwrap();
    drawing::write_scale_data(&gl, &shader_program, aspect_ratio);
    drawing::write_rotate_data(&gl, &shader_program, 0.0);

    let frames_per_second = 60;

    let mut frame_count: u32 = 0;
    let mut frame_time: u32;

    let mut game_ui_data = GameUIData::defaults();
    let board_state = &mut game_ui_data.board_state;

    let mut current_mouse_pos = MousePos { x_pos: 0, y_pos: 0 };

    let mut text_drawing_baggage = drawing::TextDrawingBaggage {
        gl: gl.clone(),
        shader_program: text_program,
        drawable_size: (window_width, window_height),
        display_dpi: (ddpi, hdpi, vdpi),
        font_face: cardinal_font_face,
        text_cache: text_cache
    };

    // Loop with label 'main (exited by the break 'main statement)
    'main: loop {
        let mut mouse_clicked = false;
        let mut mouse_moved = false;
        let mut last_mouse_click_pos = MousePos { x_pos: 0, y_pos: 0 };

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
                _ => {}
            }
        }

        // No more events to handle

        if mouse_clicked {
            let result = mouse_pos_to_game_board_pos(last_mouse_click_pos, (window_width, window_height));
            match result {
                Some(game_board_pos) => {
                    let old_type_under_mouse = board_state[game_board_pos.y_pos as usize][game_board_pos.x_pos as usize];
                    let new_type_under_mouse = match old_type_under_mouse {
                        GameBoardSpaceType::Void => GameBoardSpaceType::Water,
                        GameBoardSpaceType::Water => GameBoardSpaceType::Mountain,
                        GameBoardSpaceType::Mountain => GameBoardSpaceType::Forest,
                        GameBoardSpaceType::Forest => GameBoardSpaceType::Plains,
                        GameBoardSpaceType::Plains => GameBoardSpaceType::Field,
                        GameBoardSpaceType::Field => GameBoardSpaceType::Void
                    };
                    board_state[game_board_pos.y_pos as usize][game_board_pos.x_pos as usize] = new_type_under_mouse;
                }
                None => {}
            }
        }

        if mouse_moved {
            game_ui_data.pos_under_mouse = mouse_pos_to_board_piece_destination(current_mouse_pos, (window_width, window_height));
        }

        // Clear the color buffer.
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw
        for x in 0..game_constants::MAX_BOARD_WIDTH {
            for y in 0..game_constants::MAX_BOARD_HEIGHT {
                draw_game_board_space(&gl, &shader_program, board_state[y][x], GameBoardSpacePos {x_pos: x as u8, y_pos: y as u8});
            }
        }
        match game_ui_data.pos_under_mouse {
            Some((pos_under_mouse_a, pos_under_mouse_b, pos_under_mouse_c)) => {
                let x_a = pos_under_mouse_a.x_pos as usize;
                let y_a = pos_under_mouse_a.y_pos as usize;
                let x_b = pos_under_mouse_b.x_pos as usize;
                let y_b = pos_under_mouse_b.y_pos as usize;
                let x_c = pos_under_mouse_c.x_pos as usize;
                let y_c = pos_under_mouse_c.y_pos as usize;
                highlight_space(&gl, &shader_program, board_state[y_a][x_a], pos_under_mouse_a);
                highlight_space(&gl, &shader_program, board_state[y_b][x_b], pos_under_mouse_b);
                highlight_space(&gl, &shader_program, board_state[y_c][x_c], pos_under_mouse_c);
            }
            None => {}
        }
        drawing::draw_point(&gl, &shader_program);

        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.95, y: 0.85 }, 50, "Fast and Feudalist".to_string(), drawing::ColorSpec { r: 0xFF, g: 0xD7, b: 0x00 });

        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.08, y: 0.90 }, 25, "Game Setup".to_string(),                              drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });
        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.18, y: 0.82 }, 18, "Lay board game pieces to build the map.".to_string(), drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA });

        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.95, y: 0.70 }, 20, "5 cities".to_string(),      drawing::ColorSpec { r: 0xFF, g: 0x40, b: 0x40 });
        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.95, y: 0.62 }, 20, "14 villages".to_string(),   drawing::ColorSpec { r: 0xFF, g: 0x40, b: 0x40 });
        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.95, y: 0.54 }, 20, "2 strongholds".to_string(), drawing::ColorSpec { r: 0xFF, g: 0x40, b: 0x40 });
        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.95, y: 0.46 }, 20, "7 knights".to_string(),     drawing::ColorSpec { r: 0xFF, g: 0x40, b: 0x40 });

        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.95, y: -0.30 }, 25, "Water".to_string(),        color_for_game_board_space_type(GameBoardSpaceType::Water));
        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.95, y: -0.40 }, 25, "Mountain = 2".to_string(), color_for_game_board_space_type(GameBoardSpaceType::Mountain));
        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.95, y: -0.50 }, 25, "Forest = 3".to_string(),   color_for_game_board_space_type(GameBoardSpaceType::Forest));
        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.95, y: -0.60 }, 25, "Plains = 4".to_string(),   color_for_game_board_space_type(GameBoardSpaceType::Plains));
        drawing::draw_text(&mut text_drawing_baggage, drawing::PositionSpec{ x: -0.95, y: -0.70 }, 25, "Field = 5".to_string(),    color_for_game_board_space_type(GameBoardSpaceType::Field));

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
