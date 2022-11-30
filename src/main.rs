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
        let pressed_key = read_key();

        match pressed_key {
            Ok(key) => match key {
                Key::Ctrl('a') => break,
                Key::Char('\n') => println!("{}", termion::clear::All),
                Key::Char(c) => print!("{c}"),
                _ => panic!("Error"),
            },
            Err(error) => panic!("{}", error),
        }
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = std::io::stdin().lock().keys().next() {
            return key;
        }
    }
}
