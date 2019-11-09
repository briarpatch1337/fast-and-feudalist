
// This file has a ton of unused variables.  Don't warn about them.
#![allow(unused_variables)]

use std::cmp;
use drawing;
use gameboard;
use gameboard::gameboard::GameBoardSpacePos;
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

    fn mouse_clicked(&mut self, game_ui_data: &mut GameUIData) -> Option<Box<PlayerActionControl>>; // returns the next state, or None if the state hasn't changed

    fn key_pressed(&mut self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> Option<Box<PlayerActionControl>>; // returns the next state, or None if the state hasn't changed

    fn draw_highlight(
        &self,
        game_ui_data: &mut GameUIData,
        gl: &gl::Gl,
        shader_program: &render_gl::Program,
        image_program: &render_gl::Program,
        images: &SVGImages,
        drawable_size: (u32, u32));

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage, game_ui_data: &mut GameUIData);
}

#[derive(Clone)]
pub struct SetupBoard {}
impl PlayerActionControl for SetupBoard {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::SetupBoard
    }

    fn mouse_clicked(&mut self, game_ui_data: &mut GameUIData) -> Option<Box<PlayerActionControl>> {
        game_ui_data.drop_board_piece();

        const PIECES_PER_PLAYER: usize = 9;

        if game_ui_data.unplaced_board_pieces.len() <= gameboard::gameboard::game_constants::BOARD_PIECES.len() - PIECES_PER_PLAYER * game_ui_data.num_players as usize {
            Some(Box::new(SetupCities{}))
        } else {
            None
        }
    }

    fn key_pressed(&mut self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> Option<Box<PlayerActionControl>> {
        None
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

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage, game_ui_data: &mut GameUIData) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Game Setup".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Lay board game pieces to build the map.".to_string());
    }
}

#[derive(Clone)]
pub struct SetupCities {}
impl PlayerActionControl for SetupCities {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::SetupCities
    }

    fn mouse_clicked(&mut self, game_ui_data: &mut GameUIData) -> Option<Box<PlayerActionControl>> {
        game_ui_data.drop_city();

        if game_ui_data.game_board.num_cities() >= 3 {
            Some(Box::new(ChooseAction{}))
        } else {
            None
        }
    }

    fn key_pressed(&mut self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> Option<Box<PlayerActionControl>> {
        None
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

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage, game_ui_data: &mut GameUIData) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "City Setup".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Place cities to determine your starting positions.".to_string());
    }
}

#[derive(Clone)]
pub struct ChooseAction {}
impl PlayerActionControl for ChooseAction {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::ChooseAction
    }

    fn mouse_clicked(&mut self, game_ui_data: &mut GameUIData) -> Option<Box<PlayerActionControl>> {
        None
    }

    fn key_pressed(&mut self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> Option<Box<PlayerActionControl>> {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            Num1 | Kp1 => {
                if Recruitment::is_action_viable(game_ui_data) { Some(Box::new(Recruitment{ selected_city: None })) }
                else { None }
            }
            Num2 | Kp2 => { Some(Box::new(Movement{})) }
            Num3 | Kp3 => { Some(Box::new(Construction{})) }
            Num4 | Kp4 => { Some(Box::new(NewCity{})) }
            Num5 | Kp5 => { Some(Box::new(Expedition{})) }
            Num6 | Kp6 => { Some(Box::new(NobleTitle{})) }
            _ => { None }
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

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage, game_ui_data: &mut GameUIData) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Choose Action".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "1. Recruitment  2. Movement  3. Construction  4. New City  5. Expedition  6. Noble Title".to_string());
    }
}

