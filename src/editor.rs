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

            // if let error is keypress not found die
        }
    }

    fn move_cursor(&mut self, key: Key) {}
}
