use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};

mod bitboard;
mod board;
mod core;
mod error;
mod piece_move;
mod score;
mod square;

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

    pub fn get_actions(&self) -> Vec<BitMove> {
        self.board.generate_moves()
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