#[derive(Clone)]
pub struct Recruitment { selected_city: Option<GameBoardSpacePos> }
impl Recruitment {
    fn is_action_viable(game_ui_data: &mut GameUIData) -> bool {
        game_ui_data.player_inventory.num_knights > 0
    }
    fn is_space_viable(position: GameBoardSpacePos, game_ui_data: &mut GameUIData) -> bool {
        for city in game_ui_data.game_board.cities() {
            if city.position == position {
                return true;
            }
        }
        false
    }
    fn max_number_of_knights_to_add(position: GameBoardSpacePos, game_ui_data: &mut GameUIData) -> u8 {
        match position.all_neighboring_positions().iter().find(|&&gameboard_pos| game_ui_data.game_board.get_board_space_type(gameboard_pos) == gameboard::gameboard::GameBoardSpaceType::Water) {
            Some(_) => { cmp::min(game_ui_data.player_inventory.num_knights, 3) }
            None => { cmp::min(game_ui_data.player_inventory.num_knights, 2) }
        }
    }
    fn add_knights(&self, game_ui_data: &mut GameUIData, num_knights: usize) {
        for i in 0..num_knights {
            game_ui_data.game_board.add_knight(self.selected_city.unwrap(), game_ui_data.player_color);
        }
    }
}
impl PlayerActionControl for Recruitment {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::Recruitment
    }

    fn mouse_clicked(&mut self, game_ui_data: &mut GameUIData) -> Option<Box<PlayerActionControl>> {
        if let Some(pos_under_mouse) = game_ui_data.one_pos_under_mouse {
            match game_ui_data.game_board.get_board_space_type(pos_under_mouse) {
                gameboard::gameboard::GameBoardSpaceType::Void => {}
                _ => {
                    if Recruitment::is_space_viable(pos_under_mouse, game_ui_data) {
                        self.selected_city = Some(pos_under_mouse);
                    } else {
                        self.selected_city = None;
                    }
                }
            }
        }
        None
    }

    fn key_pressed(&mut self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> Option<Box<PlayerActionControl>> {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            Num1 | Kp1 => {
                if let Some(game_board_pos) = self.selected_city {
                    assert!(Recruitment::max_number_of_knights_to_add(game_board_pos, game_ui_data) >= 1);
                    self.add_knights(game_ui_data, 1);
                    game_ui_data.player_inventory.num_knights -= 1;
                    Some(Box::new(ChooseAction{}))
                }
                else { None }
            }
            Num2 | Kp2 => {
                if let Some(game_board_pos) = self.selected_city {
                    if Recruitment::max_number_of_knights_to_add(game_board_pos, game_ui_data) >= 2 {
                        self.add_knights(game_ui_data, 2);
                        game_ui_data.player_inventory.num_knights -= 2;
                        Some(Box::new(ChooseAction{}))
                    }
                    else { None }
                }
                else { None }
            }
            Num3 | Kp3 => {
                if let Some(game_board_pos) = self.selected_city {
                    if Recruitment::max_number_of_knights_to_add(game_board_pos, game_ui_data) >= 3 {
                        self.add_knights(game_ui_data, 3);
                        game_ui_data.player_inventory.num_knights -= 3;
                        Some(Box::new(ChooseAction{}))
                    }
                    else { None }
                }
                else { None }
            }
            // Undo action selection
            Backspace => { Some(Box::new(ChooseAction{})) }
            _ => { None }
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
        if let Some(selected_city) = self.selected_city {
            gameboard::gameboard_drawing::highlight_space_ok(gl, shader_program, selected_city);
        }
        if let Some(pos_under_mouse) = game_ui_data.one_pos_under_mouse {
            match game_ui_data.game_board.get_board_space_type(pos_under_mouse) {
                gameboard::gameboard::GameBoardSpaceType::Void => {}
                _ => {
                    if Recruitment::is_space_viable(pos_under_mouse, game_ui_data) {
                        gameboard::gameboard_drawing::highlight_space_ok(gl, shader_program, pos_under_mouse);
                    } else {
                        gameboard::gameboard_drawing::highlight_space_bad(gl, shader_program, pos_under_mouse);
                    }
                }
            }
        }
    }

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage, game_ui_data: &mut GameUIData) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Recruitment".to_string());
        match self.selected_city {
            None => {
                drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
                    "Pick a city to add knights to.".to_string());
            }
            Some(selected_city) => {
                drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
                    format!("Enter the number of knights to recruit.  Max: {}", Recruitment::max_number_of_knights_to_add(selected_city, game_ui_data)));
            }
        }
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}

