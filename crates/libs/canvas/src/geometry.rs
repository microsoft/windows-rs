use super::*;

/// A completed path geometry.
#[derive(Clone)]
pub struct Path {
    raw: ID2D1PathGeometry1,
}

/// Direct2D's default flattening tolerance for hit-testing and bounds queries.
const DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25;

impl Path {
    /// Returns the underlying Direct2D path geometry for interop.
    pub fn raw(&self) -> &ID2D1PathGeometry1 {
        &self.raw
    }

    /// Tests whether `point` lies inside the path's fill.
    pub fn fill_contains_point(&self, point: Vector2) -> bool {
        unsafe {
            self.raw
                .FillContainsPoint(point, None, DEFAULT_FLATTENING_TOLERANCE)
                .unwrap()
                .as_bool()
        }
    }

    /// Tests whether `point` lies within a stroke of the given DIP width.
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

    /// Returns the path bounds in DIPs.
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

/// Builds a path through a sequence of figures.
///
/// Start a figure with [`begin`](Self::begin) or [`begin_hollow`](Self::begin_hollow), add
/// segments, then close or end the figure. Call [`build`](Self::build) when no figure is active.
pub struct PathBuilder {
    sink: ID2D1GeometrySink,
    geometry: ID2D1PathGeometry1,
}

impl PathBuilder {
    /// Creates an empty path builder.
    pub fn new(device: &GpuDevice) -> Result<Self> {
        let geometry = unsafe { device.d2d_factory().CreatePathGeometry()? };
        let sink = unsafe { geometry.Open()? };
        Ok(Self { sink, geometry })
    }

    /// Starts a filled figure at `start`.
    pub fn begin(self, start: Vector2) -> PathFigure {
        unsafe {
            self.sink.BeginFigure(start, D2D1_FIGURE_BEGIN_FILLED);
        }
        PathFigure {
            sink: self.sink,
            geometry: self.geometry,
        }
    }

    /// Starts a hollow figure at `start`.
    pub fn begin_hollow(self, start: Vector2) -> PathFigure {
        unsafe {
            self.sink.BeginFigure(start, D2D1_FIGURE_BEGIN_HOLLOW);
        }
        PathFigure {
            sink: self.sink,
            geometry: self.geometry,
        }
    }

    /// Finishes a path when no figure is active.
    pub fn build(self) -> Result<Path> {
        unsafe { self.sink.Close().ok()? };
        Ok(Path { raw: self.geometry })
    }

    /// Builds a closed, filled polygon from a sequence of points.
    ///
    /// Returns an error if `points` is empty.
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
pub struct PathFigure {
    sink: ID2D1GeometrySink,
    geometry: ID2D1PathGeometry1,
}

impl PathFigure {
    /// Adds a straight segment to `point`.
    pub fn line_to(self, point: Vector2) -> Self {
        unsafe { self.sink.AddLine(point) };
        self
    }

    /// Adds a cubic Bezier segment.
    pub fn bezier_to(self, control1: Vector2, control2: Vector2, end: Vector2) -> Self {
        let segment = D2D1_BEZIER_SEGMENT {
            point1: control1,
            point2: control2,
            point3: end,
        };
        unsafe { self.sink.AddBezier(&segment) };
        self
    }

    /// Closes the current figure and connects it to its start point.
    pub fn close(self) -> PathBuilder {
        unsafe { self.sink.EndFigure(D2D1_FIGURE_END_CLOSED) };
        PathBuilder {
            sink: self.sink,
            geometry: self.geometry,
        }
    }

    /// Ends the current figure without closing it.
    pub fn end_open(self) -> PathBuilder {
        unsafe { self.sink.EndFigure(D2D1_FIGURE_END_OPEN) };
        PathBuilder {
            sink: self.sink,
            geometry: self.geometry,
        }
    }
}
