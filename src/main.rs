#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

mod editor;
pub mod terminal;

use editor::Editor;

fn main() {
    Editor::default().run();
}
