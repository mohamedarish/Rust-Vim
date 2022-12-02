#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

use std::io::{self, Read, Write};
use termion::input::TermRead;

use termion::event::Key;
use termion::raw::IntoRawMode;

fn main() {
    println!("move up\nmove right\nmove left\nmove down");

    let stdout = std::io::stdout().into_raw_mode().unwrap();

    loop {
        let pressed_key = read_key();

        match pressed_key {
            Ok(key) => match key {
                Key::Ctrl('a') => break,
                // Key::Char(c) => println!("{c}"),
                // Key::Char('\n') => println!("{}", termion::clear::All),
                Key::Ctrl('f') => println!("{}", termion::color::Fg(termion::color::Red)),
                Key::Ctrl('e') => println!("{}", termion::color::Bg(termion::color::White)),
                Key::Ctrl('p') => println!("{}", termion::color::Bg(termion::color::Reset)),
                Key::Ctrl('g') => println!("{}", termion::color::Fg(termion::color::Reset)),
                Key::Char('\n') => println!("New Line"),
                Key::Down => println!("{}", termion::cursor::Down(1)),
                Key::Left => println!("{}", termion::cursor::Left(1)),
                Key::Right => println!("{}", termion::cursor::Right(1)),
                Key::Up => println!("{}", termion::cursor::Up(1)),
                _ => println!("po"),
            },
            Err(error) => panic!("{error}"),
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
