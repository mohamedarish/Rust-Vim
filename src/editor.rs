use rust_vim::die;
use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {}

impl Editor {
    // pub fn default() -> Self {
    //     Self {}
    // }
    pub fn run() {
        let mut command = false;
        let mut insert = false;

        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if insert {
                            println!("{c}\r");
                        } else if command {
                            if c == 'q' {
                                println!("q");
                                break;
                            }
                            println!("\r");
                            command = false;
                        } else if !command && !insert {
                            if c == ':' {
                                print!(":");
                                command = true;
                            } else if c == 'i' {
                                insert = true;
                            }
                        }
                    }
                    Key::Ctrl('c') | Key::Esc => {
                        if insert {
                            insert = false;
                        }
                    }
                    _ => println!("{key:?}\r"),
                },
                Err(err) => die(&err),
            }
        }
    }
}
