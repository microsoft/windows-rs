use super::*;
use std::ops::{Range, RangeBounds};

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
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct HitTest {
    /// UTF-8 byte range of the cluster nearest the point.
    pub text_range: Range<usize>,
    /// `true` if the point is closer to the trailing edge of the cluster.
    pub is_trailing_hit: bool,
    /// `true` if the point falls inside the cluster's bounding box.
    pub is_inside: bool,
    /// The bounding rectangle of the cluster, relative to the layout box.
    pub bounds: Rect,
}

impl HitTest {
    /// The UTF-8 byte offset of the caret nearest the point: before the cluster, or after it for
    /// a trailing hit. Use this to place a caret or extend a selection. To draw the caret, pass
    /// the start of `text_range` and `is_trailing_hit` to [`TextLayout::caret_bounds`], which
    /// keeps a caret at the end of a wrapped line on that line.
    pub fn caret_position(&self) -> usize {
        if self.is_trailing_hit {
            self.text_range.end
        } else {
            self.text_range.start
        }
    }
}

/// The metrics of one line in a [`TextLayout`].
///
/// Returned by [`TextLayout::line_metrics`].
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct LineMetrics {
    /// UTF-8 byte range of the line, including trailing whitespace and the newline.
    pub text_range: Range<usize>,
    /// UTF-8 byte length of the whitespace at the end of the line, including the newline.
    pub trailing_whitespace_length: usize,
    /// UTF-8 byte length of the newline at the end of the line, or zero.
    pub newline_length: usize,
    /// Top edge of the line relative to the layout box.
    pub top: f32,
    /// Height of the line.
    pub height: f32,
    /// Distance from the top of the line to its baseline.
    pub baseline: f32,
}

/// The metrics of one cluster in a [`TextLayout`].
///
/// Returned by [`TextLayout::cluster_metrics`].
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ClusterMetrics {
    /// UTF-8 byte range of the cluster.
    pub text_range: Range<usize>,
    /// Advance width of the cluster.
    pub width: f32,
    /// `true` if a line can break after the cluster.
    pub can_wrap_line_after: bool,
    /// `true` if the cluster is whitespace.
    pub is_whitespace: bool,
    /// `true` if the cluster is a newline.
    pub is_newline: bool,
    /// `true` if the cluster is a soft hyphen.
    pub is_soft_hyphen: bool,
    /// `true` if the cluster is read right to left.
    pub is_right_to_left: bool,
}

/// An OpenType font feature, such as `liga` for standard ligatures or `tnum` for tabular figures.
///
/// Applied with [`TextLayout::set_font_features`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FontFeature {
    /// The four-character feature tag.
    pub tag: [u8; 4],
    /// The feature value. Most features use 1 to enable and 0 to disable; some select an
    /// alternate glyph by index.
    pub value: u32,
}

impl FontFeature {
    /// Creates a feature from its tag and value, such as `FontFeature::new(b"liga", 0)`.
    pub const fn new(tag: &[u8; 4], value: u32) -> Self {
        Self { tag: *tag, value }
    }
}

/// A block of text laid out within a box, ready to measure, hit-test, and draw.
///
/// A layout shapes the text once, then answers geometry queries without re-shaping. Text
/// positions are UTF-8 byte offsets into the string passed to [`TextLayout::new`].
pub struct TextLayout {
    raw: IDWriteTextLayout,
    offsets: Offsets,
}

// A layout can move to another thread. It is not `Sync`, because DirectWrite does not document
// concurrent calls on one layout as safe.
unsafe impl Send for TextLayout {}

impl TextLayout {
    /// Lays out `text` with `format` inside a box of `max_width` by `max_height`
    /// device-independent pixels. Wrapping and alignment come from the [`TextFormat`]. Panics if
    /// either size is negative.
    pub fn new(text: &str, format: &TextFormat, max_width: f32, max_height: f32) -> Self {
        let offsets = Offsets::new(text);
        let wide = offsets.to_utf16();
        let raw = unsafe {
            factory()
                .CreateTextLayout(
                    wide.as_ptr(),
                    wide.len().try_into().unwrap(),
                    format.raw(),
                    max_width,
                    max_height,
                )
                .unwrap()
        };
        Self { raw, offsets }
    }

