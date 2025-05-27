#![doc = include_str!("../readme.md")]

mod syntax;

pub use syntax::*;

pub fn parse(s: &str) -> nom::IResult<&str, File> {
    File::parse(s)
}
