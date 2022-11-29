use std::io;
use termion::event::Key;

use crate::terminal::Terminal;

#[derive(Default)]
pub struct Editor {
    terminal: Terminal,
    cursor_position: Position,
    should_quit: bool,
    insert: bool,
    command: bool,
}

#[derive(Default)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            // if let error is clear screen then die
            if let Err(error) = self.refresh_screen() {
                die(error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        // This returns either () or Error
        let key_pressed = Terminal::read_key()?;
        // read key being pressed

        if !self.insert {
            if self.command {
                match key_pressed {
                    Key::Char('q') => self.should_quit = true,
                    _ => (),
                }
            } else {
                match key_pressed {
                    Key::Char('i') => self.insert = true,
                    Key::Char(':') => self.command = true,
                    _ => (),
                }
            }
        } else {
            match key_pressed {
                Key::Ctrl('c') => self.insert = false,
                Key::Esc => self.insert = false,
                Key::Char(c) => todo!(),
                _ => (),
            }
        }
        // Match key to the special functions and match Key::char(c) enter c at the current position of the cursor and match _ to ()
        Ok(())
        // if Ok return ()
    }

    fn move_cursor(&mut self, _key: Key) {
        // position is cursor position

        // set size to terminal.size
        // set height to terminal height
        // set width to terminal width

        // match key to all movements like up, down, l/r, home, end, pgup, pgdn

        // set self.position to the new position
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        // Returns () or Error
        // hide cursor
        Terminal::hide_cursor();

        // set position to position default

        if self.should_quit {
            Terminal::clear_screen();
            println!("Exiting...");
            // close editor and save if required
        }

        Ok(())

        // fill text again and then set cursor position back to previous position
        // flush screen
    }
}

fn die(e: io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
