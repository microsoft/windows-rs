/// A rectangle defined by its left, top, right, and bottom edges.
///
/// The layout matches `D2D_RECT_F`, which Direct2D and DirectWrite use for rectangles.
/// It differs from `Windows.Foundation.Rect`, which stores a position and size.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Rect {
    /// The x-coordinate of the left edge.
    pub left: f32,
    /// The y-coordinate of the top edge.
    pub top: f32,
    /// The x-coordinate of the right edge.
    pub right: f32,
    /// The y-coordinate of the bottom edge.
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
