use core::time;
use std::{io::Stdout, thread, time::Duration};

use crossterm::{cursor::MoveTo, event::{poll, read, Event, KeyCode, KeyEvent}, execute, style::Print, terminal};

use crate::board;


pub fn blinker(stdout: &mut Stdout, board: board::board::Board, row: u16, col: u16) -> KeyEvent {
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
        thread::scope(|s| {
            s.spawn(|| {
                execute!(
                    stdout,
                    MoveTo(real_x, real_y),
                    Print("   ")
                ).expect("fail");

                thread::sleep(time::Duration::from_millis(500));

                execute!(
                    stdout,
                    MoveTo(real_x, real_y),
                    Print(" - ")
                ).expect("fail");
            });

        });
        

        while poll(Duration::from_millis(500)).expect("msg") {
            if let Event::Key(key_event) = read().expect("msg") {
                return key_event
            }
        }

    }
    
}