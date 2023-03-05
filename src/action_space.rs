use lazy_static::lazy_static;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::error::CustomError;
use crate::piece_move::BitMove;

pub type ActionId = u16;
pub const ACTION_SPACE_LEN: u16 = 4672;
pub const ACTION_SPACE: std::ops::Range<ActionId> = 0..ACTION_SPACE_LEN;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Action {
    // The action space for a player is discrete and has 4672 possibilities
    // - board 8x8
    // - directional move: pick a direction x8, make up to x7 steps
    // - 8 possible knight moves
    // - 9 possible underpromotions: x3 moves and x3 pieces (Rook, Bishop, Knight)
    //
    // Total: (8*8)(8*7 + 8 + 9) = 4672
    //
    // Note: this action space also includes illegal moves
    // Ref: https://ai.stackexchange.com/a/6924
    //
    #[pyo3(get)]
    pub id: ActionId,
    #[pyo3(get)]
    pub bit_move: BitMove,
}

#[pymethods]
impl Action {
    fn __repr__(&self) -> String {
        format!(
            "(id: {}) {}",
            self.id.to_string(),
            self.bit_move.to_string()
        )
    }
}

impl ToPyObject for Action {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let value = self.clone();
        let obj: PyObject = Py::new(py, value).unwrap().into_py(py);
        obj
    }
}

lazy_static! {
    pub static ref PROMOTION_MAP: HashMap<char, i16> =
        HashMap::from([('R', 0), ('B', 1), ('N', 2)]);
    pub static ref PROMOTION_REVERSE_MAP: HashMap<i16, char> =
        HashMap::from([(0, 'R'), (1, 'B'), (2, 'N')]);
}

lazy_static! {
    pub static ref KNIGHT_MOVE_MAP: HashMap<i16, (i16, i16)> = HashMap::from([
        (0, (1, 2)),
        (1, (2, 1)),
        (2, (2, -1)),
        (3, (1, -2)),
        (4, (-1, -2)),
        (5, (-2, -1)),
        (6, (-2, 1)),
        (7, (-1, 2)),
    ]);
    pub static ref DIRECTIONAL_MOVE_MAP: HashMap<i16, (i16, i16)> = HashMap::from([
        (0, (0, 1)),
        (1, (1, 1)),
        (2, (1, 0)),
        (3, (1, -1)),
        (4, (0, -1)),
        (5, (-1, -1)),
        (6, (-1, 0)),
        (7, (-1, 1)),
    ]);
}

pub fn square_to_coords(square: &str) -> Result<(i16, i16), CustomError> {
    let letter = square.chars().nth(0).unwrap();
    let num = square.chars().nth(1).unwrap();
    let x = match letter {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return Err(CustomError::new("letter must be a to h")),
    };
    let y = num
        .to_string()
        .parse::<i16>()
        .map_err(|_| CustomError::new(&format!("Unable to parse number {}", num)))?;

    if y <= 0 || 8 < y {
        return Err(CustomError::new(&format!(
            "Coordinate number {} is illegal",
            y
        )));
    }
    Ok((x, y - 1))
}

pub fn coords_to_square(x: i16, y: i16) -> Result<String, CustomError> {
    let letter = match x {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => return Err(CustomError::new(&format!("coordinate {} is illegal", x))),
    };
    let num = format!("{}", y + 1);
    Ok(format!("{}{}", letter, num))
}

pub fn action_to_move_string(action: ActionId) -> String {
    let action = action as i16;
    let origin_sq = action / 73;
    let move_type = action % 73;

    println!("origin_sq: {}, move_type: {}, ", origin_sq, move_type);
    let from_x = origin_sq / 8;
    let from_y = origin_sq % 8;

    let from_sq = coords_to_square(from_x, from_y).unwrap();
    println!("from_sq: {}", from_sq);

    if move_type >= 73 - 9 {
        println!("it's a promo move!");
        let promo = move_type - (73 - 9);
        let promo_target = promo / 4;
        let promo_direction = promo % 4;

        let target_piece = PROMOTION_REVERSE_MAP.get(&promo_target).unwrap();

        let to_y = from_y + 1;
        let dx = match promo_direction {
            0 => 1,
            1 => 0,
            2 => -1,
            _ => unreachable!(),
        };

        let to_x = from_x + dx;
        let to_sq = coords_to_square(to_x, to_y).unwrap();

        return format!("{}{}{}", from_sq, to_sq, target_piece);
    }

    if move_type >= (73 - 9 - 8) {
        println!("it's a knight move!");
        let knight_move = move_type - (73 - 9 - 8);

        let (dx, dy) = KNIGHT_MOVE_MAP.get(&knight_move).unwrap().to_owned();

        let to_x = from_x + dx;
        let to_y = from_y + dy;
        let to_sq = coords_to_square(to_x, to_y).unwrap();
        return format!("{}{}", from_sq, to_sq);
    }

    println!("it's a directional move!");
    let direction = move_type / 8;
    let num_steps = move_type % 8;
    println!("direction: {}", direction);
    println!("num_steps: {}", num_steps);

    let (x_dir, y_dir) = DIRECTIONAL_MOVE_MAP.get(&direction).unwrap().to_owned();

    let to_x = from_x + x_dir * num_steps;
    let to_y = from_y + y_dir * num_steps;
    let to_sq = coords_to_square(to_x, to_y).unwrap();
    format!("{}{}", from_sq, to_sq)
}

pub fn build_action_space() -> Vec<String> {
    let action_space = ACTION_SPACE.map(action_to_move_string).collect::<Vec<_>>();
    todo!()
}
