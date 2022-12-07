use std::io::{stdin, Write};
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
    /// # Reads the key being pressed
    ///
    /// This function reads the key entered and returns it
    ///
    /// # Usage
    /// ```
    ///     let key = Terminal::Default().process_keypress();
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if for some reason it can't match the input to some key in the termion Package
    ///
    #[must_use]
    pub fn process_keypress() -> Key {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key.unwrap();
            }
        }
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
        self.output_view.flush().unwrap();
    }
}
