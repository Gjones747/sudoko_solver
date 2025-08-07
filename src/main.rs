use std::{ io::stdout, thread::sleep, time::Duration};

use crossterm::{cursor::Hide, event::{read, Event, KeyCode, KeyEvent}, execute, style::SetAttribute, terminal::{self, Clear, ClearType, EnterAlternateScreen}};

use crate::cli_tools::blinker;

mod board;
mod cli_tools;

fn main() {
    let mut board: board::board::Board = board::board::Board::make_board();

    //this is a board setup written by chatgpt cuz the input is tedious right now
    // board.set_input_tile(0, 0, 5);
    // board.set_input_tile(0, 1, 3);
    // board.set_input_tile(0, 4, 7);

    // board.set_input_tile(1, 0, 6);
    // board.set_input_tile(1, 3, 1);
    // board.set_input_tile(1, 4, 9);
    // board.set_input_tile(1, 5, 5);

    // board.set_input_tile(2, 1, 9);
    // board.set_input_tile(2, 2, 8);
    // board.set_input_tile(2, 7, 6);

    // board.set_input_tile(3, 0, 8);
    // board.set_input_tile(3, 4, 6);
    // board.set_input_tile(3, 8, 3);

    // board.set_input_tile(4, 0, 4);
    // board.set_input_tile(4, 3, 8);
    // board.set_input_tile(4, 5, 3);
    // board.set_input_tile(4, 8, 1);

    // board.set_input_tile(5, 0, 7);
    // board.set_input_tile(5, 4, 2);
    // board.set_input_tile(5, 8, 6);

    // board.set_input_tile(6, 1, 6);
    // board.set_input_tile(6, 6, 2);
    // board.set_input_tile(6, 7, 8);

    // board.set_input_tile(7, 3, 4);
    // board.set_input_tile(7, 4, 1);
    // board.set_input_tile(7, 5, 9);
    // board.set_input_tile(7, 8, 5);

    // board.set_input_tile(8, 4, 8);
    // board.set_input_tile(8, 7, 7);
    // board.set_input_tile(8, 8, 9);



    // board.print_board();


    // board.solve();



    // board.print_board();
    let mut stdout = stdout();
    terminal::enable_raw_mode().expect("msg");

    execute!(
        stdout, 
        EnterAlternateScreen, 
        Hide,
        SetAttribute(crossterm::style::Attribute::Bold),

    ).expect("msg");
    cli_tools::draw_board::draw_board(&board, &mut stdout);
    let mut running = true;

    let (mut current_row, mut current_col) = (1, 1);

    while running {

        let key_press:KeyEvent = cli_tools::blinker::blinker(&mut stdout, board, current_row, current_col);


        if key_press.code == KeyCode::Char('q') {
            running = false;
            break;
        } else if key_press.code == KeyCode::Right {
            if current_col == 9 {
                current_col = 0;
                if current_row == 9 {
                    current_row = 0;
                }
                current_row += 1;
            }
            current_col += 1;
        } else if key_press.code == KeyCode::Left {
            if current_col == 1 {
                if current_row == 1 {
                    current_row = 10;
                }
                current_col = 10;
                current_row -= 1;
            }
            current_col -= 1;
        } else if key_press.code == KeyCode::Down {
            if current_row == 9 {
                current_row = 0;
            }
            current_row += 1;
        } else if key_press.code == KeyCode::Up {
            if current_row == 1 {
                current_row = 10
            }
            current_row -= 1;
        }



    }
    execute!(
        stdout,
        Clear(ClearType::All)
    ).expect("msg");

    terminal::disable_raw_mode().err();
}

