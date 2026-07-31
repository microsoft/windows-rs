use super::*;

/// A completed path geometry.
#[derive(Clone)]
pub struct Path {
    raw: ID2D1PathGeometry1,
}

/// Direct2D's default flattening tolerance for hit-testing and bounds queries.
const DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25;

impl Path {
    pub fn raw(&self) -> &ID2D1PathGeometry1 {
        &self.raw
    }

    pub fn fill_contains_point(&self, point: Vector2) -> bool {
        unsafe {
            self.raw
                .FillContainsPoint(point, None, DEFAULT_FLATTENING_TOLERANCE)
                .unwrap()
                .as_bool()
        }
    }

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
pub struct PathBuilder {
    sink: ID2D1GeometrySink,
    geometry: ID2D1PathGeometry1,
}

impl PathBuilder {
    pub fn new(device: &GpuDevice) -> Result<Self> {
        let geometry = unsafe { device.d2d_factory().CreatePathGeometry()? };
        let sink = unsafe { geometry.Open()? };
        Ok(Self { sink, geometry })
    }

    pub fn begin(self, start: Vector2) -> PathFigure {
        unsafe {
            self.sink.BeginFigure(start, D2D1_FIGURE_BEGIN_FILLED);
        }
        PathFigure {
            sink: self.sink,
            geometry: self.geometry,
        }
    }

    pub fn begin_hollow(self, start: Vector2) -> PathFigure {
        unsafe {
            self.sink.BeginFigure(start, D2D1_FIGURE_BEGIN_HOLLOW);
        }
        PathFigure {
            sink: self.sink,
            geometry: self.geometry,
        }
    }

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
    pub fn line_to(self, point: Vector2) -> Self {
        unsafe { self.sink.AddLine(point) };
        self
    }

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
