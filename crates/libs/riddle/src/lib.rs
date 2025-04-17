#![doc = include_str!("../readme.md")]

// TODO:
// 1. model metadata as syn-style items that can be parsed from either a string or token stream for file/string/proc-macro usage.
// 2. provide regex-fixup for rustfmt for output to file for syntax generation from metadata.

pub mod syntax;

pub fn parse(s: &str) -> syn::Result<syntax::File> {
    syn::parse_str(s)
}
