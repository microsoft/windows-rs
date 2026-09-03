//! The Direct2D bridge surface (feature `system`).
//!
//! Direct2D drawing-surface interop for system composition. Lifted composition has
//! no Direct2D-surface interop.

use super::*;

/// Direct2D-backed composition device that allocates drawing surfaces.
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

/// Composition surface that Direct2D content is drawn into.
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

    /// Begins drawing, returning the target and backing-atlas pixel offset.
    ///
    /// If this method succeeds, call [`end_draw`](Self::end_draw) even if a later drawing
    /// operation fails.
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

/// Brush that paints a visual with a [`CompositionDrawingSurface`].
#[derive(Clone)]
pub struct CompositionSurfaceBrush(pub(crate) bindings::CompositionSurfaceBrush);

impl Sealed for CompositionSurfaceBrush {}

impl Brush for CompositionSurfaceBrush {
    fn as_brush(&self) -> CompositionBrush {
        CompositionBrush(self.0.cast().unwrap())
    }
}
