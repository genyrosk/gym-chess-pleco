use pleco::core::piece_move::{MoveFlag, PreMoveInfo};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};
use std::collections::HashMap;

mod bitboard;
mod board;
mod core;
mod error;
mod piece_move;
mod score;
mod space;
mod square;

use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::core::{CastleType, GenTypes, Piece, PieceType, Player};
use crate::piece_move::{BitMove, ScoringMove};
use crate::score::Score;
use crate::space::{
    coords_to_square, square_to_coords, DIRECTIONAL_MOVE_MAP, KNIGHT_MOVE_MAP, PROMOTION_MAP,
    PROMOTION_REVERSE_MAP,
};
use crate::square::Square;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Action(u16);

#[pyclass]
pub struct ChessEnv {
    pub board: Board,
    pub step_num: u64,
    pub action_moves_map: HashMap<Action, BitMove>,
}

impl ChessEnv {
    pub fn action_to_move_string(action: Action) -> String {
        let action = action.0 as i16;
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

    pub fn move_to_action(bit_move: &pleco::BitMove) -> Action {
        let sq_src = bit_move.get_src();
        let diff_col = bit_move.dest_col() as i8 - bit_move.src_col() as i8;
        let diff_row = bit_move.dest_row() as i8 - bit_move.src_row() as i8;

        if bit_move.is_promo() {
            println!("it's a promo move!");
            let target_piece = bit_move.promo_piece();
            let target_piece = match target_piece {
                pleco::PieceType::N => 0,
                pleco::PieceType::B => 1,
                pleco::PieceType::R => 2,
                pleco::PieceType::Q => 3,
                _ => unreachable!(),
            };

            let promo_move: u16 = match diff_col {
                -1 => 0,
                0 => 1,
                1 => 2,
                _ => unreachable!(),
            };

            let promo = target_piece * 4 + promo_move;
            println!("{}", promo);

            let move_type = 8 * 7 + 8 + promo;
            let action = Action((u8::from(sq_src) as u16) * 73 + move_type);
            return action;
        }

        if (diff_col.abs() == 1 && diff_row.abs() == 2)
            || (diff_col.abs() == 2 && diff_row.abs() == 1)
        {
            println!("it's a knight move!");

            let knight_move: u16 = match (diff_col, diff_row) {
                (1, 2) => 0,
                (2, 1) => 1,
                (2, -1) => 2,
                (1, -2) => 3,
                (-1, -2) => 4,
                (-2, -1) => 5,
                (-2, 1) => 6,
                (-1, 2) => 7,
                _ => !unreachable!(),
            };
            let move_type = 8 * 7 + knight_move;
            let action = Action((u8::from(sq_src) as u16) * 73 + move_type);
            return action;
        }

        let direction: u16 = match (diff_col.signum(), diff_row.signum()) {
            (0, 1) => 0,
            (1, 1) => 1,
            (1, 0) => 2,
            (1, -1) => 3,
            (0, -1) => 4,
            (-1, -1) => 5,
            (-1, 0) => 6,
            (-1, 1) => 7,
            _ => !unreachable!(),
        };
        let num_steps = std::cmp::max(diff_row.abs(), diff_col.abs()) as u16;
        let directional_move = direction * 8 + num_steps;
        let action = Action((u8::from(sq_src) as u16) * 73 + directional_move);
        return action;
    }
}

#[pymethods]
impl ChessEnv {
    // Main API
    // step()
    // reset()
    // render()
    // close()

    // Optional
    // action_space
    // observation_space
    // reward_range
    // spec
    // metadata
    // np_random

    #[new]
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            step_num: 0,
            action_moves_map: HashMap::new(),
        }
    }

    pub fn get_state<'a>(&mut self, py: Python<'a>) -> PyResult<&'a PyList> {
        let state = self.board.state();

        let vec: Vec<Vec<Piece>> = vec![];
        let state_list = PyList::new(py, vec);

        for row in state {
            let list = PyList::new(py, row);
            state_list.append(list)?;
        }

        Ok(state_list)
    }

    pub fn get_actions(&self) -> Vec<&Action> {
        self.action_moves_map.keys().collect::<Vec<_>>()
    }

    pub fn reset<'a>(
        &mut self,
        py: Python<'a>,
        seed: Option<i64>,
        options: Option<&'a PyDict>,
    ) -> PyResult<(&'a PyList, &'a PyDict)> {
        self.board = Board::start_pos();
        let obsrv = self.get_state(py)?;

        let info = PyDict::new(py);

        Ok((obsrv, info))
    }

    pub fn step<'a>(
        &mut self,
        py: Python<'a>,
        action: BitMove,
    ) -> PyResult<(&'a PyList, i64, bool, bool)> {
        // input: action: ActType
        // output: [ObsType, SupportsFloat, bool, bool, dict[str, Any]]

        self.board.apply_move(action);
        self.step_num += 1;
        let obsrv = self.get_state(py)?;

        let is_checkmate = self.board.checkmate();
        let is_stalemate = self.board.stalemate();

        let terminated = is_checkmate && is_stalemate;
        let truncated = false;

        // add to info: zobrist, fen
        // add to state: basically the state from pleco::Board.Arc<State>
        self.action_moves_map = self
            .board
            .generate_moves()
            .into_iter()
            .map(|bit_move| (ChessEnv::move_to_action(&bit_move.clone().into()), bit_move))
            .collect::<HashMap<_, _>>();

        Ok((obsrv, 0, terminated, truncated))
    }

    pub fn render<'a>(&mut self, py: Python<'a>) -> PyResult<&'a PyString> {
        let frame = self.board.pretty_string();
        let py_frame = PyString::new(py, &frame);
        Ok(py_frame)
    }

    pub fn close<'a>(&mut self, _py: Python<'a>) -> PyResult<()> {
        Ok(())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn gym_chess_pleco(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BitMove>()?;
    m.add_class::<BitBoard>()?;
    m.add_class::<Board>()?;
    m.add_class::<CastleType>()?;
    m.add_class::<ChessEnv>()?;
    m.add_class::<GenTypes>()?;
    m.add_class::<Piece>()?;
    m.add_class::<PieceType>()?;
    m.add_class::<Player>()?;
    m.add_class::<Score>()?;
    m.add_class::<ScoringMove>()?;
    m.add_class::<Square>()?;
    Ok(())
}
