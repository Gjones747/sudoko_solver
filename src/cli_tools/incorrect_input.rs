use std::{io::Stdout, time::{Duration, Instant}};

use crossterm::{cursor::MoveTo, event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind}, execute, style::{Print, SetAttribute}, terminal};




pub fn incorrect_input(stdout: &mut Stdout, is_incorrect:bool) {

    let (term_width, term_height) = terminal::size().unwrap();

    let (start_x, start_y) = ((term_width/2)-10, (term_height/2)-9);
    let mut toggle = Instant::now();
    let mut show: bool = false;
    

    if is_incorrect {
        execute!(
            stdout, 
            SetAttribute(crossterm::style::Attribute::Reset),
            MoveTo(start_x, start_y),
            Print("Impossible Board Input")
        ).unwrap();
        execute!(
            stdout, 
            SetAttribute(crossterm::style::Attribute::Bold)
        ).unwrap();
    } else {
        execute!(
            stdout, 
            SetAttribute(crossterm::style::Attribute::Reset),
            MoveTo(start_x, start_y),
            Print("                           ")
        ).unwrap();
        execute!(
            stdout, 
            SetAttribute(crossterm::style::Attribute::Bold)
        ).unwrap();
    }
    

        

        
    

    

}