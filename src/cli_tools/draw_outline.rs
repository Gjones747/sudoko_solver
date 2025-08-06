use std::io::{stdout, Cursor, Stdout, Write};

use crossterm::{cursor::{self, MoveTo}, execute, style::{Color, Print, ResetColor, SetForegroundColor}, terminal::{self, Clear, ClearType}, ExecutableCommand, QueueableCommand};

pub fn draw_outline(stdout: &mut Stdout) {

    stdout.execute(Clear(ClearType::All)).expect("failed to clear");
    
    let (cols, rows) = terminal::size().expect("terminal size failed");

    let center: [u16; 2] = [rows/2, cols/2];
    stdout.execute(MoveTo(0, 0)).expect("failed to move");

    for i in 1..cols {
        execute!(
            stdout,
            Print("─"),
            MoveTo(i, 0)
        ).expect("failed to print");
    }

    stdout.execute(MoveTo(0, rows-3)).expect("failed to move to move to row");
    for i in 1..cols {
        execute!(
            stdout,
            Print("─"),
            MoveTo(i, rows-3)
        ).expect("failed to print");
    }

    stdout.execute(MoveTo(0, 0)).expect("failed to move");
    for i in 1..rows-1 {
        execute!(
            stdout,
            Print("|"),
            MoveTo(0, i)
        ).expect("failed to draw column");
    }

    stdout.execute(MoveTo(cols-1, 0)).expect("failed to move");
    for i in 1..rows-1 {
        execute!(
            stdout,
            Print("|"),
            MoveTo(cols-1, i)
        ).expect("failed to draw column");
    }
    stdout.flush().expect("goon");

    stdout.execute(MoveTo(center[0], center[1])).expect("failed to move");



}