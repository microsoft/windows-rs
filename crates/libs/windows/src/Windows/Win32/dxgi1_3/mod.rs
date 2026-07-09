#[inline]
pub unsafe fn CreateDXGIFactory2<T>(flags: u32) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn CreateDXGIFactory2(flags : u32, riid : *const windows_core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateDXGIFactory2(flags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn DXGIGetDebugInterface1<T>(flags: u32) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn DXGIGetDebugInterface1(flags : u32, riid : *const windows_core::GUID, pdebug : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DXGIGetDebugInterface1(flags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
pub type DXGI_FRAME_PRESENTATION_MODE = i32;
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSED: DXGI_FRAME_PRESENTATION_MODE = 0;
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE: DXGI_FRAME_PRESENTATION_MODE = 3;
pub const DXGI_FRAME_PRESENTATION_MODE_NONE: DXGI_FRAME_PRESENTATION_MODE = 2;
pub const DXGI_FRAME_PRESENTATION_MODE_OVERLAY: DXGI_FRAME_PRESENTATION_MODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
pub type DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = i32;
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 2;
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 1;
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 4;
pub type DXGI_OVERLAY_SUPPORT_FLAG = i32;
pub const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT: DXGI_OVERLAY_SUPPORT_FLAG = 1;
pub const DXGI_OVERLAY_SUPPORT_FLAG_SCALING: DXGI_OVERLAY_SUPPORT_FLAG = 2;
windows_core::imp::define_interface!(IDXGIDecodeSwapChain, IDXGIDecodeSwapChain_Vtbl, 0x2633066b_4514_4c7a_8fd8_12ea98059d18);
windows_core::imp::interface_hierarchy!(IDXGIDecodeSwapChain, windows_core::IUnknown);
impl IDXGIDecodeSwapChain {
    pub unsafe fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PresentBuffer)(windows_core::Interface::as_raw(self), buffertopresent, syncinterval, flags) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SetSourceRect(&self, prect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSourceRect)(windows_core::Interface::as_raw(self), prect) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SetTargetRect(&self, prect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTargetRect)(windows_core::Interface::as_raw(self), prect) }
    }
    pub unsafe fn SetDestSize(&self, width: u32, height: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDestSize)(windows_core::Interface::as_raw(self), width, height) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetSourceRect(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetTargetRect(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDestSize)(windows_core::Interface::as_raw(self), pwidth as _, pheight as _) }
    }
    pub unsafe fn SetColorSpace(&self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColorSpace)(windows_core::Interface::as_raw(self), colorspace) }
    }
    pub unsafe fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
        unsafe { (windows_core::Interface::vtable(self).GetColorSpace)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDecodeSwapChain_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PresentBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub SetSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SetSourceRect: usize,
    #[cfg(feature = "Win32_windef")]
    pub SetTargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SetTargetRect: usize,
    pub SetDestSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetSourceRect: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetTargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetTargetRect: usize,
    pub GetDestSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::HRESULT,
    pub GetColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGIDecodeSwapChain_Impl: windows_core::IUnknownImpl {
    fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: u32) -> windows_core::Result<()>;
    fn SetSourceRect(&self, prect: *const super::windef::RECT) -> windows_core::Result<()>;
    fn SetTargetRect(&self, prect: *const super::windef::RECT) -> windows_core::Result<()>;
    fn SetDestSize(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn GetSourceRect(&self) -> windows_core::Result<super::windef::RECT>;
    fn GetTargetRect(&self) -> windows_core::Result<super::windef::RECT>;
    fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::Result<()>;
    fn SetColorSpace(&self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::Result<()>;
    fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS;
}
#[cfg(feature = "Win32_windef")]
impl IDXGIDecodeSwapChain_Vtbl {
    pub const fn new<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PresentBuffer<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::PresentBuffer(this, core::mem::transmute_copy(&buffertopresent), core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn SetSourceRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetSourceRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn SetTargetRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetTargetRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn SetDestSize<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetDestSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn GetSourceRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDecodeSwapChain_Impl::GetSourceRect(this) {
                    Ok(ok__) => {
                        prect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTargetRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDecodeSwapChain_Impl::GetTargetRect(this) {
                    Ok(ok__) => {
                        prect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDestSize<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::GetDestSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn SetColorSpace<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetColorSpace(this, core::mem::transmute_copy(&colorspace)).into()
            }
        }
        unsafe extern "system" fn GetColorSpace<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::GetColorSpace(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PresentBuffer: PresentBuffer::<Identity, OFFSET>,
            SetSourceRect: SetSourceRect::<Identity, OFFSET>,
            SetTargetRect: SetTargetRect::<Identity, OFFSET>,
            SetDestSize: SetDestSize::<Identity, OFFSET>,
            GetSourceRect: GetSourceRect::<Identity, OFFSET>,
            GetTargetRect: GetTargetRect::<Identity, OFFSET>,
            GetDestSize: GetDestSize::<Identity, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, OFFSET>,
            GetColorSpace: GetColorSpace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDecodeSwapChain as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGIDecodeSwapChain {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::define_interface!(IDXGIDevice3, IDXGIDevice3_Vtbl, 0x6007896c_3244_4afd_bf18_a6d3beda5023);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl core::ops::Deref for IDXGIDevice3 {
    type Target = super::dxgi1_2::IDXGIDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::interface_hierarchy!(IDXGIDevice3, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIDevice, super::dxgi::IDXGIDevice1, super::dxgi1_2::IDXGIDevice2);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl IDXGIDevice3 {
    pub unsafe fn Trim(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Trim)(windows_core::Interface::as_raw(self));
        }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice3_Vtbl {
    pub base__: super::dxgi1_2::IDXGIDevice2_Vtbl,
    pub Trim: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait IDXGIDevice3_Impl: super::dxgi1_2::IDXGIDevice2_Impl {
    fn Trim(&self);
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl IDXGIDevice3_Vtbl {
    pub const fn new<Identity: IDXGIDevice3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Trim<Identity: IDXGIDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice3_Impl::Trim(this);
            }
        }
        Self { base__: super::dxgi1_2::IDXGIDevice2_Vtbl::new::<Identity, OFFSET>(), Trim: Trim::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice3 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDevice as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDevice1 as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIDevice3 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::define_interface!(IDXGIFactory3, IDXGIFactory3_Vtbl, 0x25483823_cd46_4c7d_86ca_47aa95b837bd);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl core::ops::Deref for IDXGIFactory3 {
    type Target = super::dxgi1_2::IDXGIFactory2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::interface_hierarchy!(IDXGIFactory3, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIFactory, super::dxgi::IDXGIFactory1, super::dxgi1_2::IDXGIFactory2);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl IDXGIFactory3 {
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCreationFlags)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory3_Vtbl {
    pub base__: super::dxgi1_2::IDXGIFactory2_Vtbl,
    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory3_Impl: super::dxgi1_2::IDXGIFactory2_Impl {
    fn GetCreationFlags(&self) -> u32;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory3_Vtbl {
    pub const fn new<Identity: IDXGIFactory3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCreationFlags<Identity: IDXGIFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory3_Impl::GetCreationFlags(this)
            }
        }
        Self { base__: super::dxgi1_2::IDXGIFactory2_Vtbl::new::<Identity, OFFSET>(), GetCreationFlags: GetCreationFlags::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory1 as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory3 {}
windows_core::imp::define_interface!(IDXGIFactoryMedia, IDXGIFactoryMedia_Vtbl, 0x41e7d1f2_a591_4f7b_a2e5_fa9c843e1c12);
windows_core::imp::interface_hierarchy!(IDXGIFactoryMedia, windows_core::IUnknown);
impl IDXGIFactoryMedia {
    #[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
    pub unsafe fn CreateSwapChainForCompositionSurfaceHandle<P0, P3>(&self, pdevice: P0, hsurface: Option<super::winnt::HANDLE>, pdesc: *const super::dxgi1_2::DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P3) -> windows_core::Result<super::dxgi1_2::IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<super::dxgi::IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForCompositionSurfaceHandle)(windows_core::Interface::as_raw(self), pdevice.param().abi(), hsurface.unwrap_or(core::mem::zeroed()) as _, pdesc, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dxgi", feature = "Win32_winnt"))]
    pub unsafe fn CreateDecodeSwapChainForCompositionSurfaceHandle<P0, P3, P4>(&self, pdevice: P0, hsurface: Option<super::winnt::HANDLE>, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: P3, prestricttooutput: P4) -> windows_core::Result<IDXGIDecodeSwapChain>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<super::dxgi::IDXGIResource>,
        P4: windows_core::Param<super::dxgi::IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDecodeSwapChainForCompositionSurfaceHandle)(windows_core::Interface::as_raw(self), pdevice.param().abi(), hsurface.unwrap_or(core::mem::zeroed()) as _, pdesc, pyuvdecodebuffers.param().abi(), prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactoryMedia_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
    pub CreateSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::winnt::HANDLE, *const super::dxgi1_2::DXGI_SWAP_CHAIN_DESC1, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt")))]
    CreateSwapChainForCompositionSurfaceHandle: usize,
    #[cfg(all(feature = "Win32_dxgi", feature = "Win32_winnt"))]
    pub CreateDecodeSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::winnt::HANDLE, *const DXGI_DECODE_SWAP_CHAIN_DESC, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgi", feature = "Win32_winnt")))]
    CreateDecodeSwapChainForCompositionSurfaceHandle: usize,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait IDXGIFactoryMedia_Impl: windows_core::IUnknownImpl {
    fn CreateSwapChainForCompositionSurfaceHandle(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, hsurface: super::winnt::HANDLE, pdesc: *const super::dxgi1_2::DXGI_SWAP_CHAIN_DESC1, prestricttooutput: windows_core::Ref<super::dxgi::IDXGIOutput>) -> windows_core::Result<super::dxgi1_2::IDXGISwapChain1>;
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, hsurface: super::winnt::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: windows_core::Ref<super::dxgi::IDXGIResource>, prestricttooutput: windows_core::Ref<super::dxgi::IDXGIOutput>) -> windows_core::Result<IDXGIDecodeSwapChain>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl IDXGIFactoryMedia_Vtbl {
    pub const fn new<Identity: IDXGIFactoryMedia_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSwapChainForCompositionSurfaceHandle<Identity: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hsurface: super::winnt::HANDLE, pdesc: *const super::dxgi1_2::DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactoryMedia_Impl::CreateSwapChainForCompositionSurfaceHandle(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&hsurface), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDecodeSwapChainForCompositionSurfaceHandle<Identity: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hsurface: super::winnt::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: *mut core::ffi::c_void, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactoryMedia_Impl::CreateDecodeSwapChainForCompositionSurfaceHandle(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&hsurface), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pyuvdecodebuffers), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSwapChainForCompositionSurfaceHandle: CreateSwapChainForCompositionSurfaceHandle::<Identity, OFFSET>,
            CreateDecodeSwapChainForCompositionSurfaceHandle: CreateDecodeSwapChainForCompositionSurfaceHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactoryMedia as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactoryMedia {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::define_interface!(IDXGIOutput2, IDXGIOutput2_Vtbl, 0x595e39d1_2724_4663_99b1_da969de28364);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl core::ops::Deref for IDXGIOutput2 {
    type Target = super::dxgi1_2::IDXGIOutput1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::interface_hierarchy!(IDXGIOutput2, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIOutput, super::dxgi1_2::IDXGIOutput1);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl IDXGIOutput2 {
    pub unsafe fn SupportsOverlays(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).SupportsOverlays)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput2_Vtbl {
    pub base__: super::dxgi1_2::IDXGIOutput1_Vtbl,
    pub SupportsOverlays: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDXGIOutput2_Impl: super::dxgi1_2::IDXGIOutput1_Impl {
    fn SupportsOverlays(&self) -> windows_core::BOOL;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDXGIOutput2_Vtbl {
    pub const fn new<Identity: IDXGIOutput2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SupportsOverlays<Identity: IDXGIOutput2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput2_Impl::SupportsOverlays(this)
            }
        }
        Self { base__: super::dxgi1_2::IDXGIOutput1_Vtbl::new::<Identity, OFFSET>(), SupportsOverlays: SupportsOverlays::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIOutput as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIOutput1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIOutput2 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::define_interface!(IDXGIOutput3, IDXGIOutput3_Vtbl, 0x8a6bb301_7e7e_41f4_a8e0_5b32f7f99b18);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl core::ops::Deref for IDXGIOutput3 {
    type Target = IDXGIOutput2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::interface_hierarchy!(IDXGIOutput3, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIOutput, super::dxgi1_2::IDXGIOutput1, IDXGIOutput2);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl IDXGIOutput3 {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CheckOverlaySupport<P1>(&self, enumformat: super::dxgiformat::DXGI_FORMAT, pconcerneddevice: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckOverlaySupport)(windows_core::Interface::as_raw(self), enumformat, pconcerneddevice.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput3_Vtbl {
    pub base__: IDXGIOutput2_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CheckOverlaySupport: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CheckOverlaySupport: usize,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDXGIOutput3_Impl: IDXGIOutput2_Impl {
    fn CheckOverlaySupport(&self, enumformat: super::dxgiformat::DXGI_FORMAT, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDXGIOutput3_Vtbl {
    pub const fn new<Identity: IDXGIOutput3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckOverlaySupport<Identity: IDXGIOutput3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: super::dxgiformat::DXGI_FORMAT, pconcerneddevice: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput3_Impl::CheckOverlaySupport(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&pconcerneddevice)) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDXGIOutput2_Vtbl::new::<Identity, OFFSET>(), CheckOverlaySupport: CheckOverlaySupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput3 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIOutput as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIOutput3 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::define_interface!(IDXGISwapChain2, IDXGISwapChain2_Vtbl, 0xa8be2ac4_199f_4946_b331_79599fb98de7);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl core::ops::Deref for IDXGISwapChain2 {
    type Target = super::dxgi1_2::IDXGISwapChain1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::interface_hierarchy!(IDXGISwapChain2, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIDeviceSubObject, super::dxgi::IDXGISwapChain, super::dxgi1_2::IDXGISwapChain1);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl IDXGISwapChain2 {
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSourceSize)(windows_core::Interface::as_raw(self), width, height) }
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSourceSize)(windows_core::Interface::as_raw(self), pwidth as _, pheight as _) }
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(windows_core::Interface::as_raw(self), maxlatency) }
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::winnt::HANDLE {
        unsafe { (windows_core::Interface::vtable(self).GetFrameLatencyWaitableObject)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixTransform)(windows_core::Interface::as_raw(self), pmatrix) }
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMatrixTransform)(windows_core::Interface::as_raw(self), pmatrix as _) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain2_Vtbl {
    pub base__: super::dxgi1_2::IDXGISwapChain1_Vtbl,
    pub SetSourceSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetSourceSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub GetFrameLatencyWaitableObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::winnt::HANDLE,
    #[cfg(not(feature = "Win32_winnt"))]
    GetFrameLatencyWaitableObject: usize,
    pub SetMatrixTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_MATRIX_3X2_F) -> windows_core::HRESULT,
    pub GetMatrixTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_MATRIX_3X2_F) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGISwapChain2_Impl: super::dxgi1_2::IDXGISwapChain1_Impl {
    fn SetSourceSize(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::Result<()>;
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32>;
    fn GetFrameLatencyWaitableObject(&self) -> super::winnt::HANDLE;
    fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::Result<()>;
    fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGISwapChain2_Vtbl {
    pub const fn new<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSourceSize<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::SetSourceSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn GetSourceSize<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::GetSourceSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain2_Impl::GetMaximumFrameLatency(this) {
                    Ok(ok__) => {
                        pmaxlatency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFrameLatencyWaitableObject<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::winnt::HANDLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::GetFrameLatencyWaitableObject(this)
            }
        }
        unsafe extern "system" fn SetMatrixTransform<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::SetMatrixTransform(this, core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn GetMatrixTransform<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::GetMatrixTransform(this, core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        Self {
            base__: super::dxgi1_2::IDXGISwapChain1_Vtbl::new::<Identity, OFFSET>(),
            SetSourceSize: SetSourceSize::<Identity, OFFSET>,
            GetSourceSize: GetSourceSize::<Identity, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
            GetFrameLatencyWaitableObject: GetFrameLatencyWaitableObject::<Identity, OFFSET>,
            SetMatrixTransform: SetMatrixTransform::<Identity, OFFSET>,
            GetMatrixTransform: GetMatrixTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain2 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGISwapChain as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGISwapChain1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGISwapChain2 {}
windows_core::imp::define_interface!(IDXGISwapChainMedia, IDXGISwapChainMedia_Vtbl, 0xdd95b90b_f05f_4f6a_bd65_25bfb264bd84);
windows_core::imp::interface_hierarchy!(IDXGISwapChainMedia, windows_core::IUnknown);
impl IDXGISwapChainMedia {
    pub unsafe fn GetFrameStatisticsMedia(&self, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrameStatisticsMedia)(windows_core::Interface::as_raw(self), pstats as _) }
    }
    pub unsafe fn SetPresentDuration(&self, duration: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPresentDuration)(windows_core::Interface::as_raw(self), duration) }
    }
    pub unsafe fn CheckPresentDurationSupport(&self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckPresentDurationSupport)(windows_core::Interface::as_raw(self), desiredpresentduration, pclosestsmallerpresentduration as _, pclosestlargerpresentduration as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChainMedia_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrameStatisticsMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::HRESULT,
    pub SetPresentDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CheckPresentDurationSupport: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IDXGISwapChainMedia_Impl: windows_core::IUnknownImpl {
    fn GetFrameStatisticsMedia(&self, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::Result<()>;
    fn SetPresentDuration(&self, duration: u32) -> windows_core::Result<()>;
    fn CheckPresentDurationSupport(&self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::Result<()>;
}
impl IDXGISwapChainMedia_Vtbl {
    pub const fn new<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFrameStatisticsMedia<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChainMedia_Impl::GetFrameStatisticsMedia(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn SetPresentDuration<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChainMedia_Impl::SetPresentDuration(this, core::mem::transmute_copy(&duration)).into()
            }
        }
        unsafe extern "system" fn CheckPresentDurationSupport<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChainMedia_Impl::CheckPresentDurationSupport(this, core::mem::transmute_copy(&desiredpresentduration), core::mem::transmute_copy(&pclosestsmallerpresentduration), core::mem::transmute_copy(&pclosestlargerpresentduration)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameStatisticsMedia: GetFrameStatisticsMedia::<Identity, OFFSET>,
            SetPresentDuration: SetPresentDuration::<Identity, OFFSET>,
            CheckPresentDurationSupport: CheckPresentDurationSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChainMedia as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGISwapChainMedia {}
