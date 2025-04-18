#![doc = include_str!("../readme.md")]

// TODO:
// 1. model metadata as syn-style items that can be parsed from either a string or token stream for file/string/proc-macro usage.
// 2. provide regex-fixup for rustfmt for output to file for syntax generation from metadata.

pub mod syntax;

mod format;
pub use format::*;

pub fn parse(s: &str) -> syn::Result<syntax::File> {
    syn::parse_str(s)
}

// TODO: add fmt module that includes a Format trait that all types implement
// then the fmt function here can take any F: Format and format it using an indent-aware
// printer...
// This avoids the windows-ecma335 crate from having to have any knowledge of riddle.

// TODO: can this format both ecma335 and syntax items?!

// TODO: maybe have the rust-fmt approach as an alternative?

// TODO: way to convert ecma335::reader into a syntax types
// - one to one
// - avoid auto-converting to modules as that implies filtering best left to the direct tool/task
