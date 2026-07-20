use super::*;

/// A 32-bit sRGB color with premultiplied alpha semantics, matching the
/// composition `Color` ABI (`Windows.UI.Color`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color(pub(crate) bindings::Color);

impl Color {
    /// Creates an opaque color from red, green, and blue components.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(bindings::Color { a: 255, r, g, b })
    }

    /// Creates a color from red, green, blue, and alpha components.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(bindings::Color { a, r, g, b })
    }
}
