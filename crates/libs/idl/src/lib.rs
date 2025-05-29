#![doc = include_str!("../readme.md")]

mod syntax;

pub use syntax::*;

pub fn parse(input: &str) -> nom::IResult<&str, File> {
    File::parse(input.into())
}
