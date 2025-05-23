#![doc = include_str!("../readme.md")]

pub mod format;
pub mod syntax;

pub fn parse(s: &str) -> syn::Result<syntax::File> {
    syn::parse_str(s)
}
