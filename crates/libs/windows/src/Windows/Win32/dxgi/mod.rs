#[inline]
pub unsafe fn CreateDXGIFactory<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn CreateDXGIFactory(riid : *const windows_core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateDXGIFactory(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CreateDXGIFactory1<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn CreateDXGIFactory1(riid : *const windows_core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateDXGIFactory1(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::winnt::LUID,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::winnt::LUID,
    pub Flags: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_ADAPTER_FLAG = i32;
pub const DXGI_ADAPTER_FLAG_FORCE_DWORD: DXGI_ADAPTER_FLAG = -1;
pub const DXGI_ADAPTER_FLAG_NONE: DXGI_ADAPTER_FLAG = 0;
pub const DXGI_ADAPTER_FLAG_REMOTE: DXGI_ADAPTER_FLAG = 1;
pub const DXGI_ADAPTER_FLAG_SOFTWARE: DXGI_ADAPTER_FLAG = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [[f32; 2]; 8],
    pub WhitePoints: [[f32; 2]; 16],
}
impl Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 1;
pub const DXGI_ENUM_MODES_SCALING: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
impl Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_MAP_DISCARD: u32 = 4;
pub const DXGI_MAP_READ: u32 = 1;
pub const DXGI_MAP_WRITE: u32 = 2;
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16;
pub const DXGI_MWA_NO_ALT_ENTER: u32 = 2;
pub const DXGI_MWA_NO_PRINT_SCREEN: u32 = 4;
pub const DXGI_MWA_NO_WINDOW_CHANGES: u32 = 1;
pub const DXGI_MWA_VALID: u32 = 7;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgitype", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::windef::RECT,
    pub AttachedToDesktop: windows_core::BOOL,
    pub Rotation: super::dxgitype::DXGI_MODE_ROTATION,
    pub Monitor: super::windef::HMONITOR,
}
#[cfg(all(feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_PRESENT_ALLOW_TEARING: u32 = 512;
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: u32 = 2;
pub const DXGI_PRESENT_DO_NOT_WAIT: u32 = 8;
pub const DXGI_PRESENT_RESTART: u32 = 4;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32;
pub const DXGI_PRESENT_TEST: u32 = 1;
pub const DXGI_PRESENT_USE_DURATION: u32 = 256;
pub type DXGI_RESIDENCY = i32;
pub const DXGI_RESIDENCY_EVICTED_TO_DISK: DXGI_RESIDENCY = 3;
pub const DXGI_RESIDENCY_FULLY_RESIDENT: DXGI_RESIDENCY = 1;
pub const DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY: DXGI_RESIDENCY = 2;
pub const DXGI_RESOURCE_PRIORITY_HIGH: u32 = 2684354560;
pub const DXGI_RESOURCE_PRIORITY_LOW: u32 = 1342177280;
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200;
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: u32 = 671088640;
pub const DXGI_RESOURCE_PRIORITY_NORMAL: u32 = 2013265920;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::winnt::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub SampleDesc: super::dxgicommon::DXGI_SAMPLE_DESC,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: super::dxgitype::DXGI_MODE_DESC,
    pub SampleDesc: super::dxgicommon::DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub OutputWindow: super::windef::HWND,
    pub Windowed: windows_core::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
pub type DXGI_SWAP_CHAIN_FLAG = i32;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: DXGI_SWAP_CHAIN_FLAG = 2;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: DXGI_SWAP_CHAIN_FLAG = 2048;
pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: DXGI_SWAP_CHAIN_FLAG = 32;
pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: DXGI_SWAP_CHAIN_FLAG = 128;
pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: DXGI_SWAP_CHAIN_FLAG = 64;
pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: DXGI_SWAP_CHAIN_FLAG = 256;
pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: DXGI_SWAP_CHAIN_FLAG = 4;
pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: DXGI_SWAP_CHAIN_FLAG = 1024;
pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: DXGI_SWAP_CHAIN_FLAG = 1;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: DXGI_SWAP_CHAIN_FLAG = 8;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: DXGI_SWAP_CHAIN_FLAG = 4096;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: DXGI_SWAP_CHAIN_FLAG = 16;
pub const DXGI_SWAP_CHAIN_FLAG_USE_DEFAULT_COLOR_SPACE: DXGI_SWAP_CHAIN_FLAG = 32768;
pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: DXGI_SWAP_CHAIN_FLAG = 512;
pub type DXGI_SWAP_EFFECT = i32;
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = 0;
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = 4;
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = 3;
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DXGI_USAGE(pub u32);
pub const DXGI_USAGE_BACK_BUFFER: u32 = 64;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: u32 = 512;
pub const DXGI_USAGE_READ_ONLY: u32 = 256;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: u32 = 32;
pub const DXGI_USAGE_SHADER_INPUT: u32 = 16;
pub const DXGI_USAGE_SHARED: u32 = 128;
pub const DXGI_USAGE_UNORDERED_ACCESS: u32 = 1024;
windows_core::imp::define_interface!(IDXGIAdapter, IDXGIAdapter_Vtbl, 0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0);
impl core::ops::Deref for IDXGIAdapter {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter, windows_core::IUnknown, IDXGIObject);
impl IDXGIAdapter {
    pub unsafe fn EnumOutputs(&self, output: u32, ppoutput: *mut Option<IDXGIOutput>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumOutputs)(windows_core::Interface::as_raw(self), output, core::mem::transmute(ppoutput)) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const windows_core::GUID) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckInterfaceSupport)(windows_core::Interface::as_raw(self), interfacename, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumOutputs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetDesc: usize,
    pub CheckInterfaceSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut i64) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIAdapter_Impl: IDXGIObject_Impl {
    fn EnumOutputs(&self, output: u32, ppoutput: windows_core::OutRef<IDXGIOutput>) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::Result<()>;
    fn CheckInterfaceSupport(&self, interfacename: *const windows_core::GUID) -> windows_core::Result<i64>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIAdapter_Vtbl {
    pub const fn new<Identity: IDXGIAdapter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumOutputs<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, output: u32, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter_Impl::EnumOutputs(this, core::mem::transmute_copy(&output), core::mem::transmute_copy(&ppoutput)).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn CheckInterfaceSupport<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacename: *const windows_core::GUID, pumdversion: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter_Impl::CheckInterfaceSupport(this, core::mem::transmute_copy(&interfacename)) {
                    Ok(ok__) => {
                        pumdversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            EnumOutputs: EnumOutputs::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            CheckInterfaceSupport: CheckInterfaceSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIAdapter {}
windows_core::imp::define_interface!(IDXGIAdapter1, IDXGIAdapter1_Vtbl, 0x29038f61_3839_4626_91fd_086879011a05);
impl core::ops::Deref for IDXGIAdapter1 {
    type Target = IDXGIAdapter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter1, windows_core::IUnknown, IDXGIObject, IDXGIAdapter);
impl IDXGIAdapter1 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter1_Vtbl {
    pub base__: IDXGIAdapter_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC1) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetDesc1: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIAdapter1_Impl: IDXGIAdapter_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIAdapter1_Vtbl {
    pub const fn new<Identity: IDXGIAdapter1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: IDXGIAdapter1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self { base__: IDXGIAdapter_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIAdapter1 {}
windows_core::imp::define_interface!(IDXGIDevice, IDXGIDevice_Vtbl, 0x54ec77fa_1377_44e6_8c32_88fd5f44c84c);
impl core::ops::Deref for IDXGIDevice {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice, windows_core::IUnknown, IDXGIObject);
impl IDXGIDevice {
    pub unsafe fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [Option<IDXGISurface>]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), pdesc, ppsurface.len().try_into().unwrap(), usage, psharedresource.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppsurface.as_ptr())) }
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const Option<windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryResourceResidency)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources), presidencystatus as _, numresources) }
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGPUThreadPriority)(windows_core::Interface::as_raw(self), priority) }
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGPUThreadPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
    pub CreateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_SURFACE_DESC, u32, DXGI_USAGE, *const DXGI_SHARED_RESOURCE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt")))]
    CreateSurface: usize,
    pub QueryResourceResidency: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, *mut DXGI_RESIDENCY, u32) -> windows_core::HRESULT,
    pub SetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait IDXGIDevice_Impl: IDXGIObject_Impl {
    fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter>;
    fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut Option<IDXGISurface>) -> windows_core::Result<()>;
    fn QueryResourceResidency(&self, ppresources: *const Option<windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::Result<()>;
    fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()>;
    fn GetGPUThreadPriority(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl IDXGIDevice_Vtbl {
    pub const fn new<Identity: IDXGIDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAdapter<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice_Impl::GetAdapter(this) {
                    Ok(ok__) => {
                        padapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::CreateSurface(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&numsurfaces), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&psharedresource), core::mem::transmute_copy(&ppsurface)).into()
            }
        }
        unsafe extern "system" fn QueryResourceResidency<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::QueryResourceResidency(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&presidencystatus), core::mem::transmute_copy(&numresources)).into()
            }
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::SetGPUThreadPriority(this, core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice_Impl::GetGPUThreadPriority(this) {
                    Ok(ok__) => {
                        ppriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            QueryResourceResidency: QueryResourceResidency::<Identity, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIDevice {}
windows_core::imp::define_interface!(IDXGIDevice1, IDXGIDevice1_Vtbl, 0x77db970f_6276_48ba_ba28_070143b4392c);
impl core::ops::Deref for IDXGIDevice1 {
    type Target = IDXGIDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice1, windows_core::IUnknown, IDXGIObject, IDXGIDevice);
impl IDXGIDevice1 {
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(windows_core::Interface::as_raw(self), maxlatency) }
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice1_Vtbl {
    pub base__: IDXGIDevice_Vtbl,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait IDXGIDevice1_Impl: IDXGIDevice_Impl {
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl IDXGIDevice1_Vtbl {
    pub const fn new<Identity: IDXGIDevice1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice1_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice1_Impl::GetMaximumFrameLatency(this) {
                    Ok(ok__) => {
                        pmaxlatency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIDevice_Vtbl::new::<Identity, OFFSET>(),
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIDevice1 {}
windows_core::imp::define_interface!(IDXGIDeviceSubObject, IDXGIDeviceSubObject_Vtbl, 0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6);
impl core::ops::Deref for IDXGIDeviceSubObject {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDeviceSubObject, windows_core::IUnknown, IDXGIObject);
impl IDXGIDeviceSubObject {
    pub unsafe fn GetDevice<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDeviceSubObject_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDXGIDeviceSubObject_Impl: IDXGIObject_Impl {
    fn GetDevice(&self, riid: *const windows_core::GUID, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDXGIDeviceSubObject_Vtbl {
    pub const fn new<Identity: IDXGIDeviceSubObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDXGIDeviceSubObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDeviceSubObject_Impl::GetDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        Self { base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(), GetDevice: GetDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDeviceSubObject {}
windows_core::imp::define_interface!(IDXGIFactory, IDXGIFactory_Vtbl, 0x7b7166ec_21c7_44ae_b21a_c9ae321ae369);
impl core::ops::Deref for IDXGIFactory {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory, windows_core::IUnknown, IDXGIObject);
impl IDXGIFactory {
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdapters)(windows_core::Interface::as_raw(self), adapter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn MakeWindowAssociation(&self, windowhandle: super::windef::HWND, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MakeWindowAssociation)(windows_core::Interface::as_raw(self), windowhandle, flags) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetWindowAssociation(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindowAssociation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> windows_core::Result<IDXGISwapChain>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChain)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pdesc, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn CreateSoftwareAdapter(&self, module: super::minwindef::HMODULE) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSoftwareAdapter)(windows_core::Interface::as_raw(self), module, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub MakeWindowAssociation: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    MakeWindowAssociation: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetWindowAssociation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetWindowAssociation: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
    pub CreateSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DXGI_SWAP_CHAIN_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef")))]
    CreateSwapChain: usize,
    #[cfg(feature = "Win32_minwindef")]
    pub CreateSoftwareAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::HMODULE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    CreateSoftwareAdapter: usize,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDXGIFactory_Impl: IDXGIObject_Impl {
    fn EnumAdapters(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter>;
    fn MakeWindowAssociation(&self, windowhandle: super::windef::HWND, flags: u32) -> windows_core::Result<()>;
    fn GetWindowAssociation(&self) -> windows_core::Result<super::windef::HWND>;
    fn CreateSwapChain(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> windows_core::Result<IDXGISwapChain>;
    fn CreateSoftwareAdapter(&self, module: super::minwindef::HMODULE) -> windows_core::Result<IDXGIAdapter>;
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDXGIFactory_Vtbl {
    pub const fn new<Identity: IDXGIFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapters<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory_Impl::EnumAdapters(this, core::mem::transmute_copy(&adapter)) {
                    Ok(ok__) => {
                        ppadapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MakeWindowAssociation<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::windef::HWND, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory_Impl::MakeWindowAssociation(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetWindowAssociation<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwindowhandle: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory_Impl::GetWindowAssociation(this) {
                    Ok(ok__) => {
                        pwindowhandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSwapChain<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory_Impl::CreateSwapChain(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pdesc)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSoftwareAdapter<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: super::minwindef::HMODULE, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory_Impl::CreateSoftwareAdapter(this, core::mem::transmute_copy(&module)) {
                    Ok(ok__) => {
                        ppadapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            EnumAdapters: EnumAdapters::<Identity, OFFSET>,
            MakeWindowAssociation: MakeWindowAssociation::<Identity, OFFSET>,
            GetWindowAssociation: GetWindowAssociation::<Identity, OFFSET>,
            CreateSwapChain: CreateSwapChain::<Identity, OFFSET>,
            CreateSoftwareAdapter: CreateSoftwareAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIFactory {}
windows_core::imp::define_interface!(IDXGIFactory1, IDXGIFactory1_Vtbl, 0x770aae78_f26f_4dba_a829_253c83d1b387);
impl core::ops::Deref for IDXGIFactory1 {
    type Target = IDXGIFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory1, windows_core::IUnknown, IDXGIObject, IDXGIFactory);
impl IDXGIFactory1 {
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdapters1)(windows_core::Interface::as_raw(self), adapter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsCurrent(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsCurrent)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory1_Vtbl {
    pub base__: IDXGIFactory_Vtbl,
    pub EnumAdapters1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCurrent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDXGIFactory1_Impl: IDXGIFactory_Impl {
    fn EnumAdapters1(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter1>;
    fn IsCurrent(&self) -> windows_core::BOOL;
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDXGIFactory1_Vtbl {
    pub const fn new<Identity: IDXGIFactory1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapters1<Identity: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory1_Impl::EnumAdapters1(this, core::mem::transmute_copy(&adapter)) {
                    Ok(ok__) => {
                        ppadapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCurrent<Identity: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory1_Impl::IsCurrent(this)
            }
        }
        Self { base__: IDXGIFactory_Vtbl::new::<Identity, OFFSET>(), EnumAdapters1: EnumAdapters1::<Identity, OFFSET>, IsCurrent: IsCurrent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIFactory1 {}
windows_core::imp::define_interface!(IDXGIKeyedMutex, IDXGIKeyedMutex_Vtbl, 0x9d8e1289_d7b3_465f_8126_250e349af85d);
impl core::ops::Deref for IDXGIKeyedMutex {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIKeyedMutex, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGIKeyedMutex {
    pub unsafe fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcquireSync)(windows_core::Interface::as_raw(self), key, dwmilliseconds) }
    }
    pub unsafe fn ReleaseSync(&self, key: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseSync)(windows_core::Interface::as_raw(self), key) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIKeyedMutex_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub AcquireSync: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32) -> windows_core::HRESULT,
    pub ReleaseSync: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
pub trait IDXGIKeyedMutex_Impl: IDXGIDeviceSubObject_Impl {
    fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn ReleaseSync(&self, key: u64) -> windows_core::Result<()>;
}
impl IDXGIKeyedMutex_Vtbl {
    pub const fn new<Identity: IDXGIKeyedMutex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AcquireSync<Identity: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: u64, dwmilliseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIKeyedMutex_Impl::AcquireSync(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&dwmilliseconds)).into()
            }
        }
        unsafe extern "system" fn ReleaseSync<Identity: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIKeyedMutex_Impl::ReleaseSync(this, core::mem::transmute_copy(&key)).into()
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            AcquireSync: AcquireSync::<Identity, OFFSET>,
            ReleaseSync: ReleaseSync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIKeyedMutex as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIKeyedMutex {}
windows_core::imp::define_interface!(IDXGIObject, IDXGIObject_Vtbl, 0xaec22fb8_76f3_4639_9be0_28eb43a67a2e);
windows_core::imp::interface_hierarchy!(IDXGIObject, windows_core::IUnknown);
impl IDXGIObject {
    pub unsafe fn SetPrivateData(&self, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), name, datasize, pdata) }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, name: *const windows_core::GUID, punknown: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), name, punknown.param().abi()) }
    }
    pub unsafe fn GetPrivateData(&self, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), name, pdatasize as _, pdata as _) }
    }
    pub unsafe fn GetParent<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDXGIObject_Impl: windows_core::IUnknownImpl {
    fn SetPrivateData(&self, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, name: *const windows_core::GUID, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetPrivateData(&self, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetParent(&self, riid: *const windows_core::GUID, ppparent: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDXGIObject_Vtbl {
    pub const fn new<Identity: IDXGIObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPrivateData<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetParent<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetParent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparent)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIObject {}
windows_core::imp::define_interface!(IDXGIOutput, IDXGIOutput_Vtbl, 0xae02eedb_c735_4690_8d52_5a8dc20213aa);
impl core::ops::Deref for IDXGIOutput {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput, windows_core::IUnknown, IDXGIObject);
impl IDXGIOutput {
    #[cfg(all(feature = "Win32_dxgitype", feature = "Win32_windef"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub unsafe fn GetDisplayModeList(&self, enumformat: super::dxgiformat::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: Option<*mut super::dxgitype::DXGI_MODE_DESC>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeList)(windows_core::Interface::as_raw(self), enumformat, flags, pnummodes as _, pdesc.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub unsafe fn FindClosestMatchingMode<P2>(&self, pmodetomatch: *const super::dxgitype::DXGI_MODE_DESC, pclosestmatch: *mut super::dxgitype::DXGI_MODE_DESC, pconcerneddevice: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindClosestMatchingMode)(windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch as _, pconcerneddevice.param().abi()) }
    }
    pub unsafe fn WaitForVBlank(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitForVBlank)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TakeOwnership<P0>(&self, pdevice: P0, exclusive: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).TakeOwnership)(windows_core::Interface::as_raw(self), pdevice.param().abi(), exclusive.into()) }
    }
    pub unsafe fn ReleaseOwnership(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseOwnership)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_dxgitype")]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut super::dxgitype::DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGammaControlCapabilities)(windows_core::Interface::as_raw(self), pgammacaps as _) }
    }
    #[cfg(feature = "Win32_dxgitype")]
    pub unsafe fn SetGammaControl(&self, parray: *const super::dxgitype::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGammaControl)(windows_core::Interface::as_raw(self), parray) }
    }
    #[cfg(feature = "Win32_dxgitype")]
    pub unsafe fn GetGammaControl(&self, parray: *mut super::dxgitype::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGammaControl)(windows_core::Interface::as_raw(self), parray as _) }
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDXGISurface>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDisplaySurface)(windows_core::Interface::as_raw(self), pscanoutsurface.param().abi()) }
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDXGISurface>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDisplaySurfaceData)(windows_core::Interface::as_raw(self), pdestination.param().abi()) }
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrameStatistics)(windows_core::Interface::as_raw(self), pstats as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    #[cfg(all(feature = "Win32_dxgitype", feature = "Win32_windef"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_OUTPUT_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgitype", feature = "Win32_windef")))]
    GetDesc: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub GetDisplayModeList: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, u32, *mut u32, *mut super::dxgitype::DXGI_MODE_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype")))]
    GetDisplayModeList: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub FindClosestMatchingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dxgitype::DXGI_MODE_DESC, *mut super::dxgitype::DXGI_MODE_DESC, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype")))]
    FindClosestMatchingMode: usize,
    pub WaitForVBlank: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ReleaseOwnership: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_dxgitype")]
    pub GetGammaControlCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dxgitype::DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgitype"))]
    GetGammaControlCapabilities: usize,
    #[cfg(feature = "Win32_dxgitype")]
    pub SetGammaControl: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dxgitype::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgitype"))]
    SetGammaControl: usize,
    #[cfg(feature = "Win32_dxgitype")]
    pub GetGammaControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dxgitype::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgitype"))]
    GetGammaControl: usize,
    pub SetDisplaySurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplaySurfaceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDXGIOutput_Impl: IDXGIObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::Result<()>;
    fn GetDisplayModeList(&self, enumformat: super::dxgiformat::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut super::dxgitype::DXGI_MODE_DESC) -> windows_core::Result<()>;
    fn FindClosestMatchingMode(&self, pmodetomatch: *const super::dxgitype::DXGI_MODE_DESC, pclosestmatch: *mut super::dxgitype::DXGI_MODE_DESC, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn WaitForVBlank(&self) -> windows_core::Result<()>;
    fn TakeOwnership(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, exclusive: windows_core::BOOL) -> windows_core::Result<()>;
    fn ReleaseOwnership(&self);
    fn GetGammaControlCapabilities(&self, pgammacaps: *mut super::dxgitype::DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::Result<()>;
    fn SetGammaControl(&self, parray: *const super::dxgitype::DXGI_GAMMA_CONTROL) -> windows_core::Result<()>;
    fn GetGammaControl(&self, parray: *mut super::dxgitype::DXGI_GAMMA_CONTROL) -> windows_core::Result<()>;
    fn SetDisplaySurface(&self, pscanoutsurface: windows_core::Ref<IDXGISurface>) -> windows_core::Result<()>;
    fn GetDisplaySurfaceData(&self, pdestination: windows_core::Ref<IDXGISurface>) -> windows_core::Result<()>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDXGIOutput_Vtbl {
    pub const fn new<Identity: IDXGIOutput_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetDisplayModeList<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: super::dxgiformat::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut super::dxgitype::DXGI_MODE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetDisplayModeList(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pnummodes), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn FindClosestMatchingMode<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodetomatch: *const super::dxgitype::DXGI_MODE_DESC, pclosestmatch: *mut super::dxgitype::DXGI_MODE_DESC, pconcerneddevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::FindClosestMatchingMode(this, core::mem::transmute_copy(&pmodetomatch), core::mem::transmute_copy(&pclosestmatch), core::mem::transmute_copy(&pconcerneddevice)).into()
            }
        }
        unsafe extern "system" fn WaitForVBlank<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::WaitForVBlank(this).into()
            }
        }
        unsafe extern "system" fn TakeOwnership<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, exclusive: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::TakeOwnership(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&exclusive)).into()
            }
        }
        unsafe extern "system" fn ReleaseOwnership<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::ReleaseOwnership(this);
            }
        }
        unsafe extern "system" fn GetGammaControlCapabilities<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgammacaps: *mut super::dxgitype::DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetGammaControlCapabilities(this, core::mem::transmute_copy(&pgammacaps)).into()
            }
        }
        unsafe extern "system" fn SetGammaControl<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *const super::dxgitype::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::SetGammaControl(this, core::mem::transmute_copy(&parray)).into()
            }
        }
        unsafe extern "system" fn GetGammaControl<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *mut super::dxgitype::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetGammaControl(this, core::mem::transmute_copy(&parray)).into()
            }
        }
        unsafe extern "system" fn SetDisplaySurface<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscanoutsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::SetDisplaySurface(this, core::mem::transmute_copy(&pscanoutsurface)).into()
            }
        }
        unsafe extern "system" fn GetDisplaySurfaceData<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetDisplaySurfaceData(this, core::mem::transmute_copy(&pdestination)).into()
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetDisplayModeList: GetDisplayModeList::<Identity, OFFSET>,
            FindClosestMatchingMode: FindClosestMatchingMode::<Identity, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, OFFSET>,
            ReleaseOwnership: ReleaseOwnership::<Identity, OFFSET>,
            GetGammaControlCapabilities: GetGammaControlCapabilities::<Identity, OFFSET>,
            SetGammaControl: SetGammaControl::<Identity, OFFSET>,
            GetGammaControl: GetGammaControl::<Identity, OFFSET>,
            SetDisplaySurface: SetDisplaySurface::<Identity, OFFSET>,
            GetDisplaySurfaceData: GetDisplaySurfaceData::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIOutput {}
