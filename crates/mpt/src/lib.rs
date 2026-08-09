//! Multi-part-text (mpt) container format — RFC-0001.

mod ast;
mod command_tree;
mod edit;
mod error;
mod format;
mod parse;
mod serialize;

pub use ast::{Document, HeaderBlock, Part};
pub use command_tree::render as render_command_tree;
pub use edit::{add_part, find_part, merge, remove_part, InsertPosition};
pub use error::{EditError, ParseError, Result, SerializeError};
pub use format::Format;
pub use parse::parse;
pub use serialize::to_string;
