use std::io::Stdout;

use crossterm::{cursor::MoveTo, execute, style::{Print, SetAttribute}, terminal};



pub fn draw_info(stdout: &mut Stdout) {

    let (term_width, term_height) = terminal::size().unwrap();

    let (start_x, start_y) = ((term_width/2)-14, (term_height/2)+7);

    execute!(
        stdout, 
        SetAttribute(crossterm::style::Attribute::Reset),
        MoveTo(start_x, start_y),
        Print("Quit (Esc)")

    ).unwrap();


    execute!(
        stdout, 
        SetAttribute(crossterm::style::Attribute::Reset),
        MoveTo(start_x+17, start_y),
        Print("Solve (Enter)")

    ).unwrap();

    execute!(
        stdout, 
        SetAttribute(crossterm::style::Attribute::Bold),
    ).unwrap()


}