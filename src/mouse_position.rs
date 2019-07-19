
use drawing;
use drawing_constants;
use game_constants;
use scaling_for_board;
use gameboard::GameBoardSpacePos;

#[derive(Clone, Copy)]
pub struct MousePos {
    pub x_pos: i32,
    pub y_pos: i32
}

fn mouse_pos_to_drawing_pos(mouse_position: MousePos, drawable_size: (u32, u32)) -> drawing::PositionSpec {
    let (window_width, window_height) = drawable_size;
    let (x_scale, y_scale) = scaling_for_board(drawable_size);

    let drawing_x = (mouse_position.x_pos - (window_width as i32/ 2)) as f32 / window_width as f32 * 2.0 / x_scale;
    let drawing_y = ((window_height as i32 / 2) - mouse_position.y_pos) as f32 / window_height as f32 * 2.0 / y_scale;

    drawing::PositionSpec { x: drawing_x, y: drawing_y }
}

pub fn mouse_pos_to_game_board_pos(mouse_position: MousePos, drawable_size: (u32, u32)) -> Option<GameBoardSpacePos> {
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

pub fn mouse_pos_to_board_piece_destination(mouse_position: MousePos, drawable_size: (u32, u32)) -> Option<(GameBoardSpacePos, GameBoardSpacePos, GameBoardSpacePos)> {
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
        let upper_left_pos = lower_left_pos.up().unwrap();
        let right_pos = lower_left_pos.up_right().unwrap();

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
        let upper_right_pos = left_pos.up_right().unwrap();
        let lower_right_pos = left_pos.down_right().unwrap();

        Some((
            left_pos,
            upper_right_pos,
            lower_right_pos
        ))
    }
}
