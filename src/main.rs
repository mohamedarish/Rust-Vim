#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

mod editor;
use std::io::Write;

use termion::{event::Key, input::TermRead};
// use editor::Editor;

fn main() {
    // Editor::default().run();
    // Make it clear screen after every keypress. 🤯🤯🤯🤯🤯🤯🤯🤯

    loop {
        let pressed_key = read_key();

        match pressed_key {
            Ok(key) => match key {
                Key::Ctrl('a') => break,
                // Key::Char(c) => println!("{c}"),
                Key::Char('\n') => println!("{}", termion::clear::All),
                _ => println!("{key:?} {}", termion::clear::CurrentLine),
            },
            Err(error) => panic!("{}", error),
        }

        let n = flush();

        match n {
            Ok(n) => n,
            Err(error) => panic!("{error}"),
        }
    }
}

fn flush() -> Result<(), std::io::Error> {
    std::io::stdout().flush()
}
fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = std::io::stdin().lock().keys().next() {
            return key;
        }
    }
}
