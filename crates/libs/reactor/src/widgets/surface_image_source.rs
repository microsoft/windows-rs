use super::*;

/// A `SurfaceImageSource` you create and draw into with Direct2D, then display
/// by handing it to [`Image::new`](crate::Image::new). Create it on the UI
/// thread (for example inside your render function).
///
/// Drawing must happen on the UI thread. Call [`set_device`](Self::set_device)
/// once with your Direct2D device, then bracket each frame between
/// [`begin_draw`](Self::begin_draw) and [`end_draw`](Self::end_draw). The same
/// source can be drawn into before or after it is attached to an `Image`.
#[derive(Clone, PartialEq, Debug)]
pub struct SurfaceImageSource {
    // Cast to `ImageSource` and applied as the native `Image.Source`.
    pub source: bindings::SurfaceImageSource,
    native: bindings::ISurfaceImageSourceNativeWithD2D,
}

impl SurfaceImageSource {
    /// Create a `SurfaceImageSource` of the given pixel size. The size is fixed
    /// for the lifetime of the source; create a new one to resize.
    pub fn new(pixel_width: i32, pixel_height: i32) -> Result<Self> {
        let source = bindings::SurfaceImageSource::CreateInstanceWithDimensions(
            pixel_width,
            pixel_height,
        )?;
        let native = source.cast()?;
        Ok(Self { source, native })
    }

    /// Associate the Direct2D device used for drawing. Pass an `ID2D1Device`
    /// (or `IDXGIDevice`). Must be called before [`begin_draw`](Self::begin_draw).
    pub fn set_device(&self, device: &impl Interface) -> Result<()> {
        unsafe { self.native.SetDevice(device.as_raw()).ok() }
    }

    /// Begin drawing into the surface, returning the drawing target `T`
    /// (typically `ID2D1DeviceContext`) and the `(x, y)` pixel offset within the
    /// underlying atlas at which to draw. The update region is given in pixels
    /// as `(x, y, width, height)`; apply the returned offset as a translation on
    /// the drawing target before issuing draw calls.
    pub fn begin_draw<T: Interface>(
        &self,
        update_x: i32,
        update_y: i32,
        update_width: i32,
        update_height: i32,
    ) -> Result<(T, (i32, i32))> {
        let update_rect = bindings::RECT {
            left: update_x,
            top: update_y,
            right: update_x + update_width,
            bottom: update_y + update_height,
        };
        let mut offset = bindings::POINT::default();
        let mut object = core::ptr::null_mut();
        unsafe {
            self.native
                .BeginDraw(&update_rect, &T::IID, &mut object, &mut offset)
                .ok()?;
            Ok((T::from_raw(object), (offset.x, offset.y)))
        }
    }

    /// Finish drawing and present the surface contents.
    pub fn end_draw(&self) -> Result<()> {
        unsafe { self.native.EndDraw().ok() }
    }

    /// Suspend drawing, allowing GPU resources to be reclaimed.
    pub fn suspend_draw(&self) -> Result<()> {
        unsafe { self.native.SuspendDraw().ok() }
    }

    /// Resume drawing after a [`suspend_draw`](Self::suspend_draw).
    pub fn resume_draw(&self) -> Result<()> {
        unsafe { self.native.ResumeDraw().ok() }
    }

    /// Cast the underlying source to the `ImageSource` the backend assigns to
    /// `Image.Source`.
    pub fn image_source(&self) -> Result<bindings::ImageSource> {
        self.source.cast()
    }
}
