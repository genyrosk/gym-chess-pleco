use pyo3::prelude::*;
// use pyo3::types::Py
// use std::convert::From;
// use std::fmt;
// use std::fmt::Debug;

mod error;
mod piece_move;

use crate::error::CustomError;
use crate::piece_move::{BitMove, ScoringMove};

#[pyclass]
#[repr(u8)]
enum Player {
    White = 0,
    Black = 1,
}

#[pymethods]
impl Player {
    fn __repr__(&self) -> String {
        match self {
            Player::White => "White".to_string(),
            Player::Black => "Black".to_string(),
        }
    }
}

impl From<pleco::Player> for Player {
    fn from(player: pleco::Player) -> Player {
        match player {
            pleco::Player::White => Player::White,
            pleco::Player::Black => Player::Black,
        }
    }
}

impl From<Player> for pleco::Player {
    fn from(player: Player) -> pleco::Player {
        match player {
            Player::White => pleco::Player::White,
            Player::Black => pleco::Player::Black,
        }
    }
}

#[pyclass]
#[derive(Copy, Clone, Debug)]
pub enum GenTypes {
    All,
    Captures,
    Quiets,
    QuietChecks,
    Evasions,
    NonEvasions,
}

#[pymethods]
impl GenTypes {
    fn __repr__(&self) -> String {
        match self {
            GenTypes::All => "White".to_string(),
            GenTypes::Captures => "Captures".to_string(),
            GenTypes::Quiets => "Quiets".to_string(),
            GenTypes::QuietChecks => "QuietChecks".to_string(),
            GenTypes::Evasions => "Evasions".to_string(),
            GenTypes::NonEvasions => "NonEvasions".to_string(),
        }
    }
}

impl From<pleco::core::GenTypes> for GenTypes {
    fn from(gen_type: pleco::core::GenTypes) -> GenTypes {
        match gen_type {
            pleco::core::GenTypes::All => GenTypes::All,
            pleco::core::GenTypes::Captures => GenTypes::Captures,
            pleco::core::GenTypes::Quiets => GenTypes::Quiets,
            pleco::core::GenTypes::QuietChecks => GenTypes::QuietChecks,
            pleco::core::GenTypes::Evasions => GenTypes::Evasions,
            pleco::core::GenTypes::NonEvasions => GenTypes::NonEvasions,
        }
    }
}

impl From<GenTypes> for pleco::core::GenTypes {
    fn from(gen_type: GenTypes) -> pleco::core::GenTypes {
        match gen_type {
            GenTypes::All => pleco::core::GenTypes::All,
            GenTypes::Captures => pleco::core::GenTypes::Captures,
            GenTypes::Quiets => pleco::core::GenTypes::Quiets,
            GenTypes::QuietChecks => pleco::core::GenTypes::QuietChecks,
            GenTypes::Evasions => pleco::core::GenTypes::Evasions,
            GenTypes::NonEvasions => pleco::core::GenTypes::NonEvasions,
        }
    }
}

#[pyclass]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    None = 0,
    P = 1,
    N = 2,
    B = 3,
    R = 4,
    Q = 5,
    K = 6,
    All = 7,
}

#[pymethods]
impl PieceType {
    fn __repr__(&self) -> String {
        Into::<pleco::core::PieceType>::into(*self).to_string()
    }
}

impl From<pleco::core::PieceType> for PieceType {
    fn from(piece_type: pleco::core::PieceType) -> PieceType {
        match piece_type {
            pleco::core::PieceType::P => PieceType::P,
            pleco::core::PieceType::N => PieceType::N,
            pleco::core::PieceType::B => PieceType::B,
            pleco::core::PieceType::R => PieceType::R,
            pleco::core::PieceType::Q => PieceType::Q,
            pleco::core::PieceType::K => PieceType::K,
            pleco::core::PieceType::All => PieceType::All,
            pleco::core::PieceType::None => PieceType::None,
        }
    }
}

