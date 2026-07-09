#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_ADAPTER_DESC2 {
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
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_ALPHA_MODE = i32;
pub const DXGI_ALPHA_MODE_FORCE_DWORD: DXGI_ALPHA_MODE = -1;
pub const DXGI_ALPHA_MODE_IGNORE: DXGI_ALPHA_MODE = 3;
pub const DXGI_ALPHA_MODE_PREMULTIPLIED: DXGI_ALPHA_MODE = 1;
pub const DXGI_ALPHA_MODE_STRAIGHT: DXGI_ALPHA_MODE = 2;
pub const DXGI_ALPHA_MODE_UNSPECIFIED: DXGI_ALPHA_MODE = 0;
pub const DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 1;
pub const DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 0;
pub type DXGI_COMPUTE_PREEMPTION_GRANULARITY = i32;
pub const DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 4;
pub const DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 3;
pub const DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 2;
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 8;
pub const DXGI_ENUM_MODES_STEREO: u32 = 4;
pub const DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 0;
pub type DXGI_GRAPHICS_PREEMPTION_GRANULARITY = i32;
pub const DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 4;
pub const DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 3;
pub const DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 1;
pub const DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: super::dxgicommon::DXGI_RATIONAL,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ScanlineOrdering: super::dxgitype::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: super::dxgitype::DXGI_MODE_SCALING,
    pub Stereo: windows_core::BOOL,
}
pub type DXGI_OFFER_RESOURCE_PRIORITY = i32;
pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: DXGI_OFFER_RESOURCE_PRIORITY = 3;
pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: DXGI_OFFER_RESOURCE_PRIORITY = 1;
pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: DXGI_OFFER_RESOURCE_PRIORITY = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: super::dxgitype::DXGI_MODE_DESC,
    pub Rotation: super::dxgitype::DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: windows_core::BOOL,
    pub ProtectedContentMaskedOut: windows_core::BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::windef::POINT,
    pub DestinationRect: super::windef::RECT,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::windef::POINT,
    pub Visible: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: u32,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::windef::POINT,
}
pub type DXGI_OUTDUPL_POINTER_SHAPE_TYPE = i32;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 2;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 4;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 1;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut super::windef::RECT,
    pub pScrollRect: *mut super::windef::RECT,
    pub pScrollOffset: *mut super::windef::POINT,
}
#[cfg(feature = "Win32_windef")]
impl Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_SCALING = i32;
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = 2;
pub const DXGI_SCALING_NONE: DXGI_SCALING = 1;
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = 0;
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648;
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub Stereo: windows_core::BOOL,
    pub SampleDesc: super::dxgicommon::DXGI_SAMPLE_DESC,
    pub BufferUsage: super::dxgi::DXGI_USAGE,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: super::dxgi::DXGI_SWAP_EFFECT,
    pub AlphaMode: DXGI_ALPHA_MODE,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: super::dxgicommon::DXGI_RATIONAL,
    pub ScanlineOrdering: super::dxgitype::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: super::dxgitype::DXGI_MODE_SCALING,
    pub Windowed: windows_core::BOOL,
}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::define_interface!(IDXGIAdapter2, IDXGIAdapter2_Vtbl, 0x0aa1ae0a_fa0e_4b84_8644_e05ff8e5acb5);
#[cfg(feature = "Win32_dxgi")]
impl core::ops::Deref for IDXGIAdapter2 {
    type Target = super::dxgi::IDXGIAdapter1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::interface_hierarchy!(IDXGIAdapter2, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIAdapter, super::dxgi::IDXGIAdapter1);
#[cfg(feature = "Win32_dxgi")]
impl IDXGIAdapter2 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc2)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[cfg(feature = "Win32_dxgi")]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter2_Vtbl {
    pub base__: super::dxgi::IDXGIAdapter1_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetDesc2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetDesc2: usize,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_winnt"))]
pub trait IDXGIAdapter2_Impl: super::dxgi::IDXGIAdapter1_Impl {
    fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_winnt"))]
impl IDXGIAdapter2_Vtbl {
    pub const fn new<Identity: IDXGIAdapter2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc2<Identity: IDXGIAdapter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter2_Impl::GetDesc2(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self { base__: super::dxgi::IDXGIAdapter1_Vtbl::new::<Identity, OFFSET>(), GetDesc2: GetDesc2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter2 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIAdapter as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIAdapter1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIAdapter2 {}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::define_interface!(IDXGIDevice2, IDXGIDevice2_Vtbl, 0x05008617_fbfd_4051_a790_144884b4f6a9);
#[cfg(feature = "Win32_dxgi")]
impl core::ops::Deref for IDXGIDevice2 {
    type Target = super::dxgi::IDXGIDevice1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::interface_hierarchy!(IDXGIDevice2, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIDevice, super::dxgi::IDXGIDevice1);
#[cfg(feature = "Win32_dxgi")]
impl IDXGIDevice2 {
    pub unsafe fn OfferResources(&self, ppresources: &[Option<super::dxgi::IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OfferResources)(windows_core::Interface::as_raw(self), ppresources.len().try_into().unwrap(), core::mem::transmute(ppresources.as_ptr()), priority) }
    }
    pub unsafe fn ReclaimResources(&self, numresources: u32, ppresources: *const Option<super::dxgi::IDXGIResource>, pdiscarded: Option<*mut windows_core::BOOL>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReclaimResources)(windows_core::Interface::as_raw(self), numresources, core::mem::transmute(ppresources), pdiscarded.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn EnqueueSetEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnqueueSetEvent)(windows_core::Interface::as_raw(self), hevent) }
    }
}
#[cfg(feature = "Win32_dxgi")]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice2_Vtbl {
    pub base__: super::dxgi::IDXGIDevice1_Vtbl,
    pub OfferResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::HRESULT,
    pub ReclaimResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub EnqueueSetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    EnqueueSetEvent: usize,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait IDXGIDevice2_Impl: super::dxgi::IDXGIDevice1_Impl {
    fn OfferResources(&self, numresources: u32, ppresources: *const Option<super::dxgi::IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::Result<()>;
    fn ReclaimResources(&self, numresources: u32, ppresources: *const Option<super::dxgi::IDXGIResource>, pdiscarded: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn EnqueueSetEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl IDXGIDevice2_Vtbl {
    pub const fn new<Identity: IDXGIDevice2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OfferResources<Identity: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice2_Impl::OfferResources(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn ReclaimResources<Identity: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, pdiscarded: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice2_Impl::ReclaimResources(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&pdiscarded)).into()
            }
        }
        unsafe extern "system" fn EnqueueSetEvent<Identity: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice2_Impl::EnqueueSetEvent(this, core::mem::transmute_copy(&hevent)).into()
            }
        }
        Self {
            base__: super::dxgi::IDXGIDevice1_Vtbl::new::<Identity, OFFSET>(),
            OfferResources: OfferResources::<Identity, OFFSET>,
            ReclaimResources: ReclaimResources::<Identity, OFFSET>,
            EnqueueSetEvent: EnqueueSetEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice2 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDevice as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDevice1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIDevice2 {}
windows_core::imp::define_interface!(IDXGIDisplayControl, IDXGIDisplayControl_Vtbl, 0xea9dbf1a_c88e_4486_854a_98aa0138f30c);
windows_core::imp::interface_hierarchy!(IDXGIDisplayControl, windows_core::IUnknown);
impl IDXGIDisplayControl {
    pub unsafe fn IsStereoEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsStereoEnabled)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetStereoEnabled(&self, enabled: bool) {
        unsafe {
            (windows_core::Interface::vtable(self).SetStereoEnabled)(windows_core::Interface::as_raw(self), enabled.into());
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDisplayControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsStereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetStereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL),
}
pub trait IDXGIDisplayControl_Impl: windows_core::IUnknownImpl {
    fn IsStereoEnabled(&self) -> windows_core::BOOL;
    fn SetStereoEnabled(&self, enabled: windows_core::BOOL);
}
impl IDXGIDisplayControl_Vtbl {
    pub const fn new<Identity: IDXGIDisplayControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsStereoEnabled<Identity: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDisplayControl_Impl::IsStereoEnabled(this)
            }
        }
        unsafe extern "system" fn SetStereoEnabled<Identity: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDisplayControl_Impl::SetStereoEnabled(this, core::mem::transmute_copy(&enabled));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStereoEnabled: IsStereoEnabled::<Identity, OFFSET>,
            SetStereoEnabled: SetStereoEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDisplayControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDisplayControl {}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::define_interface!(IDXGIFactory2, IDXGIFactory2_Vtbl, 0x50c83a1c_e072_4c48_87b0_3630fa36a6d0);
#[cfg(feature = "Win32_dxgi")]
impl core::ops::Deref for IDXGIFactory2 {
    type Target = super::dxgi::IDXGIFactory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::interface_hierarchy!(IDXGIFactory2, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIFactory, super::dxgi::IDXGIFactory1);
#[cfg(feature = "Win32_dxgi")]
impl IDXGIFactory2 {
    pub unsafe fn IsWindowedStereoEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsWindowedStereoEnabled)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P4>(&self, pdevice: P0, hwnd: super::windef::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P4) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<super::dxgi::IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForHwnd)(windows_core::Interface::as_raw(self), pdevice.param().abi(), hwnd, pdesc, pfullscreendesc.unwrap_or(core::mem::zeroed()) as _, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P3>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P3) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<super::dxgi::IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForCoreWindow)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pwindow.param().abi(), pdesc, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetSharedResourceAdapterLuid(&self, hresource: super::winnt::HANDLE) -> windows_core::Result<super::winnt::LUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSharedResourceAdapterLuid)(windows_core::Interface::as_raw(self), hresource, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn RegisterStereoStatusWindow(&self, windowhandle: super::windef::HWND, wmsg: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusWindow)(windows_core::Interface::as_raw(self), windowhandle, wmsg, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn RegisterStereoStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterStereoStatus)(windows_core::Interface::as_raw(self), dwcookie);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn RegisterOcclusionStatusWindow(&self, windowhandle: super::windef::HWND, wmsg: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusWindow)(windows_core::Interface::as_raw(self), windowhandle, wmsg, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn RegisterOcclusionStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterOcclusionStatus)(windows_core::Interface::as_raw(self), dwcookie);
        }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P2>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<super::dxgi::IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForComposition)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pdesc, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dxgi")]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory2_Vtbl {
    pub base__: super::dxgi::IDXGIFactory1_Vtbl,
    pub IsWindowedStereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
    pub CreateSwapChainForHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::windef::HWND, *const DXGI_SWAP_CHAIN_DESC1, *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef")))]
    CreateSwapChainForHwnd: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CreateSwapChainForCoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const DXGI_SWAP_CHAIN_DESC1, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CreateSwapChainForCoreWindow: usize,
    #[cfg(feature = "Win32_winnt")]
    pub GetSharedResourceAdapterLuid: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut super::winnt::LUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetSharedResourceAdapterLuid: usize,
    #[cfg(feature = "Win32_windef")]
    pub RegisterStereoStatusWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    RegisterStereoStatusWindow: usize,
    #[cfg(feature = "Win32_winnt")]
    pub RegisterStereoStatusEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    RegisterStereoStatusEvent: usize,
    pub UnregisterStereoStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_windef")]
    pub RegisterOcclusionStatusWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    RegisterOcclusionStatusWindow: usize,
    #[cfg(feature = "Win32_winnt")]
    pub RegisterOcclusionStatusEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    RegisterOcclusionStatusEvent: usize,
    pub UnregisterOcclusionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CreateSwapChainForComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DXGI_SWAP_CHAIN_DESC1, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CreateSwapChainForComposition: usize,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory2_Impl: super::dxgi::IDXGIFactory1_Impl {
    fn IsWindowedStereoEnabled(&self) -> windows_core::BOOL;
    fn CreateSwapChainForHwnd(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, hwnd: super::windef::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: windows_core::Ref<super::dxgi::IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
    fn CreateSwapChainForCoreWindow(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, pwindow: windows_core::Ref<windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: windows_core::Ref<super::dxgi::IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
    fn GetSharedResourceAdapterLuid(&self, hresource: super::winnt::HANDLE) -> windows_core::Result<super::winnt::LUID>;
    fn RegisterStereoStatusWindow(&self, windowhandle: super::windef::HWND, wmsg: u32) -> windows_core::Result<u32>;
    fn RegisterStereoStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterStereoStatus(&self, dwcookie: u32);
    fn RegisterOcclusionStatusWindow(&self, windowhandle: super::windef::HWND, wmsg: u32) -> windows_core::Result<u32>;
    fn RegisterOcclusionStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterOcclusionStatus(&self, dwcookie: u32);
    fn CreateSwapChainForComposition(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: windows_core::Ref<super::dxgi::IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory2_Vtbl {
    pub const fn new<Identity: IDXGIFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsWindowedStereoEnabled<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory2_Impl::IsWindowedStereoEnabled(this)
            }
        }
        unsafe extern "system" fn CreateSwapChainForHwnd<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hwnd: super::windef::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::CreateSwapChainForHwnd(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pfullscreendesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSwapChainForCoreWindow<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pwindow: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::CreateSwapChainForCoreWindow(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pwindow), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSharedResourceAdapterLuid<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::winnt::HANDLE, pluid: *mut super::winnt::LUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::GetSharedResourceAdapterLuid(this, core::mem::transmute_copy(&hresource)) {
                    Ok(ok__) => {
                        pluid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterStereoStatusWindow<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::windef::HWND, wmsg: u32, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterStereoStatusWindow(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&wmsg)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterStereoStatusEvent<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterStereoStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterStereoStatus<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory2_Impl::UnregisterStereoStatus(this, core::mem::transmute_copy(&dwcookie));
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusWindow<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::windef::HWND, wmsg: u32, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterOcclusionStatusWindow(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&wmsg)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusEvent<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterOcclusionStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterOcclusionStatus<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory2_Impl::UnregisterOcclusionStatus(this, core::mem::transmute_copy(&dwcookie));
            }
        }
        unsafe extern "system" fn CreateSwapChainForComposition<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::CreateSwapChainForComposition(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dxgi::IDXGIFactory1_Vtbl::new::<Identity, OFFSET>(),
            IsWindowedStereoEnabled: IsWindowedStereoEnabled::<Identity, OFFSET>,
            CreateSwapChainForHwnd: CreateSwapChainForHwnd::<Identity, OFFSET>,
            CreateSwapChainForCoreWindow: CreateSwapChainForCoreWindow::<Identity, OFFSET>,
            GetSharedResourceAdapterLuid: GetSharedResourceAdapterLuid::<Identity, OFFSET>,
            RegisterStereoStatusWindow: RegisterStereoStatusWindow::<Identity, OFFSET>,
            RegisterStereoStatusEvent: RegisterStereoStatusEvent::<Identity, OFFSET>,
            UnregisterStereoStatus: UnregisterStereoStatus::<Identity, OFFSET>,
            RegisterOcclusionStatusWindow: RegisterOcclusionStatusWindow::<Identity, OFFSET>,
            RegisterOcclusionStatusEvent: RegisterOcclusionStatusEvent::<Identity, OFFSET>,
            UnregisterOcclusionStatus: UnregisterOcclusionStatus::<Identity, OFFSET>,
            CreateSwapChainForComposition: CreateSwapChainForComposition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory2 {}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::define_interface!(IDXGIOutput1, IDXGIOutput1_Vtbl, 0x00cddea8_939b_4b83_a340_a685226666cc);
#[cfg(feature = "Win32_dxgi")]
impl core::ops::Deref for IDXGIOutput1 {
    type Target = super::dxgi::IDXGIOutput;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::interface_hierarchy!(IDXGIOutput1, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIOutput);
#[cfg(feature = "Win32_dxgi")]
impl IDXGIOutput1 {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: super::dxgiformat::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: Option<*mut DXGI_MODE_DESC1>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeList1)(windows_core::Interface::as_raw(self), enumformat, flags, pnummodes as _, pdesc.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub unsafe fn FindClosestMatchingMode1<P2>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindClosestMatchingMode1)(windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch as _, pconcerneddevice.param().abi()) }
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dxgi::IDXGIResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDisplaySurfaceData1)(windows_core::Interface::as_raw(self), pdestination.param().abi()) }
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> windows_core::Result<IDXGIOutputDuplication>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateOutput)(windows_core::Interface::as_raw(self), pdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dxgi")]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput1_Vtbl {
    pub base__: super::dxgi::IDXGIOutput_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub GetDisplayModeList1: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, u32, *mut u32, *mut DXGI_MODE_DESC1) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype")))]
    GetDisplayModeList1: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub FindClosestMatchingMode1: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_MODE_DESC1, *mut DXGI_MODE_DESC1, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype")))]
    FindClosestMatchingMode1: usize,
    pub GetDisplaySurfaceData1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DuplicateOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDXGIOutput1_Impl: super::dxgi::IDXGIOutput_Impl {
    fn GetDisplayModeList1(&self, enumformat: super::dxgiformat::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> windows_core::Result<()>;
    fn FindClosestMatchingMode1(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetDisplaySurfaceData1(&self, pdestination: windows_core::Ref<super::dxgi::IDXGIResource>) -> windows_core::Result<()>;
    fn DuplicateOutput(&self, pdevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDXGIOutput1_Vtbl {
    pub const fn new<Identity: IDXGIOutput1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDisplayModeList1<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: super::dxgiformat::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput1_Impl::GetDisplayModeList1(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pnummodes), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn FindClosestMatchingMode1<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput1_Impl::FindClosestMatchingMode1(this, core::mem::transmute_copy(&pmodetomatch), core::mem::transmute_copy(&pclosestmatch), core::mem::transmute_copy(&pconcerneddevice)).into()
            }
        }
        unsafe extern "system" fn GetDisplaySurfaceData1<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput1_Impl::GetDisplaySurfaceData1(this, core::mem::transmute_copy(&pdestination)).into()
            }
        }
        unsafe extern "system" fn DuplicateOutput<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, ppoutputduplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput1_Impl::DuplicateOutput(this, core::mem::transmute_copy(&pdevice)) {
                    Ok(ok__) => {
                        ppoutputduplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dxgi::IDXGIOutput_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayModeList1: GetDisplayModeList1::<Identity, OFFSET>,
            FindClosestMatchingMode1: FindClosestMatchingMode1::<Identity, OFFSET>,
            GetDisplaySurfaceData1: GetDisplaySurfaceData1::<Identity, OFFSET>,
            DuplicateOutput: DuplicateOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIOutput as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIOutput1 {}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::define_interface!(IDXGIOutputDuplication, IDXGIOutputDuplication_Vtbl, 0x191cfac3_a341_470d_b26e_a864f428319c);
#[cfg(feature = "Win32_dxgi")]
impl core::ops::Deref for IDXGIOutputDuplication {
    type Target = super::dxgi::IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::interface_hierarchy!(IDXGIOutputDuplication, windows_core::IUnknown, super::dxgi::IDXGIObject);
#[cfg(feature = "Win32_dxgi")]
impl IDXGIOutputDuplication {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn AcquireNextFrame(&self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut Option<super::dxgi::IDXGIResource>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcquireNextFrame)(windows_core::Interface::as_raw(self), timeoutinmilliseconds, pframeinfo as _, core::mem::transmute(ppdesktopresource)) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetFrameDirtyRects(&self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::windef::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrameDirtyRects)(windows_core::Interface::as_raw(self), dirtyrectsbuffersize, pdirtyrectsbuffer as _, pdirtyrectsbuffersizerequired as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetFrameMoveRects(&self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrameMoveRects)(windows_core::Interface::as_raw(self), moverectsbuffersize, pmoverectbuffer as _, pmoverectsbuffersizerequired as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetFramePointerShape(&self, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFramePointerShape)(windows_core::Interface::as_raw(self), pointershapebuffersize, ppointershapebuffer as _, ppointershapebuffersizerequired as _, ppointershapeinfo as _) }
    }
    pub unsafe fn MapDesktopSurface(&self) -> windows_core::Result<super::dxgi::DXGI_MAPPED_RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MapDesktopSurface)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnMapDesktopSurface(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnMapDesktopSurface)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReleaseFrame(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseFrame)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_dxgi")]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutputDuplication_Vtbl {
    pub base__: super::dxgi::IDXGIObject_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_OUTDUPL_DESC),
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_windef")]
    pub AcquireNextFrame: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DXGI_OUTDUPL_FRAME_INFO, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    AcquireNextFrame: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetFrameDirtyRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::windef::RECT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetFrameDirtyRects: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetFrameMoveRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DXGI_OUTDUPL_MOVE_RECT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetFrameMoveRects: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetFramePointerShape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32, *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetFramePointerShape: usize,
    pub MapDesktopSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dxgi::DXGI_MAPPED_RECT) -> windows_core::HRESULT,
    pub UnMapDesktopSurface: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDXGIOutputDuplication_Impl: super::dxgi::IDXGIObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC);
    fn AcquireNextFrame(&self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: windows_core::OutRef<super::dxgi::IDXGIResource>) -> windows_core::Result<()>;
    fn GetFrameDirtyRects(&self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::windef::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::Result<()>;
    fn GetFrameMoveRects(&self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::Result<()>;
    fn GetFramePointerShape(&self, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::Result<()>;
    fn MapDesktopSurface(&self) -> windows_core::Result<super::dxgi::DXGI_MAPPED_RECT>;
    fn UnMapDesktopSurface(&self) -> windows_core::Result<()>;
    fn ReleaseFrame(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDXGIOutputDuplication_Vtbl {
    pub const fn new<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        unsafe extern "system" fn AcquireNextFrame<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::AcquireNextFrame(this, core::mem::transmute_copy(&timeoutinmilliseconds), core::mem::transmute_copy(&pframeinfo), core::mem::transmute_copy(&ppdesktopresource)).into()
            }
        }
        unsafe extern "system" fn GetFrameDirtyRects<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::windef::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetFrameDirtyRects(this, core::mem::transmute_copy(&dirtyrectsbuffersize), core::mem::transmute_copy(&pdirtyrectsbuffer), core::mem::transmute_copy(&pdirtyrectsbuffersizerequired)).into()
            }
        }
        unsafe extern "system" fn GetFrameMoveRects<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetFrameMoveRects(this, core::mem::transmute_copy(&moverectsbuffersize), core::mem::transmute_copy(&pmoverectbuffer), core::mem::transmute_copy(&pmoverectsbuffersizerequired)).into()
            }
        }
        unsafe extern "system" fn GetFramePointerShape<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetFramePointerShape(this, core::mem::transmute_copy(&pointershapebuffersize), core::mem::transmute_copy(&ppointershapebuffer), core::mem::transmute_copy(&ppointershapebuffersizerequired), core::mem::transmute_copy(&ppointershapeinfo)).into()
            }
        }
        unsafe extern "system" fn MapDesktopSurface<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut super::dxgi::DXGI_MAPPED_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutputDuplication_Impl::MapDesktopSurface(this) {
                    Ok(ok__) => {
                        plockedrect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnMapDesktopSurface<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::UnMapDesktopSurface(this).into()
            }
        }
        unsafe extern "system" fn ReleaseFrame<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::ReleaseFrame(this).into()
            }
        }
        Self {
            base__: super::dxgi::IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            AcquireNextFrame: AcquireNextFrame::<Identity, OFFSET>,
            GetFrameDirtyRects: GetFrameDirtyRects::<Identity, OFFSET>,
            GetFrameMoveRects: GetFrameMoveRects::<Identity, OFFSET>,
            GetFramePointerShape: GetFramePointerShape::<Identity, OFFSET>,
            MapDesktopSurface: MapDesktopSurface::<Identity, OFFSET>,
            UnMapDesktopSurface: UnMapDesktopSurface::<Identity, OFFSET>,
            ReleaseFrame: ReleaseFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutputDuplication as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIOutputDuplication {}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::define_interface!(IDXGIResource1, IDXGIResource1_Vtbl, 0x30961379_4609_4a41_998e_54fe567ee0c1);
#[cfg(feature = "Win32_dxgi")]
impl core::ops::Deref for IDXGIResource1 {
    type Target = super::dxgi::IDXGIResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::interface_hierarchy!(IDXGIResource1, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIDeviceSubObject, super::dxgi::IDXGIResource);
#[cfg(feature = "Win32_dxgi")]
impl IDXGIResource1 {
    pub unsafe fn CreateSubresourceSurface(&self, index: u32) -> windows_core::Result<IDXGISurface2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSubresourceSurface)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
    pub unsafe fn CreateSharedHandle<P2>(&self, pattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, dwaccess: u32, lpname: P2) -> windows_core::Result<super::winnt::HANDLE>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSharedHandle)(windows_core::Interface::as_raw(self), pattributes.unwrap_or(core::mem::zeroed()) as _, dwaccess, lpname.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_dxgi")]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource1_Vtbl {
    pub base__: super::dxgi::IDXGIResource_Vtbl,
    pub CreateSubresourceSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
    pub CreateSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::minwinbase::SECURITY_ATTRIBUTES, u32, windows_core::PCWSTR, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwinbase", feature = "Win32_winnt")))]
    CreateSharedHandle: usize,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
pub trait IDXGIResource1_Impl: super::dxgi::IDXGIResource_Impl {
    fn CreateSubresourceSurface(&self, index: u32) -> windows_core::Result<IDXGISurface2>;
    fn CreateSharedHandle(&self, pattributes: *const super::minwinbase::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: &windows_core::PCWSTR) -> windows_core::Result<super::winnt::HANDLE>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
impl IDXGIResource1_Vtbl {
    pub const fn new<Identity: IDXGIResource1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSubresourceSurface<Identity: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource1_Impl::CreateSubresourceSurface(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSharedHandle<Identity: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *const super::minwinbase::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: windows_core::PCWSTR, phandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource1_Impl::CreateSharedHandle(this, core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&dwaccess), core::mem::transmute(&lpname)) {
                    Ok(ok__) => {
                        phandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dxgi::IDXGIResource_Vtbl::new::<Identity, OFFSET>(),
            CreateSubresourceSurface: CreateSubresourceSurface::<Identity, OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIResource1 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIResource1 {}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::define_interface!(IDXGISurface2, IDXGISurface2_Vtbl, 0xaba496dd_b617_4cb8_a866_bc44d7eb1fa2);
#[cfg(feature = "Win32_dxgi")]
impl core::ops::Deref for IDXGISurface2 {
    type Target = super::dxgi::IDXGISurface1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::interface_hierarchy!(IDXGISurface2, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIDeviceSubObject, super::dxgi::IDXGISurface, super::dxgi::IDXGISurface1);
#[cfg(feature = "Win32_dxgi")]
impl IDXGISurface2 {
    pub unsafe fn GetResource<T>(&self, psubresourceindex: *mut u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), &T::IID, &mut result__, psubresourceindex as _).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[cfg(feature = "Win32_dxgi")]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface2_Vtbl {
    pub base__: super::dxgi::IDXGISurface1_Vtbl,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
pub trait IDXGISurface2_Impl: super::dxgi::IDXGISurface1_Impl {
    fn GetResource(&self, riid: *const windows_core::GUID, ppparentresource: *mut *mut core::ffi::c_void, psubresourceindex: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl IDXGISurface2_Vtbl {
    pub const fn new<Identity: IDXGISurface2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResource<Identity: IDXGISurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparentresource: *mut *mut core::ffi::c_void, psubresourceindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface2_Impl::GetResource(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparentresource), core::mem::transmute_copy(&psubresourceindex)).into()
            }
        }
        Self { base__: super::dxgi::IDXGISurface1_Vtbl::new::<Identity, OFFSET>(), GetResource: GetResource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface2 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGISurface as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGISurface1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGISurface2 {}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::define_interface!(IDXGISwapChain1, IDXGISwapChain1_Vtbl, 0x790a45f7_0d42_4876_983a_0a55cfe6f4aa);
#[cfg(feature = "Win32_dxgi")]
impl core::ops::Deref for IDXGISwapChain1 {
    type Target = super::dxgi::IDXGISwapChain;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dxgi")]
windows_core::imp::interface_hierarchy!(IDXGISwapChain1, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIDeviceSubObject, super::dxgi::IDXGISwapChain);
#[cfg(feature = "Win32_dxgi")]
impl IDXGISwapChain1 {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype"))]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFullscreenDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetHwnd(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHwnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCoreWindow)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present1)(windows_core::Interface::as_raw(self), syncinterval, presentflags, ppresentparameters) }
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsTemporaryMonoSupported)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRestrictToOutput(&self) -> windows_core::Result<super::dxgi::IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictToOutput)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dxgitype")]
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const super::dxgitype::DXGI_RGBA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBackgroundColor)(windows_core::Interface::as_raw(self), pcolor) }
    }
    #[cfg(feature = "Win32_dxgitype")]
    pub unsafe fn GetBackgroundColor(&self) -> windows_core::Result<super::dxgitype::DXGI_RGBA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackgroundColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_dxgitype")]
    pub unsafe fn SetRotation(&self, rotation: super::dxgitype::DXGI_MODE_ROTATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), rotation) }
    }
    #[cfg(feature = "Win32_dxgitype")]
    pub unsafe fn GetRotation(&self) -> windows_core::Result<super::dxgitype::DXGI_MODE_ROTATION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRotation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_dxgi")]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain1_Vtbl {
    pub base__: super::dxgi::IDXGISwapChain_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    GetDesc1: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype"))]
    pub GetFullscreenDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype")))]
    GetFullscreenDesc: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetHwnd: usize,
    pub GetCoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub Present1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    Present1: usize,
    pub IsTemporaryMonoSupported: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetRestrictToOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dxgitype")]
    pub SetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dxgitype::DXGI_RGBA) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgitype"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "Win32_dxgitype")]
    pub GetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dxgitype::DXGI_RGBA) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgitype"))]
    GetBackgroundColor: usize,
    #[cfg(feature = "Win32_dxgitype")]
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgitype::DXGI_MODE_ROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgitype"))]
    SetRotation: usize,
    #[cfg(feature = "Win32_dxgitype")]
    pub GetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dxgitype::DXGI_MODE_ROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgitype"))]
    GetRotation: usize,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDXGISwapChain1_Impl: super::dxgi::IDXGISwapChain_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::Result<()>;
    fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::Result<()>;
    fn GetHwnd(&self) -> windows_core::Result<super::windef::HWND>;
    fn GetCoreWindow(&self, refiid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::Result<()>;
    fn IsTemporaryMonoSupported(&self) -> windows_core::BOOL;
    fn GetRestrictToOutput(&self) -> windows_core::Result<super::dxgi::IDXGIOutput>;
    fn SetBackgroundColor(&self, pcolor: *const super::dxgitype::DXGI_RGBA) -> windows_core::Result<()>;
    fn GetBackgroundColor(&self) -> windows_core::Result<super::dxgitype::DXGI_RGBA>;
    fn SetRotation(&self, rotation: super::dxgitype::DXGI_MODE_ROTATION) -> windows_core::Result<()>;
    fn GetRotation(&self) -> windows_core::Result<super::dxgitype::DXGI_MODE_ROTATION>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDXGISwapChain1_Vtbl {
    pub const fn new<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetFullscreenDesc<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::GetFullscreenDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetHwnd<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetHwnd(this) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCoreWindow<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refiid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::GetCoreWindow(this, core::mem::transmute_copy(&refiid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn Present1<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::Present1(this, core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&presentflags), core::mem::transmute_copy(&ppresentparameters)).into()
            }
        }
        unsafe extern "system" fn IsTemporaryMonoSupported<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::IsTemporaryMonoSupported(this)
            }
        }
        unsafe extern "system" fn GetRestrictToOutput<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprestricttooutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetRestrictToOutput(this) {
                    Ok(ok__) => {
                        pprestricttooutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *const super::dxgitype::DXGI_RGBA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::SetBackgroundColor(this, core::mem::transmute_copy(&pcolor)).into()
            }
        }
        unsafe extern "system" fn GetBackgroundColor<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut super::dxgitype::DXGI_RGBA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetBackgroundColor(this) {
                    Ok(ok__) => {
                        pcolor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRotation<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: super::dxgitype::DXGI_MODE_ROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::SetRotation(this, core::mem::transmute_copy(&rotation)).into()
            }
        }
        unsafe extern "system" fn GetRotation<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protation: *mut super::dxgitype::DXGI_MODE_ROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetRotation(this) {
                    Ok(ok__) => {
                        protation.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dxgi::IDXGISwapChain_Vtbl::new::<Identity, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, OFFSET>,
            GetFullscreenDesc: GetFullscreenDesc::<Identity, OFFSET>,
            GetHwnd: GetHwnd::<Identity, OFFSET>,
            GetCoreWindow: GetCoreWindow::<Identity, OFFSET>,
            Present1: Present1::<Identity, OFFSET>,
            IsTemporaryMonoSupported: IsTemporaryMonoSupported::<Identity, OFFSET>,
            GetRestrictToOutput: GetRestrictToOutput::<Identity, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
            GetRotation: GetRotation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGISwapChain as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGISwapChain1 {}
