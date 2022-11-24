use std::io::{self, Write};

pub struct Terminal {
    pub size: Size,
}

pub struct Size {
    pub height: u16,
    pub width: u16,
}

impl Default for Terminal {
    fn default() -> Self {
        let terminal_size = termion::terminal_size();

        match terminal_size {
            Ok(size) => Self {
                size: Size {
                    height: size.0,
                    width: size.1,
                },
            },
            Err(error) => panic!("Could not resolve terminal size\n{}", error),
        }
    }
}

impl Terminal {
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn flush() -> Result<(), io::Error> {
        io::stdout().flush()
    }

    pub fn show_cursor() {
        println!("{}", termion::cursor::Show);
    }

    pub fn hide_cursor() {
        println!("{}", termion::cursor::Hide);
    }
}