impl From<PieceType> for pleco::core::PieceType {
    fn from(piece_type: PieceType) -> pleco::core::PieceType {
        match piece_type {
            PieceType::P => pleco::core::PieceType::P,
            PieceType::N => pleco::core::PieceType::N,
            PieceType::B => pleco::core::PieceType::B,
            PieceType::R => pleco::core::PieceType::R,
            PieceType::Q => pleco::core::PieceType::Q,
            PieceType::K => pleco::core::PieceType::K,
            PieceType::All => pleco::core::PieceType::All,
            PieceType::None => pleco::core::PieceType::None,
        }
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct SQ(pub u8);

#[pymethods]
impl SQ {
    fn __repr__(&self) -> String {
        let sq = Into::<pleco::core::sq::SQ>::into(*self);
        sq.to_string()
    }
}

impl From<pleco::core::sq::SQ> for SQ {
    fn from(sq: pleco::core::sq::SQ) -> SQ {
        SQ(sq.0)
    }
}

impl From<SQ> for pleco::core::sq::SQ {
    fn from(sq: SQ) -> pleco::core::sq::SQ {
        pleco::core::sq::SQ(sq.0)
    }
}

// Piece

#[pyclass]
pub struct Board {
    inner: pleco::Board,
}

#[pymethods]
impl Board {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: pleco::Board::start_pos(),
        }
    }

    fn __repr__(&self) -> String {
        self.inner.pretty_string()
    }

    #[staticmethod]
    fn start_pos() -> Board {
        Board {
            inner: pleco::Board::start_pos(),
        }
    }

    // fn random(&self) -> PyResult<()> {
    //     todo!();
    // }

    #[staticmethod]
    fn from_fen(fen: &str) -> PyResult<Board> {
        let x = pleco::Board::from_fen(fen)
            .map(|board| Board { inner: board })
            .map_err(|err| Into::<CustomError>::into(err).into());
        x
    }

    fn fen(&self) -> String {
        self.inner.fen()
    }

    fn apply_move(&mut self, bit_move: BitMove) {
        self.inner.apply_move(bit_move.into());
    }

    fn undo_move(&mut self) {
        self.inner.undo_move();
    }

    fn generate_moves(&self) -> Vec<BitMove> {
        let move_list = self
            .inner
            .generate_moves()
            .to_vec()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<_>>();
        move_list
    }

    fn generate_scoring_moves(&self) -> Vec<ScoringMove> {
        let move_list = self
            .inner
            .generate_scoring_moves()
            .to_vec()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<_>>();
        move_list
    }

    fn generate_pseudolegal_moves(&self) -> Vec<BitMove> {
        let move_list = self
            .inner
            .generate_pseudolegal_moves()
            .to_vec()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<_>>();
        move_list
    }

    // fn generate_moves_of_type(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn generate_pseudolegal_moves_of_type(&self) -> PyResult<()> {
    //     todo!();
    // }

    fn turn(&self) -> Player {
        self.inner.turn().into()
    }

    // fn zobrist(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn pawn_key(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn moves_played(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn depth(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn rule_50(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn piece_captured_last_turn(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn ply(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn psq(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn ep_square(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn occupied(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn empty(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn get_occupied_player(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn occupied_white(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn occupied_black(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn piece_bb(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn sliding_piece_bb(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn diagonal_piece_bb(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn piece_bb_both_players(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn piece_two_bb_both_players(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn piece_two_bb(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn count_piece(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn count_pieces_player(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn count_all_pieces(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn piece_at_sq(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn king_sq(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn pinned_pieces(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn all_pinned_pieces(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn pinning_pieces(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn castling_bits(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn can_castle(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn player_can_castle(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn castle_impeded(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn castling_rook_square(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn last_move(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn piece_last_captured(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn material_key(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn non_pawn_material(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn non_pawn_material_all(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn in_check(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn checkmate(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn stalemate(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn checkers(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn discovered_check_candidates(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn pieces_pinned(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn attackers_to(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn attacks_from(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn pawn_passed(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn legal_move(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn pseudo_legal_move(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn opposite_bishops(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn advanced_pawn_push(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn is_capture(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn is_capture_or_promotion(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn gives_check(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn see_ge(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn moved_piece(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn captured_piece(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn key_after(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn pretty_string(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn get_piece_locations(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn print_debug_info(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn pretty_print(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn fancy_print(&self) -> PyResult<()> {
    //     todo!();
    // }

    // fn next_state<'a>(
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

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn gym_chess_pleco(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Board>()?;
    Ok(())
}
