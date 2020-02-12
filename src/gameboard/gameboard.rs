
use drawing;
use drawing_constants;
use std::collections::HashMap;
use std::error;
use std::fmt;
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    // Return a vector of all neighboring positions.
    pub fn all_neighboring_positions(&self) -> Vec<GameBoardSpacePos> {
        let mut ret_val = Vec::<GameBoardSpacePos>::new();
        if let Some(game_board_pos) = self.up()         { ret_val.push(game_board_pos) }
        if let Some(game_board_pos) = self.up_right()   { ret_val.push(game_board_pos) }
        if let Some(game_board_pos) = self.down_right() { ret_val.push(game_board_pos) }
        if let Some(game_board_pos) = self.down()       { ret_val.push(game_board_pos) }
        if let Some(game_board_pos) = self.down_left()  { ret_val.push(game_board_pos) }
        if let Some(game_board_pos) = self.up_left()    { ret_val.push(game_board_pos) }
        ret_val
    }

    pub fn is_neighbor(&self, other_pos: GameBoardSpacePos) -> bool {
        self.all_neighboring_positions().iter().any(|&neighbor_position| neighbor_position == other_pos)
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


#[derive(Clone)]
pub struct UnitInfo {
    pub position: GameBoardSpacePos,
    pub owner: PlayerColor
}

#[derive(Debug)]
pub struct KnightMoveError;
impl fmt::Display for KnightMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid knight movement attempted")
    }
}
impl error::Error for KnightMoveError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub struct GameBoard {
    board_state: [[GameBoardSpaceType; game_constants::MAX_BOARD_WIDTH]; game_constants::MAX_BOARD_HEIGHT],
    cities: std::vec::Vec<UnitInfo>,
    knights: std::vec::Vec<UnitInfo>
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            board_state: [[GameBoardSpaceType::Void; game_constants::MAX_BOARD_WIDTH]; game_constants::MAX_BOARD_HEIGHT],
            cities: std::vec::Vec::<UnitInfo>::new(),
            knights: std::vec::Vec::<UnitInfo>::new()
        }
    }

    pub fn get_board_space_type(&self, position: GameBoardSpacePos) -> GameBoardSpaceType {
        self.board_state[position.y_pos as usize][position.x_pos as usize]
    }

    pub fn set_board_space_type(&mut self, position: GameBoardSpacePos, space_type: GameBoardSpaceType) {
        self.board_state[position.y_pos as usize][position.x_pos as usize] = space_type;
    }

    pub fn cities(&self) -> std::slice::Iter<UnitInfo> {
        self.cities.iter()
    }

    pub fn num_cities(&self) -> usize {
        self.cities.len()
    }

    pub fn add_city(&mut self, position: GameBoardSpacePos, owner: PlayerColor) {
        self.cities.push(UnitInfo{ position: position, owner: owner });
    }

    pub fn space_ok_for_city(&self, position: GameBoardSpacePos) -> bool {
        match self.get_board_space_type(position) {
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

    pub fn knights(&self) -> std::slice::Iter<UnitInfo> {
        self.knights.iter()
    }

    pub fn num_knights(&self) -> usize {
        self.knights.len()
    }

    pub fn add_knight(&mut self, position: GameBoardSpacePos, owner: PlayerColor) {
        self.knights.push(UnitInfo{ position: position, owner: owner });
    }

    // Returns any knights that were "killed" as part of the move (should be returned to player's inventory).
    pub fn move_knight(&mut self, from_pos: GameBoardSpacePos, to_pos: GameBoardSpacePos, owner: PlayerColor) -> Result<std::vec::Vec<UnitInfo>, KnightMoveError> {
        // Return KnightMoveError if there isn't a knight at from_pos, or if the to_pos is not ok to move a knight into.
        // simply iterating over the vector knights... Think about tracking the knights at each position a different way.
        if let Some(knight_index) = self.knights.iter().position(|ref knight| knight.position == from_pos && knight.owner == owner) {
            if self.space_ok_for_knight(to_pos, owner) {
                // reassign position
                self.knights[knight_index].position = to_pos;
                Ok(self.resolve_coexistence(to_pos))
            }
            else {
                Err(KnightMoveError{})
            }
        }
        else {
            Err(KnightMoveError{})
        }
    }

    pub fn space_ok_for_knight(&self, position: GameBoardSpacePos, owner: PlayerColor) -> bool {
        if self.cities.iter().any(|ref city| city.position == position && city.owner != owner) {
            return false;
        }

        let opposing_unit_count = self.opposing_unit_count_at_pos(position, owner);
        let space_type = self.get_board_space_type(position);

        use GameBoardSpaceType::*;
        match space_type {
            Void | Water => {false}
            Mountain => { opposing_unit_count == 0 }
            Forest | Plains | Field => { opposing_unit_count < 2 }
        }
    }

    // Returns any knights that were "killed" as part of the move (should be returned to player's inventory).
    pub fn resolve_coexistence(&mut self, position: GameBoardSpacePos) -> std::vec::Vec<UnitInfo> {
        let units_at_pos: std::vec::Vec<UnitInfo> = self.knights.iter().filter(|ref knight| knight.position == position).map(|x| x.clone()).collect();

        // never any units killed if there are 0, 1, or 2 total units in the space.
        if units_at_pos.iter().count() < 3 { return Vec::new(); }

        let mut player_unit_count = HashMap::new();
        for unit in units_at_pos.iter() {
            *player_unit_count.entry(unit.owner).or_insert(0) += 1
        }

        // Assumption here is that there can only ever be one player with multiple units on the same space,
        // because the function space_ok_for_knight() enforces that.
        assert!(player_unit_count.keys().filter(|x| *player_unit_count.get(x).unwrap() > 1).count() <= 1);

        let winning_owner = player_unit_count.keys().find(|x| *player_unit_count.get(x).unwrap() > 1);
        match winning_owner {
            Some(winning_owner) => {
                let dead_units: std::vec::Vec<UnitInfo> = units_at_pos.iter().filter(|ref knight| knight.owner != *winning_owner).map(|x| x.clone()).collect();
                self.knights.retain(|ref knight| knight.position != position || knight.owner == *winning_owner);
                dead_units
            }
            None => {
                // Nobody has more than 1 unit at this space.
                // Should only hit this case if there are 3 or 4 players all occupying the same space, each with one unit.
                Vec::new()
            }
        }
    }

    fn opposing_unit_count_at_pos(&self, position: GameBoardSpacePos, owner: PlayerColor) -> usize {
        self.knights.iter().filter(|ref knight| knight.position == position && knight.owner != owner).count()
    }

}
