use super::*;
use std::rc::Rc;

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

/// A text format describing font family, size, weight, and alignment.
///
/// Cloning is cheap and a clone is independent: the `with_*` methods on one value never change
/// another. Layouts copy the format when they are created, so later changes do not affect them.
#[derive(Clone)]
pub struct TextFormat(Rc<Inner>);

#[derive(Clone)]
struct Inner {
    raw: IDWriteTextFormat,
    family: Vec<u16>,
    size: f32,
    weight: FontWeight,
    alignment: TextAlignment,
    paragraph_alignment: ParagraphAlignment,
    word_wrapping: WordWrapping,
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
        let family: Vec<u16> = family.encode_utf16().chain(std::iter::once(0)).collect();
        let raw = create(&family, size, weight)?;

        Ok(Self(Rc::new(Inner {
            raw,
            family,
            size,
            weight,
            alignment: TextAlignment::default(),
            paragraph_alignment: ParagraphAlignment::default(),
            word_wrapping: WordWrapping::default(),
        })))
    }

    /// Sets the horizontal text alignment.
    pub fn with_alignment(self, alignment: TextAlignment) -> Self {
        self.update(|inner| inner.alignment = alignment)
    }

    /// Sets the vertical paragraph alignment.
    pub fn with_paragraph_alignment(self, alignment: ParagraphAlignment) -> Self {
        self.update(|inner| inner.paragraph_alignment = alignment)
    }

    /// Sets the word wrapping mode.
    pub fn with_word_wrapping(self, wrapping: WordWrapping) -> Self {
        self.update(|inner| inner.word_wrapping = wrapping)
    }

    /// Casts the underlying `IDWriteTextFormat` to `T`.
    ///
    /// Use this to pass the format to DirectWrite or Direct2D through your own bindings, such
    /// as the `windows` crate's `IDWriteTextFormat`. Treat the result as read-only.
    pub fn to_interface<T: Interface>(&self) -> Result<T> {
        self.0.raw.cast()
    }

    pub(crate) fn raw(&self) -> &IDWriteTextFormat {
        &self.0.raw
    }

    /// Applies `change` to this value, first giving it its own DirectWrite format if the current
    /// one is shared with a clone.
    fn update(mut self, change: impl FnOnce(&mut Inner)) -> Self {
        if Rc::get_mut(&mut self.0).is_none() {
            let mut inner = (*self.0).clone();
            inner.raw = create(&inner.family, inner.size, inner.weight).unwrap();
            self.0 = Rc::new(inner);
        }

        let inner = Rc::get_mut(&mut self.0).unwrap();
        change(inner);

        let alignment = match inner.alignment {
            TextAlignment::Leading => DWRITE_TEXT_ALIGNMENT_LEADING,
            TextAlignment::Center => DWRITE_TEXT_ALIGNMENT_CENTER,
            TextAlignment::Trailing => DWRITE_TEXT_ALIGNMENT_TRAILING,
        };
        let paragraph_alignment = match inner.paragraph_alignment {
            ParagraphAlignment::Top => DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
            ParagraphAlignment::Center => DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
            ParagraphAlignment::Bottom => DWRITE_PARAGRAPH_ALIGNMENT_FAR,
        };
        let word_wrapping = match inner.word_wrapping {
            WordWrapping::Wrap => DWRITE_WORD_WRAPPING_WRAP,
            WordWrapping::NoWrap => DWRITE_WORD_WRAPPING_NO_WRAP,
        };

        unsafe {
            inner.raw.SetTextAlignment(alignment).unwrap();
            inner
                .raw
                .SetParagraphAlignment(paragraph_alignment)
                .unwrap();
            inner.raw.SetWordWrapping(word_wrapping).unwrap();
        }

        self
    }
}

/// Creates a DirectWrite format from a null-terminated family name.
fn create(family: &[u16], size: f32, weight: FontWeight) -> Result<IDWriteTextFormat> {
    unsafe {
        factory().CreateTextFormat(
            family.as_ptr(),
            None,
            weight.0.into(),
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            size,
            w!("en-us").as_ptr(),
        )
    }
}
