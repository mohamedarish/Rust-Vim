#![warn(
    clippy::nursery,
    clippy::unwrap_used,
    clippy::pedantic,
    clippy::unwrap_or_else_default,
)]

pub mod editor;
pub mod terminal;
pub mod file;

use editor::Editor;

fn main() {
    Editor::default().run();
}
