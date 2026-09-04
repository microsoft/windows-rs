//! Direct2D bridge surfaces for system and lifted Composition.
//!
//! The selected Composition stack supplies its matching graphics-device and
//! drawing-surface interop interfaces.

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

    /// Replaces the Direct2D or DXGI device backing this Composition graphics
    /// device while preserving its surfaces.
    pub fn set_rendering_device(&self, rendering_device: &impl Interface) -> Result<()> {
        #[cfg(all(feature = "system", not(feature = "reactor")))]
        let interop: bindings::ICompositionGraphicsDeviceInterop = self.0.cast()?;
        #[cfg(feature = "reactor")]
        let interop: bindings::IMicrosoftCompositionGraphicsDeviceInterop = self.0.cast()?;
        let device: windows_core::IUnknown = rendering_device.cast()?;
        unsafe { interop.SetRenderingDevice(&device).ok() }
    }
}

/// Composition surface that Direct2D content is drawn into.
#[derive(Clone)]
pub struct CompositionDrawingSurface {
    surface: bindings::CompositionDrawingSurface,
    interop: DrawingSurfaceInterop,
}

#[cfg(all(feature = "system", not(feature = "reactor")))]
type DrawingSurfaceInterop = bindings::ICompositionDrawingSurfaceInterop;
#[cfg(feature = "reactor")]
type DrawingSurfaceInterop = bindings::IMicrosoftCompositionDrawingSurfaceInterop;

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

/// How a surface brush maps its source into the visual bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceStretch {
    /// Preserve the source size.
    None,
    /// Scale the source independently on both axes to fill the destination.
    Fill,
    /// Scale the source uniformly so the entire source fits in the destination.
    Uniform,
    /// Scale the source uniformly so the destination is filled, cropping as needed.
    UniformToFill,
}

impl CompositionSurfaceBrush {
    /// Sets how the source is stretched into the visual bounds.
    pub fn set_stretch(&self, stretch: SurfaceStretch) {
        let stretch = match stretch {
            SurfaceStretch::None => bindings::CompositionStretch::None,
            SurfaceStretch::Fill => bindings::CompositionStretch::Fill,
            SurfaceStretch::Uniform => bindings::CompositionStretch::Uniform,
            SurfaceStretch::UniformToFill => bindings::CompositionStretch::UniformToFill,
        };
        self.0.SetStretch(stretch).unwrap();
    }
}

impl Sealed for CompositionSurfaceBrush {}

impl Brush for CompositionSurfaceBrush {
    fn as_brush(&self) -> CompositionBrush {
        CompositionBrush(self.0.cast().unwrap())
    }
}
