/// A rectangle defined by its edges, in device-independent pixels.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    /// Creates a rectangle from its edge positions.
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Creates a rectangle from a position and size.
    pub const fn from_xywh(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            left: x,
            top: y,
            right: x + width,
            bottom: y + height,
        }
    }

    /// Returns the width of the rectangle.
    pub const fn width(&self) -> f32 {
        self.right - self.left
    }

    /// Returns the height of the rectangle.
    pub const fn height(&self) -> f32 {
        self.bottom - self.top
    }

    /// Returns a copy of the rectangle translated by `dx` and `dy`.
    pub const fn offset(&self, dx: f32, dy: f32) -> Self {
        Self {
            left: self.left + dx,
            top: self.top + dy,
            right: self.right + dx,
            bottom: self.bottom + dy,
        }
    }
}

/// Font weight on the OpenType scale from 1 to 999.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FontWeight(pub u16);

impl FontWeight {
    /// Thin weight, 100.
    pub const THIN: Self = Self(100);
    /// Extra-light weight, 200.
    pub const EXTRA_LIGHT: Self = Self(200);
    /// Light weight, 300.
    pub const LIGHT: Self = Self(300);
    /// Semi-light weight, 350.
    pub const SEMI_LIGHT: Self = Self(350);
    /// Normal (regular) weight, 400.
    pub const NORMAL: Self = Self(400);
    /// Medium weight, 500.
    pub const MEDIUM: Self = Self(500);
    /// Semi-bold weight, 600.
    pub const SEMI_BOLD: Self = Self(600);
    /// Bold weight, 700.
    pub const BOLD: Self = Self(700);
    /// Extra-bold weight, 800.
    pub const EXTRA_BOLD: Self = Self(800);
    /// Black (heavy) weight, 900.
    pub const BLACK: Self = Self(900);
    /// Extra-black weight, 950.
    pub const EXTRA_BLACK: Self = Self(950);
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::NORMAL
    }
}
