use std::io::{stdout, Write};

use crossterm::{cursor::{Hide, MoveTo, Show}, event::{ KeyCode, KeyEvent}, execute, style::SetAttribute, terminal::{self, disable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};

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



    let mut stdout = stdout();


    // board.print_board();
    // board.solve(&mut stdout);

    terminal::enable_raw_mode().expect("msg");

    execute!(
        stdout, 
        EnterAlternateScreen, 
        Hide,
        SetAttribute(crossterm::style::Attribute::Bold),

    ).expect("msg");
    cli_tools::draw_board::draw_board(&board, &mut stdout);
    cli_tools::draw_info::draw_info(&mut stdout);

    let (mut current_row, mut current_col) = (1, 1);
    let mut move_to = false; 

    loop {
        let key_press:KeyEvent;
        if !board.check_board() {
            cli_tools::incorrect_input::incorrect_input(&mut stdout, true);
        } else {
            cli_tools::incorrect_input::incorrect_input(&mut stdout, false);
        }
        if !board.solved {
            key_press = cli_tools::blinker::blinker(&mut stdout, board, current_row, current_col, move_to);
        } else {
            current_row = 1;
            current_col = 1;
            key_press = cli_tools::solved_screen::solved_screen(&mut stdout);
            if key_press.code == KeyCode::Char('r') {
                board = board::board::Board::make_board();
                stdout.execute(        Clear(ClearType::All)).unwrap();
                cli_tools::draw_board::draw_board(&board, &mut stdout);
                cli_tools::draw_info::draw_info(&mut stdout);
            }
        }
        move_to = true;
        

        if key_press.code == KeyCode::Esc {
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
        } else if key_press.code == KeyCode::Backspace {
            board.delete_set((current_row-1) as i8, (current_col-1) as i8);
            move_to = false;
        
        } else if key_press.code == KeyCode::Enter {
            board.solve(&mut stdout);
            move_to = false;
            

        } else {
            let pressed_key = key_press.code.as_char().expect("oof ts failed");

            if pressed_key.is_numeric() {
                
                let pressed_num = pressed_key as u32 - '0' as u32;
                if pressed_num == 0 {
                    board.delete_set((current_row-1) as i8, (current_col-1) as i8);

                } else {
                    board.set_input_tile((current_row-1) as i8, (current_col-1) as i8, pressed_num as i8);

                }
            }
            move_to = false;
        }



    }

    execute!(
        stdout,
        LeaveAlternateScreen,
        MoveTo(0,0),
        Show,
        Clear(ClearType::All),
    ).expect("msg");
    print!("\x1bc");
    stdout.flush().unwrap();

    terminal::disable_raw_mode().unwrap();
}

