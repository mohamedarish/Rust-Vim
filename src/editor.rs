use crate::terminal::Terminal;

#[derive(Default)]
struct Position {
    x_pos: u16,
    y_pos: u16,
}

#[derive(Default)]
pub struct Editor {
    file_name: String,
    terminal: Terminal,
    insert_mode: bool,
    comand_mode: bool,
    exit_issued: bool,
    cursor_position: Position,
}
