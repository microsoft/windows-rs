use super::*;

/// Horizontal text alignment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextAlignment {
    /// Align to the leading edge (left in LTR).
    #[default]
    Leading,
    /// Center horizontally.
    Center,
    /// Align to the trailing edge (right in LTR).
    Trailing,
}

/// Vertical paragraph alignment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ParagraphAlignment {
    /// Align to the top edge.
    #[default]
    Top,
    /// Center vertically.
    Center,
    /// Align to the bottom edge.
    Bottom,
}

/// How text wraps when it reaches the layout width.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WordWrapping {
    /// Wrap onto the next line at word boundaries.
    #[default]
    Wrap,
    /// Do not wrap; text may extend past the layout width.
    NoWrap,
}

impl WordWrapping {
    pub(crate) fn to_abi(self) -> DWRITE_WORD_WRAPPING {
        match self {
            Self::Wrap => DWRITE_WORD_WRAPPING_WRAP,
            Self::NoWrap => DWRITE_WORD_WRAPPING_NO_WRAP,
        }
    }
}

/// Font weight.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontWeight(pub i32);

impl FontWeight {
    /// Normal (regular) weight, 400.
    pub const NORMAL: Self = Self(400);
    /// Bold weight, 700.
    pub const BOLD: Self = Self(700);
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::NORMAL
    }
}

/// A text format describing font family, size, weight, and alignment.
///
/// ```ignore
/// let format = TextFormat::new("Segoe UI", 24.0)?
///     .with_alignment(TextAlignment::Center);
/// ```
#[derive(Clone)]
pub struct TextFormat {
    raw: IDWriteTextFormat,
}

impl TextFormat {
    /// Creates a text format with normal weight.
    pub fn new(family: &str, size: f32) -> Result<Self> {
        Self::with_weight(family, size, FontWeight::NORMAL)
    }

    /// Creates a text format with bold weight.
    pub fn new_bold(family: &str, size: f32) -> Result<Self> {
        Self::with_weight(family, size, FontWeight::BOLD)
    }

    /// Creates a text format with the given font weight.
    pub fn with_weight(family: &str, size: f32, weight: FontWeight) -> Result<Self> {
        let factory = dwrite_factory()?;

        let family_wide: Vec<u16> = family.encode_utf16().chain(std::iter::once(0)).collect();
        let locale_wide: Vec<u16> = "en-us\0".encode_utf16().collect();

        let raw = unsafe {
            factory.CreateTextFormat(
                PCWSTR(family_wide.as_ptr()),
                None,
                weight.0,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                size,
                PCWSTR(locale_wide.as_ptr()),
            )?
        };

        Ok(Self { raw })
    }

    /// Sets the horizontal text alignment.
    pub fn with_alignment(self, alignment: TextAlignment) -> Self {
        let value = match alignment {
            TextAlignment::Leading => DWRITE_TEXT_ALIGNMENT_LEADING,
            TextAlignment::Center => DWRITE_TEXT_ALIGNMENT_CENTER,
            TextAlignment::Trailing => DWRITE_TEXT_ALIGNMENT_TRAILING,
        };
        unsafe { _ = self.raw.SetTextAlignment(value) };
        self
    }

    /// Sets the vertical paragraph alignment.
    pub fn with_paragraph_alignment(self, alignment: ParagraphAlignment) -> Self {
        let value = match alignment {
            ParagraphAlignment::Top => DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
            ParagraphAlignment::Center => DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
            ParagraphAlignment::Bottom => DWRITE_PARAGRAPH_ALIGNMENT_FAR,
        };
        unsafe { _ = self.raw.SetParagraphAlignment(value) };
        self
    }

    /// Sets the word wrapping mode.
    pub fn with_word_wrapping(self, wrapping: WordWrapping) -> Self {
        unsafe { _ = self.raw.SetWordWrapping(wrapping.to_abi()) };
        self
    }

    /// Returns the underlying `IDWriteTextFormat`.
    pub fn raw(&self) -> &IDWriteTextFormat {
        &self.raw
    }
}

/// Measured size and position of laid-out text, in device-independent pixels.
///
/// Returned by [`TextLayout::metrics`]. `left`/`top` are the offset of the
/// inked text within the layout box (for example, when centered).
#[derive(Copy, Clone, Debug, Default, PartialEq)]
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
pub struct HitTest {
    /// The text position (UTF-16 code-unit index) nearest the point.
    pub text_position: u32,
    /// `true` if the point is closer to the trailing edge of the character.
    pub is_trailing_hit: bool,
    /// `true` if the point falls inside the character's bounding box.
    pub is_inside: bool,
    /// The bounding rectangle of the character, relative to the layout box.
    pub bounds: Rect,
}

