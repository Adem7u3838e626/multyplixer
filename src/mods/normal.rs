use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{enable_raw_mode},
};
use std::time::Duration;

pub fn Nmode() {
    enable_raw_mode().unwrap();
    print!("you are now in normal mode");
    loop {
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {

                match (key.code, key.modifiers) {

                    (KeyCode::Char('i'), KeyModifiers::NONE) => {
                        println!("insert mode");
                        return;
                    }

                    // they do nothing for now 
                    (KeyCode::Char(c), KeyModifiers::CONTROL) => {
                    }
                    (KeyCode::Char(c), KeyModifiers::NONE) => {
                    }
                    _ => {}
                }

            }
        }
    }
}
