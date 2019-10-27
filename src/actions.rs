
// This file has a ton of unused variables.  Don't warn about them.
#![allow(unused_variables)]

use drawing;
use gameboard;
use GameUIData;
use gl;
use highlight_space_for_city_setup;
use highlight_spaces_for_board_setup;
use images::SVGImages;
use render_gl;
use sdl2;

pub enum PlayerActionType
{
    SetupBoard,
    SetupCities,
    ChooseAction,
    Recruitment,
    Movement,
    Construction,
    NewCity,
    Expedition,
    NobleTitle,
    End
}

// This is like defining an interface.
pub trait PlayerActionControl {
    fn get_action_type(&self) -> PlayerActionType;
    fn mouse_clicked(&self, game_ui_data: &mut GameUIData) -> &PlayerActionControl; // returns the next state
    fn key_pressed(&self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> &PlayerActionControl; // returns the next state
    fn draw_highlight(
        &self,
        game_ui_data: &mut GameUIData,
        gl: &gl::Gl,
        shader_program: &render_gl::Program,
        image_program: &render_gl::Program,
        images: &SVGImages,
        drawable_size: (u32, u32));
    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage);
}

pub struct SetupBoard {}
pub struct SetupCities {}
pub struct ChooseAction {}
pub struct Recruitment {}
pub struct Movement {}
pub struct Construction {}
pub struct NewCity {}
pub struct Expedition {}
pub struct NobleTitle {}
pub struct End {}

impl PlayerActionControl for SetupBoard {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::SetupBoard
    }

    fn mouse_clicked(&self, game_ui_data: &mut GameUIData) -> &PlayerActionControl {
        game_ui_data.drop_board_piece();

        const PIECES_PER_PLAYER: usize = 9;

        if game_ui_data.unplaced_board_pieces.len() <= gameboard::gameboard::game_constants::BOARD_PIECES.len() - PIECES_PER_PLAYER * game_ui_data.num_players as usize {
            &SetupCities{}
        } else {
            self
        }
    }

    fn key_pressed(&self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> &PlayerActionControl {
        self
    }

    fn draw_highlight(
        &self,
        game_ui_data: &mut GameUIData,
        gl: &gl::Gl,
        shader_program: &render_gl::Program,
        image_program: &render_gl::Program,
        images: &SVGImages,
        drawable_size: (u32, u32))
    {
        match game_ui_data.three_pos_under_mouse {
            Some((pos_under_mouse_a, pos_under_mouse_b, pos_under_mouse_c)) => {
                highlight_spaces_for_board_setup(
                    &gl,
                    &shader_program,
                    (pos_under_mouse_a, pos_under_mouse_b, pos_under_mouse_c),
                    &game_ui_data.game_board);
            }
            None => {}
        }
    }

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Game Setup".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Lay board game pieces to build the map.".to_string());
    }
}

impl PlayerActionControl for SetupCities {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::SetupCities
    }

    fn mouse_clicked(&self, game_ui_data: &mut GameUIData) -> &PlayerActionControl {
        game_ui_data.drop_city();

        if game_ui_data.game_board.num_cities() >= 3 {
            &ChooseAction{}
        } else {
            self
        }
    }

    fn key_pressed(&self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> &PlayerActionControl {
        self
    }

    fn draw_highlight(
        &self,
        game_ui_data: &mut GameUIData,
        gl: &gl::Gl,
        shader_program: &render_gl::Program,
        image_program: &render_gl::Program,
        images: &SVGImages,
        drawable_size: (u32, u32))
    {
        match game_ui_data.one_pos_under_mouse {
            Some(pos_under_mouse) => {
                highlight_space_for_city_setup(
                    &gl,
                    &shader_program,
                    &image_program,
                    &images.city_image,
                    pos_under_mouse,
                    &game_ui_data.game_board,
                    drawable_size);
            }
            None => {}
        }
    }

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "City Setup".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Place cities to determine your starting positions.".to_string());
    }
}

impl PlayerActionControl for ChooseAction {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::ChooseAction
    }

    fn mouse_clicked(&self, game_ui_data: &mut GameUIData) -> &PlayerActionControl {
        self
    }

    fn key_pressed(&self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> &PlayerActionControl {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            Num1 | Kp1 => { &Recruitment{} }
            Num2 | Kp2 => { &Movement{} }
            Num3 | Kp3 => { &Construction{} }
            Num4 | Kp4 => { &NewCity{} }
            Num5 | Kp5 => { &Expedition{} }
            Num6 | Kp6 => { &NobleTitle{} }
            _ => { self }
        }
    }

    fn draw_highlight(
        &self,
        game_ui_data: &mut GameUIData,
        gl: &gl::Gl,
        shader_program: &render_gl::Program,
        image_program: &render_gl::Program,
        images: &SVGImages,
        drawable_size: (u32, u32))
    {
    }

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Choose Action".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "1. Recruitment  2. Movement  3. Construction  4. New City  5. Expedition  6. Noble Title".to_string());
    }
}

