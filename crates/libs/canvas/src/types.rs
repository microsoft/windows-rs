use super::*;

/// How the alpha channel of a bitmap's pixels is interpreted.
///
/// Only the two modes supported by Direct2D for 32-bit BGRA bitmaps are exposed.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum AlphaMode {
    /// Color components are premultiplied by the alpha value. This is the most
    /// common mode for GPU compositing and the one produced by most drawing.
    #[default]
    Premultiplied,
    /// The alpha channel is ignored and the bitmap is treated as opaque.
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
    /// Position of the left edge.
    pub left: f32,
    /// Position of the top edge.
    pub top: f32,
    /// Position of the right edge.
    pub right: f32,
    /// Position of the bottom edge.
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
    /// Center point of the ellipse.
    pub center: Vector2,
    /// Radius along the x-axis.
    pub radius_x: f32,
    /// Radius along the y-axis.
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
    /// The base rectangle.
    pub rect: Rect,
    /// Corner radius along the x-axis.
    pub radius_x: f32,
    /// Corner radius along the y-axis.
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

/// Trait for types usable as brushes in draw calls. Sealed.
pub trait Paint: sealed::Sealed {
    #[doc(hidden)]
    fn as_raw_brush(&self) -> &ID2D1Brush;
}

/// A solid color brush.
pub struct Brush(pub(crate) ID2D1SolidColorBrush);

impl Brush {
    /// Sets the color of the brush.
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

/// A gradient stop (position 0.0–1.0 along the gradient axis).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GradientStop {
    /// Position along the gradient axis, from 0.0 to 1.0.
    pub position: f32,
    /// Color at this stop.
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
    /// Flat cap flush with the line end.
    #[default]
    Flat,
    /// Square cap extending half the line width past the end.
    Square,
    /// Rounded cap centered on the line end.
    Round,
    /// Triangular cap pointing past the line end.
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
    /// Sharp mitered corner.
    #[default]
    Miter,
    /// Beveled (flattened) corner.
    Bevel,
    /// Rounded corner.
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
    /// Solid, unbroken line.
    #[default]
    Solid,
    /// Repeating dashes.
    Dash,
    /// Repeating dots.
    Dot,
    /// Repeating dash-dot pattern.
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
/// ```ignore
/// let style = device.create_stroke_style(
///     StrokeStyleBuilder::new()
///         .caps(CapStyle::Round)
///         .line_join(LineJoin::Round)
/// )?;
/// ```
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
    /// Creates a new builder with default stroke properties.
    pub fn new() -> Self {
        Self {
            miter_limit: 10.0,
            ..Default::default()
        }
    }

    /// Sets the cap style for the start of the line.
    pub fn start_cap(mut self, cap: CapStyle) -> Self {
        self.start_cap = cap;
        self
    }

    /// Sets the cap style for the end of the line.
    pub fn end_cap(mut self, cap: CapStyle) -> Self {
        self.end_cap = cap;
        self
    }

    /// Sets both the start and end cap styles.
    pub fn caps(mut self, cap: CapStyle) -> Self {
        self.start_cap = cap;
        self.end_cap = cap;
        self
    }

    /// Sets the cap style applied to the ends of each dash.
    pub fn dash_cap(mut self, cap: CapStyle) -> Self {
        self.dash_cap = cap;
        self
    }

    /// Sets the join style for connected segments.
    pub fn line_join(mut self, join: LineJoin) -> Self {
        self.line_join = join;
        self
    }

    /// Sets the limit on the ratio of miter length to stroke width.
    pub fn miter_limit(mut self, limit: f32) -> Self {
        self.miter_limit = limit;
        self
    }

    /// Sets the dash pattern.
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
