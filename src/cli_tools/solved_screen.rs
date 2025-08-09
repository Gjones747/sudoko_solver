use std::{io::Stdout, time::Duration};

use crossterm::{cursor::MoveTo, event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind}, execute, style::{Print, SetAttribute}, terminal};




pub fn solved_screen(stdout: &mut Stdout) -> KeyEvent {

    let (term_width, term_height) = terminal::size().unwrap();

    let (start_x, start_y) = ((term_width/2)-14, (term_height/2)-9);

    loop {
        execute!(
            stdout, 
            SetAttribute(crossterm::style::Attribute::Reset),
            MoveTo(start_x, start_y),
            Print("Board Solved ('r' to refresh)")

        ).unwrap();

        if poll(Duration::from_millis(10)).expect("msg") {
                if let Event::Key(key_event) = read().expect("msg") {
                    if key_event.kind == KeyEventKind::Press {
                        execute!(
                            stdout, 
                            SetAttribute(crossterm::style::Attribute::Bold),

                        ).unwrap();
                        
                        if key_event.code == KeyCode::Esc || key_event.code == KeyCode::Char('r'){
                            return key_event
                        } 
                    }
                    

                }
            }
    }

    

}