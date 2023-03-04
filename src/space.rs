use lazy_static::lazy_static;
use pleco::BitMove;
use std::collections::HashMap;

use crate::error::CustomError;

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

lazy_static! {
    pub static ref PROMOTION_MAP: HashMap<char, i16> =
        HashMap::from([('Q', 0), ('B', 1), ('N', 2), ('R', 3)]);
    pub static ref PROMOTION_REVERSE_MAP: HashMap<i16, char> =
        HashMap::from([(0, 'Q'), (1, 'B'), (2, 'N'), (3, 'R')]);
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

// pub struct Move {
//     from: String,
//     to: String,
//     promotion: Option<
// }

// impl Move {
//     pub fn new(value: &str) -> Self {
//         Self(value.to_string())
//     }
// }

// pub struct Action
