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
mod terminal;
mod document;
mod filetype;
mod highlighting;

pub use row::Row;
use editor::Editor;
pub use editor::Position;
pub use document::Document;
pub use filetype::FileType;
pub use terminal::Terminal;
pub use editor::SearchDirection;
pub use filetype::HighlightingOptions;

fn main() {
	Editor::default().run();
}
