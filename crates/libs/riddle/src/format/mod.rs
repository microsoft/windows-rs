use super::*;
mod formatter;
pub use formatter::*;

mod ecma335;

// TODO: or instead of formatting ecma335 we convert to riddle and then

pub trait Format {
    fn format(&self, f: &mut Formatter);
}

pub fn format<F: format::Format>(f: &F) -> String {
    let mut formatter = format::Formatter::new();

    f.format(&mut formatter);

    formatter.into_string()
}
