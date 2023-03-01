use crate::bitboard::BitBoard;
use crate::core::{CastleType, GenTypes, Piece, PieceType, Player};
use crate::error::CustomError;
use crate::piece_move::{BitMove, ScoringMove};
use crate::score::Score;
use crate::square::Square;
use pyo3::prelude::*;

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

    pub fn __repr__(&self) -> String {
        self.inner.pretty_string()
    }

    #[staticmethod]
    pub fn start_pos() -> Board {
        Board {
            inner: pleco::Board::start_pos(),
        }
    }

    pub fn state(&self) -> [[Piece; 8]; 8] {
        let piece_locations = self.inner.get_piece_locations();

        let mut state = [[Piece::None; 8]; 8];
        for (idx, (_, piece)) in piece_locations.into_iter().enumerate() {
            let i = idx % 8;
            let j = idx / 8;
            state[i][j] = piece.into();
        }
        state
    }

    // pub fn random(&self) -> PyResult<()> {
    //     todo!();
    // }

    #[staticmethod]
    pub fn from_fen(fen: &str) -> PyResult<Board> {
        let x = pleco::Board::from_fen(fen)
            .map(|board| Board { inner: board })
            .map_err(|err| Into::<CustomError>::into(err).into());
        x
    }

    pub fn fen(&self) -> String {
        self.inner.fen()
    }

    pub fn apply_move(&mut self, bit_move: BitMove) {
        self.inner.apply_move(bit_move.into());
    }

    pub fn undo_move(&mut self) {
        self.inner.undo_move();
    }

    pub fn generate_moves(&self) -> Vec<BitMove> {
        let move_list = self
            .inner
            .generate_moves()
            .to_vec()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<_>>();
        move_list
    }

    pub fn generate_scoring_moves(&self) -> Vec<ScoringMove> {
        let move_list = self
            .inner
            .generate_scoring_moves()
            .to_vec()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<_>>();
        move_list
    }

    pub fn generate_pseudolegal_moves(&self) -> Vec<BitMove> {
        let move_list = self
            .inner
            .generate_pseudolegal_moves()
            .to_vec()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<_>>();
        move_list
    }

    pub fn generate_moves_of_type(&self, gen_type: GenTypes) -> Vec<BitMove> {
        let move_list = self
            .inner
            .generate_moves_of_type(gen_type.into())
            .to_vec()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<_>>();
        move_list
    }

    pub fn generate_pseudolegal_moves_of_type(&self, gen_type: GenTypes) -> Vec<BitMove> {
        let move_list = self
            .inner
            .generate_pseudolegal_moves_of_type(gen_type.into())
            .to_vec()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<_>>();
        move_list
    }

    pub fn turn(&self) -> Player {
        self.inner.turn().into()
    }

    pub fn zobrist(&self) -> u64 {
        self.inner.zobrist()
    }

    pub fn pawn_key(&self) -> u64 {
        self.inner.pawn_key()
    }

    pub fn moves_played(&self) -> u16 {
        self.inner.moves_played()
    }

    pub fn depth(&self) -> u16 {
        self.inner.depth()
    }

    pub fn rule_50(&self) -> i16 {
        self.inner.rule_50()
    }

    pub fn piece_captured_last_turn(&self) -> PieceType {
        self.inner.piece_captured_last_turn().into()
    }

    pub fn ply(&self) -> u16 {
        self.inner.ply()
    }

    pub fn psq(&self) -> Score {
        self.inner.psq().into()
    }

    pub fn ep_square(&self) -> Square {
        self.inner.ep_square().into()
    }

    pub fn occupied(&self) -> BitBoard {
        self.inner.occupied().into()
    }

    pub fn empty(&self, sq: Square) -> bool {
        self.inner.empty(sq.into())
    }

    pub fn get_occupied_player(&self, player: Player) -> BitBoard {
        self.inner.get_occupied_player(player.into()).into()
    }

    pub fn occupied_white(&self) -> BitBoard {
        self.inner.occupied_white().into()
    }

    pub fn occupied_black(&self) -> BitBoard {
        self.inner.occupied_black().into()
    }

    pub fn piece_bb(&self, player: Player, piece: PieceType) -> BitBoard {
        self.inner.piece_bb(player.into(), piece.into()).into()
    }

    pub fn sliding_piece_bb(&self, player: Player) -> BitBoard {
        self.inner.sliding_piece_bb(player.into()).into()
    }

    pub fn diagonal_piece_bb(&self, player: Player) -> BitBoard {
        self.inner.diagonal_piece_bb(player.into()).into()
    }

    pub fn piece_bb_both_players(&self, piece: PieceType) -> BitBoard {
        self.inner.piece_bb_both_players(piece.into()).into()
    }

    pub fn piece_two_bb_both_players(&self, piece: PieceType, piece2: PieceType) -> BitBoard {
        self.inner
            .piece_two_bb_both_players(piece.into(), piece2.into())
            .into()
    }

    pub fn piece_two_bb(&self, piece: PieceType, piece2: PieceType, player: Player) -> BitBoard {
        self.inner
            .piece_two_bb(piece.into(), piece2.into(), player.into())
            .into()
    }

    pub fn count_piece(&self, player: Player, piece: PieceType) -> u8 {
        self.inner.count_piece(player.into(), piece.into())
    }

    pub fn count_pieces_player(&self, player: Player) -> u8 {
        self.inner.count_pieces_player(player.into())
    }

    pub fn count_all_pieces(&self) -> u8 {
        self.inner.count_all_pieces()
    }

    pub fn piece_at_sq(&self, sq: Square) -> Piece {
        self.inner.piece_at_sq(sq.into()).into()
    }

    pub fn king_sq(&self, player: Player) -> Square {
        self.inner.king_sq(player.into()).into()
    }

    pub fn pinned_pieces(&self, player: Player) -> BitBoard {
        self.inner.pinned_pieces(player.into()).into()
    }

    pub fn all_pinned_pieces(&self, player: Player) -> BitBoard {
        self.inner.all_pinned_pieces(player.into()).into()
    }

    pub fn pinning_pieces(&self, player: Player) -> BitBoard {
        self.inner.pinning_pieces(player.into()).into()
    }

    pub fn castling_bits(&self) -> u8 {
        self.inner.castling_bits()
    }

    pub fn can_castle(&self, player: Player, castle_type: CastleType) -> bool {
        self.inner.can_castle(player.into(), castle_type.into())
    }

    pub fn castle_impeded(&self, castle_type: CastleType) -> bool {
        self.inner.castle_impeded(castle_type.into())
    }

    pub fn castling_rook_square(&self, castle_type: CastleType) -> Square {
        self.inner.castling_rook_square(castle_type.into()).into()
    }

    pub fn last_move(&self) -> Option<BitMove> {
        self.inner.last_move().map(|m| m.into())
    }

    pub fn piece_last_captured(&self) -> PieceType {
        self.inner.piece_last_captured().into()
    }

    pub fn material_key(&self) -> u64 {
        self.inner.material_key()
    }

    pub fn non_pawn_material(&self, player: Player) -> i32 {
        self.inner.non_pawn_material(player.into())
    }

    pub fn non_pawn_material_all(&self) -> i32 {
        self.inner.non_pawn_material_all()
    }

    //  ------- CHECKING  -------

    pub fn in_check(&self) -> bool {
        self.inner.in_check()
    }

    pub fn checkmate(&self) -> bool {
        self.inner.checkmate()
    }

    pub fn stalemate(&self) -> bool {
        self.inner.stalemate()
    }

    pub fn checkers(&self) -> BitBoard {
        self.inner.checkers().into()
    }

    pub fn discovered_check_candidates(&self) -> BitBoard {
        self.inner.discovered_check_candidates().into()
    }

    pub fn pieces_pinned(&self, player: Player) -> BitBoard {
        self.inner.pieces_pinned(player.into()).into()
    }

    pub fn attackers_to(&self, sq: Square, occupied: BitBoard) -> BitBoard {
        self.inner.attackers_to(sq.into(), occupied.into()).into()
    }

    pub fn attacks_from(&self, piece: PieceType, sq: Square, player: Player) -> BitBoard {
        self.inner
            .attacks_from(piece.into(), sq.into(), player.into())
            .into()
    }

    pub fn pawn_passed(&self, player: Player, sq: Square) -> bool {
        self.inner.pawn_passed(player.into(), sq.into()).into()
    }

    //  ------- Move Testing -------

    pub fn legal_move(&self, m: BitMove) -> bool {
        self.inner.legal_move(m.into())
    }

    pub fn pseudo_legal_move(&self, m: BitMove) -> bool {
        self.inner.pseudo_legal_move(m.into())
    }

    pub fn opposite_bishops(&self) -> bool {
        self.inner.opposite_bishops()
    }

    pub fn advanced_pawn_push(&self, mov: BitMove) -> bool {
        self.inner.advanced_pawn_push(mov.into())
    }

    pub fn is_capture(&self, mov: BitMove) -> bool {
        self.inner.is_capture(mov.into())
    }

    pub fn is_capture_or_promotion(&self, mov: BitMove) -> bool {
        self.inner.is_capture_or_promotion(mov.into())
    }

    pub fn gives_check(&self, m: BitMove) -> bool {
        self.inner.gives_check(m.into())
    }

    pub fn see_ge(&self, mov: BitMove, threshold: i32) -> bool {
        self.inner.see_ge(mov.into(), threshold)
    }

    pub fn moved_piece(&self, m: BitMove) -> Piece {
        self.inner.moved_piece(m.into()).into()
    }

    pub fn captured_piece(&self, m: BitMove) -> PieceType {
        self.inner.captured_piece(m.into()).into()
    }

    pub fn key_after(&self, m: BitMove) -> u64 {
        self.inner.key_after(m.into())
    }

    pub fn pretty_string(&self) -> String {
        self.inner.pretty_string()
    }

    pub fn print_debug_info(&self) {
        self.inner.print_debug_info();
    }

    pub fn pretty_print(&self) {
        self.inner.pretty_print();
    }

    pub fn fancy_print(&self) {
        self.inner.fancy_print();
    }
}
