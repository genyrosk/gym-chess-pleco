use pyo3::prelude::*;

#[pyclass]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
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

    fn other_player(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
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
#[derive(Copy, Clone, PartialEq, Debug)]
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
#[derive(Copy, Clone, PartialEq, Debug)]
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
        Into::<pleco::PieceType>::into(*self).to_string()
    }
}

impl From<pleco::PieceType> for PieceType {
    fn from(piece_type: pleco::PieceType) -> PieceType {
        match piece_type {
            pleco::PieceType::P => PieceType::P,
            pleco::PieceType::N => PieceType::N,
            pleco::PieceType::B => PieceType::B,
            pleco::PieceType::R => PieceType::R,
            pleco::PieceType::Q => PieceType::Q,
            pleco::PieceType::K => PieceType::K,
            pleco::PieceType::All => PieceType::All,
            pleco::PieceType::None => PieceType::None,
        }
    }
}

impl From<PieceType> for pleco::PieceType {
    fn from(piece_type: PieceType) -> pleco::PieceType {
        match piece_type {
            PieceType::P => pleco::PieceType::P,
            PieceType::N => pleco::PieceType::N,
            PieceType::B => pleco::PieceType::B,
            PieceType::R => pleco::PieceType::R,
            PieceType::Q => pleco::PieceType::Q,
            PieceType::K => pleco::PieceType::K,
            PieceType::All => pleco::PieceType::All,
            PieceType::None => pleco::PieceType::None,
        }
    }
}

#[pyclass]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    None = 0b0000,
    WhitePawn = 0b0001,
    WhiteKnight = 0b0010,
    WhiteBishop = 0b0011,
    WhiteRook = 0b0100,
    WhiteQueen = 0b0101,
    WhiteKing = 0b0110,
    BlackPawn = 0b1001,
    BlackKnight = 0b1010,
    BlackBishop = 0b1011,
    BlackRook = 0b1100,
    BlackQueen = 0b1101,
    BlackKing = 0b1110,
}

#[pymethods]
impl Piece {
    fn __repr__(&self) -> String {
        Into::<pleco::Piece>::into(*self).to_string()
    }

    pub fn player(&self) -> Option<Player> {
        Into::<pleco::Piece>::into(*self).player().map(|p| p.into())
    }

    pub fn type_of(&self) -> PieceType {
        Into::<pleco::Piece>::into(*self).type_of().into()
    }

    pub fn player_piece(&self) -> Option<(Player, PieceType)> {
        Into::<pleco::Piece>::into(*self)
            .player_piece()
            .map(|(player, piece_type)| (player.into(), piece_type.into()))
    }

    #[staticmethod]
    pub fn make(player: Player, piece_type: PieceType) -> Option<Piece> {
        pleco::Piece::make(player.into(), piece_type.into()).map(|p| p.into())
    }
}

impl ToPyObject for Piece {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let obj: PyObject = Py::new(py, *self).unwrap().into_py(py);
        obj
    }
}

impl From<pleco::Piece> for Piece {
    fn from(piece: pleco::Piece) -> Piece {
        match piece {
            pleco::Piece::None => Piece::None,
            pleco::Piece::WhitePawn => Piece::WhitePawn,
            pleco::Piece::WhiteKnight => Piece::WhiteKnight,
            pleco::Piece::WhiteBishop => Piece::WhiteBishop,
            pleco::Piece::WhiteRook => Piece::WhiteRook,
            pleco::Piece::WhiteQueen => Piece::WhiteQueen,
            pleco::Piece::WhiteKing => Piece::WhiteKing,
            pleco::Piece::BlackPawn => Piece::BlackPawn,
            pleco::Piece::BlackKnight => Piece::BlackKnight,
            pleco::Piece::BlackBishop => Piece::BlackBishop,
            pleco::Piece::BlackRook => Piece::BlackRook,
            pleco::Piece::BlackQueen => Piece::BlackQueen,
            pleco::Piece::BlackKing => Piece::BlackKing,
        }
    }
}

impl From<Piece> for pleco::Piece {
    fn from(piece: Piece) -> pleco::Piece {
        match piece {
            Piece::None => pleco::Piece::None,
            Piece::WhitePawn => pleco::Piece::WhitePawn,
            Piece::WhiteKnight => pleco::Piece::WhiteKnight,
            Piece::WhiteBishop => pleco::Piece::WhiteBishop,
            Piece::WhiteRook => pleco::Piece::WhiteRook,
            Piece::WhiteQueen => pleco::Piece::WhiteQueen,
            Piece::WhiteKing => pleco::Piece::WhiteKing,
            Piece::BlackPawn => pleco::Piece::BlackPawn,
            Piece::BlackKnight => pleco::Piece::BlackKnight,
            Piece::BlackBishop => pleco::Piece::BlackBishop,
            Piece::BlackRook => pleco::Piece::BlackRook,
            Piece::BlackQueen => pleco::Piece::BlackQueen,
            Piece::BlackKing => pleco::Piece::BlackKing,
        }
    }
}

#[pyclass]
#[derive(Copy, Clone, PartialEq)]
pub struct CastleType(pleco::core::CastleType);

#[pymethods]
impl CastleType {
    fn __repr__(&self) -> String {
        match self.0 {
            pleco::core::CastleType::KingSide => "KingSide".to_string(),
            pleco::core::CastleType::QueenSide => "QueenSide".to_string(),
        }
    }
}

impl From<pleco::core::CastleType> for CastleType {
    fn from(castle_type: pleco::core::CastleType) -> CastleType {
        CastleType(castle_type)
    }
}

impl From<CastleType> for pleco::core::CastleType {
    fn from(castle_type: CastleType) -> pleco::core::CastleType {
        castle_type.0
    }
}
