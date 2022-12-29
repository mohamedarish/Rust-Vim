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
            Key::Char(c) => print!("{c}"),
            Key::Ctrl(c) => print!("^{c}"),
            Key::Alt(c) => print!("\\{c}"),
            Key::Backspace | Key::Delete => print!("{} ", termion::cursor::Left(1)),
            _ => print!(""),
        }

        self.flush();
    }

    fn flush(&mut self) {
        self.terminal.flush();
    }

    fn clear_screen() {
        print!("{}", termion::clear::All);
    }
}
