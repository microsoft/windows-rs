//! Consolidated GPU device shared by every Direct2D sample in this app.
//!
//! A single [`SharedDevice`] is created once and shared via the [`Gpu`] context,
//! so every sample renders with the same device.
//!
//! The D2D factory is `MULTI_THREADED` because the swap-chain sample presents
//! from a worker thread while the surface-image-source sample draws on the UI
//! thread. That only serializes D2D's own calls, not raw D3D/DXGI interop; the
//! `ID2D1Multithread` locking needed to fully harden that interop is not added
//! here.

use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;

use windows::Win32::Foundation::D2DERR_RECREATE_TARGET;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::core::{HRESULT, Interface, Result};
use windows_reactor::{Context, Updater};

/// The app-wide shared GPU device: the D3D11 device, the `MULTI_THREADED` D2D
/// factory and device, and the DXGI factory.
///
/// Every interface is an agile COM object, so a clone can be moved onto the
/// swap-chain sample's render thread; see [`Device::to_send`].
#[derive(Clone)]
pub struct SharedDevice {
    d3d_device: ID3D11Device,
    d2d_device: ID2D1Device,
    dxgi_factory: IDXGIFactory2,
}

impl SharedDevice {
    /// Create a hardware-backed shared device.
    fn new() -> Result<Self> {
        let mut d3d_device: Option<ID3D11Device> = None;
        unsafe {
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                None,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                Some(&[D3D_FEATURE_LEVEL_11_0]),
                D3D11_SDK_VERSION,
                Some(&mut d3d_device),
                None,
                None,
            )?;
        }
        let d3d_device = d3d_device.unwrap();

        // MULTI_THREADED so the one D2D device works from both the UI and render
        // threads.
        let d2d_factory: ID2D1Factory1 =
            unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_MULTI_THREADED, None)? };

        let dxgi_device: IDXGIDevice = d3d_device.cast()?;
        let d2d_device = unsafe { d2d_factory.CreateDevice(&dxgi_device)? };

        let dxgi_adapter = unsafe { dxgi_device.GetAdapter()? };
        let dxgi_factory: IDXGIFactory2 = unsafe { dxgi_adapter.GetParent()? };

        Ok(Self {
            d3d_device,
            d2d_device,
            dxgi_factory,
        })
    }

    /// The shared D3D11 device.
    pub fn d3d_device(&self) -> &ID3D11Device {
        &self.d3d_device
    }

    /// The shared Direct2D device. Create a per-thread device context from this.
    pub fn d2d_device(&self) -> &ID2D1Device {
        &self.d2d_device
    }

    /// The DXGI factory, for creating the composition swap chain.
    pub fn dxgi_factory(&self) -> &IDXGIFactory2 {
        &self.dxgi_factory
    }
}

/// Reference-counted handle to a [`SharedDevice`], usable as a context value and
/// a `use_effect` dependency.
///
/// Equality is by identity (`Rc::ptr_eq`), so a recreated device compares
/// unequal to the old one, driving device-keyed dependents to rebuild.
#[derive(Clone)]
pub struct Device(Rc<SharedDevice>);

impl Device {
    /// Create a new shared device.
    pub fn new() -> Result<Self> {
        Ok(Self(Rc::new(SharedDevice::new()?)))
    }

    /// A `Send` snapshot of the agile COM interfaces, for moving onto the render
    /// thread. Shares the same underlying COM objects as this handle.
    pub fn to_send(&self) -> SharedDevice {
        (*self.0).clone()
    }
}

impl Deref for Device {
    type Target = SharedDevice;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// What every sample needs from the root: the shared [`Device`] (`None` until
/// first created) and a way to request recovery.
#[derive(Clone, PartialEq)]
pub struct Gpu {
    device: Option<Device>,
    recover: Updater<u32>,
}

impl Gpu {
    pub fn new(device: Option<Device>, recover: Updater<u32>) -> Self {
        Self { device, recover }
    }

    /// The shared device, or `None` before the first successful creation.
    pub fn device(&self) -> Option<Device> {
        self.device.clone()
    }

    /// Ask the root to recreate the shared device. Bumps a counter that re-runs
    /// the root's create/recover effect; recreation is unconditional.
    pub fn request_recovery(&self) {
        self.recover.call(|g| g.wrapping_add(1));
    }
}

/// Stable id for the GPU context. Held as a `Context<()>` so the `static` is
/// `Sync`; the real value holds `Rc`s, so [`gpu_context`] rebuilds the typed
/// [`Context`] on demand.
static GPU_KEY: LazyLock<Context<()>> = LazyLock::new(|| Context::new(()));

/// The app-wide GPU context. `None` until the root installs it.
pub fn gpu_context() -> Context<Option<Gpu>> {
    Context {
        default: None,
        id: GPU_KEY.id,
    }
}

/// Whether an `HRESULT` means the GPU device was lost and must be recreated.
/// Matches the set Win2D treats as device-lost in
/// `DeviceLostException::IsDeviceLostHResult`.
pub fn is_device_lost(hr: HRESULT) -> bool {
    matches!(
        hr,
        DXGI_ERROR_DEVICE_HUNG
            | DXGI_ERROR_DEVICE_REMOVED
            | DXGI_ERROR_DEVICE_RESET
            | DXGI_ERROR_DRIVER_INTERNAL_ERROR
            | DXGI_ERROR_INVALID_CALL
            | D2DERR_RECREATE_TARGET
    )
}
