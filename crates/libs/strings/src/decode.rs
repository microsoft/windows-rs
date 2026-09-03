/// Displays decoded characters, replacing errors with the Unicode replacement character.
pub struct Decode<F>(pub F);

impl<F, R, E> core::fmt::Display for Decode<F>
where
    F: Clone + FnOnce() -> R,
    R: IntoIterator<Item = Result<char, E>>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use core::fmt::Write;
        let iter = self.0.clone();
        for c in iter().into_iter() {
            f.write_char(c.unwrap_or(core::char::REPLACEMENT_CHARACTER))?;
        }
        Ok(())
    }
}

/// Decodes UTF-8 incrementally and reports invalid sequences.
pub fn decode_utf8(
    mut buffer: &[u8],
) -> impl Iterator<Item = Result<char, core::str::Utf8Error>> + '_ {
    let mut current = "".chars();
    let mut previous_error = None;
    core::iter::from_fn(move || {
        loop {
            match (current.next(), previous_error) {
                (Some(c), _) => return Some(Ok(c)),
                (None, Some(e)) => {
                    previous_error = None;
                    return Some(Err(e));
                }
                (None, None) if buffer.is_empty() => return None,
                (None, None) => {
                    match core::str::from_utf8(buffer) {
                        Ok(s) => {
                            current = s.chars();
                            buffer = &[];
                        }
                        Err(e) => {
                            let (valid, rest) = buffer.split_at(e.valid_up_to());
                            let invalid_sequence_length = e.error_len()?;
                            buffer = &rest[invalid_sequence_length..];

                            // Return the valid prefix before reporting the decode error.
                            // SAFETY: `valid` is the prefix ending at `e.valid_up_to()`, which
                            // `Utf8Error::valid_up_to()` guarantees is valid UTF-8.
                            current = unsafe { core::str::from_utf8_unchecked(valid) }.chars();
                            previous_error = Some(e);
                        }
                    }
                }
            }
        }
    })
}
