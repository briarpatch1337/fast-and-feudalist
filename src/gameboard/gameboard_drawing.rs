use colors::Color;
use drawing;
use gameboard::gameboard::{GameBoard,GameBoardSpaceType,GameBoardSpacePos,game_board_pos_to_drawing_pos,game_constants};
use gl;
use images::SVGImages;
use render_gl;
use std::collections::HashMap;
use PlayerColor;

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

pub trait Draw {
    fn draw_board(&self, gl: &gl::Gl, shader_program: &render_gl::Program);
    fn draw_border(gl: &gl::Gl, shader_program: &render_gl::Program);
    fn draw_cities(&self, gl: &gl::Gl, shader_program: &render_gl::Program, drawable_size: (u32, u32), images: &SVGImages);
    fn draw_knight(gl: &gl::Gl, shader_program: &render_gl::Program, images: &SVGImages, owner: &PlayerColor, drawing_pos: &drawing::PositionSpec, scaling_for_board: (f32, f32), margin_and_offset: (f32, f32, f32, f32));
    fn draw_knights(&self, gl: &gl::Gl, shader_program: &render_gl::Program, drawable_size: (u32, u32), images: &SVGImages, baggage: &mut drawing::TextDrawingBaggage);
}

impl Draw for GameBoard {
    fn draw_board(&self, gl: &gl::Gl, shader_program: &render_gl::Program) {
        for x in 0..game_constants::MAX_BOARD_WIDTH {
            for y in 0..game_constants::MAX_BOARD_HEIGHT {
                let position = GameBoardSpacePos {x_pos: x as u8, y_pos: y as u8};
                let space_type = self.get_board_space_type(position);
                draw_game_board_space(&gl, &shader_program, space_type, position);
            }
        }
    }

    fn draw_border(gl: &gl::Gl, shader_program: &render_gl::Program) {
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
    }

    fn draw_cities(&self, gl: &gl::Gl, shader_program: &render_gl::Program, drawable_size: (u32, u32), images: &SVGImages) {
        for city in self.cities() {
            let (x_scale, y_scale) = scaling_for_board(drawable_size);
            let drawing_pos = game_board_pos_to_drawing_pos(city.position);
            {
                let x_margin = 0.25;
                let y_margin = 0.25;
                let x_offset = 0.0;
                let y_offset = 0.5;

                drawing::draw_image(
                    &gl,
                    &shader_program,
                    images.get_city_image(&city.owner),
                    drawing::PositionSpec{
                        x: drawing_pos.x * x_scale - 0.5 * drawing_constants::HEXAGON_WIDTH * x_scale + drawing_constants::HEXAGON_WIDTH * x_scale * (x_margin + x_offset),
                        y: drawing_pos.y * y_scale - 0.5 * drawing_constants::HEXAGON_HEIGHT * y_scale + drawing_constants::HEXAGON_WIDTH * x_scale * (y_margin + y_offset)},
                    drawing::SizeSpec{
                        x: drawing_constants::HEXAGON_WIDTH * x_scale * (1.0 - x_margin * 2.0),
                        y: drawing_constants::HEXAGON_HEIGHT * y_scale * (1.0 - y_margin * 2.0)});
            }
        }
    }

    fn draw_knight(gl: &gl::Gl, shader_program: &render_gl::Program, images: &SVGImages, owner: &PlayerColor, drawing_pos: &drawing::PositionSpec, scaling_for_board: (f32, f32), margin_and_offset: (f32, f32, f32, f32)) {
        let (x_scale, y_scale) = scaling_for_board;
        let (x_margin, y_margin, x_offset, y_offset) = margin_and_offset;
        drawing::draw_image(
            &gl,
            &shader_program,
            images.get_knight_image(&owner),
            drawing::PositionSpec{
                x: drawing_pos.x * x_scale - 0.5 * drawing_constants::HEXAGON_WIDTH * x_scale + drawing_constants::HEXAGON_WIDTH * x_scale * (x_margin + x_offset),
                y: drawing_pos.y * y_scale - 0.5 * drawing_constants::HEXAGON_HEIGHT * y_scale + drawing_constants::HEXAGON_WIDTH * y_scale * (y_margin + y_offset)},
            drawing::SizeSpec{
                x: drawing_constants::HEXAGON_WIDTH * x_scale * (1.0 - x_margin * 2.0),
                y: drawing_constants::HEXAGON_HEIGHT * y_scale * (1.0 - y_margin * 2.0)});
    }

