#![warn(clippy::nursery, clippy::expect_used, clippy::pedantic)]

pub mod editor;
pub mod terminal;

use editor::Editor;

fn main() {
    Editor::default().run();
}
