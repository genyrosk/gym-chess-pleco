use pleco::{Board, PieceType, Player};

fn main() {
    let board = Board::start_pos();
    let board_fen = board.fen();
    println!("board_fen: {}", board_fen);

    let moves = board.generate_moves();

    for move_ in moves {
        println!("move: {}", move_);
    }

    println!("Hello World");
}
