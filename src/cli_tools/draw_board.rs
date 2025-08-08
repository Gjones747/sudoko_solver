use std::io::Stdout;

use crossterm::{cursor::MoveTo, execute, style::{Color, Print, SetAttribute, SetForegroundColor}, terminal, ExecutableCommand};

use crate::board;



pub fn draw_board(board: &board::board::Board, stdout: &mut Stdout) {

    let (center_x, center_y) = terminal::size().expect("failed to get size");

    // sudoku print out is exactly 31 characters wide
    // should be 13 characters in height

    let (start_x, start_y) = ((center_x /2) - 15, (center_y /2) -7);

    fn print_row(row: [board::board::Tile; 9], stdout: &mut Stdout) {
        print!("│");
        for i in 0..9 {
            if i == 3 || i == 6 {
                print!("│")
            }
            if row[i].val == 0 {
                print!(" - ")
            } else {
                if !row[i].locked {
                    execute!(
                        stdout, 
                        SetAttribute(crossterm::style::Attribute::Reset),
                        SetAttribute(crossterm::style::Attribute::Dim)
                    ).expect("msg");
                }
                print!(" {0} ", row[i].val);

                execute!(
                    stdout, 
                    SetAttribute(crossterm::style::Attribute::Reset),

                    SetAttribute(crossterm::style::Attribute::Bold),
                    SetForegroundColor(Color::Reset)
                ).expect("msg");
            }
        }
        print!("│")
    }

    execute!(
        stdout, 
        MoveTo(start_x, start_y),
        Print("┌─────────────────────────────┐"),
    ).expect("");

    let mut current_y = start_y ;

    for i in 0..9 {
        current_y+=1;
        if i == 3 || i == 6 {
                stdout.execute(MoveTo(start_x, current_y as u16)).expect("failed");

                println!("│─────────────────────────────│");
                current_y += 1;
        }
        stdout.execute(MoveTo(start_x, current_y as u16)).expect("failed");

        print_row(board.board_array[i],stdout);
    }
    stdout.execute(MoveTo(start_x, current_y+1 as u16)).expect("failed");
    print_row(board.board_array[8],stdout);
    stdout.execute(MoveTo(start_x, current_y+1 as u16)).expect("failed");
    println!("└─────────────────────────────┘");



}