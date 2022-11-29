use std::default;

enum Modes {
    default,
    insert,
    command,
}

pub struct Editor {
    content: String, // This is to be set to the file content struct when the struct has been completed
    cursor_position: (u16, u16), // This should also be changed when thhe struct is implemented
    mode: Modes,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            content: String::from(""),
            cursor_position: (0, 0),
            mode: Modes::default,
        }
    }
}

impl Editor {
    pub fn run(self) {
        // clear screen and display content
    }

    pub fn load_content() {
        // first line shall be reserved for filename if exists, otherwise, it will be blank

        // display file content in the rest of the lines except last 2 lines

        // display command entering area in second last line and also the cursor position

        // show the mode and current working file's directory
    }
}
