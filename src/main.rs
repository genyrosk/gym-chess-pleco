use pleco::{Board, Piece, PieceType, Player, SQ};

fn main() {
    println!("Hello World");

    let board = Board::start_pos();
    let board_fen = board.fen();
    println!("board_fen: {}", board_fen);

    let moves = board.generate_moves();

    for move_ in moves {
        println!("move: {}", move_);
    }

    let sq = SQ::A1;
    println!("square: {}", sq);
}
