/// Cross-axis spacing (margins, padding, etc.) — `f64` per side.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Thickness {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Thickness {
    pub const fn uniform(v: f64) -> Self {
        Self {
            left: v,
            top: v,
            right: v,
            bottom: v,
        }
    }
    pub const fn xy(x: f64, y: f64) -> Self {
        Self {
            left: x,
            top: y,
            right: x,
            bottom: y,
        }
    }
}

impl From<f64> for Thickness {
    fn from(v: f64) -> Self {
        Self::uniform(v)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { a: 255, r, g, b }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Brush {
    Solid(Color),
}

impl From<Color> for Brush {
    fn from(c: Color) -> Self {
        Self::Solid(c)
    }
}

pub use crate::bindings::HorizontalAlignment;
pub use crate::bindings::ScrollBarVisibility;
pub use crate::bindings::VerticalAlignment;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GridLength {
    Auto,
    Pixel(f64),
    Star(f64),
}

impl GridLength {
    pub const STAR: Self = GridLength::Star(1.0);
}
