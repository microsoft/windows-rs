use crate::bindings::*;
use windows_numerics::Vector2;

/// A completed path geometry, ready for drawing or filling.
///
/// Created via [`PathBuilder`] using the typestate pattern:
///
/// ```ignore
/// let path = PathBuilder::new(&device)?
///     .begin(Vector2::new(0.0, 0.0))
///     .line_to(Vector2::new(100.0, 0.0))
///     .line_to(Vector2::new(50.0, 80.0))
///     .close()
///     .build()?;
///
/// session.fill_path(&path, &brush);
/// ```
#[derive(Clone)]
pub struct Path {
    raw: ID2D1PathGeometry1,
}

impl Path {
    /// Access the raw `ID2D1PathGeometry1`.
    pub fn raw(&self) -> &ID2D1PathGeometry1 {
        &self.raw
    }
}

// -- Typestate markers --

/// Builder state: no figure started yet. Can begin a new figure or build.
pub struct Empty {
    sink: ID2D1GeometrySink,
    geometry: ID2D1PathGeometry1,
}

/// Builder state: inside a figure. Must close or end before building.
pub struct InFigure {
    sink: ID2D1GeometrySink,
    geometry: ID2D1PathGeometry1,
}

/// Type-safe path builder using D2D geometry sinks.
///
/// The typestate pattern ensures:
/// - You can't add segments without starting a figure
/// - You can't build without closing all figures
/// - You can't begin a figure while one is already open
///
/// ```ignore
/// let path = PathBuilder::new(&device)?
///     .begin(point)       // Empty → InFigure
///     .line_to(point2)    // InFigure → InFigure
///     .bezier_to(c1, c2, end)
///     .close()            // InFigure → Empty
///     .build()?;          // Empty → Path
/// ```
pub struct PathBuilder<S> {
    state: S,
}

impl PathBuilder<Empty> {
    /// Create a new path builder from a device's D2D factory.
    pub fn new(device: &crate::GpuDevice) -> crate::Result<Self> {
        let geometry = unsafe { device.d2d_factory().CreatePathGeometry()? };
        let sink = unsafe { geometry.Open()? };
        Ok(Self {
            state: Empty { sink, geometry },
        })
    }

    /// Begin a filled figure at the given start point.
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

    /// Begin a hollow (stroke-only) figure at the given start point.
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

    /// Finalize the path geometry. All figures must be closed before calling this.
    pub fn build(self) -> crate::Result<Path> {
        unsafe { self.state.sink.Close()? };
        Ok(Path {
            raw: self.state.geometry,
        })
    }
}

impl PathBuilder<InFigure> {
    /// Add a line segment to the given point.
    pub fn line_to(self, point: Vector2) -> Self {
        unsafe { self.state.sink.AddLine(point) };
        self
    }

    /// Add a cubic bezier curve.
    pub fn bezier_to(self, control1: Vector2, control2: Vector2, end: Vector2) -> Self {
        let segment = D2D1_BEZIER_SEGMENT {
            point1: control1,
            point2: control2,
            point3: end,
        };
        unsafe { self.state.sink.AddBezier(&segment) };
        self
    }

    /// Close the current figure (connects back to start point) and return
    /// to the `Empty` state for additional figures or building.
    pub fn close(self) -> PathBuilder<Empty> {
        unsafe { self.state.sink.EndFigure(D2D1_FIGURE_END_CLOSED) };
        PathBuilder {
            state: Empty {
                sink: self.state.sink,
                geometry: self.state.geometry,
            },
        }
    }

    /// End the current figure without closing (open-ended stroke).
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