    fn draw_knights(&self, gl: &gl::Gl, shader_program: &render_gl::Program, drawable_size: (u32, u32), images: &SVGImages, baggage: &mut drawing::TextDrawingBaggage) {
        if self.knights().len() == 0 { return; }

        // TODO Don't calculate this on the fly each time this gets called to render each frame.
        // The hash map only needs to be updated when adding or moving knights.
        let mut positions = HashMap::new();
        for knight in self.knights() {
            *positions.entry(knight.position).or_insert(HashMap::new()).entry(knight.owner).or_insert(0) += 1
        }
        for (position, counts) in positions {
            let (x_scale, y_scale) = scaling_for_board(drawable_size);
            let drawing_pos = game_board_pos_to_drawing_pos(position);
            assert!(!counts.is_empty());
            let owner = counts.keys().next().unwrap();
            let count = *counts.get(owner).unwrap();
            let player_color = owner.color();

            {
                let x_margin = 3.0 / 8.0;
                let y_margin = 3.0 / 8.0;
                let x_offset = -0.2;
                let y_offset = -0.2;
                Self::draw_knight(&gl, &shader_program, &images, &owner, &drawing_pos, (x_scale, y_scale), (x_margin, y_margin, x_offset, y_offset));
            }
            if count == 2
            {
                let x_margin = 3.0 / 8.0;
                let y_margin = 3.0 / 8.0;
                let x_offset = 0.2;
                let y_offset = -0.2;
                Self::draw_knight(&gl, &shader_program, &images, &owner, &drawing_pos, (x_scale, y_scale), (x_margin, y_margin, x_offset, y_offset));
            }
            if count >= 3
            {
                let x_margin = 3.0 / 8.0;
                let y_margin = 3.0 / 8.0;
                let x_offset = 0.2;
                let y_offset = -0.2;

                drawing::draw_text(
                    baggage,
                    drawing::PositionSpec{
                        x: drawing_pos.x * x_scale - 0.5 * drawing_constants::HEXAGON_WIDTH * x_scale + drawing_constants::HEXAGON_WIDTH * x_scale * (x_margin + x_offset),
                        y: drawing_pos.y * y_scale - 0.5 * drawing_constants::HEXAGON_HEIGHT * y_scale + drawing_constants::HEXAGON_WIDTH * y_scale * (y_margin + y_offset)},
                    drawing::ObjectOriginLocation::Center,
                    24,
                    player_color,
                    format!("x{}", count));
            }
        }
    }
}

pub fn highlight_space_ok(
    gl: &gl::Gl,
    shader_program: &render_gl::Program,
    position: GameBoardSpacePos)
{
    let drawing_pos = game_board_pos_to_drawing_pos(position);

    drawing::draw_hexagon_outline(
        &gl,
        &shader_program,
        drawing::HexagonSpec {
            color: drawing::ColorSpec { r: 0xFF, g: 0xFF, b: 0xFF },
            pos: drawing_pos,
            width: drawing_constants::HEXAGON_WIDTH },
        3.0);
}

pub fn highlight_space_bad(
    gl: &gl::Gl,
    shader_program: &render_gl::Program,
    position: GameBoardSpacePos)
{
    let drawing_pos = game_board_pos_to_drawing_pos(position);

    drawing::draw_hexagon_outline(
        &gl,
        &shader_program,
        drawing::HexagonSpec {
            color: drawing::ColorSpec { r: 0xFF, g: 0x00, b: 0x00 },
            pos: drawing_pos,
            width: drawing_constants::HEXAGON_WIDTH },
        3.0);
}

pub fn highlight_space_for_city_setup(
    gl: &gl::Gl,
    shader_program: &render_gl::Program,
    image_program: &render_gl::Program,
    city_image: &nsvg::image::RgbaImage,
    position: GameBoardSpacePos,
    game_board: &GameBoard,
    drawable_size: (u32, u32))
{
    match game_board.get_board_space_type(position) {
        GameBoardSpaceType::Void => {},
        _ => {
            let (x_scale, y_scale) = scaling_for_board(drawable_size);

            if game_board.space_ok_for_city(position) {
                let drawing_pos = game_board_pos_to_drawing_pos(position);
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
                let drawing_pos = game_board_pos_to_drawing_pos(position);
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

pub fn highlight_spaces_for_board_setup(gl: &gl::Gl, shader_program: &render_gl::Program, positions: (GameBoardSpacePos, GameBoardSpacePos, GameBoardSpacePos), game_board: &GameBoard) {
    let (pos_under_mouse_a, pos_under_mouse_b, pos_under_mouse_c) = positions;
    let space_type_a = game_board.get_board_space_type(pos_under_mouse_a);
    let space_type_b = game_board.get_board_space_type(pos_under_mouse_b);
    let space_type_c = game_board.get_board_space_type(pos_under_mouse_c);

    highlight_space_for_board_setup(&gl, &shader_program, space_type_a, pos_under_mouse_a);
    highlight_space_for_board_setup(&gl, &shader_program, space_type_b, pos_under_mouse_b);
    highlight_space_for_board_setup(&gl, &shader_program, space_type_c, pos_under_mouse_c);
}
