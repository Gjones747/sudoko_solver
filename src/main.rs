pub mod board;
fn main() {
    let mut board: board::board::Board = board::board::Board::make_board();
    board.set_input_tile(0, 0, 5);
    board.set_input_tile(0, 1, 3);
    board.set_input_tile(0, 4, 7);

    board.set_input_tile(1, 0, 6);
    board.set_input_tile(1, 3, 1);
    board.set_input_tile(1, 4, 9);
    board.set_input_tile(1, 5, 5);

    board.set_input_tile(2, 1, 9);
    board.set_input_tile(2, 2, 8);
    board.set_input_tile(2, 7, 6);

    board.set_input_tile(3, 0, 8);
    board.set_input_tile(3, 4, 6);
    board.set_input_tile(3, 8, 3);

    board.set_input_tile(4, 0, 4);
    board.set_input_tile(4, 3, 8);
    board.set_input_tile(4, 5, 3);
    board.set_input_tile(4, 8, 1);

    board.set_input_tile(5, 0, 7);
    board.set_input_tile(5, 4, 2);
    board.set_input_tile(5, 8, 6);

    board.set_input_tile(6, 1, 6);
    board.set_input_tile(6, 6, 2);
    board.set_input_tile(6, 7, 8);

    board.set_input_tile(7, 3, 4);
    board.set_input_tile(7, 4, 1);
    board.set_input_tile(7, 5, 9);
    board.set_input_tile(7, 8, 5);

    board.set_input_tile(8, 4, 8);
    board.set_input_tile(8, 7, 7);
    board.set_input_tile(8, 8, 9);


    board.print_board();

    if board.check_board() {
        println!("solved")
    } else {
        println!("not solved")
    }

}

