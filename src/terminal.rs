use std::io::{stdin, Write};
use termion::{
    self,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Position {
    x_pos: u16,
    y_pos: u16,
}

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    // terminal_size: Size,
    output_view: RawTerminal<std::io::Stdout>,
    position: Position,
}

impl Default for Terminal {
    fn default() -> Self {
        // let current_size = Self::size();

        Self {
            // terminal_size: Size {
            //     height: current_size.1,
            //     width: current_size.0,
            // },
            output_view: std::io::stdout().into_raw_mode().expect("Cannot parse into raw mode"),
            position: Position { x_pos: 1, y_pos: 2 },
        }
    }
}

impl Terminal {
    #[must_use]
    pub fn process_keypress() -> Key {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key.expect("The key cannot be parsed into a Key");
            }
        }
    }


    #[must_use] pub fn size() -> (u16, u16) {
        termion::terminal_size().expect("Cannot read the terminal's size")
    }

    pub fn move_cursor(self) {
        print!(
            "{}",
            termion::cursor::Goto(self.position.x_pos, self.position.y_pos)
        );
    }

    /// # Flushes the stack storage
    ///
    /// This function just fluished the terminal
    ///
    /// # Usage
    /// ```
    ///     Terminal::default().flush()
    /// ```
    ///
    /// # Panics
    /// This function panics if there is some sort of exception blocking the program from flushing the stack
    ///
    pub fn flush(&mut self) {
        self.output_view.flush().expect("Cannot flush the current input to os");
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn clear_after_cursor() {
        print!("{}", termion::clear::AfterCursor);
    }

    pub fn clear_before_cursor() {
        print!("{}", termion::clear::BeforeCursor);
    }
}
