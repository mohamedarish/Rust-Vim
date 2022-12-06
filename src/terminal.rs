use termion;

struct Size {
    height: u16,
    width: u16,
}

pub struct Terminal {
    terminal_size: Size,
}

impl Default for Terminal {
    fn default() -> Self {
        let current_size = termion::terminal_size().unwrap();

        Self {
            terminal_size: Size {
                height: current_size.1,
                width: current_size.0,
            },
        }
    }
}