    /// Returns the text being laid out.
    pub fn text(&self) -> &str {
        self.offsets.text()
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
        let mut cursor = self.offsets.cursor();
        let start = cursor.utf8(m.textPosition);
        let end = cursor.utf8(m.textPosition + m.length);
        HitTest {
            text_range: start..end,
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
                    self.offsets.utf16(position),
                    trailing,
                    &mut x,
                    &mut y,
                    &mut m,
                )
                .unwrap();
        }
        Rect::from_xywh(x, y, 0.0, m.height)
    }

    /// Resizes the layout box. Wrapping, alignment, and trimming reflow to the new size. Panics if
    /// either size is negative.
    pub fn set_max_size(&mut self, max_width: f32, max_height: f32) {
        unsafe {
            self.raw.SetMaxWidth(max_width).unwrap();
            self.raw.SetMaxHeight(max_height).unwrap();
        }
    }

    /// Returns the metrics of each line, in order.
    pub fn line_metrics(&self) -> Vec<LineMetrics> {
        let metrics = self.metrics();
        let lines = read_array(metrics.line_count, |buffer, len, count| unsafe {
            self.raw.GetLineMetrics(buffer, len, count)
        });

        let mut cursor = self.offsets.cursor();
        let mut start = 0;
        // Lines stack down from the top of the text.
        let mut top = metrics.top;

        lines
            .iter()
            .map(|line| {
                let end = start + line.length;
                let text_start = cursor.utf8(start);
                // Trailing whitespace includes the newline.
                let whitespace = cursor.utf8(end - line.trailingWhitespaceLength);
                let newline = cursor.utf8(end - line.newlineLength);
                let text_end = cursor.utf8(end);
                start = end;
                let line_top = top;
                top += line.height;

                LineMetrics {
                    text_range: text_start..text_end,
                    trailing_whitespace_length: text_end - whitespace,
                    newline_length: text_end - newline,
                    top: line_top,
                    height: line.height,
                    baseline: line.baseline,
                }
            })
            .collect()
    }

    /// Returns the metrics of each cluster, in text order. A cluster is the smallest unit of text
    /// a caret can move across, such as a character with its combining marks or a ligature.
    pub fn cluster_metrics(&self) -> Vec<ClusterMetrics> {
        // A cluster spans at least one UTF-16 code unit.
        let clusters = read_array(self.offsets.utf16_len(), |buffer, len, count| unsafe {
            self.raw.GetClusterMetrics(buffer, len, count)
        });

        let mut cursor = self.offsets.cursor();
        let mut start = 0;

        clusters
            .iter()
            .map(|cluster| {
                let end = start + u32::from(cluster.length);
                let text_range = cursor.utf8(start)..cursor.utf8(end);
                start = end;

                ClusterMetrics {
                    text_range,
                    width: cluster.width,
                    can_wrap_line_after: cluster.canWrapLineAfter() != 0,
                    is_whitespace: cluster.isWhitespace() != 0,
                    is_newline: cluster.isNewline() != 0,
                    is_soft_hyphen: cluster.isSoftHyphen() != 0,
                    is_right_to_left: cluster.isRightToLeft() != 0,
                }
            })
            .collect()
    }

    /// Returns the rectangles covering `range`, a UTF-8 byte range, relative to the layout box.
    /// A range that spans lines or changes direction has more than one rectangle, and an empty
    /// range has one zero-width rectangle at its position. Use this to draw a selection.
    pub fn range_bounds(&self, range: impl RangeBounds<usize>) -> Vec<Rect> {
        let range = self.offsets.range(range);
        // A selection usually covers a few lines, with more rectangles where direction changes.
        let hits = read_array(16, |buffer, len, count| unsafe {
            self.raw.HitTestTextRange(
                range.startPosition,
                range.length,
                0.0,
                0.0,
                buffer,
                len,
                count,
            )
        });

        hits.iter()
            .map(|m| Rect::from_xywh(m.left, m.top, m.width, m.height))
            .collect()
    }

    /// Returns the shaped glyphs in drawing order, positioned relative to the layout box. Use
    /// this to draw the layout with your own renderer, such as a glyph atlas, along with
    /// [`decorations`](Self::decorations).
    pub fn glyph_runs(&self) -> Vec<GlyphRun> {
        Collector::glyph_runs(&self.raw, &self.offsets)
    }

    /// Returns the underlines and strikethroughs set with [`set_underline`](Self::set_underline)
    /// and [`set_strikethrough`](Self::set_strikethrough), relative to the layout box.
    pub fn decorations(&self) -> Vec<Decoration> {
        Collector::decorations(&self.raw, &self.offsets)
    }

    /// Sets the font family for `range`, a UTF-8 byte range. The family comes from the format's
    /// collection unless [`set_font_collection`](Self::set_font_collection) changes it.
    pub fn set_font_family(&mut self, range: impl RangeBounds<usize>, family: &str) {
        let range = self.offsets.range(range);
        unsafe {
            self.raw
                .SetFontFamilyName(wide(family).as_ptr(), range)
                .unwrap();
        }
    }

    /// Sets the font collection that family names are found in for `range`.
    pub fn set_font_collection(
        &mut self,
        range: impl RangeBounds<usize>,
        collection: &FontCollection,
    ) {
        let range = self.offsets.range(range);
        unsafe {
            self.raw.SetFontCollection(collection.raw(), range).unwrap();
        }
    }

    /// Sets the font size, in device-independent pixels, for `range`.
    pub fn set_font_size(&mut self, range: impl RangeBounds<usize>, size: f32) {
        let range = self.offsets.range(range);
        unsafe { self.raw.SetFontSize(size, range).unwrap() };
    }

    /// Sets the font weight for `range`.
    pub fn set_font_weight(&mut self, range: impl RangeBounds<usize>, weight: FontWeight) {
        let range = self.offsets.range(range);
        unsafe { self.raw.SetFontWeight(weight.0.into(), range).unwrap() };
    }

    /// Sets the font style for `range`.
    pub fn set_font_style(&mut self, range: impl RangeBounds<usize>, style: FontStyle) {
        let range = self.offsets.range(range);
        unsafe { self.raw.SetFontStyle(style.to_abi(), range).unwrap() };
    }

    /// Underlines `range`, or removes the underline.
    pub fn set_underline(&mut self, range: impl RangeBounds<usize>, underline: bool) {
        let range = self.offsets.range(range);
        unsafe { self.raw.SetUnderline(underline, range).unwrap() };
    }

    /// Strikes through `range`, or removes the strikethrough.
    pub fn set_strikethrough(&mut self, range: impl RangeBounds<usize>, strikethrough: bool) {
        let range = self.offsets.range(range);
        unsafe { self.raw.SetStrikethrough(strikethrough, range).unwrap() };
    }

    /// Sets the locale, such as `"ar-eg"`, used to shape and break lines in `range`.
    pub fn set_locale(&mut self, range: impl RangeBounds<usize>, locale: &str) {
        let range = self.offsets.range(range);
        unsafe {
            self.raw
                .SetLocaleName(wide(locale).as_ptr(), range)
                .unwrap();
        }
    }

    /// Shapes `range` with the OpenType `features`, such as `liga` or `tnum`. The list replaces
    /// the features DirectWrite applies by default, so include any defaults to keep, such as
    /// `liga` and `kern`. An empty list restores the defaults. Pass `..` for all of the text.
    pub fn set_font_features(&mut self, range: impl RangeBounds<usize>, features: &[FontFeature]) {
        let range = self.offsets.range(range);

        if features.is_empty() {
            unsafe {
                self.raw
                    .SetTypography(None::<&IDWriteTypography>, range)
                    .unwrap();
            }
            return;
        }

        unsafe {
            let typography = factory().CreateTypography().unwrap();
            for feature in features {
                typography
                    .AddFontFeature(DWRITE_FONT_FEATURE {
                        nameTag: i32::from_le_bytes(feature.tag),
                        parameter: feature.value,
                    })
                    .unwrap();
            }
            self.raw.SetTypography(&typography, range).unwrap();
        }
    }

    /// Borrows the underlying `IDWriteTextLayout` as `T` without a `QueryInterface` call.
    ///
    /// `T` is `IDWriteTextLayout` from other bindings, such as the `windows` crate; other
    /// interfaces fail to build. Use [`to_interface`](Self::to_interface) to query for other
    /// interfaces.
    pub fn as_interface<T: Interface>(&self) -> &T {
        self.raw.as_interface()
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

// Calls a DirectWrite method that fills a caller-supplied array. `capacity` is the expected
// length; if the array needs more, the method reports the length and is called again.
fn read_array<T: Clone + Default>(
    capacity: u32,
    read: impl Fn(Option<*mut T>, u32, *mut u32) -> HRESULT,
) -> Vec<T> {
    let mut buffer = vec![T::default(); capacity as usize];
    let mut len = 0;
    let mut result = read(Some(buffer.as_mut_ptr()), capacity, &mut len);

    if result == E_NOT_SUFFICIENT_BUFFER {
        buffer.resize(len as usize, T::default());
        result = read(Some(buffer.as_mut_ptr()), len, &mut len);
    }

    result.unwrap();
    buffer.truncate(len as usize);
    buffer
}
