use super::*;

/// Off-screen Direct2D render target for headless drawing and CPU pixel readback.
pub struct RenderTarget {
    context: ID2D1DeviceContext,
    target: ID2D1Bitmap1,
    device_lost_flag: Cell<bool>,
    width: u32,
    height: u32,
}

impl RenderTarget {
    pub(crate) fn new(device: &GpuDevice, width: u32, height: u32) -> Result<Self> {
        if width == 0 || height == 0 {
            return Err(Error::from_hresult(E_INVALIDARG));
        }
        let context = unsafe {
            device
                .d2d_device()
                .CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)?
        };
        let properties = D2D1_BITMAP_PROPERTIES1 {
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: 96.0,
            dpiY: 96.0,
            bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET,
            ..Default::default()
        };
        let target =
            unsafe { context.CreateBitmap(D2D_SIZE_U { width, height }, None, 0, &properties)? };
        Ok(Self {
            context,
            target,
            device_lost_flag: Cell::new(false),
            width,
            height,
        })
    }

    /// The width of the target, in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of the target, in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Draws one frame into the target with a [`DrawingSession`].
    pub fn draw(&self, f: impl FnOnce(&DrawingSession<'_>) -> Result<()>) -> Result<()> {
        self.device_lost_flag.set(false);
        unsafe { self.context.SetTarget(&self.target) };
        let draw_result = {
            let session = DrawingSession::new(&self.context, &self.device_lost_flag)?;
            f(&session)
        }; // `EndDraw` runs here, flagging any device loss.
        unsafe { self.context.SetTarget(None::<&ID2D1Image>) };
        if self.device_lost_flag.get() {
            return Err(Error::from_hresult(D2DERR_RECREATE_TARGET));
        }
        draw_result
    }

    /// Copies the rendered pixels as tightly packed, top-down 32-bit premultiplied BGRA.
    pub fn read_pixels(&self) -> Result<Vec<u8>> {
        let properties = D2D1_BITMAP_PROPERTIES1 {
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: 96.0,
            dpiY: 96.0,
            bitmapOptions: D2D1_BITMAP_OPTIONS_CPU_READ | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
            ..Default::default()
        };
        unsafe {
            let staging = self.context.CreateBitmap(
                D2D_SIZE_U {
                    width: self.width,
                    height: self.height,
                },
                None,
                0,
                &properties,
            )?;
            staging.CopyFromBitmap(None, &self.target, None).ok()?;
            let mapped = staging.Map(D2D1_MAP_OPTIONS_READ)?;
            let pixels = copy_mapped_pixels(&mapped, self.width, self.height);
            staging.Unmap().ok()?;
            pixels
        }
    }
}

/// Repacks a mapped D2D bitmap, dropping any hardware row padding.
fn copy_mapped_pixels(mapped: &D2D1_MAPPED_RECT, width: u32, height: u32) -> Result<Vec<u8>> {
    if mapped.bits.is_null() {
        return Err(Error::from_hresult(E_FAIL));
    }
    let width = width as usize;
    let height = height as usize;
    let row_bytes = width
        .checked_mul(4)
        .ok_or_else(|| Error::from_hresult(E_FAIL))?;
    let pitch = mapped.pitch as usize;
    if pitch < row_bytes {
        return Err(Error::from_hresult(E_FAIL));
    }
    let total = row_bytes
        .checked_mul(height)
        .ok_or_else(|| Error::from_hresult(E_FAIL))?;
    let mut bgra = vec![0u8; total];
    for y in 0..height {
        let src_start = y * pitch;
        let dst_start = y * row_bytes;
        // SAFETY: D2D guarantees at least `pitch` bytes per row and `row_bytes <= pitch`.
        let src = unsafe { std::slice::from_raw_parts(mapped.bits.add(src_start), row_bytes) };
        bgra[dst_start..dst_start + row_bytes].copy_from_slice(src);
    }
    Ok(bgra)
}
