use rust_vim::{self, Arguments};
use std::process;

#[test]
fn check_file_readability() {
    let file = Arguments::new(&vec![String::from("main_file"), String::from(".gitignore")])
        .unwrap_or_else(|err| {
            eprintln!(
                "There was a problem while parsing the provided arguments: \n {}",
                err
            );
            process::exit(1)
        });

    let content = file.content;

    let actual_file_content = "/target\n\nCargo.lock";

    assert_eq!(content, actual_file_content);
}
