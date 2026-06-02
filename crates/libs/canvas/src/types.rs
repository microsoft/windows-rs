use crate::bindings;
use crate::color::Color;
use windows_numerics::Vector2;

/// A rectangle defined by its edges.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    /// Create a rectangle from edge coordinates.
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Create a rectangle from position and size.
    pub const fn from_xywh(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            left: x,
            top: y,
            right: x + width,
            bottom: y + height,
        }
    }

    /// Width of the rectangle.
    pub const fn width(&self) -> f32 {
        self.right - self.left
    }

    /// Height of the rectangle.
    pub const fn height(&self) -> f32 {
        self.bottom - self.top
    }

    pub(crate) fn to_abi(self) -> bindings::D2D_RECT_F {
        bindings::D2D_RECT_F {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }
}

/// An ellipse defined by center point and radii.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Ellipse {
    pub center: Vector2,
    pub radius_x: f32,
    pub radius_y: f32,
}

impl Ellipse {
    /// Create an ellipse from center and radii.
    pub const fn new(center: Vector2, radius_x: f32, radius_y: f32) -> Self {
        Self {
            center,
            radius_x,
            radius_y,
        }
    }

    /// Create a circle from center and radius.
    pub const fn circle(center: Vector2, radius: f32) -> Self {
        Self {
            center,
            radius_x: radius,
            radius_y: radius,
        }
    }

    pub(crate) fn to_abi(self) -> bindings::D2D1_ELLIPSE {
        bindings::D2D1_ELLIPSE {
            point: self.center,
            radiusX: self.radius_x,
            radiusY: self.radius_y,
        }
    }
}

/// A rectangle with rounded corners.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct RoundedRect {
    pub rect: Rect,
    pub radius_x: f32,
    pub radius_y: f32,
}

impl RoundedRect {
    /// Create a rounded rectangle.
    pub const fn new(rect: Rect, radius_x: f32, radius_y: f32) -> Self {
        Self {
            rect,
            radius_x,
            radius_y,
        }
    }

    /// Create a rounded rectangle with uniform corner radius.
    pub const fn uniform(rect: Rect, radius: f32) -> Self {
        Self {
            rect,
            radius_x: radius,
            radius_y: radius,
        }
    }

    pub(crate) fn to_abi(self) -> bindings::D2D1_ROUNDED_RECT {
        bindings::D2D1_ROUNDED_RECT {
            rect: self.rect.to_abi(),
            radiusX: self.radius_x,
            radiusY: self.radius_y,
        }
    }
}

// --- Brush types ---

mod sealed {
    pub trait Sealed {}
}

/// Trait for types that can be used as brushes in draw calls.
///
/// Implemented by [`Brush`], [`LinearGradient`], and [`RadialGradient`].
/// Sealed — cannot be implemented outside this crate.
pub trait Paint: sealed::Sealed {
    #[doc(hidden)]
    fn as_raw_brush(&self) -> &bindings::ID2D1Brush;
}

/// A solid color brush.
///
/// Cheap to create per-frame, or cache and reuse across frames by
/// changing the color with [`Brush::set_color`].
pub struct Brush(pub(crate) bindings::ID2D1SolidColorBrush);

impl Brush {
    /// Change the brush color (zero-cost — no reallocation).
    pub fn set_color(&self, color: Color) {
        let c: bindings::D2D1_COLOR_F = color.into();
        unsafe { self.0.SetColor(&c) };
    }
}

impl sealed::Sealed for Brush {}
impl Paint for Brush {
    fn as_raw_brush(&self) -> &bindings::ID2D1Brush {
        // ID2D1SolidColorBrush Derefs to ID2D1Brush (COM hierarchy, zero-cost)
        &self.0
    }
}

/// A linear gradient brush with a start point, end point, and color stops.
pub struct LinearGradient(pub(crate) bindings::ID2D1LinearGradientBrush);

impl sealed::Sealed for LinearGradient {}
impl Paint for LinearGradient {
    fn as_raw_brush(&self) -> &bindings::ID2D1Brush {
        &self.0
    }
}

/// A radial gradient brush with a center, radii, and color stops.
pub struct RadialGradient(pub(crate) bindings::ID2D1RadialGradientBrush);

impl sealed::Sealed for RadialGradient {}
impl Paint for RadialGradient {
    fn as_raw_brush(&self) -> &bindings::ID2D1Brush {
        &self.0
    }
}

/// A color stop in a gradient (position 0.0–1.0 along the gradient axis).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GradientStop {
    pub position: f32,
    pub color: Color,
}

impl GradientStop {
    pub const fn new(position: f32, color: Color) -> Self {
        Self { position, color }
    }

    pub(crate) fn to_abi(self) -> bindings::D2D1_GRADIENT_STOP {
        bindings::D2D1_GRADIENT_STOP {
            position: self.position,
            color: self.color.into(),
        }
    }
}