impl PlayerActionControl for Recruitment {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::Recruitment
    }

    fn mouse_clicked(&self, game_ui_data: &mut GameUIData) -> &PlayerActionControl {
        self
    }

    fn key_pressed(&self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> &PlayerActionControl {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            // Undo action selection
            Backspace => { &ChooseAction{} }
            _ => { self }
        }
    }

    fn draw_highlight(
        &self,
        game_ui_data: &mut GameUIData,
        gl: &gl::Gl,
        shader_program: &render_gl::Program,
        image_program: &render_gl::Program,
        images: &SVGImages,
        drawable_size: (u32, u32))
    {
    }

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Recruitment".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Pick a city to add knights to.".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}

impl PlayerActionControl for Movement {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::Movement
    }

    fn mouse_clicked(&self, game_ui_data: &mut GameUIData) -> &PlayerActionControl {
        self
    }

    fn key_pressed(&self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> &PlayerActionControl {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            // Undo action selection
            Backspace => { &ChooseAction{} }
            _ => { self }
        }
    }

    fn draw_highlight(
        &self,
        game_ui_data: &mut GameUIData,
        gl: &gl::Gl,
        shader_program: &render_gl::Program,
        image_program: &render_gl::Program,
        images: &SVGImages,
        drawable_size: (u32, u32))
    {
    }

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Movement".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Select a knight to move.".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}

impl PlayerActionControl for Construction {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::Construction
    }

    fn mouse_clicked(&self, game_ui_data: &mut GameUIData) -> &PlayerActionControl {
        self
    }

    fn key_pressed(&self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> &PlayerActionControl {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            // Undo action selection
            Backspace => { &ChooseAction{} }
            _ => { self }
        }
    }

    fn draw_highlight(
        &self,
        game_ui_data: &mut GameUIData,
        gl: &gl::Gl,
        shader_program: &render_gl::Program,
        image_program: &render_gl::Program,
        images: &SVGImages,
        drawable_size: (u32, u32))
    {
    }

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Construction".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Select a knight to build with.".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}

impl PlayerActionControl for NewCity {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::NewCity
    }

    fn mouse_clicked(&self, game_ui_data: &mut GameUIData) -> &PlayerActionControl {
        self
    }

    fn key_pressed(&self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> &PlayerActionControl {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            // Undo action selection
            Backspace => { &ChooseAction{} }
            _ => { self }
        }
    }

    fn draw_highlight(
        &self,
        game_ui_data: &mut GameUIData,
        gl: &gl::Gl,
        shader_program: &render_gl::Program,
        image_program: &render_gl::Program,
        images: &SVGImages,
        drawable_size: (u32, u32))
    {
    }

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "New City".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Select a village to upgrade to a city.".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}

impl PlayerActionControl for Expedition {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::Expedition
    }

    fn mouse_clicked(&self, game_ui_data: &mut GameUIData) -> &PlayerActionControl {
        self
    }

    fn key_pressed(&self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> &PlayerActionControl {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            // Undo action selection
            Backspace => { &ChooseAction{} }
            _ => { self }
        }
    }

    fn draw_highlight(
        &self,
        game_ui_data: &mut GameUIData,
        gl: &gl::Gl,
        shader_program: &render_gl::Program,
        image_program: &render_gl::Program,
        images: &SVGImages,
        drawable_size: (u32, u32))
    {
    }

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Expedition".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Select a board space on the edge of the map.".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}

impl PlayerActionControl for NobleTitle {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::NobleTitle
    }

    fn mouse_clicked(&self, game_ui_data: &mut GameUIData) -> &PlayerActionControl {
        self
    }

    fn key_pressed(&self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> &PlayerActionControl {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            // Undo action selection
            Backspace => { &ChooseAction{} }
            _ => { self }
        }
    }

    fn draw_highlight(
        &self,
        game_ui_data: &mut GameUIData,
        gl: &gl::Gl,
        shader_program: &render_gl::Program,
        image_program: &render_gl::Program,
        images: &SVGImages,
        drawable_size: (u32, u32))
    {
    }

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Noble Title".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press 'Y' to upgrade your noble title.".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}
