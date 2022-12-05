#![warn(clippy::pedantic, clippy::nursery, clippy::expect_used)]

use std::io::{self, Write};
use termion::input::TermRead;

use termion::event::Key;
use termion::raw::IntoRawMode;

fn main() {
    println!("move up\nmove right\nmove left\nmove down");

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{} {} Press ^c to exit {}",
        termion::clear::All,
        termion::cursor::Show,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();

    stdout.flush().unwrap();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('\n') => println!("\r"),
            Key::Ctrl('c') => break,
            Key::Char(c) => print!("{c}"),
            Key::Ctrl(c) => print!("^{c}"),
            Key::Alt(c) => print!("#{c}"),
            Key::Left => print!("{}", termion::cursor::Left(1)),
            Key::Right => print!("{}", termion::cursor::Right(1)),
            Key::Up => print!("{}", termion::cursor::Up(1)),
            Key::Down => print!("{}", termion::cursor::Down(1)),
            _ => panic!("Error happened"),
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::clear::All).unwrap();
}
