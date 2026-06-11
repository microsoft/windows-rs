use crate::bindings::*;
use crate::swap_chain::SwapChain;
use windows_core::Interface;

/// Shared GPU device with automatic device-lost recovery.
///
/// Encapsulates D3D11 device + D2D1 factory + D2D1 device creation.
/// Provides the foundation for all canvas rendering operations.
pub struct GpuDevice {
    d3d_device: ID3D11Device,
    d2d_factory: ID2D1Factory1,
    d2d_device: ID2D1Device,
    dxgi_factory: IDXGIFactory2,
    dwrite_factory: IDWriteFactory,
}

impl GpuDevice {
    /// Create a GPU device with default settings (hardware, BGRA support).
    pub fn new() -> windows_core::Result<Self> {
        unsafe { Self::create(false) }
    }

    /// Create a software (WARP) device — useful for testing or headless rendering.
    pub fn new_warp() -> windows_core::Result<Self> {
        unsafe { Self::create(true) }
    }

    unsafe fn create(software: bool) -> windows_core::Result<Self> {
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
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                feature_levels.as_ptr(),
                feature_levels.len() as u32,
                D3D11_SDK_VERSION,
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

        let dwrite_factory = crate::text::dwrite_factory()?;

        Ok(Self {
            d3d_device,
            d2d_factory,
            d2d_device,
            dxgi_factory,
            dwrite_factory,
        })
    }

    /// Create a composition swap chain for the given dimensions.
    pub fn create_swap_chain(&self, width: u32, height: u32) -> windows_core::Result<SwapChain> {
        SwapChain::new(self, width, height)
    }

    /// Create an HWND-targeting swap chain for standalone (non-reactor) windowed rendering.
    ///
    /// # Safety
    ///
    /// `hwnd` must be a valid window handle for the lifetime of the returned `SwapChain`.
    pub unsafe fn create_swap_chain_for_hwnd(
        &self,
        hwnd: *mut core::ffi::c_void,
        width: u32,
        height: u32,
    ) -> windows_core::Result<SwapChain> {
        SwapChain::new_for_hwnd(self, hwnd, width, height)
    }

    /// Access the raw D3D11 device.
    pub fn d3d_device(&self) -> &ID3D11Device {
        &self.d3d_device
    }

    /// Access the raw D2D1 device.
    pub fn d2d_device(&self) -> &ID2D1Device {
        &self.d2d_device
    }

    /// Access the raw D2D1 factory.
    pub fn d2d_factory(&self) -> &ID2D1Factory1 {
        &self.d2d_factory
    }

    /// Create a stroke style from a builder.
    ///
    /// Stroke styles are device-independent and can be reused across frames.
    pub fn create_stroke_style(
        &self,
        builder: &crate::types::StrokeStyleBuilder,
    ) -> windows_core::Result<crate::types::StrokeStyle> {
        let props = builder.to_abi();
        unsafe {
            self.d2d_factory
                .CreateStrokeStyle(&props, None)
                .map(crate::types::StrokeStyle)
        }
    }

    /// Access the raw DXGI factory.
    pub fn dxgi_factory(&self) -> &IDXGIFactory2 {
        &self.dxgi_factory
    }

    /// Access the raw DWrite factory.
    pub fn dwrite_factory(&self) -> &IDWriteFactory {
        &self.dwrite_factory
    }
}
