#![doc = include_str!("../readme.md")]

// TODO: 
// 1. model metadata as syn-style items that can be parsed from either a string or token stream for file/string/proc-macro usage.
// 2. provide regex-fixup for rustfmt for output to file for syntax generation from metadata.

mod file;
pub use file::*;

mod item;
pub use item::*;

mod item_struct;
pub use item_struct::*;