#[derive(Clone)]
pub struct Movement {}
impl PlayerActionControl for Movement {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::Movement
    }

    fn mouse_clicked(&mut self, game_ui_data: &mut GameUIData) -> Option<Box<PlayerActionControl>> {
        None
    }

    fn key_pressed(&mut self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> Option<Box<PlayerActionControl>> {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            // Undo action selection
            Backspace => { Some(Box::new(ChooseAction{})) }
            _ => { None }
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

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage, game_ui_data: &mut GameUIData) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Movement".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Select a knight to move.".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}

#[derive(Clone)]
pub struct Construction {}
impl PlayerActionControl for Construction {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::Construction
    }

    fn mouse_clicked(&mut self, game_ui_data: &mut GameUIData) -> Option<Box<PlayerActionControl>> {
        None
    }

    fn key_pressed(&mut self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> Option<Box<PlayerActionControl>> {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            // Undo action selection
            Backspace => { Some(Box::new(ChooseAction{})) }
            _ => { None }
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

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage, game_ui_data: &mut GameUIData) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Construction".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Select a knight to build with.".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}

#[derive(Clone)]
pub struct NewCity {}
impl PlayerActionControl for NewCity {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::NewCity
    }

    fn mouse_clicked(&mut self, game_ui_data: &mut GameUIData) -> Option<Box<PlayerActionControl>> {
        None
    }

    fn key_pressed(&mut self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> Option<Box<PlayerActionControl>> {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            // Undo action selection
            Backspace => { Some(Box::new(ChooseAction{})) }
            _ => { None }
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

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage, game_ui_data: &mut GameUIData) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "New City".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Select a village to upgrade to a city.".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}

#[derive(Clone)]
pub struct Expedition {}
impl PlayerActionControl for Expedition {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::Expedition
    }

    fn mouse_clicked(&mut self, game_ui_data: &mut GameUIData) -> Option<Box<PlayerActionControl>> {
        None
    }

    fn key_pressed(&mut self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> Option<Box<PlayerActionControl>> {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            // Undo action selection
            Backspace => { Some(Box::new(ChooseAction{})) }
            _ => { None }
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

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage, game_ui_data: &mut GameUIData) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Expedition".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Select a board space on the edge of the map.".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}

#[derive(Clone)]
pub struct NobleTitle {}
impl PlayerActionControl for NobleTitle {
    fn get_action_type(&self) -> PlayerActionType {
        PlayerActionType::NobleTitle
    }

    fn mouse_clicked(&mut self, game_ui_data: &mut GameUIData) -> Option<Box<PlayerActionControl>> {
        None
    }

    fn key_pressed(&mut self, game_ui_data: &mut GameUIData, scancode: &sdl2::keyboard::Scancode) -> Option<Box<PlayerActionControl>> {
        use sdl2::keyboard::Scancode::*;
        match scancode {
            // Undo action selection
            Backspace => { Some(Box::new(ChooseAction{})) }
            _ => { None }
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

    fn draw_text(&self, baggage: &mut drawing::TextDrawingBaggage, game_ui_data: &mut GameUIData) {
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.90 }, drawing::ObjectOriginLocation::Center, 24, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Noble Title".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.82 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press 'Y' to upgrade your noble title.".to_string());
        drawing::draw_text(baggage, drawing::PositionSpec{ x: 0.0, y: 0.74 }, drawing::ObjectOriginLocation::Center, 18, drawing::ColorSpec { r: 0xEE, g: 0xE8, b: 0xAA },
            "Press Backspace to cancel.".to_string());
    }
}

#[derive(Clone)]
pub struct End {}
