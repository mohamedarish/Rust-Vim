#![warn(clippy::pedantic, clippy::nursery, clippy::expect_used)]

use std::io::{self, Read, Write};
use termion::input::TermRead;

use termion::event::Key;
use termion::raw::IntoRawMode;

fn main() {
    println!("move up\nmove right\nmove left\nmove down");

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{} {} Press :q to exit {}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
    )
    .unwrap();

    stdout.flush().unwrap();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Ctrl('a') => break,
            Key::Char(c) => println!("{c}\r"),
            _ => panic!("Error happened"),
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{} {}", termion::clear::All, termion::cursor::Show).unwrap();
}
