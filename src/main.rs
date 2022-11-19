use rust_vim::{die, to_ctrl_byte};
use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let mut command = false;
    let mut insert = false;

    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;

                if insert {
                    if !c.is_control() {
                        println!("{}\r", c);
                    } else {
                        if b == 27 {
                            insert = false;
                        } else {
                            if b == to_ctrl_byte('c') {
                                insert = false;
                            } else {
                                println!("{:?}\r", b);
                            }
                        }
                    }
                } else if command {
                    if c == 'q' {
                        println!("q");
                        break;
                    } else {
                        command = false;
                    }
                }

                if insert == false && command == false && c == 'i' {
                    insert = true;
                } else if insert == false && command == false && c == ':' {
                    command = true;
                    print!(":");
                }
            }
            Err(err) => die(err),
        }
    }
}
