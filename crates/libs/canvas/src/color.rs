use super::*;

/// RGBA color with f32 components.
#[doc(alias = "Color")]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct ColorF {
    /// Red component, in the range 0.0-1.0.
    pub r: f32,
    /// Green component, in the range 0.0-1.0.
    pub g: f32,
    /// Blue component, in the range 0.0-1.0.
    pub b: f32,
    /// Alpha (opacity) component, in the range 0.0-1.0.
    pub a: f32,
}

impl ColorF {
    /// Creates a color from red, green, blue, and alpha components.
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates an opaque color from red, green, and blue components.
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a color from 8-bit red, green, blue, and alpha components.
    pub const fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Creates an opaque color from 8-bit red, green, and blue components.
    pub const fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }

    /// Fully transparent black.
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    /// Opaque black.
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    /// Opaque white.
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    /// Opaque red.
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    /// Opaque green.
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    /// Opaque blue.
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    /// Opaque cornflower blue.
    pub const CORNFLOWER_BLUE: Self = Self::rgb(0.392, 0.584, 0.929);
    /// Opaque dark slate blue.
    pub const DARK_SLATE_BLUE: Self = Self::rgb(0.05, 0.05, 0.1);
}

impl From<ColorF> for D2D_COLOR_F {
    fn from(c: ColorF) -> Self {
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}
