#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

mod row;
mod editor;
mod filetype;
mod terminal;
mod document;
mod highlighting;

pub use row::Row;
pub use editor::Position;
pub use filetype::FileType;
pub use document::Document;
pub use terminal::Terminal;
pub use editor::SearchDirection;
pub use filetype::HighlightingOptions;

use editor::Editor;

fn main() {
    Editor::default().run();
}
