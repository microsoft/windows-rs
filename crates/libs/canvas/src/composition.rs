//! The composition bridge (feature `composition`).
//!
//! Draw Direct2D content into a `windows-composition`
//! [`CompositionDrawingSurface`] and paint a composition visual with it. This
//! mirrors Win2D's `CanvasComposition`: the app owns the composition graph
//! (graphics device → drawing surface → surface brush → visual) and this bridge
//! only lends the Direct2D drawing.
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
//!     let white = session.create_solid_brush(ColorF::WHITE).unwrap();
//!     session.fill_ellipse(&Ellipse::circle(Vector2::new(128.0, 128.0), 96.0), &white);
//! })?;
//! # Ok(())
//! # }
//! ```

use super::*;
use windows_composition::{CompositionDrawingSurface, CompositionGraphicsDevice, Compositor};

impl GpuDevice {
    /// Creates a composition graphics device backed by this device.
    ///
    /// Use the returned [`CompositionGraphicsDevice`] to allocate
    /// [`CompositionDrawingSurface`]s that Direct2D content can be drawn into with
    /// [`CanvasCompositionExt::draw`] and displayed through a surface brush.
    pub fn create_graphics_device(
        &self,
        compositor: &Compositor,
    ) -> Result<CompositionGraphicsDevice> {
        compositor.create_graphics_device(self.d2d_device())
    }
}

/// Extends [`CompositionDrawingSurface`] with Direct2D drawing.
///
/// Import this trait to draw into a composition surface with the safe canvas API.
/// The app owns the composition graph; this trait only lends drawing, mirroring
/// Win2D's `CanvasComposition`.
pub trait CanvasCompositionExt {
    /// Redraws the surface: runs `f` to draw, then presents.
    ///
    /// Coordinates in `f` are in pixels with the surface origin at `(0, 0)`; the
    /// backing-atlas offset is applied for you. The surface content is undefined
    /// on entry, so clear it or draw over the whole surface (there is no implicit
    /// clear — matching Win2D). Returns `Ok(false)` if the GPU device was lost —
    /// recreate the device, graphics device, and surface, then draw again.
    fn draw(&self, f: impl FnOnce(&DrawingSession<'_>)) -> Result<bool>;
}

impl CanvasCompositionExt for CompositionDrawingSurface {
    fn draw(&self, f: impl FnOnce(&DrawingSession<'_>)) -> Result<bool> {
        let (context, (offset_x, offset_y)) = match self.begin_draw::<ID2D1DeviceContext>() {
            Ok(v) => v,
            Err(e) if is_device_lost(e.code()) => return Ok(false),
            Err(e) => return Err(e),
        };

        // The surface draws at the default 96 DPI, so pixels and DIPs coincide and
        // the atlas offset applies directly as a translation.
        let offset = Matrix3x2::translation(offset_x as f32, offset_y as f32);

        // Pair every successful begin_draw with an end_draw, even if `f` panics,
        // so the surface is never left mid-draw.
        let guard = EndDrawGuard(self);
        {
            let session = DrawingSession::from_borrowed_context(&context, offset);
            f(&session);
        }
        std::mem::forget(guard);

        match self.end_draw() {
            Ok(()) => Ok(true),
            Err(e) if is_device_lost(e.code()) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

/// Ends the surface draw on the panic path so a successful `begin_draw` is never
/// left unpaired. On the normal path the guard is forgotten and `draw` calls
/// `end_draw` itself to observe the result.
struct EndDrawGuard<'a>(&'a CompositionDrawingSurface);

impl Drop for EndDrawGuard<'_> {
    fn drop(&mut self) {
        let _ = self.0.end_draw();
    }
}