/// A block of text laid out within a box, ready to measure, hit-test, and draw.
///
/// A layout shapes the text once, then answers geometry queries and draws
/// without re-shaping. Reuse it across frames instead of calling
/// [`DrawingSession::draw_text`], which re-shapes every call.
///
/// ```ignore
/// let format = TextFormat::new("Segoe UI", 24.0)?;
/// let layout = TextLayout::new("Hello, Canvas!", &format, 400.0, 200.0)?;
/// let size = layout.metrics();
/// session.draw_text_layout(Vector2::new(8.0, 8.0), &layout, &brush);
/// ```
#[derive(Clone)]
pub struct TextLayout {
    raw: IDWriteTextLayout,
}

impl TextLayout {
    /// Lays out `text` with `format` inside a box of `max_width` by `max_height`
    /// device-independent pixels. Wrapping, alignment, and trimming come from
    /// the [`TextFormat`].
    pub fn new(text: &str, format: &TextFormat, max_width: f32, max_height: f32) -> Result<Self> {
        let factory = dwrite_factory()?;
        let wide: Vec<u16> = text.encode_utf16().collect();
        let raw = unsafe { factory.CreateTextLayout(&wide, format.raw(), max_width, max_height)? };
        Ok(Self { raw })
    }

    /// Measures the laid-out text.
    pub fn metrics(&self) -> TextMetrics {
        let mut m = DWRITE_TEXT_METRICS::default();
        unsafe { _ = self.raw.GetMetrics(&mut m) };
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

    /// Finds the character nearest `point`, where `point` is relative to the
    /// layout box origin.
    pub fn hit_test_point(&self, point: Vector2) -> HitTest {
        let mut trailing = BOOL::default();
        let mut inside = BOOL::default();
        let mut m = DWRITE_HIT_TEST_METRICS::default();
        unsafe {
            _ = self
                .raw
                .HitTestPoint(point.x, point.y, &mut trailing, &mut inside, &mut m);
        }
        HitTest {
            text_position: m.textPosition,
            is_trailing_hit: trailing.as_bool(),
            is_inside: inside.as_bool(),
            bounds: Rect::from_xywh(m.left, m.top, m.width, m.height),
        }
    }

    /// Returns the caret rectangle for the character at `position` (a UTF-16
    /// code-unit index). Pass `trailing = true` for the caret after the
    /// character. The rect is a zero-width vertical line at the caret, spanning
    /// the line height, for drawing a text cursor.
    pub fn caret_bounds(&self, position: u32, trailing: bool) -> Rect {
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        let mut m = DWRITE_HIT_TEST_METRICS::default();
        unsafe {
            _ = self
                .raw
                .HitTestTextPosition(position, trailing, &mut x, &mut y, &mut m);
        }
        Rect::from_xywh(x, y, 0.0, m.height)
    }

    /// Returns the layout box size (`max_width`, `max_height`).
    pub fn max_size(&self) -> (f32, f32) {
        unsafe { (self.raw.GetMaxWidth(), self.raw.GetMaxHeight()) }
    }

    /// Resizes the layout box. Wrapping and alignment reflow to the new size.
    pub fn set_max_size(&self, max_width: f32, max_height: f32) {
        unsafe {
            _ = self.raw.SetMaxWidth(max_width);
            _ = self.raw.SetMaxHeight(max_height);
        }
    }

    /// Returns the underlying `IDWriteTextLayout`.
    pub fn raw(&self) -> &IDWriteTextLayout {
        &self.raw
    }
}

pub(crate) fn dwrite_factory() -> Result<IDWriteFactory> {
    // The DirectWrite shared factory is thread-safe, but the in-house
    // metadata does not mark `IDWriteFactory` `[agile]`, so it is neither `Send` nor
    // `Sync`. Wrap it for the process-wide `OnceLock`.
    struct SharedFactory(IDWriteFactory);
    unsafe impl Send for SharedFactory {}
    unsafe impl Sync for SharedFactory {}

    static SHARED: std::sync::OnceLock<SharedFactory> = std::sync::OnceLock::new();

    if let Some(factory) = SHARED.get() {
        return Ok(factory.0.clone());
    }

    let mut factory: Option<IDWriteFactory> = None;
    unsafe {
        DWriteCreateFactory(
            DWRITE_FACTORY_TYPE_SHARED,
            &IDWriteFactory::IID,
            &mut factory as *mut _ as *mut _,
        )
        .ok()?;
    }
    let factory = factory.ok_or_else(Error::empty)?;
    Ok(SHARED.get_or_init(|| SharedFactory(factory)).0.clone())
}
