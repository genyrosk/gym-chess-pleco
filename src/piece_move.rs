use pyo3::prelude::*;
use std::convert::From;

/// Represents a singular move.
///
/// A `BitMove` consists of 16 bits, all of which to include a source square, destination square,
/// and special move-flags to differentiate types of moves.
#[derive(Clone)]
#[pyclass]
pub struct BitMove {
    data: u16,
}

impl BitMove {
    pub fn new(input: u16) -> BitMove {
        BitMove { data: input }
    }
}

#[pymethods]
impl BitMove {
    fn __repr__(&self) -> String {
        pleco::BitMove::new(self.data).stringify()
    }
}

impl From<pleco::BitMove> for BitMove {
    fn from(bit_move: pleco::BitMove) -> BitMove {
        BitMove::new(bit_move.get_raw())
    }
}

impl From<BitMove> for pleco::BitMove {
    fn from(bit_move: BitMove) -> pleco::BitMove {
        pleco::BitMove::new(bit_move.data)
    }
}

impl From<&BitMove> for pleco::BitMove {
    fn from(bit_move: &BitMove) -> pleco::BitMove {
        pleco::BitMove::new(bit_move.data)
    }
}

/// Structure containing both a score (represented as a i16) and a `BitMove`.
///
/// This is useful for tracking a list of moves alongside each of their scores.
#[derive(Clone)]
#[pyclass]
pub struct ScoringMove {
    pub bit_move: BitMove,
    pub score: i16,
}

impl From<pleco::ScoringMove> for ScoringMove {
    fn from(mov: pleco::ScoringMove) -> ScoringMove {
        ScoringMove {
            bit_move: mov.bit_move.into(),
            score: mov.score,
        }
    }
}

impl From<ScoringMove> for pleco::ScoringMove {
    fn from(mov: ScoringMove) -> pleco::ScoringMove {
        pleco::ScoringMove {
            bit_move: mov.bit_move.into(),
            score: mov.score,
        }
    }
}