windows_core::imp::define_interface!(IDXGIResource, IDXGIResource_Vtbl, 0x035f3ab4_482e_4e50_b41f_8a7f8bd8960b);
impl core::ops::Deref for IDXGIResource {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIResource, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGIResource {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetSharedHandle(&self) -> windows_core::Result<super::winnt::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSharedHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUsage(&self) -> windows_core::Result<DXGI_USAGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUsage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEvictionPriority)(windows_core::Interface::as_raw(self), evictionpriority) }
    }
    pub unsafe fn GetEvictionPriority(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEvictionPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetSharedHandle: usize,
    pub GetUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_USAGE) -> windows_core::HRESULT,
    pub SetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIResource_Impl: IDXGIDeviceSubObject_Impl {
    fn GetSharedHandle(&self) -> windows_core::Result<super::winnt::HANDLE>;
    fn GetUsage(&self) -> windows_core::Result<DXGI_USAGE>;
    fn SetEvictionPriority(&self, evictionpriority: u32) -> windows_core::Result<()>;
    fn GetEvictionPriority(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIResource_Vtbl {
    pub const fn new<Identity: IDXGIResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSharedHandle<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource_Impl::GetSharedHandle(this) {
                    Ok(ok__) => {
                        psharedhandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUsage<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusage: *mut DXGI_USAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource_Impl::GetUsage(this) {
                    Ok(ok__) => {
                        pusage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, evictionpriority: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIResource_Impl::SetEvictionPriority(this, core::mem::transmute_copy(&evictionpriority)).into()
            }
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevictionpriority: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource_Impl::GetEvictionPriority(this) {
                    Ok(ok__) => {
                        pevictionpriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            GetSharedHandle: GetSharedHandle::<Identity, OFFSET>,
            GetUsage: GetUsage::<Identity, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIResource as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIResource {}
windows_core::imp::define_interface!(IDXGISurface, IDXGISurface_Vtbl, 0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec);
impl core::ops::Deref for IDXGISurface {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISurface, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGISurface {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), plockedrect as _, mapflags) }
    }
    pub unsafe fn Unmap(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SURFACE_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    GetDesc: usize,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_MAPPED_RECT, u32) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait IDXGISurface_Impl: IDXGIDeviceSubObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> windows_core::Result<()>;
    fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> windows_core::Result<()>;
    fn Unmap(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl IDXGISurface_Vtbl {
    pub const fn new<Identity: IDXGISurface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn Map<Identity: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface_Impl::Map(this, core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&mapflags)).into()
            }
        }
        unsafe extern "system" fn Unmap<Identity: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface_Impl::Unmap(this).into()
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for IDXGISurface {}
windows_core::imp::define_interface!(IDXGISurface1, IDXGISurface1_Vtbl, 0x4ae63092_6327_4c1b_80ae_bfe12ea32b86);
impl core::ops::Deref for IDXGISurface1 {
    type Target = IDXGISurface;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISurface1, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISurface);
impl IDXGISurface1 {
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetDC(&self, discard: bool) -> windows_core::Result<super::windef::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), discard.into(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn ReleaseDC(&self, pdirtyrect: Option<*const super::windef::RECT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), pdirtyrect.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface1_Vtbl {
    pub base__: IDXGISurface_Vtbl,
    #[cfg(feature = "Win32_windef")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut super::windef::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetDC: usize,
    #[cfg(feature = "Win32_windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    ReleaseDC: usize,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
pub trait IDXGISurface1_Impl: IDXGISurface_Impl {
    fn GetDC(&self, discard: windows_core::BOOL) -> windows_core::Result<super::windef::HDC>;
    fn ReleaseDC(&self, pdirtyrect: *const super::windef::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl IDXGISurface1_Vtbl {
    pub const fn new<Identity: IDXGISurface1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDC<Identity: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discard: windows_core::BOOL, phdc: *mut super::windef::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISurface1_Impl::GetDC(this, core::mem::transmute_copy(&discard)) {
                    Ok(ok__) => {
                        phdc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtyrect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface1_Impl::ReleaseDC(this, core::mem::transmute_copy(&pdirtyrect)).into()
            }
        }
        Self { base__: IDXGISurface_Vtbl::new::<Identity, OFFSET>(), GetDC: GetDC::<Identity, OFFSET>, ReleaseDC: ReleaseDC::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISurface as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGISurface1 {}
windows_core::imp::define_interface!(IDXGISwapChain, IDXGISwapChain_Vtbl, 0x310d36a0_d2e7_4c0a_aa04_6a9d23b8886a);
impl core::ops::Deref for IDXGISwapChain {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGISwapChain {
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), syncinterval, flags) }
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), buffer, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn SetFullscreenState<P1>(&self, fullscreen: bool, ptarget: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDXGIOutput>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFullscreenState)(windows_core::Interface::as_raw(self), fullscreen.into(), ptarget.param().abi()) }
    }
    pub unsafe fn GetFullscreenState(&self, pfullscreen: Option<*mut windows_core::BOOL>, pptarget: Option<*mut Option<IDXGIOutput>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFullscreenState)(windows_core::Interface::as_raw(self), pfullscreen.unwrap_or(core::mem::zeroed()) as _, pptarget.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: super::dxgiformat::DXGI_FORMAT, swapchainflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResizeBuffers)(windows_core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const super::dxgitype::DXGI_MODE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResizeTarget)(windows_core::Interface::as_raw(self), pnewtargetparameters) }
    }
    pub unsafe fn GetContainingOutput(&self) -> windows_core::Result<IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainingOutput)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrameStatistics)(windows_core::Interface::as_raw(self), pstats as _) }
    }
    pub unsafe fn GetLastPresentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPresentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFullscreenState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFullscreenState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub ResizeBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, super::dxgiformat::DXGI_FORMAT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    ResizeBuffers: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub ResizeTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dxgitype::DXGI_MODE_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype")))]
    ResizeTarget: usize,
    pub GetContainingOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT,
    pub GetLastPresentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDXGISwapChain_Impl: IDXGIDeviceSubObject_Impl {
    fn Present(&self, syncinterval: u32, flags: u32) -> windows_core::Result<()>;
    fn GetBuffer(&self, buffer: u32, riid: *const windows_core::GUID, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetFullscreenState(&self, fullscreen: windows_core::BOOL, ptarget: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<()>;
    fn GetFullscreenState(&self, pfullscreen: *mut windows_core::BOOL, pptarget: windows_core::OutRef<IDXGIOutput>) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::Result<()>;
    fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: super::dxgiformat::DXGI_FORMAT, swapchainflags: u32) -> windows_core::Result<()>;
    fn ResizeTarget(&self, pnewtargetparameters: *const super::dxgitype::DXGI_MODE_DESC) -> windows_core::Result<()>;
    fn GetContainingOutput(&self) -> windows_core::Result<IDXGIOutput>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()>;
    fn GetLastPresentCount(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDXGISwapChain_Vtbl {
    pub const fn new<Identity: IDXGISwapChain_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Present<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncinterval: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::Present(this, core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: u32, riid: *const windows_core::GUID, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::GetBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppsurface)).into()
            }
        }
        unsafe extern "system" fn SetFullscreenState<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullscreen: windows_core::BOOL, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::SetFullscreenState(this, core::mem::transmute_copy(&fullscreen), core::mem::transmute_copy(&ptarget)).into()
            }
        }
        unsafe extern "system" fn GetFullscreenState<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfullscreen: *mut windows_core::BOOL, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::GetFullscreenState(this, core::mem::transmute_copy(&pfullscreen), core::mem::transmute_copy(&pptarget)).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn ResizeBuffers<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: super::dxgiformat::DXGI_FORMAT, swapchainflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::ResizeBuffers(this, core::mem::transmute_copy(&buffercount), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&newformat), core::mem::transmute_copy(&swapchainflags)).into()
            }
        }
        unsafe extern "system" fn ResizeTarget<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewtargetparameters: *const super::dxgitype::DXGI_MODE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::ResizeTarget(this, core::mem::transmute_copy(&pnewtargetparameters)).into()
            }
        }
        unsafe extern "system" fn GetContainingOutput<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain_Impl::GetContainingOutput(this) {
                    Ok(ok__) => {
                        ppoutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn GetLastPresentCount<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastpresentcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain_Impl::GetLastPresentCount(this) {
                    Ok(ok__) => {
                        plastpresentcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            Present: Present::<Identity, OFFSET>,
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            SetFullscreenState: SetFullscreenState::<Identity, OFFSET>,
            GetFullscreenState: GetFullscreenState::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            ResizeBuffers: ResizeBuffers::<Identity, OFFSET>,
            ResizeTarget: ResizeTarget::<Identity, OFFSET>,
            GetContainingOutput: GetContainingOutput::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
            GetLastPresentCount: GetLastPresentCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGISwapChain {}
