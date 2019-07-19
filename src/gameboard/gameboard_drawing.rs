use colors::Color;
use drawing;
use gameboard::gameboard::{GameBoardSpaceType,GameBoardSpacePos,game_board_pos_to_drawing_pos};
use gl;
use render_gl;

pub mod drawing_constants {
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

pub fn scaling_for_board(drawable_size: (u32, u32)) -> (f32, f32) {
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

pub fn draw_game_board_space(gl: &gl::Gl, shader_program: &render_gl::Program, space_type: GameBoardSpaceType, position: GameBoardSpacePos) {
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

pub fn highlight_space_for_board_setup(gl: &gl::Gl, shader_program: &render_gl::Program, space_type: GameBoardSpaceType, position: GameBoardSpacePos) {
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
