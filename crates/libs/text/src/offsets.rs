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

    /// Returns the length of the text in UTF-16 code units.
    pub fn utf16_len(&self) -> u32 {
        match self.checkpoints.last() {
            Some(&len) => len,
            None => self.text.len().try_into().unwrap(),
        }
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

    /// Returns a cursor that converts increasing UTF-16 offsets to UTF-8 byte offsets without
    /// rescanning the text.
    pub fn cursor(&self) -> Cursor<'_> {
        Cursor {
            offsets: self,
            index: 0,
            units: 0,
        }
    }
}

/// Converts UTF-16 offsets to UTF-8 byte offsets, moving forward through the text.
pub(crate) struct Cursor<'a> {
    offsets: &'a Offsets,
    // The byte offset of the cursor and its UTF-16 offset.
    index: usize,
    units: u32,
}

impl Cursor<'_> {
    /// Converts a UTF-16 offset to a UTF-8 byte offset, rounding down to a character boundary
    /// and clamping to the end of the text. Offsets must not decrease between calls.
    pub fn utf8(&mut self, offset: u32) -> usize {
        let Offsets { text, checkpoints } = self.offsets;

        if checkpoints.is_empty() {
            return (offset as usize).min(text.len());
        }

        // Jump to the chunk holding `offset` when it lies beyond the cursor's chunk.
        if checkpoints
            .get(self.index / CHUNK + 1)
            .is_some_and(|&units| units <= offset)
        {
            let chunk = checkpoints.partition_point(|&units| units <= offset) - 1;
            self.index = (chunk * CHUNK).min(text.len());
            self.units = checkpoints[chunk];
        }

        while let Some(&byte) = text.as_bytes().get(self.index) {
            let len = utf16_len(byte);
            if len > 0 && self.units + len > offset {
                break;
            }
            self.units += len;
            self.index += 1;
        }

        self.index
    }
}

/// Returns the number of UTF-16 code units for the character that starts with `byte`, or zero for
/// a continuation byte.
fn utf16_len(byte: u8) -> u32 {
    u32::from(byte & 0xC0 != 0x80) + u32::from(byte >= 0xF0)
}
