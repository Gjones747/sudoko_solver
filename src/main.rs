pub mod board;
fn main() {
    let mut board: board::board::Board = board::board::Board::make_board();

    //this is a board setup written by chatgpt cuz the input is tedious right now
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


    // this is a solved board just for testing purposes -> this board was written by chatgpt cuz the input system is so annoying rn 

    // board.set_input_tile(0, 0, 5);
    // board.set_input_tile(0, 1, 3);
    // board.set_input_tile(0, 2, 4);
    // board.set_input_tile(0, 3, 6);
    // board.set_input_tile(0, 4, 7);
    // board.set_input_tile(0, 5, 8);
    // board.set_input_tile(0, 6, 9);
    // board.set_input_tile(0, 7, 1);
    // board.set_input_tile(0, 8, 2);

    // board.set_input_tile(1, 0, 6);
    // board.set_input_tile(1, 1, 7);
    // board.set_input_tile(1, 2, 2);
    // board.set_input_tile(1, 3, 1);
    // board.set_input_tile(1, 4, 9);
    // board.set_input_tile(1, 5, 5);
    // board.set_input_tile(1, 6, 3);
    // board.set_input_tile(1, 7, 4);
    // board.set_input_tile(1, 8, 8);

    // board.set_input_tile(2, 0, 1);
    // board.set_input_tile(2, 1, 9);
    // board.set_input_tile(2, 2, 8);
    // board.set_input_tile(2, 3, 3);
    // board.set_input_tile(2, 4, 4);
    // board.set_input_tile(2, 5, 2);
    // board.set_input_tile(2, 6, 5);
    // board.set_input_tile(2, 7, 6);
    // board.set_input_tile(2, 8, 7);

    // board.set_input_tile(3, 0, 8);
    // board.set_input_tile(3, 1, 5);
    // board.set_input_tile(3, 2, 9);
    // board.set_input_tile(3, 3, 7);
    // board.set_input_tile(3, 4, 6);
    // board.set_input_tile(3, 5, 1);
    // board.set_input_tile(3, 6, 4);
    // board.set_input_tile(3, 7, 2);
    // board.set_input_tile(3, 8, 3);

    // board.set_input_tile(4, 0, 4);
    // board.set_input_tile(4, 1, 2);
    // board.set_input_tile(4, 2, 6);
    // board.set_input_tile(4, 3, 8);
    // board.set_input_tile(4, 4, 5);
    // board.set_input_tile(4, 5, 3);
    // board.set_input_tile(4, 6, 7);
    // board.set_input_tile(4, 7, 9);
    // board.set_input_tile(4, 8, 1);

    // board.set_input_tile(5, 0, 7);
    // board.set_input_tile(5, 1, 1);
    // board.set_input_tile(5, 2, 3);
    // board.set_input_tile(5, 3, 9);
    // board.set_input_tile(5, 4, 2);
    // board.set_input_tile(5, 5, 4);
    // board.set_input_tile(5, 6, 8);
    // board.set_input_tile(5, 7, 5);
    // board.set_input_tile(5, 8, 6);

    // board.set_input_tile(6, 0, 9);
    // board.set_input_tile(6, 1, 6);
    // board.set_input_tile(6, 2, 1);
    // board.set_input_tile(6, 3, 5);
    // board.set_input_tile(6, 4, 3);
    // board.set_input_tile(6, 5, 7);
    // board.set_input_tile(6, 6, 2);
    // board.set_input_tile(6, 7, 8);
    // board.set_input_tile(6, 8, 4);

    // board.set_input_tile(7, 0, 2);
    // board.set_input_tile(7, 1, 8);
    // board.set_input_tile(7, 2, 7);
    // board.set_input_tile(7, 3, 4);
    // board.set_input_tile(7, 4, 1);
    // board.set_input_tile(7, 5, 9);
    // board.set_input_tile(7, 6, 6);
    // board.set_input_tile(7, 7, 3);
    // board.set_input_tile(7, 8, 5);

    // board.set_input_tile(8, 0, 3);
    // board.set_input_tile(8, 1, 4);
    // board.set_input_tile(8, 2, 5);
    // board.set_input_tile(8, 3, 2);
    // board.set_input_tile(8, 4, 8);
    // board.set_input_tile(8, 5, 6);
    // board.set_input_tile(8, 6, 1);
    // board.set_input_tile(8, 7, 7);
    // board.set_input_tile(8, 8, 9);


    board.print_board();

    if board.check_board() {
        println!("board not broken")
    } else {
        println!("board broken")
    }

    if board.solved {
        println!("board solved")
    } else {
        println!("board not solved")
    }

    board.solve();

    if board.check_board() {
        println!("board not broken")
    } else {
        println!("board broken")
    }

    if board.solved {
        println!("board solved")
    } else {
        println!("board not solved")
    }

    board.print_board();

}

