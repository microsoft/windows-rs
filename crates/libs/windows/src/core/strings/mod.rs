mod hstring;
mod literals;
mod pcstr;
mod pcwstr;
mod pstr;
mod pwstr;

pub use hstring::*;
#[doc(hidden)]
pub use literals::*;
pub use pcstr::*;
pub use pcwstr::*;
pub use pstr::*;
pub use pwstr::*;

use super::*;

extern "C" {
    pub fn strlen(s: PCSTR) -> usize;
    pub fn wcslen(s: PCWSTR) -> usize;
}

/// An internal helper for decoding an iterator of chars and displaying them
struct Decode<F>(F);

impl<F, R, E> core::fmt::Display for Decode<F>
where
    F: Clone + FnOnce() -> R,
    R: IntoIterator<Item = core::result::Result<char, E>>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Write;
        let iter = self.0.clone();
        for c in iter().into_iter() {
            f.write_char(c.unwrap_or_else(|_| std::char::REPLACEMENT_CHARACTER))?
        }
        Ok(())
    }
}
