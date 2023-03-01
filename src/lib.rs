use pyo3::prelude::*;

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
pub struct ChessEnv {}

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

    // pub fn next_state<'a>(
    //     &mut self,
    //     _py: Python<'a>,
    //     state_py: &'a PyDict,
    //     _player: &str,
    //     _move: &str,
    // ) -> PyResult<(&'a PyDict, isize)> {
    //     todo!();
    //     // parse state
    //     // let state: State = convert_py_state(_py, state_py)?;

    //     // // parse arguments
    //     // let player: Color = player_string_to_enum(_player);

    //     // // next state
    //     // let move_union = engine::convert_move_to_type(_move);
    //     // let (mut new_state, reward) = engine::next_state(&state, player, move_union);

    //     // // update kings under attack
    //     // engine::update_state(&mut new_state);
    //     // // if both kings are checked, this position is impossible => raise exception
    //     // if new_state.white_king_is_checked == true && new_state.black_king_is_checked == true {
    //     //     println!("Both Kings are in check: this position is impossible");
    //     //     PyException::new_err("Both Kings are in check: this position is impossible")
    //     //         .restore(_py);
    //     // }

    //     // // return new state
    //     // let new_state_py = PyDict::new(_py);
    //     // new_state.to_py_object(new_state_py);
    //     // return Ok((new_state_py, reward));
    // }
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
