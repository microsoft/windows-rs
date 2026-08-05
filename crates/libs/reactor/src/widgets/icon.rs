use super::*;

/// An icon displayed by controls that accept a WinUI `IconElement` - buttons,
/// [`NavViewItem`]s, command-bar buttons, and [`SelectorBarItemDef`]s.
///
/// Construct one from a built-in [`Symbol`], an [`ImageSource`], a bitmap mask,
/// a font glyph, or vector path data. A bare [`Symbol`] converts into an `Icon`
/// automatically (`impl Into<Icon>`), so `.icon(Symbol::Home)` keeps working
/// alongside `.icon(Icon::image(...))`.
#[derive(Clone, Debug, PartialEq)]
pub enum Icon {
    /// A built-in system glyph from the [`Symbol`] enum (WinUI `SymbolIcon`).
    Symbol(Symbol),
    /// An image rendered in full color using the source's native format.
    Image(ImageSource),
    /// A URI image rendered by WinUI `BitmapIcon`.
    Bitmap {
        /// The image URI.
        uri: String,
        /// Whether WinUI replaces non-transparent pixels with the icon foreground.
        show_as_monochrome: bool,
    },
    /// A glyph from a font (WinUI `FontIcon`). When `family` is `None`, the
    /// control's default icon font is used.
    Font {
        /// The glyph to display, e.g. `"\u{E790}"`.
        glyph: String,
        /// The font family to select the glyph from, e.g. `"Segoe Fluent Icons"`.
        family: Option<String>,
    },
    /// XAML path mini-language data rendered by WinUI `PathIcon`.
    Path(String),
}

impl Icon {
    /// A built-in [`Symbol`] system icon.
    pub fn symbol(symbol: Symbol) -> Self {
        Self::Symbol(symbol)
    }

    /// An image icon loaded from a URI, surface, or other [`ImageSource`].
    pub fn image(source: impl Into<ImageSource>) -> Self {
        Self::Image(source.into())
    }

    /// A native WinUI `BitmapIcon` loaded from a URI.
    ///
    /// Set `show_as_monochrome` to `true` for a foreground-tinted mask or
    /// `false` to preserve the bitmap's colors. Use [`Icon::image`] for SVG
    /// sources, surfaces, and full-color images that do not need `BitmapIcon`
    /// behavior.
    pub fn bitmap_icon(uri: impl Into<String>, show_as_monochrome: bool) -> Self {
        Self::Bitmap {
            uri: uri.into(),
            show_as_monochrome,
        }
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

    /// A vector icon described with the XAML path mini-language.
    ///
    /// WinUI parses the path data when the native icon is created.
    pub fn path(data: impl Into<String>) -> Self {
        Self::Path(data.into())
    }
}

impl From<Symbol> for Icon {
    fn from(symbol: Symbol) -> Self {
        Self::Symbol(symbol)
    }
}

impl From<ImageSource> for Icon {
    fn from(source: ImageSource) -> Self {
        Self::Image(source)
    }
}
