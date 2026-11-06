use super::*;
use std::sync::Arc;

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

/// A text format describing font family, size, weight, style, and alignment.
///
/// Cloning is cheap and a clone is independent: the `with_*` methods on one value never change
/// another. Layouts copy the format when they are created, so later changes do not affect them.
#[derive(Clone)]
pub struct TextFormat(Arc<Inner>);

struct Inner {
    raw: IDWriteTextFormat1,
    settings: Settings,
}

// The format is only changed while uniquely owned, so sharing it is safe. DirectWrite objects
// are not tied to a thread.
unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

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
    fallback: Option<FontFallback>,
}

impl TextFormat {
    /// Creates a text format with normal weight and style from the system fonts.
    pub fn new(family: &str, size: f32) -> Result<Self> {
        Self::create(family, size, FontWeight::NORMAL)
    }

    /// Creates a text format with bold weight and normal style from the system fonts.
    pub fn new_bold(family: &str, size: f32) -> Result<Self> {
        Self::create(family, size, FontWeight::BOLD)
    }

    fn create(family: &str, size: f32, weight: FontWeight) -> Result<Self> {
        let settings = Settings {
            family: wide(family),
            size,
            weight,
            style: FontStyle::Normal,
            collection: None,
            locale: None,
            alignment: TextAlignment::default(),
            paragraph_alignment: ParagraphAlignment::default(),
            word_wrapping: WordWrapping::default(),
            fallback: None,
        };
        let raw = settings.create()?;
        Ok(Self(Arc::new(Inner { raw, settings })))
    }

    /// Sets the font weight.
    pub fn with_weight(self, weight: FontWeight) -> Self {
        self.recreate(|settings| settings.weight = weight)
    }

    /// Sets the font style.
    pub fn with_style(self, style: FontStyle) -> Self {
        self.recreate(|settings| settings.style = style)
    }

    /// Looks up the family in `collection` instead of the system fonts.
    pub fn with_collection(self, collection: &FontCollection) -> Self {
        self.recreate(|settings| settings.collection = Some(collection.clone()))
    }

    /// Sets the locale, such as `"en-us"`, used for language-specific shaping and line breaking.
    /// The default is the user's locale.
    pub fn with_locale(self, locale: &str) -> Self {
        self.recreate(|settings| settings.locale = Some(wide(locale)))
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

    /// Sets the fonts to try for characters the format's font cannot display. The default is the
    /// system fallback.
    pub fn with_fallback(self, fallback: &FontFallback) -> Self {
        self.update(|settings| settings.fallback = Some(fallback.clone()))
    }

    /// Casts the underlying `IDWriteTextFormat1` to `T`.
    ///
    /// Use this to pass the format to DirectWrite or Direct2D through your own bindings, such
    /// as the `windows` crate's `IDWriteTextFormat`. Treat the result as read-only.
    pub fn to_interface<T: Interface>(&self) -> Result<T> {
        self.0.raw.cast()
    }

    pub(crate) fn raw(&self) -> &IDWriteTextFormat {
        &self.0.raw
    }

    /// Changes a setting that DirectWrite fixes when it creates a format, so a new format is
    /// needed.
    fn recreate(self, change: impl FnOnce(&mut Settings)) -> Self {
        let mut settings = self.0.settings.clone();
        change(&mut settings);
        let raw = settings.create().unwrap();
        settings.apply(&raw);
        Self(Arc::new(Inner { raw, settings }))
    }

    /// Changes a setting that an existing format can take, first giving this value its own
    /// format if the current one is shared with a clone.
    fn update(mut self, change: impl FnOnce(&mut Settings)) -> Self {
        let Some(inner) = Arc::get_mut(&mut self.0) else {
            return self.recreate(change);
        };

        change(&mut inner.settings);
        inner.settings.apply(&inner.raw);
        self
    }
}

impl Settings {
    fn create(&self) -> Result<IDWriteTextFormat1> {
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
                )?
                .cast()
        }
    }

    fn apply(&self, raw: &IDWriteTextFormat1) {
        let alignment = match self.alignment {
            TextAlignment::Leading => DWRITE_TEXT_ALIGNMENT_LEADING,
            TextAlignment::Center => DWRITE_TEXT_ALIGNMENT_CENTER,
            TextAlignment::Trailing => DWRITE_TEXT_ALIGNMENT_TRAILING,
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

        unsafe {
            raw.SetTextAlignment(alignment).unwrap();
            raw.SetParagraphAlignment(paragraph_alignment).unwrap();
            raw.SetWordWrapping(word_wrapping).unwrap();
            raw.SetFontFallback(self.fallback.as_ref().map(FontFallback::raw))
                .unwrap();
        }
    }
}
