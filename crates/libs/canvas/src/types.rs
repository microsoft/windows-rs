use super::*;

/// How a 32-bit BGRA bitmap's alpha channel is interpreted.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum AlphaMode {
    /// Color channels have already been multiplied by alpha.
    #[default]
    Premultiplied,
    /// Alpha is ignored and pixels are treated as opaque.
    Ignore,
}

impl AlphaMode {
    pub(crate) fn to_abi(self) -> D2D1_ALPHA_MODE {
        match self {
            Self::Premultiplied => D2D1_ALPHA_MODE_PREMULTIPLIED,
            Self::Ignore => D2D1_ALPHA_MODE_IGNORE,
        }
    }
}

/// A rectangle defined by its edges.
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

    pub(crate) fn to_abi(self) -> D2D_RECT_F {
        D2D_RECT_F {
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
    /// Creates an ellipse from a center point and radii.
    pub const fn new(center: Vector2, radius_x: f32, radius_y: f32) -> Self {
        Self {
            center,
            radius_x,
            radius_y,
        }
    }

    /// Creates a circle from a center point and radius.
    pub const fn circle(center: Vector2, radius: f32) -> Self {
        Self {
            center,
            radius_x: radius,
            radius_y: radius,
        }
    }

    pub(crate) fn to_abi(self) -> D2D1_ELLIPSE {
        D2D1_ELLIPSE {
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
    /// Creates a rounded rectangle from a rectangle and corner radii.
    pub const fn new(rect: Rect, radius_x: f32, radius_y: f32) -> Self {
        Self {
            rect,
            radius_x,
            radius_y,
        }
    }

    /// Creates a rounded rectangle with equal x and y corner radii.
    pub const fn uniform(rect: Rect, radius: f32) -> Self {
        Self {
            rect,
            radius_x: radius,
            radius_y: radius,
        }
    }

    pub(crate) fn to_abi(self) -> D2D1_ROUNDED_RECT {
        D2D1_ROUNDED_RECT {
            rect: self.rect.to_abi(),
            radiusX: self.radius_x,
            radiusY: self.radius_y,
        }
    }
}

mod sealed {
    pub trait Sealed {}
}

/// A brush accepted by drawing and filling operations.
///
/// This sealed trait is implemented by solid, linear-gradient, and radial-gradient brushes.
pub trait Paint: sealed::Sealed {
    #[doc(hidden)]
    fn as_raw_brush(&self) -> &ID2D1Brush;
}

/// A solid color brush.
pub struct Brush(pub(crate) ID2D1SolidColorBrush);

impl Brush {
    /// Changes the brush color.
    pub fn set_color(&self, color: ColorF) {
        let c: D2D_COLOR_F = color.into();
        unsafe { self.0.SetColor(&c) };
    }
}

impl sealed::Sealed for Brush {}
impl Paint for Brush {
    fn as_raw_brush(&self) -> &ID2D1Brush {
        &self.0
    }
}

/// A linear gradient brush.
pub struct LinearGradient(pub(crate) ID2D1LinearGradientBrush);

impl sealed::Sealed for LinearGradient {}
impl Paint for LinearGradient {
    fn as_raw_brush(&self) -> &ID2D1Brush {
        &self.0
    }
}

/// A radial gradient brush.
pub struct RadialGradient(pub(crate) ID2D1RadialGradientBrush);

impl sealed::Sealed for RadialGradient {}
impl Paint for RadialGradient {
    fn as_raw_brush(&self) -> &ID2D1Brush {
        &self.0
    }
}

/// A color stop along a gradient axis, where `position` is 0.0-1.0.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GradientStop {
    pub position: f32,
    pub color: ColorF,
}

impl GradientStop {
    /// Creates a gradient stop at the given position and color.
    pub const fn new(position: f32, color: ColorF) -> Self {
        Self { position, color }
    }

    pub(crate) fn to_abi(self) -> D2D1_GRADIENT_STOP {
        D2D1_GRADIENT_STOP {
            position: self.position,
            color: self.color.into(),
        }
    }
}

/// Cap style applied to the start and end of stroked lines.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum CapStyle {
    /// Ends at the endpoint without extending beyond it.
    #[default]
    Flat,
    /// Extends by half the stroke width with a square end.
    Square,
    /// Extends by half the stroke width with a round end.
    Round,
    /// Extends with a triangular end.
    Triangle,
}

impl CapStyle {
    pub(crate) fn to_abi(self) -> D2D1_CAP_STYLE {
        match self {
            Self::Flat => D2D1_CAP_STYLE_FLAT,
            Self::Square => D2D1_CAP_STYLE_SQUARE,
            Self::Round => D2D1_CAP_STYLE_ROUND,
            Self::Triangle => D2D1_CAP_STYLE_TRIANGLE,
        }
    }
}

/// Line join style for connected segments.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum LineJoin {
    /// Extends edges until they meet, subject to the miter limit.
    #[default]
    Miter,
    /// Cuts the corner between segments.
    Bevel,
    /// Rounds the corner between segments.
    Round,
}

