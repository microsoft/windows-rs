use super::*;

/// A completed path geometry.
///
/// ```ignore
/// let path = PathBuilder::new(&device)?
///     .begin(Vector2::new(0.0, 0.0))
///     .line_to(Vector2::new(100.0, 0.0))
///     .line_to(Vector2::new(50.0, 80.0))
///     .close()
///     .build()?;
/// ```
#[derive(Clone)]
pub struct Path {
    raw: ID2D1PathGeometry1,
}

/// Direct2D's default flattening tolerance for hit-testing and bounds queries.
const DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25;

impl Path {
    /// Returns the underlying `ID2D1PathGeometry1`.
    pub fn raw(&self) -> &ID2D1PathGeometry1 {
        &self.raw
    }

    /// Returns whether the point lies within the filled area of the path.
    pub fn fill_contains_point(&self, point: Vector2) -> bool {
        unsafe {
            self.raw
                .FillContainsPoint(point, None, DEFAULT_FLATTENING_TOLERANCE)
                .unwrap()
                .as_bool()
        }
    }

    /// Returns whether the point lies on the path's stroke at the given width.
    pub fn stroke_contains_point(&self, point: Vector2, stroke_width: f32) -> bool {
        unsafe {
            self.raw
                .StrokeContainsPoint(
                    point,
                    stroke_width,
                    None,
                    None,
                    DEFAULT_FLATTENING_TOLERANCE,
                )
                .unwrap()
                .as_bool()
        }
    }

    /// Returns the axis-aligned bounding rectangle of the path.
    pub fn compute_bounds(&self) -> Rect {
        let bounds = unsafe { self.raw.GetBounds(None).unwrap() };
        Rect {
            left: bounds.left,
            top: bounds.top,
            right: bounds.right,
            bottom: bounds.bottom,
        }
    }
}

/// Type-safe path builder.
///
/// ```ignore
/// let path = PathBuilder::new(&device)?
///     .begin(point)
///     .line_to(point2)
///     .bezier_to(c1, c2, end)
///     .close()
///     .build()?;
/// ```
pub struct PathBuilder {
    sink: ID2D1GeometrySink,
    geometry: ID2D1PathGeometry1,
}

impl PathBuilder {
    /// Creates a new path builder for the given device.
    pub fn new(device: &GpuDevice) -> Result<Self> {
        let geometry = unsafe { device.d2d_factory().CreatePathGeometry()? };
        let sink = unsafe { geometry.Open()? };
        Ok(Self { sink, geometry })
    }

    /// Begin a filled figure.
    pub fn begin(self, start: Vector2) -> PathFigure {
        unsafe {
            self.sink.BeginFigure(start, D2D1_FIGURE_BEGIN_FILLED);
        }
        PathFigure {
            sink: self.sink,
            geometry: self.geometry,
        }
    }

    /// Begin a hollow (stroke-only) figure.
    pub fn begin_hollow(self, start: Vector2) -> PathFigure {
        unsafe {
            self.sink.BeginFigure(start, D2D1_FIGURE_BEGIN_HOLLOW);
        }
        PathFigure {
            sink: self.sink,
            geometry: self.geometry,
        }
    }

    /// Finalize the path geometry.
    pub fn build(self) -> Result<Path> {
        unsafe { self.sink.Close().ok()? };
        Ok(Path { raw: self.geometry })
    }

    /// Builds a closed, filled polygon from a sequence of points.
    ///
    /// Convenience for `begin(first).line_to(rest)...close().build()`. Returns an
    /// error if `points` yields no points.
    pub fn polygon(self, points: impl IntoIterator<Item = Vector2>) -> Result<Path> {
        let mut points = points.into_iter();
        let Some(first) = points.next() else {
            return Err(Error::empty());
        };
        let mut figure = self.begin(first);
        for point in points {
            figure = figure.line_to(point);
        }
        figure.close().build()
    }
}

/// A figure within a path being built.
///
/// Returned by [`PathBuilder::begin`]. Add segments with [`line_to`](Self::line_to)
/// and [`bezier_to`](Self::bezier_to), then call [`close`](Self::close) or
/// [`end_open`](Self::end_open) to return to `PathBuilder`.
pub struct PathFigure {
    sink: ID2D1GeometrySink,
    geometry: ID2D1PathGeometry1,
}

impl PathFigure {
    /// Adds a straight line segment to the given point.
    pub fn line_to(self, point: Vector2) -> Self {
        unsafe { self.sink.AddLine(point) };
        self
    }

    /// Adds a cubic Bezier segment with the given control points and end point.
    pub fn bezier_to(self, control1: Vector2, control2: Vector2, end: Vector2) -> Self {
        let segment = D2D1_BEZIER_SEGMENT {
            point1: control1,
            point2: control2,
            point3: end,
        };
        unsafe { self.sink.AddBezier(&segment) };
        self
    }

    /// Close the current figure and connect back to the start point.
    pub fn close(self) -> PathBuilder {
        unsafe { self.sink.EndFigure(D2D1_FIGURE_END_CLOSED) };
        PathBuilder {
            sink: self.sink,
            geometry: self.geometry,
        }
    }

    /// End the current figure without closing.
    pub fn end_open(self) -> PathBuilder {
        unsafe { self.sink.EndFigure(D2D1_FIGURE_END_OPEN) };
        PathBuilder {
            sink: self.sink,
            geometry: self.geometry,
        }
    }
}
