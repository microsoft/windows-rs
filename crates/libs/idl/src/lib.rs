#![doc = include_str!("../readme.md")]

pub mod syntax;

pub fn parse(s: &str) -> nom::IResult<&str, syntax::File> {
    syntax::File::parse(s)
}
