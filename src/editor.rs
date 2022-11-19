use rust_vim::die;
use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {}

impl Editor {
    // pub fn default() -> Self {
    //     Self {}
    // }

    pub fn run() {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = Self::process_keypress() {
                die(&error);
            }
        }
    }

    fn process_keypress() -> Result<(), io::Error> {
        let pressed_key = read_key()?;
        if let Key::Ctrl('q') = pressed_key {
            panic!("Program end")
        }
        Ok(())
    }
}

fn read_key() -> Result<Key, io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}
