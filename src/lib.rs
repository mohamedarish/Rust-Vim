use std::fs;

pub struct Arguments {
    pub file_name: String,
    pub content: String,
}

pub fn open_file(file_path: String) -> String {
    fs::read_to_string(file_path).expect("Permission denied")
}

impl Arguments {
    pub fn new(args: &Vec<String>) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let file_path = &args[1];

        let s = Arguments {
            file_name: file_path.to_string(),
            content: open_file(file_path.to_string()),
        };

        Ok(s)
    }

    pub fn print_window(self) {}
}

pub fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

pub fn die(e: std::io::Error) {
    panic!("{}", e);
}
