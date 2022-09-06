mod bstr;
mod hstring;
mod literals;
mod pcstr;
mod pcwstr;
mod pstr;
mod pwstr;

pub use bstr::*;
pub use hstring::*;
#[doc(hidden)]
pub use literals::*;
pub use pcstr::*;
pub use pcwstr::*;
pub use pstr::*;
pub use pwstr::*;

use super::*;

extern "C" {
    #[doc(hidden)]
    pub fn strlen(s: PCSTR) -> usize;
    #[doc(hidden)]
    pub fn wcslen(s: PCWSTR) -> usize;
}

/// An internal helper for decoding an iterator of chars and displaying them
#[doc(hidden)]
pub struct Decode<F>(pub F);

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

/// Mirror of `std::char::decode_utf16` for utf-8.
fn decode_utf8<'a>(mut buffer: &'a [u8]) -> impl Iterator<Item = core::result::Result<char, std::str::Utf8Error>> + 'a {
    let mut current = "".chars();
    let mut previous_error = None;
    std::iter::from_fn(move || {
        loop {
            match (current.next(), previous_error) {
                (Some(c), _) => return Some(Ok(c)),
                // Return the previous error
                (None, Some(e)) => {
                    previous_error = None;
                    return Some(Err(e));
                }
                // We're completely done
                (None, None) if buffer.is_empty() => return None,
                (None, None) => {
                    match std::str::from_utf8(buffer) {
                        Ok(s) => {
                            current = s.chars();
                            buffer = &[];
                        }
                        Err(e) => {
                            let (valid, rest) = buffer.split_at(e.valid_up_to());
                            // Skip the invalid sequence and stop completely if we ended early
                            let invalid_sequence_length = e.error_len()?;
                            buffer = &rest[invalid_sequence_length..];

                            // Set the current iterator to the valid section and indicate previous error
                            // SAFETY: `valid` is known to be valid utf-8 from error
                            current = unsafe { std::str::from_utf8_unchecked(valid) }.chars();
                            previous_error = Some(e);
                        }
                    }
                }
            }
        }
    })
}
