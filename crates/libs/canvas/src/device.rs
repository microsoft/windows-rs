use super::*;

/// Shared Direct3D, Direct2D, DXGI, and DirectWrite device state.
#[derive(Clone)]
pub struct GpuDevice {
    d3d_device: ID3D11Device,
    d2d_factory: ID2D1Factory1,
    d2d_device: ID2D1Device,
    dxgi_factory: IDXGIFactory2,
    dwrite_factory: IDWriteFactory,
}

impl GpuDevice {
    pub fn new() -> Result<Self> {
        unsafe { Self::create(false) }
    }

    pub fn new_warp() -> Result<Self> {
        unsafe { Self::create(true) }
    }

    /// Creates a hardware device, falling back to a software (WARP) device when
    /// no GPU is available (headless sessions, VMs, or RDP). Use this for render
    /// loops that must produce output on any machine.
    pub fn new_or_warp() -> Result<Self> {
        Self::new().or_else(|_| Self::new_warp())
    }

    unsafe fn create(software: bool) -> Result<Self> {
        let driver_type = if software {
            D3D_DRIVER_TYPE_WARP
        } else {
            D3D_DRIVER_TYPE_HARDWARE
        };

        let mut d3d_device: Option<ID3D11Device> = None;
        let feature_levels = [D3D_FEATURE_LEVEL_11_0];
        unsafe {
            D3D11CreateDevice(
                std::ptr::null_mut(),
                driver_type,
                std::ptr::null_mut(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT as u32,
                feature_levels.as_ptr(),
                feature_levels.len() as u32,
                D3D11_SDK_VERSION as u32,
                &mut d3d_device as *mut _ as *mut _,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
            .ok()?;
        }
        let d3d_device = d3d_device.unwrap();

        let mut d2d_factory: Option<ID2D1Factory1> = None;
        unsafe {
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                &ID2D1Factory1::IID,
                std::ptr::null(),
                &mut d2d_factory as *mut _ as *mut _,
            )
            .ok()?;
        }
        let d2d_factory = d2d_factory.unwrap();

        let dxgi_device: IDXGIDevice = d3d_device.cast()?;
        let d2d_device = unsafe { d2d_factory.CreateDevice(&dxgi_device)? };

        let dxgi_adapter: IDXGIAdapter = unsafe { dxgi_device.GetAdapter()? };
        let dxgi_factory: IDXGIFactory2 = unsafe { dxgi_adapter.GetParent()? };

        let dwrite_factory = dwrite_factory()?;

        Ok(Self {
            d3d_device,
            d2d_factory,
            d2d_device,
            dxgi_factory,
            dwrite_factory,
        })
    }

    pub fn create_swap_chain(&self, width: u32, height: u32) -> Result<SwapChain> {
        SwapChain::new(self, width, height)
    }

    /// Creates an off-screen [`RenderTarget`] whose pixels can be copied to CPU memory.
    pub fn create_render_target(&self, width: u32, height: u32) -> Result<RenderTarget> {
        RenderTarget::new(self, width, height)
    }

    /// Creates a swap chain that renders directly into the given window.
    pub fn create_swap_chain_for_window(
        &self,
        window: &windows_window::Window,
        width: u32,
        height: u32,
    ) -> Result<SwapChain> {
        // SAFETY: `window` owns a live window handle for as long as the borrow
        // lasts.
        unsafe { self.create_swap_chain_for_hwnd(window.hwnd(), width, height) }
    }

    /// Create an HWND swap chain for standalone windowed rendering.
    ///
    /// # Safety
    ///
    /// `hwnd` must be a valid window handle for the lifetime of the returned `SwapChain`.
    pub unsafe fn create_swap_chain_for_hwnd(
        &self,
        hwnd: *mut core::ffi::c_void,
        width: u32,
        height: u32,
    ) -> Result<SwapChain> {
        SwapChain::new_for_hwnd(self, hwnd, width, height)
    }

    pub fn d3d_device(&self) -> &ID3D11Device {
        &self.d3d_device
    }

    pub fn d2d_device(&self) -> &ID2D1Device {
        &self.d2d_device
    }

    pub fn d2d_factory(&self) -> &ID2D1Factory1 {
        &self.d2d_factory
    }

    pub fn create_stroke_style(&self, builder: &StrokeStyleBuilder) -> Result<StrokeStyle> {
        let props = builder.to_abi();
        unsafe {
            self.d2d_factory
                .CreateStrokeStyle(&props, None)
                .map(StrokeStyle)
        }
    }

    pub fn dxgi_factory(&self) -> &IDXGIFactory2 {
        &self.dxgi_factory
    }

    pub fn dwrite_factory(&self) -> &IDWriteFactory {
        &self.dwrite_factory
    }
}
