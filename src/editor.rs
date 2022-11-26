use std::io;
use termion::event::Key;

use crate::terminal::Terminal;

#[derive(Default)]
pub struct Editor {
    terminal: Terminal,
    cursor_position: Position,
    should_quit: bool,
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

            if self.should_quit {
                break;
            }

            // if let error is keypress die
        }
    }

    fn process_keypress(&mut self) {
        // This returns either () or Error

        // read key being pressed

        // Match key to the special functions and match Key::char(c) enter c at the current position of the cursor and match _ to ()

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

    fn refresh_screen(&self) {
        // Returns () or Error
        // hide cursor

        // set position to position default

        if self.should_quit {
            // close editor and save if required
        }

        // fill text again and then set cursor position back to previous position

        // flush screen
    }
}

fn die(e: io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
