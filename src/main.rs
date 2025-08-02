pub mod board;
fn main() {
    let board: board::board::Board = board::board::Board::make_board();

    board.print_board();

}

