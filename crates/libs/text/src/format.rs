use super::*;
use std::sync::{Arc, OnceLock};

/// Horizontal text alignment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum TextAlignment {
    /// Align to the leading edge (left in LTR).
    #[default]
    Leading,
    /// Center horizontally.
    Center,
    /// Align to the trailing edge (right in LTR).
    Trailing,
    /// Stretch each wrapped line to fill the width by widening the spaces between words. The last
    /// line of a paragraph aligns to the leading edge.
    Justified,
}

/// Vertical paragraph alignment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
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
#[non_exhaustive]
pub enum WordWrapping {
    /// Wrap onto the next line at word boundaries.
    #[default]
    Wrap,
    /// Do not wrap; text may extend past the layout width.
    NoWrap,
}

/// The direction in which a paragraph reads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReadingDirection {
    /// Left to right, as in English. The leading edge is on the left.
    #[default]
    LeftToRight,
    /// Right to left, as in Arabic and Hebrew. The leading edge is on the right.
    RightToLeft,
}

/// Where text that overflows the layout box is cut off and replaced by an ellipsis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Trimming {
    /// Do not trim; text may extend past the layout box.
    #[default]
    None,
    /// End the visible text with an ellipsis at a character boundary.
    Character,
    /// End the visible text with an ellipsis at a word boundary.
    Word,
}

/// The distance between lines of text.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum LineSpacing {
    /// Each line is as tall as its fonts and inline content need.
    #[default]
    Default,
    /// Each line is this many times its default height, with the extra space split evenly above
    /// and below the text, as in CSS `line-height`.
    Proportional(f32),
    /// Every line is `height` tall, with its baseline `baseline` below the top of the line.
    Uniform {
        /// The height of each line in device-independent pixels.
        height: f32,
        /// The distance from the top of each line to its baseline.
        baseline: f32,
    },
}

/// A text format describing font family, size, weight, style, and alignment.
///
/// Cloning is cheap and a clone is independent: the `with_*` methods on one value never change
/// another. Layouts copy the format when they are created, so later changes do not affect them.
/// The native format is created and validated on first use, including interface access. Invalid
/// settings panic at that point.
#[derive(Clone)]
pub struct TextFormat(Arc<Inner>);

struct Inner {
    raw: OnceLock<IDWriteTextFormat1>,
    settings: Settings,
}

// The native format is initialized once and never mutated. DirectWrite objects are not tied to
// a thread.
unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

impl Clone for Inner {
    fn clone(&self) -> Self {
        Self {
            raw: OnceLock::new(),
            settings: self.settings.clone(),
        }
    }
}

#[derive(Clone)]
struct Settings {
    family: Vec<u16>,
    size: f32,
    weight: FontWeight,
    style: FontStyle,
    collection: Option<FontCollection>,
    locale: Option<Vec<u16>>,
    alignment: TextAlignment,
    paragraph_alignment: ParagraphAlignment,
    word_wrapping: WordWrapping,
    reading_direction: ReadingDirection,
    trimming: Trimming,
    line_spacing: LineSpacing,
    tab_width: Option<f32>,
    fallback: Option<FontFallback>,
}

impl TextFormat {
    /// Creates a text format for `family` at an em size of `size` device-independent pixels, with
    /// normal weight and style from the system fonts. If the family is not installed, text uses
    /// the fallback fonts. `size` must be positive.
    pub fn new(family: &str, size: f32) -> Self {
        let settings = Settings {
            family: wide(family),
            size,
            weight: FontWeight::NORMAL,
            style: FontStyle::Normal,
            collection: None,
            locale: None,
            alignment: TextAlignment::default(),
            paragraph_alignment: ParagraphAlignment::default(),
            word_wrapping: WordWrapping::default(),
            reading_direction: ReadingDirection::default(),
            trimming: Trimming::default(),
            line_spacing: LineSpacing::default(),
            tab_width: None,
            fallback: None,
        };
        Self(Arc::new(Inner {
            raw: OnceLock::new(),
            settings,
        }))
    }

    /// Sets the font weight, which must be from 1 to 999.
    pub fn with_weight(self, weight: FontWeight) -> Self {
        self.update(|settings| settings.weight = weight)
    }

    /// Sets the font style.
    pub fn with_style(self, style: FontStyle) -> Self {
        self.update(|settings| settings.style = style)
    }

