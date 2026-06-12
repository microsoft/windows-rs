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

impl Path {
    pub fn raw(&self) -> &ID2D1PathGeometry1 {
        &self.raw
    }
}

// -- Typestate markers --

#[doc(hidden)]
pub struct Empty {
    sink: ID2D1GeometrySink,
    geometry: ID2D1PathGeometry1,
}

#[doc(hidden)]
pub struct InFigure {
    sink: ID2D1GeometrySink,
    geometry: ID2D1PathGeometry1,
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
pub struct PathBuilder<S> {
    state: S,
}

impl PathBuilder<Empty> {
    pub fn new(device: &GpuDevice) -> Result<Self> {
        let geometry = unsafe { device.d2d_factory().CreatePathGeometry()? };
        let sink = unsafe { geometry.Open()? };
        Ok(Self {
            state: Empty { sink, geometry },
        })
    }

    /// Begin a filled figure.
    pub fn begin(self, start: Vector2) -> PathBuilder<InFigure> {
        unsafe {
            self.state.sink.BeginFigure(start, D2D1_FIGURE_BEGIN_FILLED);
        }
        PathBuilder {
            state: InFigure {
                sink: self.state.sink,
                geometry: self.state.geometry,
            },
        }
    }

    /// Begin a hollow (stroke-only) figure.
    pub fn begin_hollow(self, start: Vector2) -> PathBuilder<InFigure> {
        unsafe {
            self.state.sink.BeginFigure(start, D2D1_FIGURE_BEGIN_HOLLOW);
        }
        PathBuilder {
            state: InFigure {
                sink: self.state.sink,
                geometry: self.state.geometry,
            },
        }
    }

    /// Finalize the path geometry.
    pub fn build(self) -> Result<Path> {
        unsafe { self.state.sink.Close()? };
        Ok(Path {
            raw: self.state.geometry,
        })
    }
}

impl PathBuilder<InFigure> {
    pub fn line_to(self, point: Vector2) -> Self {
        unsafe { self.state.sink.AddLine(point) };
        self
    }

    pub fn bezier_to(self, control1: Vector2, control2: Vector2, end: Vector2) -> Self {
        let segment = D2D1_BEZIER_SEGMENT {
            point1: control1,
            point2: control2,
            point3: end,
        };
        unsafe { self.state.sink.AddBezier(&segment) };
        self
    }

    /// Close the current figure and connect back to the start point.
    pub fn close(self) -> PathBuilder<Empty> {
        unsafe { self.state.sink.EndFigure(D2D1_FIGURE_END_CLOSED) };
        PathBuilder {
            state: Empty {
                sink: self.state.sink,
                geometry: self.state.geometry,
            },
        }
    }

    /// End the current figure without closing.
    pub fn end_open(self) -> PathBuilder<Empty> {
        unsafe { self.state.sink.EndFigure(D2D1_FIGURE_END_OPEN) };
        PathBuilder {
            state: Empty {
                sink: self.state.sink,
                geometry: self.state.geometry,
            },
        }
    }
}
