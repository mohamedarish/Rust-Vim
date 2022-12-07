use termion::event::Key;

use crate::terminal::Terminal;

#[derive(Default)]
struct Position {
    x_pos: u16,
    y_pos: u16,
}

#[derive(Default)]
pub struct Editor {
    // file_name: String, // This is to store the filename of the file being displayed right now
    terminal: Terminal,
    insert_mode: bool,
    comand_mode: bool,
    exit_issued: bool,
    cursor_position: Position,
}

impl Editor {
    pub fn run(&mut self) {
        Self::clear_screen();

        self.cursor_position.x_pos = 1;
        self.cursor_position.y_pos = 1;

        self.change_cursor_position();

        self.flush();

        loop {
            if self.exit_issued {
                self.cursor_position.x_pos = 1;
                self.cursor_position.y_pos = 1;
                self.change_cursor_position();
                Self::clear_screen();
                println!("GoodBye👋👋");
                break;
            }

            self.process_keypress();
        }
    }

    fn process_keypress(&mut self) {
        let key_entered = Terminal::process_keypress();
        if !self.insert_mode && !self.comand_mode {
            match key_entered {
                Key::Char('i') => self.insert_mode = true,
                Key::Char(':') => {
                    self.comand_mode = true;
                    print!(
                        "{}:",
                        termion::cursor::Goto(1, self.terminal.terminal_size.height)
                    );
                }
                Key::Char('y') | Key::Up => print!("{}", termion::cursor::Up(1)),
                Key::Char('g') | Key::Left => print!("{}", termion::cursor::Left(1)),
                Key::Char('h') | Key::Down => print!("{}", termion::cursor::Down(1)),
                Key::Char('j') | Key::Right => print!("{}", termion::cursor::Right(1)),
                Key::Delete | Key::Char('x') => print!("{} ", termion::cursor::Left(1)),
                _ => print!(""),
            }
        } else if self.insert_mode {
            match key_entered {
                Key::Char('\n') => println!("\r"),
                Key::Ctrl('c') | Key::Esc => self.insert_mode = false,
                Key::Char(c) => print!("{c}"),
                Key::Ctrl(c) => print!("^{c}"),
                Key::Alt(c) => print!("#{c}"),
                Key::Left => print!("{}", termion::cursor::Left(1)),
                Key::Right => print!("{}", termion::cursor::Right(1)),
                Key::Up => print!("{}", termion::cursor::Up(1)),
                Key::Down => print!("{}", termion::cursor::Down(1)),
                _ => print!(""),
            }
        } else if self.comand_mode {
            if key_entered == Key::Char('q') {
                self.exit_issued = true;
            } else {
                print!(
                    "{} ",
                    termion::cursor::Goto(1, self.terminal.terminal_size.height)
                );
                self.comand_mode = false;
                print!("{}", termion::cursor::Restore);
            }
        }

        self.flush();
    }

    fn change_cursor_position(&self) {
        let current_position = &self.cursor_position;

        println!(
            "{}",
            termion::cursor::Goto(current_position.x_pos, current_position.y_pos),
        );
    }

    fn flush(&mut self) {
        self.terminal.flush();
    }

    fn clear_screen() {
        print!("{}", termion::clear::All);
    }
}
