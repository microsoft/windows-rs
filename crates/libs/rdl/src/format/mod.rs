mod metadata;

mod formatter;
pub use formatter::*;

pub trait Format {
    fn format(&self, f: &mut Formatter);
}

pub fn format<F: Format>(f: &F) -> String {
    let mut formatter = Formatter::new();

    f.format(&mut formatter);

    formatter.into_string()
}
