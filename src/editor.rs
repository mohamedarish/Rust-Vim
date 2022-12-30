use termion::event::Key;

use crate::terminal::Terminal;

#[derive(Default)]
pub struct Editor {
    terminal: Terminal,
    exit_issued: bool,
}

impl Editor {
    pub fn run(&mut self) {
        Self::clear_screen();

        self.flush();

        loop {
            if self.exit_issued {
                Self::clear_screen();
                println!("GoodBye👋👋");
                break;
            }

            self.process_keypress();
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn process_keypress(&mut self) {
        let key_entered = Terminal::process_keypress();

        match key_entered {
            Key::Ctrl('g') => self.exit_issued = true,
            Key::Ctrl('n') => Self::clear_screen(),
            Key::Ctrl('m') => Self::clear_line(),
            Key::Ctrl('l') => Self::clear_after_cursor(),
            Key::Ctrl('k') => Self::clear_before_cursor(),
            Key::Char(c) => print!("{c}"),
            Key::Ctrl(c) => print!("^{c}"),
            Key::Alt(c) => print!("\\{c}"),
            Key::Backspace | Key::Delete => print!("{} ", termion::cursor::Left(1)),
            Key::Down => print!("{}", termion::cursor::Down(1)),
            Key::Up => print!("{}", termion::cursor::Up(1)),
            Key::Right => print!("{}", termion::cursor::Right(1)),
            Key::Left => print!("{}", termion::cursor::Left(1)),
            _ => print!(""),
        }

        self.flush();
    }

    fn flush(&mut self) {
        self.terminal.flush();
    }

    fn clear_screen() {
        Terminal::clear_screen();
    }

    fn clear_line() {
        Terminal::clear_line();
    }

    fn clear_after_cursor() {
        Terminal::clear_after_cursor();
    }

    fn clear_before_cursor() {
        Terminal::clear_before_cursor();
    }
}
