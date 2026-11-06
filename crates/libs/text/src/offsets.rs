use super::*;
use std::ops::{Bound, Range, RangeBounds};

// Bytes between checkpoints. A conversion scans at most this many bytes.
const CHUNK: usize = 64;

/// Text with conversions between UTF-8 byte offsets and DirectWrite's UTF-16 code-unit offsets.
pub(crate) struct Offsets {
    text: Box<str>,
    // The UTF-16 offset at the start of each `CHUNK` of bytes, then the total length. Empty for
    // ASCII text, where both offsets are the same.
    checkpoints: Box<[u32]>,
}

impl Offsets {
    pub fn new(text: &str) -> Self {
        let checkpoints = if text.is_ascii() {
            Box::default()
        } else {
            let mut units = 0;
            let mut checkpoints: Vec<u32> = text
                .as_bytes()
                .chunks(CHUNK)
                .map(|chunk| {
                    let start = units;
                    units += chunk.iter().map(|&byte| utf16_len(byte)).sum::<u32>();
                    start
                })
                .collect();
            checkpoints.push(units);
            checkpoints.into()
        };

        Self {
            text: text.into(),
            checkpoints,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// Encodes the text as UTF-16 for DirectWrite.
    pub fn to_utf16(&self) -> Vec<u16> {
        if self.checkpoints.is_empty() {
            self.text.bytes().map(u16::from).collect()
        } else {
            self.text.encode_utf16().collect()
        }
    }

    /// Converts a UTF-8 byte offset to a UTF-16 offset. Panics if `offset` is not a character
    /// boundary in the text.
    pub fn utf16(&self, offset: usize) -> u32 {
        assert!(self.text.is_char_boundary(offset));

        if self.checkpoints.is_empty() {
            return offset.try_into().unwrap();
        }

        let chunk = offset / CHUNK;
        let bytes = &self.text.as_bytes()[chunk * CHUNK..offset];
        self.checkpoints[chunk] + bytes.iter().map(|&byte| utf16_len(byte)).sum::<u32>()
    }

    /// Converts a UTF-8 byte range to a DirectWrite text range. Panics if the range is out of
    /// order or either end is not a character boundary.
    pub fn range(&self, range: impl RangeBounds<usize>) -> DWRITE_TEXT_RANGE {
        let range = self.bounds(range);
        let start = self.utf16(range.start);
        let end = self.utf16(range.end);
        assert!(start <= end);

        DWRITE_TEXT_RANGE {
            startPosition: start,
            length: end - start,
        }
    }

    fn bounds(&self, range: impl RangeBounds<usize>) -> Range<usize> {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start.checked_add(1).unwrap(),
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end.checked_add(1).unwrap(),
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.text.len(),
        };
        start..end
    }

    /// Converts a UTF-16 offset to a UTF-8 byte offset, rounding down to a character boundary
    /// and clamping to the end of the text.
    pub fn utf8(&self, offset: u32) -> usize {
        if self.checkpoints.is_empty() {
            return (offset as usize).min(self.text.len());
        }

        let chunk = self.checkpoints.partition_point(|&units| units <= offset) - 1;
        let start = chunk * CHUNK;
        let mut units = self.checkpoints[chunk];

        for (index, &byte) in self.text.as_bytes().iter().enumerate().skip(start) {
            let len = utf16_len(byte);
            if len > 0 && units + len > offset {
                return index;
            }
            units += len;
        }

        self.text.len()
    }

    /// Returns a function that converts UTF-16 offsets to UTF-8 byte offsets in one pass. The
    /// offsets passed to it must not decrease.
    pub fn utf8_cursor(&self) -> impl FnMut(u32) -> usize + '_ {
        let mut index = 0;
        let mut units = 0;

        move |offset| {
            if self.checkpoints.is_empty() {
                return (offset as usize).min(self.text.len());
            }

            while let Some(&byte) = self.text.as_bytes().get(index) {
                let len = utf16_len(byte);
                if len > 0 && units + len > offset {
                    break;
                }
                units += len;
                index += 1;
            }

            index
        }
    }
}

/// Returns the number of UTF-16 code units for the character that starts with `byte`, or zero for
/// a continuation byte.
fn utf16_len(byte: u8) -> u32 {
    u32::from(byte & 0xC0 != 0x80) + u32::from(byte >= 0xF0)
}
