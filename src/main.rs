use std::{env, process};

use rust_vim::{open_file, Arguments};

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:#?}", args);

    let file = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!(
            "There was a problem while parsing the provided arguments: \n {}",
            err
        );
        process::exit(1)
    });

    let content = open_file(file.file_name);
}
