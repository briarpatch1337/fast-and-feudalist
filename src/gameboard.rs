
use game_constants::{MAX_BOARD_HEIGHT,MAX_BOARD_WIDTH};

#[derive(Copy,Clone,PartialEq)]
pub enum GameBoardSpaceType
{
    Void,
    Water,
    Mountain,
    Forest,
    Plains,
    Field
}

#[derive(Clone, Copy, PartialEq)]
pub struct GameBoardSpacePos {
    pub x_pos: u8,
    pub y_pos: u8
}

impl GameBoardSpacePos {
    // Return the position of the space which is above this space.
    pub fn up(&self) -> Option<GameBoardSpacePos> {
        let next_y = self.y_pos + 1;
        if next_y < MAX_BOARD_HEIGHT as u8 {
            Some(GameBoardSpacePos {
                x_pos: self.x_pos,
                y_pos: next_y})
        } else {
            None
        }
    }

    // Return the position of the space which is up and to the right of this space.
    pub fn up_right(&self) -> Option<GameBoardSpacePos> {
        let next_x = self.x_pos + 1;
        let next_y = if self.x_pos % 2 == 1 {self.y_pos + 1} else {self.y_pos};
        if next_x < MAX_BOARD_WIDTH as u8 && next_y < MAX_BOARD_HEIGHT as u8 {
            Some(GameBoardSpacePos {
                x_pos: next_x,
                y_pos: next_y})
        } else {
            None
        }
    }

    // Return the position of the space which is down and to the right of this space.
    pub fn down_right(&self) -> Option<GameBoardSpacePos> {
        let next_x = self.x_pos + 1;
        let next_y = if self.x_pos % 2 == 1 {self.y_pos as i8} else {self.y_pos as i8 - 1};
        if next_x < MAX_BOARD_WIDTH as u8 && next_y >= 0 {
            Some(GameBoardSpacePos {
                x_pos: next_x,
                y_pos: next_y as u8})
        } else {
            None
        }
    }

    // Return the position of the space which is below this space.
    pub fn down(&self) -> Option<GameBoardSpacePos> {
        let next_y = self.y_pos as i8 - 1;
        if next_y >= 0 {
            Some(GameBoardSpacePos {
                x_pos: self.x_pos,
                y_pos: next_y as u8})
        } else {
            None
        }
    }

    // Return the position of the space which is down and to the left of this space.
    pub fn down_left(&self) -> Option<GameBoardSpacePos> {
        let next_x = self.x_pos as i8 - 1;
        let next_y = if self.x_pos % 2 == 1 {self.y_pos as i8} else {self.y_pos as i8 - 1};
        if next_x >= 0 && next_y >= 0 {
            Some(GameBoardSpacePos {
                x_pos: next_x as u8,
                y_pos: next_y as u8})
        } else {
            None
        }
    }

    // Return the position of the space which is up and to the left of this space.
    pub fn up_left(&self) -> Option<GameBoardSpacePos> {
        let next_x = self.x_pos as i8 - 1;
        let next_y = if self.x_pos % 2 == 1 {self.y_pos + 1} else {self.y_pos};
        if next_x >= 0 && next_y < MAX_BOARD_HEIGHT as u8 {
            Some(GameBoardSpacePos {
                x_pos: next_x as u8,
                y_pos: next_y})
        } else {
            None
        }
    }
}
