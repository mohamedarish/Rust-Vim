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

    fn move_cursor(&mut self, key: Key) {
        // position is cursor position

        // set size to terminal.size
        // set height to terminal height
        // set width to terminal width

        // match key to all movements like up, down, l/r, home, end, pgup, pgdn

        // set self.position to the new position
    }
}

fn die(e: io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
