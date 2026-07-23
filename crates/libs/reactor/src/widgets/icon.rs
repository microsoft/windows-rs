use super::*;

/// An icon displayed by controls that accept a WinUI `IconElement` — buttons,
/// [`NavViewItem`]s, command-bar buttons, and [`SelectorBarItemDef`]s.
///
/// Construct one from a built-in [`Symbol`], an image URI, or a font glyph.
/// A bare [`Symbol`] converts into an `Icon` automatically (`impl Into<Icon>`),
/// so `.icon(Symbol::Home)` keeps working alongside `.icon(Icon::bitmap(...))`.
#[derive(Clone, Debug, PartialEq)]
pub enum Icon {
    /// A built-in system glyph from the [`Symbol`] enum (WinUI `SymbolIcon`).
    Symbol(Symbol),
    /// A raster image loaded from a URI — PNG, JPG, or other bitmap formats
    /// (WinUI `BitmapIcon`). Note that `BitmapIcon` does not render SVG; use an
    /// `ImageIcon` with an `SvgImageSource` for vector artwork.
    ///
    /// The URI may be an app package path (`ms-appx:///Assets/logo.png`) or an
    /// absolute `http(s)` URL. Rendered in full color (not tinted monochrome).
    Bitmap {
        /// The image URI.
        uri: String,
    },
    /// A glyph from a font (WinUI `FontIcon`). When `family` is `None`, the
    /// control's default icon font is used.
    Font {
        /// The glyph to display, e.g. `"\u{E790}"`.
        glyph: String,
        /// The font family to select the glyph from, e.g. `"Segoe Fluent Icons"`.
        family: Option<String>,
    },
}

impl Icon {
    /// A built-in [`Symbol`] system icon.
    pub fn symbol(symbol: Symbol) -> Self {
        Self::Symbol(symbol)
    }

    /// A bitmap icon loaded from a URI (e.g. `ms-appx:///Assets/logo.png` or an
    /// `https://` URL).
    pub fn bitmap(uri: impl Into<String>) -> Self {
        Self::Bitmap { uri: uri.into() }
    }

    /// A font glyph rendered with the control's default icon font.
    pub fn font(glyph: impl Into<String>) -> Self {
        Self::Font {
            glyph: glyph.into(),
            family: None,
        }
    }

    /// A font glyph rendered with a specific font family.
    pub fn font_family(glyph: impl Into<String>, family: impl Into<String>) -> Self {
        Self::Font {
            glyph: glyph.into(),
            family: Some(family.into()),
        }
    }
}

impl From<Symbol> for Icon {
    fn from(symbol: Symbol) -> Self {
        Self::Symbol(symbol)
    }
}
