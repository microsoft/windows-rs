use super::*;

/// Measured size and position of laid-out text, in device-independent pixels.
///
/// Returned by [`TextLayout::metrics`]. `left`/`top` are the offset of the inked text within the
/// layout box (for example, when centered).
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct TextMetrics {
    /// Left edge of the text relative to the layout box.
    pub left: f32,
    /// Top edge of the text relative to the layout box.
    pub top: f32,
    /// Width of the formatted text, ignoring trailing whitespace.
    pub width: f32,
    /// Width of the formatted text, including trailing whitespace.
    pub width_including_trailing_whitespace: f32,
    /// Height of the formatted text.
    pub height: f32,
    /// Maximum width given to the layout (its layout box width).
    pub layout_width: f32,
    /// Maximum height given to the layout (its layout box height).
    pub layout_height: f32,
    /// Number of lines the text was broken into.
    pub line_count: u32,
}

impl TextMetrics {
    /// The bounding rectangle of the inked text within the layout box.
    pub fn bounds(&self) -> Rect {
        Rect::from_xywh(self.left, self.top, self.width, self.height)
    }
}

/// The result of a point hit-test against a [`TextLayout`].
///
/// Returned by [`TextLayout::hit_test_point`].
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct HitTest {
    /// UTF-8 byte offset of the start of the cluster nearest the point.
    pub text_position: usize,
    /// UTF-8 byte length of that cluster. A trailing-edge caret goes at
    /// `text_position + length`.
    pub length: usize,
    /// `true` if the point is closer to the trailing edge of the cluster.
    pub is_trailing_hit: bool,
    /// `true` if the point falls inside the cluster's bounding box.
    pub is_inside: bool,
    /// The bounding rectangle of the cluster, relative to the layout box.
    pub bounds: Rect,
}

/// A block of text laid out within a box, ready to measure, hit-test, and draw.
///
/// A layout shapes the text once, then answers geometry queries without re-shaping. Text
/// positions are UTF-8 byte offsets into the string passed to [`TextLayout::new`].
pub struct TextLayout {
    raw: IDWriteTextLayout,
    text: Box<str>,
}

// A layout can move to another thread. It is not `Sync`, because DirectWrite does not document
// concurrent calls on one layout as safe.
unsafe impl Send for TextLayout {}

impl TextLayout {
    /// Lays out `text` with `format` inside a box of `max_width` by `max_height`
    /// device-independent pixels. Wrapping and alignment come from the [`TextFormat`].
    pub fn new(text: &str, format: &TextFormat, max_width: f32, max_height: f32) -> Result<Self> {
        let wide: Vec<u16> = text.encode_utf16().collect();
        let raw = unsafe {
            factory().CreateTextLayout(
                wide.as_ptr(),
                wide.len().try_into().unwrap(),
                format.raw(),
                max_width,
                max_height,
            )?
        };
        Ok(Self {
            raw,
            text: text.into(),
        })
    }

    /// Returns the text being laid out.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Measures the laid-out text.
    pub fn metrics(&self) -> TextMetrics {
        let mut m = DWRITE_TEXT_METRICS::default();
        unsafe { self.raw.GetMetrics(&mut m).unwrap() };
        TextMetrics {
            left: m.left,
            top: m.top,
            width: m.width,
            width_including_trailing_whitespace: m.widthIncludingTrailingWhitespace,
            height: m.height,
            layout_width: m.layoutWidth,
            layout_height: m.layoutHeight,
            line_count: m.lineCount,
        }
    }

    /// Finds the cluster nearest `point`, where `point` is relative to the layout box origin.
    pub fn hit_test_point(&self, point: Vector2) -> HitTest {
        let mut trailing = BOOL::default();
        let mut inside = BOOL::default();
        let mut m = DWRITE_HIT_TEST_METRICS::default();
        unsafe {
            self.raw
                .HitTestPoint(point.x, point.y, &mut trailing, &mut inside, &mut m)
                .unwrap();
        }
        let start = utf8_offset(&self.text, m.textPosition);
        let end = utf8_offset(&self.text, m.textPosition + m.length);
        HitTest {
            text_position: start,
            length: end - start,
            is_trailing_hit: trailing.as_bool(),
            is_inside: inside.as_bool(),
            bounds: Rect::from_xywh(m.left, m.top, m.width, m.height),
        }
    }

    /// Returns the caret rectangle for the character at `position`, a UTF-8 byte offset that
    /// must lie on a character boundary. Pass `trailing = true` for the caret after the
    /// character. The rect is a zero-width vertical line at the caret, spanning the line
    /// height.
    pub fn caret_bounds(&self, position: usize, trailing: bool) -> Rect {
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        let mut m = DWRITE_HIT_TEST_METRICS::default();
        unsafe {
            self.raw
                .HitTestTextPosition(
                    utf16_offset(&self.text, position),
                    trailing,
                    &mut x,
                    &mut y,
                    &mut m,
                )
                .unwrap();
        }
        Rect::from_xywh(x, y, 0.0, m.height)
    }

    /// Returns the layout box size (`max_width`, `max_height`).
    pub fn max_size(&self) -> (f32, f32) {
        unsafe { (self.raw.GetMaxWidth(), self.raw.GetMaxHeight()) }
    }

    /// Resizes the layout box. Wrapping and alignment reflow to the new size.
    pub fn set_max_size(&mut self, max_width: f32, max_height: f32) {
        unsafe {
            self.raw.SetMaxWidth(max_width).unwrap();
            self.raw.SetMaxHeight(max_height).unwrap();
        }
    }

    /// Casts the underlying `IDWriteTextLayout` to `T`.
    ///
    /// Use this to draw the layout with Direct2D or query DirectWrite through your own bindings,
    /// such as the `windows` crate's `IDWriteTextLayout`. Text positions in DirectWrite calls are
    /// UTF-16 code-unit offsets.
    pub fn to_interface<T: Interface>(&self) -> Result<T> {
        self.raw.cast()
    }
}

/// Converts a UTF-8 byte offset to a UTF-16 code-unit offset. Panics if `offset` is not a
/// character boundary in `text`.
fn utf16_offset(text: &str, offset: usize) -> u32 {
    text[..offset].encode_utf16().count().try_into().unwrap()
}

/// Converts a UTF-16 code-unit offset to a UTF-8 byte offset, rounding down to a character
/// boundary and clamping to the end of `text`.
fn utf8_offset(text: &str, offset: u32) -> usize {
    let offset = offset as usize;
    let mut utf16 = 0;
    for (byte, ch) in text.char_indices() {
        let next = utf16 + ch.len_utf16();
        if next > offset {
            return byte;
        }
        utf16 = next;
    }
    text.len()
}
