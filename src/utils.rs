use std::cmp::min;
use crate::Coord;

#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub struct GradualMove {
    pub x_direction: Coord,
    pub y_direction: Coord,
    pub both_move: Coord,
    pub move_only_x: Coord,
    pub move_only_y: Coord,
}

impl GradualMove {
    pub fn calculate(x: Coord, y: Coord) -> Self {
        // println!("Diff X: {}, Diff Y: {}", mouse_diff.x, mouse_diff.y);

        let x_direction = x.signum();
        let y_direction = y.signum();

        let move_x = x.abs();
        let move_y = y.abs();

        let both_move = min(move_x, move_y);

        // println!("Dir X: {}, Dir Y: {}, Move both: {}", x_direction, y_direction, both_move);

        let move_only_x = move_x - both_move;
        let move_only_y = move_y - both_move;

        // println!("Only X: {}, Only Y: {}\n", move_only_x, move_only_y);

        Self {
            x_direction,
            y_direction,
            both_move,
            move_only_x,
            move_only_y,
        }
    }
}