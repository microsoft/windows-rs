use super::*;

/// An icon displayed by controls that accept a WinUI `IconElement` - buttons,
/// [`NavViewItem`]s, command-bar buttons, and [`SelectorBarItemDef`]s.
///
/// Construct one from a built-in [`Symbol`], an [`ImageSource`], or a font glyph.
/// A bare [`Symbol`] converts into an `Icon` automatically (`impl Into<Icon>`),
/// so `.icon(Symbol::Home)` keeps working alongside `.icon(Icon::image(...))`.
#[derive(Clone, Debug, PartialEq)]
pub enum Icon {
    /// A built-in system glyph from the [`Symbol`] enum (WinUI `SymbolIcon`).
    Symbol(Symbol),
    /// An image rendered in full color using the source's native format.
    Image(ImageSource),
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

    /// An image icon loaded from a URI, surface, or other [`ImageSource`].
    pub fn image(source: impl Into<ImageSource>) -> Self {
        Self::Image(source.into())
    }

    /// A raster image loaded from a URI.
    ///
    /// This is a compatibility shorthand for [`Icon::image`].
    pub fn bitmap(uri: impl Into<String>) -> Self {
        Self::image(ImageSource::uri(uri))
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

impl From<ImageSource> for Icon {
    fn from(source: ImageSource) -> Self {
        Self::Image(source)
    }
}
