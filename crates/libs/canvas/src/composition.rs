//! The composition bridge (feature `composition`).
//!
//! Draw Direct2D content into a `windows-composition` [`CompositionDrawingSurface`].
//!
//! ```ignore
//! use windows_canvas::{CanvasCompositionExt, ColorF, GpuDevice, Vector2};
//! use windows_composition::Compositor;
//! # fn demo(device: &GpuDevice, compositor: &Compositor, visual: &windows_composition::SpriteVisual)
//! #     -> windows_core::Result<()> {
//! let graphics = device.create_graphics_device(compositor)?;
//! let surface = graphics.create_drawing_surface(256.0, 256.0)?;
//! let brush = compositor.create_surface_brush(&surface);
//! visual.set_brush(&brush);
//!
//! surface.draw(|session| {
//!     session.clear(ColorF::CORNFLOWER_BLUE);
//!     let white = session.create_solid_brush(ColorF::WHITE)?;
//!     session.fill_ellipse(&Ellipse::circle(Vector2::new(128.0, 128.0), 96.0), &white);
//!     Ok(())
//! })?;
//! # Ok(())
//! # }
//! ```

use super::*;
use windows_composition::{CompositionDrawingSurface, CompositionGraphicsDevice, Compositor};

impl GpuDevice {
    /// Creates a composition graphics device backed by this device.
    ///
    pub fn create_graphics_device(
        &self,
        compositor: &Compositor,
    ) -> Result<CompositionGraphicsDevice> {
        compositor.create_graphics_device(self.d2d_device())
    }
}

/// Extends [`CompositionDrawingSurface`] with Direct2D drawing.
///
pub trait CanvasCompositionExt {
    /// Redraws the surface: runs `f` to draw, then presents.
    ///
    /// Returns `Ok(false)` if the GPU device was lost and the surface must be recreated.
    fn draw(&self, f: impl FnOnce(&DrawingSession<'_>) -> Result<()>) -> Result<bool>;
}

impl CanvasCompositionExt for CompositionDrawingSurface {
    fn draw(&self, f: impl FnOnce(&DrawingSession<'_>) -> Result<()>) -> Result<bool> {
        let (context, (offset_x, offset_y)) = match self.begin_draw::<ID2D1DeviceContext>() {
            Ok(v) => v,
            Err(e) if is_device_lost(e.code()) => return Ok(false),
            Err(e) => return Err(e),
        };

        let offset = Matrix3x2::translation(offset_x as f32, offset_y as f32);

        // Do not leave a successful begin_draw unpaired if `f` panics.
        let guard = EndDrawGuard(self);
        let draw_result = {
            let session = DrawingSession::from_borrowed_context(&context, offset);
            f(&session)
        };
        std::mem::forget(guard);

        match self.end_draw() {
            Ok(()) => match draw_result {
                Ok(()) => Ok(true),
                Err(e) if is_device_lost(e.code()) => Ok(false),
                Err(e) => Err(e),
            },
            Err(e) if is_device_lost(e.code()) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

/// Ends the surface draw only on the panic path.
struct EndDrawGuard<'a>(&'a CompositionDrawingSurface);

impl Drop for EndDrawGuard<'_> {
    fn drop(&mut self) {
        let _ = self.0.end_draw();
    }
}
