use super::*;
use std::hash::{Hash, Hasher};

/// The slant of a font.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FontStyle {
    /// Upright.
    #[default]
    Normal,
    /// Slanted version of the upright design.
    Oblique,
    /// Cursive design, usually slanted.
    Italic,
}

impl FontStyle {
    pub(crate) fn to_abi(self) -> DWRITE_FONT_STYLE {
        match self {
            Self::Normal => DWRITE_FONT_STYLE_NORMAL,
            Self::Oblique => DWRITE_FONT_STYLE_OBLIQUE,
            Self::Italic => DWRITE_FONT_STYLE_ITALIC,
        }
    }

    fn from_abi(style: DWRITE_FONT_STYLE) -> Self {
        match style {
            DWRITE_FONT_STYLE_OBLIQUE => Self::Oblique,
            DWRITE_FONT_STYLE_ITALIC => Self::Italic,
            _ => Self::Normal,
        }
    }
}

/// A set of font families: the fonts installed on the system, or fonts loaded from memory.
///
/// Cloning is cheap and clones share the same collection.
#[derive(Clone)]
pub struct FontCollection(IDWriteFontCollection1);

// DirectWrite font collections are immutable and thread-safe.
unsafe impl Send for FontCollection {}
unsafe impl Sync for FontCollection {}

impl FontCollection {
    /// Returns the fonts installed on the system.
    pub fn system() -> Self {
        let mut collection = None;
        unsafe {
            factory()
                .GetSystemFontCollection(false, &mut collection, false)
                .unwrap();
        }
        Self(collection.unwrap())
    }

    /// Creates a collection from font file data, such as the contents of `.ttf`, `.otf`, or
    /// `.ttc` files. Fails if any font is invalid.
    ///
    /// The data is copied and kept until the process exits, so load each font once and reuse the
    /// collection.
    pub fn from_data<'a>(fonts: impl IntoIterator<Item = &'a [u8]>) -> Result<Self> {
        unsafe {
            let builder = factory().CreateFontSetBuilder()?;

            for data in fonts {
                let file = memory_loader().CreateInMemoryFontFileReference(
                    factory(),
                    data.as_ptr().cast(),
                    data.len().try_into().unwrap(),
                    None,
                )?;
                builder.AddFontFile(&file).ok()?;
            }

            let set = builder.CreateFontSet()?;
            factory().CreateFontCollectionFromFontSet(&set).map(Self)
        }
    }

    /// Returns the family names in the collection, in the user's locale where available.
    pub fn family_names(&self) -> Vec<String> {
        unsafe {
            (0..self.0.GetFontFamilyCount())
                .map(|index| {
                    localized(
                        &self
                            .0
                            .GetFontFamily(index)
                            .unwrap()
                            .GetFamilyNames()
                            .unwrap(),
                    )
                })
                .collect()
        }
    }

    /// Finds the face in `family` that best matches `weight` and `style`, or `None` if the
    /// collection has no such family. Family names match in any locale, ignoring case.
    pub fn find(&self, family: &str, weight: FontWeight, style: FontStyle) -> Option<FontFace> {
        unsafe {
            let mut index = 0;
            let mut exists = BOOL::default();
            self.0
                .FindFamilyName(wide(family).as_ptr(), &mut index, &mut exists)
                .unwrap();

            if !exists.as_bool() {
                return None;
            }

            let font = self
                .0
                .GetFontFamily(index)
                .unwrap()
                .GetFirstMatchingFont(weight.0.into(), DWRITE_FONT_STRETCH_NORMAL, style.to_abi())
                .unwrap();

            Some(FontFace(font.CreateFontFace().unwrap().cast().unwrap()))
        }
    }

    /// Casts the underlying `IDWriteFontCollection1` to `T`.
    pub fn to_interface<T: Interface>(&self) -> Result<T> {
        self.0.cast()
    }

    pub(crate) fn raw(&self) -> &IDWriteFontCollection1 {
        &self.0
    }
}

/// A single font face: one weight and style of a family.
///
/// Two faces are equal if they wrap the same DirectWrite font face object, so a face can key a
/// glyph or atlas cache. Cloning is cheap.
#[derive(Clone)]
pub struct FontFace(IDWriteFontFace3);

// DirectWrite font faces are immutable and thread-safe.
unsafe impl Send for FontFace {}
unsafe impl Sync for FontFace {}

impl FontFace {
    /// Returns the family name, in the user's locale where available.
    pub fn family_name(&self) -> String {
        unsafe { localized(&self.0.GetFamilyNames().unwrap()) }
    }

    /// Returns the weight.
    pub fn weight(&self) -> FontWeight {
        FontWeight(unsafe { self.0.GetWeight() }.try_into().unwrap())
    }

    /// Returns the style.
    pub fn style(&self) -> FontStyle {
        FontStyle::from_abi(unsafe { self.0.GetStyle() })
    }

