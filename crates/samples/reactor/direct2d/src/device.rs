//! Consolidated GPU device shared by every Direct2D sample in this app.
//!
//! A single [`SharedDevice`] (D3D11 device + Direct2D factory/device) is created
//! once and handed to the samples through the reactor [`DEVICE`] context, so they
//! all render with the same device.
//!
//! The D2D factory is created `MULTI_THREADED` because the swap-chain sample
//! presents from a dedicated worker thread while the surface-image-source sample
//! draws on the UI thread; a multithreaded factory serializes D2D's own calls so
//! the one D2D device can be used from both.
//!
//! Note: a multithreaded factory only serializes D2D's *own* calls, not raw
//! D3D/DXGI interop on the shared device. This sample does not yet add the
//! explicit `ID2D1Multithread` locking that would be needed to fully harden that
//! interop; the focus here is on consolidating the device.

use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;

use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::core::{Interface, Result};
use windows_reactor::Context;

/// The app-wide shared GPU device. Owns the D3D11 device, the `MULTI_THREADED`
/// Direct2D factory and device, and the DXGI factory.
///
/// Every interface held here is an agile (free-threaded) COM object, so a clone
/// of this struct can be moved onto the swap-chain sample's render thread; see
/// [`Device::to_send`].
#[derive(Clone)]
pub struct SharedDevice {
    d3d_device: ID3D11Device,
    d2d_device: ID2D1Device,
    dxgi_factory: IDXGIFactory2,
}

// SAFETY: every interface held here is an agile (free-threaded) COM object.
unsafe impl Send for SharedDevice {}

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

        // MULTI_THREADED so the single D2D device can be used from both the UI
        // thread and the swap-chain sample's render thread.
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

/// Reference-counted handle to a [`SharedDevice`], suitable for use as a reactor
/// context value and as a `use_effect` dependency.
///
/// Equality is by identity (`Rc::ptr_eq`), so recreating the device after a
/// device-lost event yields a value that compares unequal to the old one. That
/// difference is what drives dependents (keyed on the device) to rebuild their
/// device-specific resources.
#[derive(Clone)]
pub struct Device(Rc<SharedDevice>);

impl Device {
    /// Create a new shared device.
    pub fn new() -> Result<Self> {
        Ok(Self(Rc::new(SharedDevice::new()?)))
    }

    /// A `Send` snapshot of the device's agile COM interfaces, for moving onto a
    /// render worker thread. The clone shares the same underlying COM objects as
    /// this handle.
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

/// Stable key for the device context. Stored as a `Context<()>` so the `static`
/// is `Sync` — the real context value holds an `Rc` (UI-thread only), so the
/// typed [`Context`] is rebuilt on demand via [`device_context`] using this id.
static DEVICE_KEY: LazyLock<Context<()>> = LazyLock::new(|| Context::new(()));

/// The app-wide device context, carrying the shared [`Device`]. `None` until the
/// root creates it (and, in the device-lost flow, while it is being recreated).
pub fn device_context() -> Context<Option<Device>> {
    Context {
        default: None,
        id: DEVICE_KEY.id,
    }
}
