use rust_vim;

#[test]
fn check_file_readability() {
    let content_to_check = rust_vim::open_file(String::from(".gitignore"));

    let actual_file_content = "/target\n\nCargo.lock";

    assert_eq!(content_to_check, actual_file_content);
}
