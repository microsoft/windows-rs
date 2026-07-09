#[inline]
pub unsafe fn DXGIDeclareAdapterRemovalSupport() -> windows_core::HRESULT {
    windows_core::link!("dxgi.dll" "system" fn DXGIDeclareAdapterRemovalSupport() -> windows_core::HRESULT);
    unsafe { DXGIDeclareAdapterRemovalSupport() }
}
#[inline]
pub unsafe fn DXGIDisableVBlankVirtualization() -> windows_core::HRESULT {
    windows_core::link!("dxgi.dll" "system" fn DXGIDisableVBlankVirtualization() -> windows_core::HRESULT);
    unsafe { DXGIDisableVBlankVirtualization() }
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgi1_2", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::winnt::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: super::dxgi1_2::DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: super::dxgi1_2::DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(all(feature = "Win32_dxgi1_2", feature = "Win32_winnt"))]
impl Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_ADAPTER_FLAG3 = u32;
pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: DXGI_ADAPTER_FLAG3 = 4;
pub const DXGI_ADAPTER_FLAG3_FORCE_DWORD: DXGI_ADAPTER_FLAG3 = 4294967295;
pub const DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE: DXGI_ADAPTER_FLAG3 = 32;
pub const DXGI_ADAPTER_FLAG3_NONE: DXGI_ADAPTER_FLAG3 = 0;
pub const DXGI_ADAPTER_FLAG3_REMOTE: DXGI_ADAPTER_FLAG3 = 1;
pub const DXGI_ADAPTER_FLAG3_SOFTWARE: DXGI_ADAPTER_FLAG3 = 2;
pub const DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = 8;
pub const DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = 16;
pub type DXGI_GPU_PREFERENCE = i32;
pub const DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE: DXGI_GPU_PREFERENCE = 2;
pub const DXGI_GPU_PREFERENCE_MINIMUM_POWER: DXGI_GPU_PREFERENCE = 1;
pub const DXGI_GPU_PREFERENCE_UNSPECIFIED: DXGI_GPU_PREFERENCE = 0;
pub type DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = u32;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 4;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 1;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::windef::RECT,
    pub AttachedToDesktop: windows_core::BOOL,
    pub Rotation: super::dxgitype::DXGI_MODE_ROTATION,
    pub Monitor: super::windef::HMONITOR,
    pub BitsPerColor: u32,
    pub ColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [f32; 2],
    pub GreenPrimary: [f32; 2],
    pub BluePrimary: [f32; 2],
    pub WhitePoint: [f32; 2],
    pub MinLuminance: f32,
    pub MaxLuminance: f32,
    pub MaxFullFrameLuminance: f32,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl Default for DXGI_OUTPUT_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_4"))]
windows_core::imp::define_interface!(IDXGIAdapter4, IDXGIAdapter4_Vtbl, 0x3c8d99d1_4fbf_4181_a82c_af66bf7bd24e);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_4"))]
impl core::ops::Deref for IDXGIAdapter4 {
    type Target = super::dxgi1_4::IDXGIAdapter3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_4"))]
windows_core::imp::interface_hierarchy!(IDXGIAdapter4, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIAdapter, super::dxgi::IDXGIAdapter1, super::dxgi1_2::IDXGIAdapter2, super::dxgi1_4::IDXGIAdapter3);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_4"))]
impl IDXGIAdapter4 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetDesc3(&self, pdesc: *mut DXGI_ADAPTER_DESC3) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc3)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_4"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter4_Vtbl {
    pub base__: super::dxgi1_4::IDXGIAdapter3_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetDesc3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetDesc3: usize,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_4", feature = "Win32_winnt"))]
pub trait IDXGIAdapter4_Impl: super::dxgi1_4::IDXGIAdapter3_Impl {
    fn GetDesc3(&self, pdesc: *mut DXGI_ADAPTER_DESC3) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_4", feature = "Win32_winnt"))]
impl IDXGIAdapter4_Vtbl {
    pub const fn new<Identity: IDXGIAdapter4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc3<Identity: IDXGIAdapter4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter4_Impl::GetDesc3(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self { base__: super::dxgi1_4::IDXGIAdapter3_Vtbl::new::<Identity, OFFSET>(), GetDesc3: GetDesc3::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter4 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIAdapter as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIAdapter2 as windows_core::Interface>::IID || iid == &<super::dxgi1_4::IDXGIAdapter3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_4", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIAdapter4 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
windows_core::imp::define_interface!(IDXGIFactory6, IDXGIFactory6_Vtbl, 0xc1b6694f_ff09_44a9_b03c_77900a0a1d17);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
impl core::ops::Deref for IDXGIFactory6 {
    type Target = super::dxgi1_5::IDXGIFactory5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
windows_core::imp::interface_hierarchy!(IDXGIFactory6, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIFactory, super::dxgi::IDXGIFactory1, super::dxgi1_2::IDXGIFactory2, super::dxgi1_3::IDXGIFactory3, super::dxgi1_4::IDXGIFactory4, super::dxgi1_5::IDXGIFactory5);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
impl IDXGIFactory6 {
    pub unsafe fn EnumAdapterByGpuPreference<T>(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).EnumAdapterByGpuPreference)(windows_core::Interface::as_raw(self), adapter, gpupreference, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory6_Vtbl {
    pub base__: super::dxgi1_5::IDXGIFactory5_Vtbl,
    pub EnumAdapterByGpuPreference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DXGI_GPU_PREFERENCE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory6_Impl: super::dxgi1_5::IDXGIFactory5_Impl {
    fn EnumAdapterByGpuPreference(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory6_Vtbl {
    pub const fn new<Identity: IDXGIFactory6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapterByGpuPreference<Identity: IDXGIFactory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory6_Impl::EnumAdapterByGpuPreference(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&gpupreference), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
            }
        }
        Self {
            base__: super::dxgi1_5::IDXGIFactory5_Vtbl::new::<Identity, OFFSET>(),
            EnumAdapterByGpuPreference: EnumAdapterByGpuPreference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory6 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory1 as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIFactory2 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGIFactory3 as windows_core::Interface>::IID || iid == &<super::dxgi1_4::IDXGIFactory4 as windows_core::Interface>::IID || iid == &<super::dxgi1_5::IDXGIFactory5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory6 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
windows_core::imp::define_interface!(IDXGIFactory7, IDXGIFactory7_Vtbl, 0xa4966eed_76db_44da_84c1_ee9a7afb20a8);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
impl core::ops::Deref for IDXGIFactory7 {
    type Target = IDXGIFactory6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
windows_core::imp::interface_hierarchy!(IDXGIFactory7, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIFactory, super::dxgi::IDXGIFactory1, super::dxgi1_2::IDXGIFactory2, super::dxgi1_3::IDXGIFactory3, super::dxgi1_4::IDXGIFactory4, super::dxgi1_5::IDXGIFactory5, IDXGIFactory6);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
impl IDXGIFactory7 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn RegisterAdaptersChangedEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterAdaptersChangedEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterAdaptersChangedEvent)(windows_core::Interface::as_raw(self), dwcookie) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory7_Vtbl {
    pub base__: IDXGIFactory6_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub RegisterAdaptersChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    RegisterAdaptersChangedEvent: usize,
    pub UnregisterAdaptersChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory7_Impl: IDXGIFactory6_Impl {
    fn RegisterAdaptersChangedEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory7_Vtbl {
    pub const fn new<Identity: IDXGIFactory7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterAdaptersChangedEvent<Identity: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory7_Impl::RegisterAdaptersChangedEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterAdaptersChangedEvent<Identity: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory7_Impl::UnregisterAdaptersChangedEvent(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: IDXGIFactory6_Vtbl::new::<Identity, OFFSET>(),
            RegisterAdaptersChangedEvent: RegisterAdaptersChangedEvent::<Identity, OFFSET>,
            UnregisterAdaptersChangedEvent: UnregisterAdaptersChangedEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory7 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory1 as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIFactory2 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGIFactory3 as windows_core::Interface>::IID || iid == &<super::dxgi1_4::IDXGIFactory4 as windows_core::Interface>::IID || iid == &<super::dxgi1_5::IDXGIFactory5 as windows_core::Interface>::IID || iid == &<IDXGIFactory6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory7 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
windows_core::imp::define_interface!(IDXGIOutput6, IDXGIOutput6_Vtbl, 0x068346e8_aaec_4b84_add7_137f513f77a1);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
impl core::ops::Deref for IDXGIOutput6 {
    type Target = super::dxgi1_5::IDXGIOutput5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
windows_core::imp::interface_hierarchy!(IDXGIOutput6, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIOutput, super::dxgi1_2::IDXGIOutput1, super::dxgi1_3::IDXGIOutput2, super::dxgi1_3::IDXGIOutput3, super::dxgi1_4::IDXGIOutput4, super::dxgi1_5::IDXGIOutput5);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
impl IDXGIOutput6 {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype", feature = "Win32_windef"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_OUTPUT_DESC1) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn CheckHardwareCompositionSupport(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckHardwareCompositionSupport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput6_Vtbl {
    pub base__: super::dxgi1_5::IDXGIOutput5_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype", feature = "Win32_windef"))]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_OUTPUT_DESC1) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype", feature = "Win32_windef")))]
    GetDesc1: usize,
    pub CheckHardwareCompositionSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDXGIOutput6_Impl: super::dxgi1_5::IDXGIOutput5_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_OUTPUT_DESC1) -> windows_core::Result<()>;
    fn CheckHardwareCompositionSupport(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDXGIOutput6_Vtbl {
    pub const fn new<Identity: IDXGIOutput6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput6_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn CheckHardwareCompositionSupport<Identity: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput6_Impl::CheckHardwareCompositionSupport(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dxgi1_5::IDXGIOutput5_Vtbl::new::<Identity, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, OFFSET>,
            CheckHardwareCompositionSupport: CheckHardwareCompositionSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput6 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIOutput as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIOutput1 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGIOutput2 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGIOutput3 as windows_core::Interface>::IID || iid == &<super::dxgi1_4::IDXGIOutput4 as windows_core::Interface>::IID || iid == &<super::dxgi1_5::IDXGIOutput5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIOutput6 {}