    /// Returns `true` if the face has color glyphs, such as emoji.
    pub fn is_color_font(&self) -> bool {
        unsafe { self.0.IsColorFont().as_bool() }
    }

    /// Returns the face's metrics in design units.
    pub fn metrics(&self) -> FontMetrics {
        let mut m = DWRITE_FONT_METRICS1::default();
        unsafe { self.0.GetMetrics(&mut m) };
        FontMetrics {
            design_units_per_em: m.base.designUnitsPerEm,
            ascent: m.base.ascent,
            descent: m.base.descent,
            line_gap: m.base.lineGap,
            cap_height: m.base.capHeight,
            x_height: m.base.xHeight,
            underline_position: m.base.underlinePosition,
            underline_thickness: m.base.underlineThickness,
            strikethrough_position: m.base.strikethroughPosition,
            strikethrough_thickness: m.base.strikethroughThickness,
            glyph_box_left: m.glyphBoxLeft,
            glyph_box_top: m.glyphBoxTop,
            glyph_box_right: m.glyphBoxRight,
            glyph_box_bottom: m.glyphBoxBottom,
        }
    }

    /// Returns the glyph that the face's character map assigns to `ch`, or `None` if the face
    /// has no glyph for it.
    pub fn glyph_index(&self, ch: char) -> Option<u16> {
        let mut glyph = 0;
        unsafe {
            self.0.GetGlyphIndices(&(ch as u32), 1, &mut glyph).unwrap();
        }
        (glyph != 0).then_some(glyph)
    }

    /// Returns the metrics of `glyph` in design units.
    pub fn glyph_metrics(&self, glyph: u16) -> GlyphMetrics {
        let mut m = DWRITE_GLYPH_METRICS::default();
        unsafe {
            self.0
                .GetDesignGlyphMetrics(&glyph, 1, &mut m, false)
                .unwrap();
        }
        GlyphMetrics {
            left_side_bearing: m.leftSideBearing,
            advance_width: m.advanceWidth,
            right_side_bearing: m.rightSideBearing,
            top_side_bearing: m.topSideBearing,
            advance_height: m.advanceHeight,
            bottom_side_bearing: m.bottomSideBearing,
            vertical_origin_y: m.verticalOriginY,
        }
    }

    /// Casts the underlying `IDWriteFontFace3` to `T`.
    pub fn to_interface<T: Interface>(&self) -> Result<T> {
        self.0.cast()
    }
}

impl PartialEq for FontFace {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_raw() == other.0.as_raw()
    }
}

impl Eq for FontFace {}

impl Hash for FontFace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_raw().hash(state);
    }
}

impl std::fmt::Debug for FontFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FontFace")
            .field("family", &self.family_name())
            .field("weight", &self.weight())
            .field("style", &self.style())
            .finish()
    }
}

/// Font-wide metrics in design units. Divide by `design_units_per_em` and multiply by the font
/// size to convert to device-independent pixels.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct FontMetrics {
    /// Design units in one em.
    pub design_units_per_em: u16,
    /// Distance from the baseline to the top of the line.
    pub ascent: u16,
    /// Distance from the baseline to the bottom of the line.
    pub descent: u16,
    /// Recommended extra space between lines.
    pub line_gap: i16,
    /// Height of flat capital letters above the baseline.
    pub cap_height: u16,
    /// Height of flat lowercase letters above the baseline.
    pub x_height: u16,
    /// Position of the underline relative to the baseline; negative is below.
    pub underline_position: i16,
    /// Thickness of the underline.
    pub underline_thickness: u16,
    /// Position of the strikethrough relative to the baseline.
    pub strikethrough_position: i16,
    /// Thickness of the strikethrough.
    pub strikethrough_thickness: u16,
    /// Left edge of the box enclosing all glyphs.
    pub glyph_box_left: i16,
    /// Top edge of the box enclosing all glyphs, above the baseline.
    pub glyph_box_top: i16,
    /// Right edge of the box enclosing all glyphs.
    pub glyph_box_right: i16,
    /// Bottom edge of the box enclosing all glyphs; negative is below the baseline.
    pub glyph_box_bottom: i16,
}

/// Metrics of one glyph in design units.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct GlyphMetrics {
    /// Distance from the horizontal origin to the left edge of the glyph's black box.
    pub left_side_bearing: i32,
    /// Horizontal advance.
    pub advance_width: u32,
    /// Distance from the right edge of the black box to the advance.
    pub right_side_bearing: i32,
    /// Distance from the vertical origin to the top of the black box.
    pub top_side_bearing: i32,
    /// Vertical advance.
    pub advance_height: u32,
    /// Distance from the bottom of the black box to the vertical advance.
    pub bottom_side_bearing: i32,
    /// Vertical origin, measured upward from the baseline.
    pub vertical_origin_y: i32,
}
