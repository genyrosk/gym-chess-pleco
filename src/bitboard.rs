use pyo3::prelude::*;

#[pyclass]
#[derive(Copy, Clone, Debug)]
pub struct BitBoard(pleco::BitBoard);

#[pymethods]
impl BitBoard {
    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

impl From<pleco::BitBoard> for BitBoard {
    fn from(bit_board: pleco::BitBoard) -> BitBoard {
        BitBoard(bit_board)
    }
}

impl From<BitBoard> for pleco::BitBoard {
    fn from(bit_board: BitBoard) -> pleco::BitBoard {
        bit_board.0
    }
}