impl LineJoin {
    pub(crate) fn to_abi(self) -> D2D1_LINE_JOIN {
        match self {
            Self::Miter => D2D1_LINE_JOIN_MITER,
            Self::Bevel => D2D1_LINE_JOIN_BEVEL,
            Self::Round => D2D1_LINE_JOIN_ROUND,
        }
    }
}

/// Dash pattern for stroked lines.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum DashStyle {
    /// A continuous stroke.
    #[default]
    Solid,
    /// A repeating dash pattern.
    Dash,
    /// A repeating dot pattern.
    Dot,
    /// Alternating dashes and dots.
    DashDot,
}

impl DashStyle {
    pub(crate) fn to_abi(self) -> D2D1_DASH_STYLE {
        match self {
            Self::Solid => D2D1_DASH_STYLE_SOLID,
            Self::Dash => D2D1_DASH_STYLE_DASH,
            Self::Dot => D2D1_DASH_STYLE_DOT,
            Self::DashDot => D2D1_DASH_STYLE_DASH_DOT,
        }
    }
}

/// Describes how to stroke lines, outlines, and paths.
///
/// Create via [`StrokeStyleBuilder`] and [`GpuDevice::create_stroke_style`].
#[derive(Clone)]
pub struct StrokeStyle(pub(crate) ID2D1StrokeStyle1);

/// Builder for [`StrokeStyle`].
///
/// The default is a solid miter-joined stroke with flat caps, zero dash offset, and a miter limit
/// of 10.
#[derive(Clone, Debug, Default)]
pub struct StrokeStyleBuilder {
    pub(crate) start_cap: CapStyle,
    pub(crate) end_cap: CapStyle,
    pub(crate) dash_cap: CapStyle,
    pub(crate) line_join: LineJoin,
    pub(crate) miter_limit: f32,
    pub(crate) dash_style: DashStyle,
    pub(crate) dash_offset: f32,
}

impl StrokeStyleBuilder {
    /// Creates a builder with the default Direct2D stroke settings.
    pub fn new() -> Self {
        Self {
            miter_limit: 10.0,
            ..Default::default()
        }
    }

    /// Sets the cap at the start of an open stroke.
    pub fn start_cap(mut self, cap: CapStyle) -> Self {
        self.start_cap = cap;
        self
    }

    /// Sets the cap at the end of an open stroke.
    pub fn end_cap(mut self, cap: CapStyle) -> Self {
        self.end_cap = cap;
        self
    }

    /// Sets both the start and end caps.
    pub fn caps(mut self, cap: CapStyle) -> Self {
        self.start_cap = cap;
        self.end_cap = cap;
        self
    }

    /// Sets the cap used at the ends of each dash.
    pub fn dash_cap(mut self, cap: CapStyle) -> Self {
        self.dash_cap = cap;
        self
    }

    /// Sets how connected segments are joined.
    pub fn line_join(mut self, join: LineJoin) -> Self {
        self.line_join = join;
        self
    }

    /// Sets the limit on the ratio of miter length to stroke width.
    pub fn miter_limit(mut self, limit: f32) -> Self {
        self.miter_limit = limit;
        self
    }

    /// Sets the stroke's dash pattern.
    pub fn dash_style(mut self, style: DashStyle) -> Self {
        self.dash_style = style;
        self
    }

    /// Sets how far into the dash pattern the line starts.
    pub fn dash_offset(mut self, offset: f32) -> Self {
        self.dash_offset = offset;
        self
    }

    pub(crate) fn to_abi(&self) -> D2D1_STROKE_STYLE_PROPERTIES1 {
        D2D1_STROKE_STYLE_PROPERTIES1 {
            startCap: self.start_cap.to_abi(),
            endCap: self.end_cap.to_abi(),
            dashCap: self.dash_cap.to_abi(),
            lineJoin: self.line_join.to_abi(),
            miterLimit: self.miter_limit,
            dashStyle: self.dash_style.to_abi(),
            dashOffset: self.dash_offset,
            transformType: 0,
        }
    }
}
