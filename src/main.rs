#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod row;
mod terminal;

use editor::Editor;

pub use row::Row;
pub use editor::Position;
pub use document::Document;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
