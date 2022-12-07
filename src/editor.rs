use termion::event::Key;

use crate::terminal::Terminal;
use std::io::{self, Read, Write};

#[derive(Default)]
struct Position {
    x_pos: u16,
    y_pos: u16,
}

#[derive(Default)]
pub struct Editor {
    file_name: String,
    terminal: Terminal,
    insert_mode: bool,
    comand_mode: bool,
    exit_issued: bool,
    cursor_position: Position,
}

impl Editor {
    pub fn run(&mut self) {
        self.clear_screen();

        self.cursor_position.x_pos = 1;
        self.cursor_position.y_pos = 1;

        self.change_cursor_position();

        self.terminal.flush();

        loop {
            if self.exit_issued {
                self.cursor_position.x_pos = 1;
                self.cursor_position.y_pos = 1;
                self.change_cursor_position();
                self.clear_screen();
                println!("GoodBye👋👋");
                break;
            }

            self.process_keypress();
        }
    }

    fn process_keypress(&mut self) {
        let key_entered = Terminal::process_keypress().unwrap();

        match key_entered {
            Key::Char('\n') => println!("\r"),
            Key::Ctrl('c') => self.exit_issued = true,
            Key::Char(c) => print!("{c}"),
            Key::Ctrl(c) => print!("^{c}"),
            Key::Alt(c) => print!("#{c}"),
            Key::Left => print!("{}", termion::cursor::Left(1)),
            Key::Right => print!("{}", termion::cursor::Right(1)),
            Key::Up => print!("{}", termion::cursor::Up(1)),
            Key::Down => print!("{}", termion::cursor::Down(1)),
            _ => panic!("Error happened"),
        }

        self.terminal.flush();
    }

    fn change_cursor_position(&self) {
        let current_position = &self.cursor_position;

        println!(
            "{}",
            termion::cursor::Goto(current_position.x_pos, current_position.y_pos),
        );
    }

    fn clear_screen(&mut self) {
        print!("{}", termion::clear::All);
    }
}

fn die(e: &io::Error) {
    panic!("{e}");
}
