#![doc = include_str!("../readme.md")]

mod syntax;
pub use syntax::*;

pub fn parse(input: &str) -> FileResult<File> {
    File::parse(input.into())
}
