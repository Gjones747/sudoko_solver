use std::{io::Stdout, time::{Duration, Instant}};

use crossterm::{cursor::MoveTo, event::{poll, read, Event, KeyEvent, KeyEventKind}, execute, style::{Print, SetAttribute}, terminal};

use crate::board;


pub fn blinker(stdout: &mut Stdout, board: board::board::Board, row: u16, col: u16, move_to: bool) -> KeyEvent {
    let (center_x, center_y) = terminal::size().expect("failed to get size");

    // sudoku print out is exactly 31 characters wide
    // should be 13 characters in height

    let (start_x, start_y) = ((center_x /2) - 15, (center_y /2) -7);

    loop {

        let mut real_x = start_x+((col-1) *3)+1;
        let mut real_y = start_y+row;

        if col >= 4 {
            real_x += 1;
        }

        if col >= 7 {
            real_x += 1;
        }

        if row >= 4 {
            real_y += 1;
        }

        if row >= 7 {
            real_y += 1;
        }

        let mut toggle = Instant::now();
        let mut show_dash = false;

        if !board.board_array[(row-1) as usize][(col-1) as usize].locked &&  board.board_array[(row-1) as usize][(col-1) as usize].val != 0 {
            execute!(
                stdout, 
                SetAttribute(crossterm::style::Attribute::Reset),
                SetAttribute(crossterm::style::Attribute::Dim)
            ).expect("msg");
        }


        if board.board_array[(row-1) as usize][(col-1) as usize].val == 0 || move_to{
            execute!(
                stdout,
                MoveTo(real_x, real_y),
                Print("  ")
                ).expect("fail");
        } else {
            execute!(
                stdout,
                MoveTo(real_x, real_y),
                ).expect("fail");
            println!(" {0} ", board.board_array[(row-1) as usize][(col-1) as usize].val)
        }

        loop {
            if toggle.elapsed() >= Duration::from_millis(400) {
                show_dash = !show_dash;

                if board.board_array[(row-1) as usize][(col-1) as usize].val == 0 || !show_dash {
                    execute!(
                        stdout,
                        MoveTo(real_x, real_y),
                        Print(if show_dash {" - "} else {"  "})
                        ).expect("fail");
                } else {
                    execute!(
                        stdout,
                        MoveTo(real_x, real_y),
                        ).expect("fail");
                        println!(" {0} ", board.board_array[(row-1) as usize][(col-1) as usize].val)
                }
                toggle = Instant::now();

            }
            

            if poll(Duration::from_millis(10)).expect("msg") {
                if let Event::Key(key_event) = read().expect("msg") {
                    if key_event.kind == KeyEventKind::Press {
                            if board.board_array[(row-1) as usize][(col-1) as usize].val == 0 {
                            execute!(
                                stdout,
                                MoveTo(real_x, real_y),
                                Print(" - ")
                                ).expect("fail");
                        } else {
                            execute!(
                                stdout,
                                MoveTo(real_x, real_y),
                                ).expect("fail");
                                println!(" {0} ", board.board_array[(row-1) as usize][(col-1) as usize].val)
                        }

                        execute!(
                            stdout, 
                            SetAttribute(crossterm::style::Attribute::Reset),
                            SetAttribute(crossterm::style::Attribute::Bold)
                        ).expect("msg");
                        return key_event
                    }
                }
            }
        }

    }
    
}