use std::{
    io::Error,
    io::{stdin, Write},
};
use termion::{
    self,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    pub terminal_size: Size,
    pub output_view: RawTerminal<std::io::Stdout>,
}

impl Default for Terminal {
    fn default() -> Self {
        let current_size = termion::terminal_size().unwrap();

        Self {
            terminal_size: Size {
                height: current_size.1,
                width: current_size.0,
            },
            output_view: std::io::stdout().into_raw_mode().unwrap(),
        }
    }
}

impl Terminal {
    pub fn process_keypress() -> Result<Key, Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn flush(&mut self) {
        self.output_view.flush().unwrap();
    }
}
