#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

mod editor;
use termion::{event::Key, input::TermRead};

// use editor::Editor;

fn main() {
    // Editor::default().run();

    loop {
        if let Some(key) = std::io::stdin().lock().keys().next() {
            match key {
                Ok(k) => match k {
                    Key::Ctrl('a') => {
                        break;
                    }
                    Key::Char('\n') => {
                        println!("{}", termion::clear::All);
                    }
                    _ => println!("{k:?}\r"),
                },
                Err(error) => panic!("{}", error),
            }
        }
    }
}
