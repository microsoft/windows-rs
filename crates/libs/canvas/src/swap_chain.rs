use super::*;

/// Manages a DXGI swap chain.
pub struct SwapChain {
    swap_chain: IDXGISwapChain1,
    d2d_context: ID2D1DeviceContext,
    device_lost_flag: Cell<bool>,
    width: u32,
    height: u32,
    dpi_x: f32,
    dpi_y: f32,
}

impl SwapChain {
    pub(crate) fn new(device: &GpuDevice, width: u32, height: u32) -> Result<Self> {
        let desc = DXGI_SWAP_CHAIN_DESC1 {
            Width: width,
            Height: height,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 2,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
            AlphaMode: DXGI_ALPHA_MODE_PREMULTIPLIED,
            ..Default::default()
        };

        let swap_chain = unsafe {
            device
                .dxgi_factory()
                .CreateSwapChainForComposition(device.d3d_device(), &desc, None)?
        };

        Self::from_swap_chain(device, swap_chain, width, height)
    }

    pub(crate) fn new_for_hwnd(
        device: &GpuDevice,
        hwnd: *mut core::ffi::c_void,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let desc = DXGI_SWAP_CHAIN_DESC1 {
            Width: width,
            Height: height,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 2,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
            ..Default::default()
        };

        let swap_chain = unsafe {
            device.dxgi_factory().CreateSwapChainForHwnd(
                device.d3d_device(),
                hwnd,
                &desc,
                None,
                None,
            )?
        };

        Self::from_swap_chain(device, swap_chain, width, height)
    }

    fn from_swap_chain(
        device: &GpuDevice,
        swap_chain: IDXGISwapChain1,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let d2d_context = unsafe {
            device
                .d2d_device()
                .CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)?
        };

        let mut result = Self {
            swap_chain,
            d2d_context,
            device_lost_flag: Cell::new(false),
            width,
            height,
            dpi_x: 96.0,
            dpi_y: 96.0,
        };
        result.set_target()?;
        Ok(result)
    }

    /// Resizes the swap chain buffers. A zero width or height is ignored.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        if width == 0 || height == 0 {
            return Ok(());
        }
        unsafe {
            self.d2d_context.SetTarget(None);
            self.swap_chain
                .ResizeBuffers(0, width, height, DXGI_FORMAT_UNKNOWN, 0)?;
        }
        self.width = width;
        self.height = height;
        self.set_target()
    }

    /// Begins drawing a frame, returning a [`DrawingSession`].
    pub fn begin_draw(&mut self) -> Result<DrawingSession<'_>> {
        self.device_lost_flag.set(false);
        DrawingSession::new(&self.d2d_context, &self.device_lost_flag)
    }

    /// Returns `Ok(true)` on success or `Ok(false)` if the device was lost.
    pub fn present(&self) -> Result<bool> {
        // If EndDraw detected device-lost, don't bother presenting.
        if self.device_lost_flag.get() {
            return Ok(false);
        }
        let result = unsafe { self.swap_chain.Present(1, 0) };
        if check_device_lost(&result) {
            return Ok(false);
        }
        result.map(|()| true)
    }

    /// Creates a solid color brush.
    pub fn create_solid_brush(&self, color: ColorF) -> Result<Brush> {
        let c: D2D_COLOR_F = color.into();
        unsafe { self.d2d_context.CreateSolidColorBrush(&c, None).map(Brush) }
    }

    /// Loads a bitmap from an image file.
    pub fn load_bitmap(&self, path: impl AsRef<std::path::Path>) -> Result<Bitmap> {
        Bitmap::load_from_file(&self.d2d_context, path.as_ref())
    }

    /// Returns the underlying `IDXGISwapChain1`.
    pub fn raw_swap_chain(&self) -> &IDXGISwapChain1 {
        &self.swap_chain
    }

    /// Returns the width of the swap chain, in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the swap chain, in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the DPI so that Direct2D renders at the correct resolution.
    pub fn set_dpi(&mut self, dpi_x: f32, dpi_y: f32) {
        self.dpi_x = dpi_x;
        self.dpi_y = dpi_y;
        unsafe { self.d2d_context.SetDpi(dpi_x, dpi_y) }
        // Recreate the target bitmap with the updated DPI.
        let _ = self.set_target();
    }

    /// Apply an inverse composition scale so that a pixel-sized buffer
    /// is presented at the correct DIP size.
    pub fn set_composition_scale(&self, scale_x: f32, scale_y: f32) {
        if let Ok(sc2) = self.swap_chain.cast::<IDXGISwapChain2>() {
            let matrix = DXGI_MATRIX_3X2_F {
                _11: 1.0 / scale_x,
                _12: 0.0,
                _21: 0.0,
                _22: 1.0 / scale_y,
                _31: 0.0,
                _32: 0.0,
            };
            unsafe {
                let _ = sc2.SetMatrixTransform(&matrix);
            }
        }
    }

    /// Returns `true` if the device was lost during the last frame.
    pub fn is_device_lost(&self) -> bool {
        self.device_lost_flag.get()
    }

    fn set_target(&mut self) -> Result<()> {
        unsafe {
            let surface: IDXGISurface = self.swap_chain.GetBuffer(0)?;
            let props = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: self.dpi_x,
                dpiY: self.dpi_y,
                bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
                ..Default::default()
            };
            let bitmap = self
                .d2d_context
                .CreateBitmapFromDxgiSurface(&surface, Some(&props))?;
            self.d2d_context.SetTarget(&bitmap);
            // Ensure context DPI matches after SetTarget (some target types reset it).
            self.d2d_context.SetDpi(self.dpi_x, self.dpi_y);
            Ok(())
        }
    }
}
