use pyo3::prelude::*;

#[pyclass]
#[derive(Copy, Clone, Debug)]
pub struct Square(pleco::SQ);

#[pymethods]
impl Square {
    fn __repr__(&self) -> String {
        self.0.to_string()
    }

    #[classattr]
    pub const A1: Square = Square(pleco::SQ::A1);
    #[classattr]
    pub const B1: Square = Square(pleco::SQ::B1);
    #[classattr]
    pub const C1: Square = Square(pleco::SQ::C1);
    #[classattr]
    pub const D1: Square = Square(pleco::SQ::D1);
    #[classattr]
    pub const E1: Square = Square(pleco::SQ::E1);
    #[classattr]
    pub const F1: Square = Square(pleco::SQ::F1);
    #[classattr]
    pub const G1: Square = Square(pleco::SQ::G1);
    #[classattr]
    pub const H1: Square = Square(pleco::SQ::H1);
    #[classattr]
    pub const A2: Square = Square(pleco::SQ::A2);
    #[classattr]
    pub const B2: Square = Square(pleco::SQ::B2);
    #[classattr]
    pub const C2: Square = Square(pleco::SQ::C2);
    #[classattr]
    pub const D2: Square = Square(pleco::SQ::D2);
    #[classattr]
    pub const E2: Square = Square(pleco::SQ::E2);
    #[classattr]
    pub const F2: Square = Square(pleco::SQ::F2);
    #[classattr]
    pub const G2: Square = Square(pleco::SQ::G2);
    #[classattr]
    pub const H2: Square = Square(pleco::SQ::H2);
    #[classattr]
    pub const A3: Square = Square(pleco::SQ::A3);
    #[classattr]
    pub const B3: Square = Square(pleco::SQ::B3);
    #[classattr]
    pub const C3: Square = Square(pleco::SQ::C3);
    #[classattr]
    pub const D3: Square = Square(pleco::SQ::D3);
    #[classattr]
    pub const E3: Square = Square(pleco::SQ::E3);
    #[classattr]
    pub const F3: Square = Square(pleco::SQ::F3);
    #[classattr]
    pub const G3: Square = Square(pleco::SQ::G3);
    #[classattr]
    pub const H3: Square = Square(pleco::SQ::H3);
    #[classattr]
    pub const A4: Square = Square(pleco::SQ::A4);
    #[classattr]
    pub const B4: Square = Square(pleco::SQ::B4);
    #[classattr]
    pub const C4: Square = Square(pleco::SQ::C4);
    #[classattr]
    pub const D4: Square = Square(pleco::SQ::D4);
    #[classattr]
    pub const E4: Square = Square(pleco::SQ::E4);
    #[classattr]
    pub const F4: Square = Square(pleco::SQ::F4);
    #[classattr]
    pub const G4: Square = Square(pleco::SQ::G4);
    #[classattr]
    pub const H4: Square = Square(pleco::SQ::H4);
    #[classattr]
    pub const A5: Square = Square(pleco::SQ::A5);
    #[classattr]
    pub const B5: Square = Square(pleco::SQ::B5);
    #[classattr]
    pub const C5: Square = Square(pleco::SQ::C5);
    #[classattr]
    pub const D5: Square = Square(pleco::SQ::D5);
    #[classattr]
    pub const E5: Square = Square(pleco::SQ::E5);
    #[classattr]
    pub const F5: Square = Square(pleco::SQ::F5);
    #[classattr]
    pub const G5: Square = Square(pleco::SQ::G5);
    #[classattr]
    pub const H5: Square = Square(pleco::SQ::H5);
    #[classattr]
    pub const A6: Square = Square(pleco::SQ::A6);
    #[classattr]
    pub const B6: Square = Square(pleco::SQ::B6);
    #[classattr]
    pub const C6: Square = Square(pleco::SQ::C6);
    #[classattr]
    pub const D6: Square = Square(pleco::SQ::D6);
    #[classattr]
    pub const E6: Square = Square(pleco::SQ::E6);
    #[classattr]
    pub const F6: Square = Square(pleco::SQ::F6);
    #[classattr]
    pub const G6: Square = Square(pleco::SQ::G6);
    #[classattr]
    pub const H6: Square = Square(pleco::SQ::H6);
    #[classattr]
    pub const A7: Square = Square(pleco::SQ::A7);
    #[classattr]
    pub const B7: Square = Square(pleco::SQ::B7);
    #[classattr]
    pub const C7: Square = Square(pleco::SQ::C7);
    #[classattr]
    pub const D7: Square = Square(pleco::SQ::D7);
    #[classattr]
    pub const E7: Square = Square(pleco::SQ::E7);
    #[classattr]
    pub const F7: Square = Square(pleco::SQ::F7);
    #[classattr]
    pub const G7: Square = Square(pleco::SQ::G7);
    #[classattr]
    pub const H7: Square = Square(pleco::SQ::H7);
    #[classattr]
    pub const A8: Square = Square(pleco::SQ::A8);
    #[classattr]
    pub const B8: Square = Square(pleco::SQ::B8);
    #[classattr]
    pub const C8: Square = Square(pleco::SQ::C8);
    #[classattr]
    pub const D8: Square = Square(pleco::SQ::D8);
    #[classattr]
    pub const E8: Square = Square(pleco::SQ::E8);
    #[classattr]
    pub const F8: Square = Square(pleco::SQ::F8);
    #[classattr]
    pub const G8: Square = Square(pleco::SQ::G8);
    #[classattr]
    pub const H8: Square = Square(pleco::SQ::H8);
}

impl From<pleco::SQ> for Square {
    fn from(sq: pleco::SQ) -> Square {
        Square(sq)
    }
}

impl From<Square> for pleco::SQ {
    fn from(sq: Square) -> pleco::SQ {
        sq.0
    }
}
