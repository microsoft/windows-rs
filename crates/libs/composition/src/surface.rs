//! The Direct2D bridge surface (feature `system`).
//!
//! These types let an app draw Direct2D content into a composition surface and
//! paint a visual with it. The graphics-device and drawing-surface interop exists
//! only on the system stack, so the whole bridge is system-only; lifted
//! composition has no Direct2D-surface interop.
//!
//! The [`begin_draw`](CompositionDrawingSurface::begin_draw) /
//! [`end_draw`](CompositionDrawingSurface::end_draw) seam is what
//! [`windows-canvas`](https://docs.rs/windows-canvas)'s `composition` feature
//! draws through; most callers use that bridge rather than these methods
//! directly.

use super::*;

/// A Direct2D-backed composition device that allocates drawing surfaces.
///
/// Create one from a [`Compositor`](crate::Compositor) and the app's Direct2D (or
/// DXGI) rendering device with
/// [`Compositor::create_graphics_device`](crate::Compositor::create_graphics_device),
/// then allocate [`CompositionDrawingSurface`]s to draw into.
#[derive(Clone)]
pub struct CompositionGraphicsDevice(pub(crate) bindings::CompositionGraphicsDevice);

impl CompositionGraphicsDevice {
    /// Creates a drawing surface `width`x`height` pixels in size, using a
    /// premultiplied BGRA pixel format.
    pub fn create_drawing_surface(
        &self,
        width: f32,
        height: f32,
    ) -> Result<CompositionDrawingSurface> {
        let surface = self.0.CreateDrawingSurface(
            bindings::Size { width, height },
            bindings::DirectXPixelFormat::B8G8R8A8UIntNormalized,
            bindings::DirectXAlphaMode::Premultiplied,
        )?;
        CompositionDrawingSurface::new(surface)
    }
}

/// A composition surface that Direct2D content is drawn into and painted onto a
/// visual through a [`CompositionSurfaceBrush`].
///
/// Allocate one from a [`CompositionGraphicsDevice`]. Draw into it with the
/// canvas bridge, which brackets each redraw between
/// [`begin_draw`](Self::begin_draw) and [`end_draw`](Self::end_draw).
#[derive(Clone)]
pub struct CompositionDrawingSurface {
    surface: bindings::CompositionDrawingSurface,
    interop: bindings::ICompositionDrawingSurfaceInterop,
}

impl CompositionDrawingSurface {
    fn new(surface: bindings::CompositionDrawingSurface) -> Result<Self> {
        let interop = surface.cast()?;
        Ok(Self { surface, interop })
    }

    /// Begins drawing into the surface, returning the drawing target `T`
    /// (typically `ID2D1DeviceContext`) and the `(x, y)` pixel offset within the
    /// backing atlas at which to draw. Apply the offset as a translation on the
    /// target before issuing draw calls, and pair every call with
    /// [`end_draw`](Self::end_draw).
    ///
    /// This is the interop seam the canvas bridge draws through; most callers use
    /// that bridge instead of calling this directly.
    pub fn begin_draw<T: Interface>(&self) -> Result<(T, (i32, i32))> {
        let mut offset = bindings::POINT::default();
        let object = unsafe { self.interop.BeginDraw::<T>(None, &mut offset)? };
        Ok((object, (offset.x, offset.y)))
    }

    /// Finishes drawing begun with [`begin_draw`](Self::begin_draw) and presents
    /// the surface contents.
    pub fn end_draw(&self) -> Result<()> {
        unsafe { self.interop.EndDraw().ok() }
    }

    /// Resizes the surface to `width`x`height` pixels.
    pub fn resize(&self, width: i32, height: i32) -> Result<()> {
        unsafe {
            self.interop
                .Resize(bindings::SIZE {
                    cx: width,
                    cy: height,
                })
                .ok()
        }
    }

    /// The surface as the `ICompositionSurface` a surface brush paints with.
    pub(crate) fn as_surface(&self) -> bindings::ICompositionSurface {
        self.surface.cast().unwrap()
    }
}

/// A brush that paints a visual with the contents of a
/// [`CompositionDrawingSurface`].
///
/// Create one with
/// [`Compositor::create_surface_brush`](crate::Compositor::create_surface_brush).
#[derive(Clone)]
pub struct CompositionSurfaceBrush(pub(crate) bindings::CompositionSurfaceBrush);

impl Sealed for CompositionSurfaceBrush {}

impl Brush for CompositionSurfaceBrush {
    fn as_brush(&self) -> CompositionBrush {
        CompositionBrush(self.0.cast().unwrap())
    }
}
