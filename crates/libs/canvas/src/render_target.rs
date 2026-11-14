use super::*;

/// Off-screen Direct2D render target for headless drawing and CPU pixel readback.
pub struct RenderTarget {
    context: ID2D1DeviceContext,
    target: ID2D1Bitmap1,
    end_draw_result: Cell<HRESULT>,
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
            end_draw_result: Cell::new(HRESULT(0)),
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
    ///
    /// Reports deferred native drawing errors as well as errors returned by `f`.
    ///
    /// | Condition (first match) | Result |
    /// | --- | --- |
    /// | `EndDraw` reports device loss | [`device_lost_error`] |
    /// | `f` reports device loss | Original callback error |
    /// | `EndDraw` fails | Native drawing error |
    /// | Otherwise | Result returned by `f` |
    pub fn draw(&self, f: impl FnOnce(&DrawingSession<'_>) -> Result<()>) -> Result<()> {
        self.end_draw_result.set(HRESULT(0));
        unsafe { self.context.SetTarget(&self.target) };
        let draw_result = {
            let session = DrawingSession::new(&self.context, &self.end_draw_result)?;
            f(&session)
        };
        unsafe { self.context.SetTarget(None::<&ID2D1Image>) };
        Self::finish_draw(draw_result, self.end_draw_result.get())
    }

    fn finish_draw(draw_result: Result<()>, end_result: HRESULT) -> Result<()> {
        if is_device_lost(end_result) {
            Err(device_lost_error())
        } else if check_device_lost(&draw_result) {
            draw_result
        } else {
            end_result.ok().and(draw_result)
        }
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
            let mut mapped = D2D1_MAPPED_RECT::default();
            staging.Map(D2D1_MAP_OPTIONS_READ, &mut mapped).ok()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_completion_error_precedence() {
        let success = HRESULT(0);
        let callbacks = [success, E_INVALIDARG, DXGI_ERROR_DEVICE_REMOVED];
        let completions = [success, E_FAIL, DXGI_ERROR_DEVICE_RESET];
        let expected = [
            [success, E_FAIL, D2DERR_RECREATE_TARGET],
            [E_INVALIDARG, E_FAIL, D2DERR_RECREATE_TARGET],
            [
                DXGI_ERROR_DEVICE_REMOVED,
                DXGI_ERROR_DEVICE_REMOVED,
                D2DERR_RECREATE_TARGET,
            ],
        ];
        for (draw_result, expected) in callbacks.into_iter().zip(expected) {
            for (end_result, expected) in completions.into_iter().zip(expected) {
                let actual = RenderTarget::finish_draw(draw_result.ok(), end_result)
                    .map_or_else(|error| error.code(), |()| success);
                assert_eq!(
                    actual, expected,
                    "callback {draw_result:?}, EndDraw {end_result:?}"
                );
            }
        }
    }
}
