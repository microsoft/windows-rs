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

    /// Returns the underlying `IDWriteTextFormat`.
    pub fn raw(&self) -> &IDWriteTextFormat {
        &self.raw
    }
}

pub(crate) fn dwrite_factory() -> Result<IDWriteFactory> {
    static SHARED: std::sync::OnceLock<IDWriteFactory> = std::sync::OnceLock::new();

    if let Some(factory) = SHARED.get() {
        return Ok(factory.clone());
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
    Ok(SHARED.get_or_init(|| factory).clone())
}
