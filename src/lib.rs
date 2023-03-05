use action_space::{action_to_move_string, ACTION_SPACE_LEN};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};
use std::collections::HashMap;

mod action_space;
mod bitboard;
mod board;
mod core;
mod error;
mod piece_move;
mod score;
mod square;

use crate::action_space::{Action, ActionId, ACTION_SPACE};
use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::core::{CastleType, GenTypes, Piece, PieceType, Player};
use crate::piece_move::{BitMove, ScoringMove};
use crate::score::Score;
use crate::square::Square;

#[pyclass]
pub struct ChessEnv {
    pub board: Board,
    pub step_num: u64,
    pub action_map: HashMap<ActionId, Action>,
}

impl ChessEnv {
    pub fn move_to_action(bit_move: &pleco::BitMove) -> ActionId {
        let sq_src = bit_move.get_src();
        let diff_col = bit_move.dest_col() as i8 - bit_move.src_col() as i8;
        let diff_row = bit_move.dest_row() as i8 - bit_move.src_row() as i8;

        // Underpromotion
        if bit_move.is_promo() && bit_move.promo_piece() != pleco::PieceType::Q {
            // println!("it's an underpromotion move!");
            let target_piece = bit_move.promo_piece();
            let target_piece = match target_piece {
                pleco::PieceType::R => 0,
                pleco::PieceType::B => 1,
                pleco::PieceType::N => 2,
                _ => unreachable!(),
            };

            let promo_move: u16 = match diff_col {
                -1 => 0,
                0 => 1,
                1 => 2,
                _ => unreachable!(),
            };

            let promo = target_piece * 4 + promo_move;
            // println!("promo: {}", promo);

            let move_type = 8 * 7 + 8 + promo;
            let action = (u8::from(sq_src) as ActionId) * 73 + move_type;
            return action;
        }

        if (diff_col.abs() == 1 && diff_row.abs() == 2)
            || (diff_col.abs() == 2 && diff_row.abs() == 1)
        {
            // println!("it's a knight move!");
            let knight_move: u16 = match (diff_col, diff_row) {
                (1, 2) => 0,
                (2, 1) => 1,
                (2, -1) => 2,
                (1, -2) => 3,
                (-1, -2) => 4,
                (-2, -1) => 5,
                (-2, 1) => 6,
                (-1, 2) => 7,
                #[allow(unreachable_code)]
                _ => !unreachable!(),
            };
            let move_type = 8 * 7 + knight_move;
            let action = (u8::from(sq_src) as ActionId) * 73 + move_type;
            return action;
        }

        // println!("it's a directional move!");
        let direction: u16 = match (diff_col.signum(), diff_row.signum()) {
            (0, 1) => 0,
            (1, 1) => 1,
            (1, 0) => 2,
            (1, -1) => 3,
            (0, -1) => 4,
            (-1, -1) => 5,
            (-1, 0) => 6,
            (-1, 1) => 7,
            #[allow(unreachable_code)]
            _ => !unreachable!(),
        };
        let num_steps = std::cmp::max(diff_row.abs(), diff_col.abs()) as u16;
        let directional_move = direction * 8 + num_steps;
        let action = (u8::from(sq_src) as ActionId) * 73 + directional_move;
        return action;
    }

    fn generate_actions(&mut self) {
        self.action_map = self
            .board
            .generate_moves()
            .into_iter()
            .map(|bit_move| {
                let action_id = ChessEnv::move_to_action(&bit_move.clone().into());
                let action = Action {
                    id: action_id,
                    bit_move,
                };
                (action_id, action)
            })
            .collect::<HashMap<_, _>>();
    }
}

#[pymethods]
impl ChessEnv {
    // As specified by gymnasium's Env class specification
    //
    // https://gymnasium.farama.org/api/env/#gymnasium.Env.close
    //
    // Main API
    // - step()
    // - reset()
    // - render()
    // - close()
    //
    // Optional
    // - action_space
    // - observation_space
    // - reward_range
    // - spec
    // - metadata
    // - np_random

    #[new]
    pub fn new() -> Self {
        let mut env = Self {
            board: Board::new(),
            step_num: 0,
            action_map: HashMap::new(),
        };
        env.generate_actions();
        env
    }

    pub fn reset<'a>(
        &mut self,
        py: Python<'a>,
        _seed: Option<i64>,
        _options: Option<&'a PyDict>,
    ) -> PyResult<(&'a PyList, &'a PyDict)> {
        self.board = Board::start_pos();
        let obsrv = self.get_state(py)?;

        let info = PyDict::new(py);

        Ok((obsrv, info))
    }

    pub fn step<'a>(
        &mut self,
        py: Python<'a>,
        action_id: u16,
    ) -> PyResult<(&'a PyList, i64, bool, bool)> {
        // input: action: ActType
        // output: [ObsType, SupportsFloat, bool, bool, dict[str, Any]]

        let action = self.action_map.get(&action_id).unwrap().clone();
        let bit_move = action.bit_move;

        self.board.apply_move(bit_move);
        self.step_num += 1;
        let obsrv = self.get_state(py)?;

        let is_checkmate = self.board.checkmate();
        let is_stalemate = self.board.stalemate();

        let terminated = is_checkmate && is_stalemate;
        let truncated = false;

        // add to info: zobrist, fen
        // add to state: basically the state from pleco::Board.Arc<State>
        self.generate_actions();

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

    // Additioanl methods
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

    pub fn get_actions<'a>(&self, py: Python<'a>) -> &'a PyList {
        let actions = self.action_map.values().collect::<Vec<_>>();
        let y = PyList::new(py, actions);
        y
    }

    pub fn get_action_mask<'a>(&self, py: Python<'a>) -> &'a PyList {
        let action_ids = self.action_map.keys().copied().collect::<Vec<_>>();
        let mut action_mask = [0 as u16; ACTION_SPACE_LEN as usize];
        for action_id in action_ids {
            action_mask[action_id as usize] = 1;
        }
        PyList::new(py, action_mask)
    }

    #[staticmethod]
    pub fn build_action_space<'a>(py: Python<'a>) -> &'a PyList {
        // Includes illegal moves which need to be filtered out
        let action_space = ACTION_SPACE.map(action_to_move_string).collect::<Vec<_>>();
        let list = PyList::new(py, action_space);
        todo!()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn gym_chess_pleco(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Action>()?;
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
