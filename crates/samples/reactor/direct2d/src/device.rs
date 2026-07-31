use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;

use crate::bindings::*;
use windows_core::{HRESULT, Interface, Result};
use windows_reactor::{Context, Updater};

#[derive(Clone)]
pub struct SharedDevice {
    d3d_device: ID3D11Device,
    d2d_device: ID2D1Device,
    dxgi_factory: IDXGIFactory2,
}

// SAFETY: these COM interfaces are agile, though their metadata cannot express it.
unsafe impl Send for SharedDevice {}

impl SharedDevice {
    fn new() -> Result<Self> {
        let mut d3d_device: Option<ID3D11Device> = None;
        unsafe {
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT as u32,
                Some(&[D3D_FEATURE_LEVEL_11_0]),
                D3D11_SDK_VERSION as u32,
                Some(&mut d3d_device),
                None,
                None,
            )
            .ok()?;
        }
        let d3d_device = d3d_device.unwrap();

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

    pub fn d3d_device(&self) -> &ID3D11Device {
        &self.d3d_device
    }

    pub fn d2d_device(&self) -> &ID2D1Device {
        &self.d2d_device
    }

    pub fn dxgi_factory(&self) -> &IDXGIFactory2 {
        &self.dxgi_factory
    }
}

#[derive(Clone)]
pub struct Device(Rc<SharedDevice>);

impl Device {
    pub fn new() -> Result<Self> {
        Ok(Self(Rc::new(SharedDevice::new()?)))
    }

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

#[derive(Clone, PartialEq)]
pub struct Gpu {
    device: Option<Device>,
    recover: Updater<u32>,
}

impl Gpu {
    pub fn new(device: Option<Device>, recover: Updater<u32>) -> Self {
        Self { device, recover }
    }

    pub fn device(&self) -> Option<Device> {
        self.device.clone()
    }

    pub fn request_recovery(&self) {
        self.recover.call(|g| g.wrapping_add(1));
    }
}

static GPU_KEY: LazyLock<Context<()>> = LazyLock::new(|| Context::new(()));

pub fn gpu_context() -> Context<Option<Gpu>> {
    Context {
        default: None,
        id: GPU_KEY.id,
    }
}

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