    /// Looks up the family in `collection` instead of the system fonts.
    pub fn with_collection(self, collection: &FontCollection) -> Self {
        self.update(|settings| settings.collection = Some(collection.clone()))
    }

    /// Sets the locale, such as `"en-us"`, used for language-specific shaping and line breaking.
    /// The default is the user's locale.
    pub fn with_locale(self, locale: &str) -> Self {
        self.update(|settings| settings.locale = Some(wide(locale)))
    }

    /// Sets the horizontal text alignment.
    pub fn with_alignment(self, alignment: TextAlignment) -> Self {
        self.update(|settings| settings.alignment = alignment)
    }

    /// Sets the vertical paragraph alignment.
    pub fn with_paragraph_alignment(self, alignment: ParagraphAlignment) -> Self {
        self.update(|settings| settings.paragraph_alignment = alignment)
    }

    /// Sets the word wrapping mode.
    pub fn with_word_wrapping(self, wrapping: WordWrapping) -> Self {
        self.update(|settings| settings.word_wrapping = wrapping)
    }

    /// Sets the reading direction of each paragraph, which also decides which edge is leading.
    pub fn with_reading_direction(self, direction: ReadingDirection) -> Self {
        self.update(|settings| settings.reading_direction = direction)
    }

    /// Sets how text that overflows the layout box is trimmed. Combine with
    /// [`WordWrapping::NoWrap`] to trim a single line to the width.
    pub fn with_trimming(self, trimming: Trimming) -> Self {
        self.update(|settings| settings.trimming = trimming)
    }

    /// Sets the distance between lines.
    pub fn with_line_spacing(self, spacing: LineSpacing) -> Self {
        self.update(|settings| settings.line_spacing = spacing)
    }

    /// Sets the distance between tab stops in device-independent pixels. The default is four
    /// times the font size. `width` must be positive.
    pub fn with_tab_width(self, width: f32) -> Self {
        self.update(|settings| settings.tab_width = Some(width))
    }

    /// Sets the fonts to try for characters the format's font cannot display. The default is the
    /// system fallback.
    pub fn with_fallback(self, fallback: &FontFallback) -> Self {
        self.update(|settings| settings.fallback = Some(fallback.clone()))
    }

    /// Borrows the underlying `IDWriteTextFormat1` as `T` without a `QueryInterface` call.
    ///
    /// `T` is `IDWriteTextFormat1` from other bindings, such as the `windows` crate; other
    /// interfaces fail to build. Use [`to_interface`](Self::to_interface) to query for other
    /// interfaces. Treat the result as read-only.
    pub fn as_interface<T: Interface>(&self) -> &T {
        self.native().as_interface()
    }

    /// Casts the underlying `IDWriteTextFormat1` to `T`.
    ///
    /// Use this to pass the format to DirectWrite or Direct2D through your own bindings, such
    /// as the `windows` crate's `IDWriteTextFormat`. Treat the result as read-only.
    pub fn to_interface<T: Interface>(&self) -> Result<T> {
        self.native().cast()
    }

    pub(crate) fn raw(&self) -> &IDWriteTextFormat {
        self.native()
    }

    fn native(&self) -> &IDWriteTextFormat1 {
        self.0.raw.get_or_init(|| {
            let raw = self.0.settings.create();
            self.0.settings.apply(&raw);
            raw
        })
    }

    fn update(mut self, change: impl FnOnce(&mut Settings)) -> Self {
        let inner = Arc::make_mut(&mut self.0);
        inner.raw = OnceLock::new();
        change(&mut inner.settings);
        self
    }
}

impl Settings {
    fn create(&self) -> IDWriteTextFormat1 {
        let locale = self.locale.as_deref().unwrap_or(user_locale());

        unsafe {
            factory()
                .CreateTextFormat(
                    self.family.as_ptr(),
                    self.collection.as_ref().map(|c| &**c.raw()),
                    self.weight.0.into(),
                    self.style.to_abi(),
                    DWRITE_FONT_STRETCH_NORMAL,
                    self.size,
                    locale.as_ptr(),
                )
                .unwrap()
                .cast()
                .unwrap()
        }
    }

