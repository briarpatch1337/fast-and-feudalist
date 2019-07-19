
use drawing;
use drawing_constants;
use PlayerColor;

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
        if next_y < game_constants::MAX_BOARD_HEIGHT as u8 {
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
        if next_x < game_constants::MAX_BOARD_WIDTH as u8 && next_y < game_constants::MAX_BOARD_HEIGHT as u8 {
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
        if next_x < game_constants::MAX_BOARD_WIDTH as u8 && next_y >= 0 {
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
        if next_x >= 0 && next_y < game_constants::MAX_BOARD_HEIGHT as u8 {
            Some(GameBoardSpacePos {
                x_pos: next_x as u8,
                y_pos: next_y})
        } else {
            None
        }
    }
}

pub fn game_board_pos_to_drawing_pos(position: GameBoardSpacePos) -> drawing::PositionSpec {
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

// a, b, c spaces are in clockwise order
#[derive(Clone)]
pub struct BoardPiece {
    pub a: GameBoardSpaceType,
    pub b: GameBoardSpaceType,
    pub c: GameBoardSpaceType
}

pub mod game_constants {
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

    pub const MAX_BOARD_HEIGHT: usize = 7;
    pub const MAX_BOARD_WIDTH: usize = 13;
}


pub struct CityInfo {
    pub position: GameBoardSpacePos,
    owner: PlayerColor
}

pub struct GameBoard {
    board_state: [[GameBoardSpaceType; game_constants::MAX_BOARD_WIDTH]; game_constants::MAX_BOARD_HEIGHT],
    cities: std::vec::Vec<CityInfo>
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            board_state: [[GameBoardSpaceType::Void; game_constants::MAX_BOARD_WIDTH]; game_constants::MAX_BOARD_HEIGHT],
            cities: std::vec::Vec::<CityInfo>::new()
        }
    }

    pub fn getBoardSpaceType(&self, position: GameBoardSpacePos) -> GameBoardSpaceType {
        self.board_state[position.y_pos as usize][position.x_pos as usize]
    }

    pub fn setBoardSpaceType(&mut self, position: GameBoardSpacePos, space_type: GameBoardSpaceType) {
        self.board_state[position.y_pos as usize][position.x_pos as usize] = space_type;
    }

    pub fn cities(&self) -> std::slice::Iter<CityInfo> {
        self.cities.iter()
    }

    pub fn numCities(&self) -> usize {
        self.cities.len()
    }

    pub fn addCity(&mut self, position: GameBoardSpacePos, owner: PlayerColor) {
        self.cities.push(CityInfo{ position: position, owner: owner });
    }

    pub fn space_ok_for_city(&self, position: GameBoardSpacePos) -> bool {
        match self.getBoardSpaceType(position) {
            GameBoardSpaceType::Void | GameBoardSpaceType::Water | GameBoardSpaceType::Forest => {
                false
            }
            _ => {
                for city in self.cities() {
                    if city.position == position {
                        return false;
                    }
                }
                let neighbor_positions = [position.up(), position.down(), position.up_right(), position.up_left(), position.down_right(), position.down_left()];
                for position in &neighbor_positions {
                    for city in self.cities() {
                        if position.is_some() && city.position == position.unwrap() {
                            return false;
                        }
                    }
                }
                true
            }
        }
    }
}