    fn apply(&self, raw: &IDWriteTextFormat1) {
        let alignment = match self.alignment {
            TextAlignment::Leading => DWRITE_TEXT_ALIGNMENT_LEADING,
            TextAlignment::Center => DWRITE_TEXT_ALIGNMENT_CENTER,
            TextAlignment::Trailing => DWRITE_TEXT_ALIGNMENT_TRAILING,
            TextAlignment::Justified => DWRITE_TEXT_ALIGNMENT_JUSTIFIED,
        };
        let paragraph_alignment = match self.paragraph_alignment {
            ParagraphAlignment::Top => DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
            ParagraphAlignment::Center => DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
            ParagraphAlignment::Bottom => DWRITE_PARAGRAPH_ALIGNMENT_FAR,
        };
        let word_wrapping = match self.word_wrapping {
            WordWrapping::Wrap => DWRITE_WORD_WRAPPING_WRAP,
            WordWrapping::NoWrap => DWRITE_WORD_WRAPPING_NO_WRAP,
        };
        let reading_direction = match self.reading_direction {
            ReadingDirection::LeftToRight => DWRITE_READING_DIRECTION_LEFT_TO_RIGHT,
            ReadingDirection::RightToLeft => DWRITE_READING_DIRECTION_RIGHT_TO_LEFT,
        };
        let granularity = match self.trimming {
            Trimming::None => DWRITE_TRIMMING_GRANULARITY_NONE,
            Trimming::Character => DWRITE_TRIMMING_GRANULARITY_CHARACTER,
            Trimming::Word => DWRITE_TRIMMING_GRANULARITY_WORD,
        };
        let (method, height, baseline, leading_before) = match self.line_spacing {
            LineSpacing::Default => (DWRITE_LINE_SPACING_METHOD_DEFAULT, 0.0, 0.0, 0.0),
            LineSpacing::Proportional(height) => {
                (DWRITE_LINE_SPACING_METHOD_PROPORTIONAL, height, 1.0, 0.5)
            }
            LineSpacing::Uniform { height, baseline } => {
                (DWRITE_LINE_SPACING_METHOD_UNIFORM, height, baseline, 0.0)
            }
        };

        unsafe {
            raw.SetTextAlignment(alignment).unwrap();
            raw.SetParagraphAlignment(paragraph_alignment).unwrap();
            raw.SetWordWrapping(word_wrapping).unwrap();
            raw.SetReadingDirection(reading_direction).unwrap();

            // The ellipsis is drawn in the format's font, so it is created from the format.
            let sign = (self.trimming != Trimming::None)
                .then(|| factory().CreateEllipsisTrimmingSign(raw).unwrap());
            let trimming = DWRITE_TRIMMING {
                granularity,
                ..Default::default()
            };
            raw.SetTrimming(&trimming, sign.as_ref()).unwrap();

            raw.cast::<IDWriteTextFormat2>()
                .unwrap()
                .SetLineSpacing(&DWRITE_LINE_SPACING {
                    method,
                    height,
                    baseline,
                    leadingBefore: leading_before,
                    fontLineGapUsage: DWRITE_FONT_LINE_GAP_USAGE_DEFAULT,
                })
                .unwrap();

            if let Some(width) = self.tab_width {
                raw.SetIncrementalTabStop(width).unwrap();
            }

            raw.SetFontFallback(self.fallback.as_ref().map(FontFallback::raw))
                .unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_defers_native_creation() {
        let format = TextFormat::new("Segoe UI", 24.0)
            .with_weight(FontWeight::BOLD)
            .with_style(FontStyle::Italic)
            .with_alignment(TextAlignment::Center);
        assert!(format.0.raw.get().is_none());

        std::thread::scope(|scope| {
            let threads: Vec<_> = (0..4)
                .map(|_| {
                    scope.spawn(|| format.as_interface::<IDWriteTextFormat1>().as_raw() as usize)
                })
                .collect();
            let pointers: Vec<_> = threads.into_iter().map(|t| t.join().unwrap()).collect();
            assert!(pointers.iter().all(|p| *p == pointers[0]));
        });

        let original = format.to_interface::<IDWriteTextFormat1>().unwrap();
        let changed = format.with_weight(FontWeight::NORMAL);
        assert!(changed.0.raw.get().is_none());
        assert_ne!(changed.native().as_raw(), original.as_raw());
    }
}
