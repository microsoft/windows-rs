pub const E_SURFACE_CONTENTS_LOST: u32 = 2150301728;
windows_core::imp::define_interface!(ISurfaceImageSourceManagerNative, ISurfaceImageSourceManagerNative_Vtbl, 0x4c8798b7_1d88_4a0f_b59b_b93f600de8c8);
windows_core::imp::interface_hierarchy!(ISurfaceImageSourceManagerNative, windows_core::IUnknown);
impl ISurfaceImageSourceManagerNative {
    pub unsafe fn FlushAllSurfacesWithDevice<P0>(&self, device: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).FlushAllSurfacesWithDevice)(windows_core::Interface::as_raw(self), device.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceManagerNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FlushAllSurfacesWithDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISurfaceImageSourceManagerNative_Impl: windows_core::IUnknownImpl {
    fn FlushAllSurfacesWithDevice(&self, device: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ISurfaceImageSourceManagerNative_Vtbl {
    pub const fn new<Identity: ISurfaceImageSourceManagerNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FlushAllSurfacesWithDevice<Identity: ISurfaceImageSourceManagerNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfaceImageSourceManagerNative_Impl::FlushAllSurfacesWithDevice(this, core::mem::transmute_copy(&device)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FlushAllSurfacesWithDevice: FlushAllSurfacesWithDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfaceImageSourceManagerNative as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISurfaceImageSourceManagerNative {}
windows_core::imp::define_interface!(ISurfaceImageSourceNative, ISurfaceImageSourceNative_Vtbl, 0xf2e9edc1_d307_4525_9886_0fafaa44163c);
windows_core::imp::interface_hierarchy!(ISurfaceImageSourceNative, windows_core::IUnknown);
impl ISurfaceImageSourceNative {
    #[cfg(feature = "dxgi")]
    pub unsafe fn SetDevice<P0>(&self, device: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDXGIDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDevice)(windows_core::Interface::as_raw(self), device.param().abi()) }
    }
    #[cfg(all(feature = "dxgi", feature = "windef"))]
    pub unsafe fn BeginDraw(&self, updaterect: super::RECT, surface: *mut Option<super::IDXGISurface>, offset: *mut super::POINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self), updaterect, core::mem::transmute(surface), offset as _) }
    }
    pub unsafe fn EndDraw(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "dxgi")]
    pub SetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    SetDevice: usize,
    #[cfg(all(feature = "dxgi", feature = "windef"))]
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void, super::RECT, *mut *mut core::ffi::c_void, *mut super::POINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "dxgi", feature = "windef")))]
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "dxgi", feature = "windef"))]
pub trait ISurfaceImageSourceNative_Impl: windows_core::IUnknownImpl {
    fn SetDevice(&self, device: windows_core::Ref<super::IDXGIDevice>) -> windows_core::Result<()>;
    fn BeginDraw(&self, updaterect: &super::RECT, surface: windows_core::OutRef<super::IDXGISurface>, offset: *mut super::POINT) -> windows_core::Result<()>;
    fn EndDraw(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "dxgi", feature = "windef"))]
impl ISurfaceImageSourceNative_Vtbl {
    pub const fn new<Identity: ISurfaceImageSourceNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDevice<Identity: ISurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfaceImageSourceNative_Impl::SetDevice(this, core::mem::transmute_copy(&device)).into()
            }
        }
        unsafe extern "system" fn BeginDraw<Identity: ISurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updaterect: super::RECT, surface: *mut *mut core::ffi::c_void, offset: *mut super::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfaceImageSourceNative_Impl::BeginDraw(this, core::mem::transmute(&updaterect), core::mem::transmute_copy(&surface), core::mem::transmute_copy(&offset)).into()
            }
        }
        unsafe extern "system" fn EndDraw<Identity: ISurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfaceImageSourceNative_Impl::EndDraw(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDevice: SetDevice::<Identity, OFFSET>,
            BeginDraw: BeginDraw::<Identity, OFFSET>,
            EndDraw: EndDraw::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfaceImageSourceNative as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "dxgi", feature = "windef"))]
impl windows_core::RuntimeName for ISurfaceImageSourceNative {}
windows_core::imp::define_interface!(ISurfaceImageSourceNativeWithD2D, ISurfaceImageSourceNativeWithD2D_Vtbl, 0x54298223_41e1_4a41_9c08_02e8256864a1);
windows_core::imp::interface_hierarchy!(ISurfaceImageSourceNativeWithD2D, windows_core::IUnknown);
impl ISurfaceImageSourceNativeWithD2D {
    pub unsafe fn SetDevice<P0>(&self, device: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDevice)(windows_core::Interface::as_raw(self), device.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn BeginDraw<T>(&self, updaterect: *const super::RECT, offset: *mut super::POINT) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self), updaterect, &T::IID, &mut result__, offset as _).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn EndDraw(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SuspendDraw(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SuspendDraw)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ResumeDraw(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResumeDraw)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNativeWithD2D_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut super::POINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait ISurfaceImageSourceNativeWithD2D_Impl: windows_core::IUnknownImpl {
    fn SetDevice(&self, device: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn BeginDraw(&self, updaterect: *const super::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, offset: *mut super::POINT) -> windows_core::Result<()>;
    fn EndDraw(&self) -> windows_core::Result<()>;
    fn SuspendDraw(&self) -> windows_core::Result<()>;
    fn ResumeDraw(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl ISurfaceImageSourceNativeWithD2D_Vtbl {
    pub const fn new<Identity: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDevice<Identity: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfaceImageSourceNativeWithD2D_Impl::SetDevice(this, core::mem::transmute_copy(&device)).into()
            }
        }
        unsafe extern "system" fn BeginDraw<Identity: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updaterect: *const super::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, offset: *mut super::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfaceImageSourceNativeWithD2D_Impl::BeginDraw(this, core::mem::transmute_copy(&updaterect), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&updateobject), core::mem::transmute_copy(&offset)).into()
            }
        }
        unsafe extern "system" fn EndDraw<Identity: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfaceImageSourceNativeWithD2D_Impl::EndDraw(this).into()
            }
        }
        unsafe extern "system" fn SuspendDraw<Identity: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfaceImageSourceNativeWithD2D_Impl::SuspendDraw(this).into()
            }
        }
        unsafe extern "system" fn ResumeDraw<Identity: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfaceImageSourceNativeWithD2D_Impl::ResumeDraw(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDevice: SetDevice::<Identity, OFFSET>,
            BeginDraw: BeginDraw::<Identity, OFFSET>,
            EndDraw: EndDraw::<Identity, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfaceImageSourceNativeWithD2D as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ISurfaceImageSourceNativeWithD2D {}
windows_core::imp::define_interface!(ISwapChainBackgroundPanelNative, ISwapChainBackgroundPanelNative_Vtbl, 0x43bebd4e_add5_4035_8f85_5608d08e9dc9);
windows_core::imp::interface_hierarchy!(ISwapChainBackgroundPanelNative, windows_core::IUnknown);
impl ISwapChainBackgroundPanelNative {
    #[cfg(feature = "dxgi")]
    pub unsafe fn SetSwapChain<P0>(&self, swapchain: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDXGISwapChain>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSwapChain)(windows_core::Interface::as_raw(self), swapchain.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainBackgroundPanelNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    SetSwapChain: usize,
}
#[cfg(feature = "dxgi")]
pub trait ISwapChainBackgroundPanelNative_Impl: windows_core::IUnknownImpl {
    fn SetSwapChain(&self, swapchain: windows_core::Ref<super::IDXGISwapChain>) -> windows_core::Result<()>;
}
#[cfg(feature = "dxgi")]
impl ISwapChainBackgroundPanelNative_Vtbl {
    pub const fn new<Identity: ISwapChainBackgroundPanelNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSwapChain<Identity: ISwapChainBackgroundPanelNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, swapchain: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISwapChainBackgroundPanelNative_Impl::SetSwapChain(this, core::mem::transmute_copy(&swapchain)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSwapChain: SetSwapChain::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISwapChainBackgroundPanelNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "dxgi")]
impl windows_core::RuntimeName for ISwapChainBackgroundPanelNative {}
windows_core::imp::define_interface!(ISwapChainPanelNative, ISwapChainPanelNative_Vtbl, 0xf92f19d2_3ade_45a6_a20c_f6f1ea90554b);
windows_core::imp::interface_hierarchy!(ISwapChainPanelNative, windows_core::IUnknown);
impl ISwapChainPanelNative {
    #[cfg(feature = "dxgi")]
    pub unsafe fn SetSwapChain<P0>(&self, swapchain: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDXGISwapChain>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSwapChain)(windows_core::Interface::as_raw(self), swapchain.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    SetSwapChain: usize,
}
#[cfg(feature = "dxgi")]
pub trait ISwapChainPanelNative_Impl: windows_core::IUnknownImpl {
    fn SetSwapChain(&self, swapchain: windows_core::Ref<super::IDXGISwapChain>) -> windows_core::Result<()>;
}
#[cfg(feature = "dxgi")]
impl ISwapChainPanelNative_Vtbl {
    pub const fn new<Identity: ISwapChainPanelNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSwapChain<Identity: ISwapChainPanelNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, swapchain: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISwapChainPanelNative_Impl::SetSwapChain(this, core::mem::transmute_copy(&swapchain)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSwapChain: SetSwapChain::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISwapChainPanelNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "dxgi")]
impl windows_core::RuntimeName for ISwapChainPanelNative {}
windows_core::imp::define_interface!(ISwapChainPanelNative2, ISwapChainPanelNative2_Vtbl, 0xd5a2f60c_37b2_44a2_937b_8d8eb9726821);
impl core::ops::Deref for ISwapChainPanelNative2 {
    type Target = ISwapChainPanelNative;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISwapChainPanelNative2, windows_core::IUnknown, ISwapChainPanelNative);
impl ISwapChainPanelNative2 {
    #[cfg(feature = "winnt")]
    pub unsafe fn SetSwapChainHandle(&self, swapchainhandle: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSwapChainHandle)(windows_core::Interface::as_raw(self), swapchainhandle) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative2_Vtbl {
    pub base__: ISwapChainPanelNative_Vtbl,
    #[cfg(feature = "winnt")]
    pub SetSwapChainHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetSwapChainHandle: usize,
}
#[cfg(all(feature = "dxgi", feature = "winnt"))]
pub trait ISwapChainPanelNative2_Impl: ISwapChainPanelNative_Impl {
    fn SetSwapChainHandle(&self, swapchainhandle: super::HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "dxgi", feature = "winnt"))]
impl ISwapChainPanelNative2_Vtbl {
    pub const fn new<Identity: ISwapChainPanelNative2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSwapChainHandle<Identity: ISwapChainPanelNative2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, swapchainhandle: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISwapChainPanelNative2_Impl::SetSwapChainHandle(this, core::mem::transmute_copy(&swapchainhandle)).into()
            }
        }
        Self { base__: ISwapChainPanelNative_Vtbl::new::<Identity, OFFSET>(), SetSwapChainHandle: SetSwapChainHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISwapChainPanelNative2 as windows_core::Interface>::IID || iid == &<ISwapChainPanelNative as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "dxgi", feature = "winnt"))]
impl windows_core::RuntimeName for ISwapChainPanelNative2 {}
windows_core::imp::define_interface!(IVirtualSurfaceImageSourceNative, IVirtualSurfaceImageSourceNative_Vtbl, 0xe9550983_360b_4f53_b391_afd695078691);
impl core::ops::Deref for IVirtualSurfaceImageSourceNative {
    type Target = ISurfaceImageSourceNative;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVirtualSurfaceImageSourceNative, windows_core::IUnknown, ISurfaceImageSourceNative);
impl IVirtualSurfaceImageSourceNative {
    #[cfg(feature = "windef")]
    pub unsafe fn Invalidate(&self, updaterect: super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Invalidate)(windows_core::Interface::as_raw(self), updaterect) }
    }
    pub unsafe fn GetUpdateRectCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdateRectCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetUpdateRects(&self, updates: &mut [super::RECT]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUpdateRects)(windows_core::Interface::as_raw(self), updates.as_mut_ptr(), updates.len().try_into().unwrap()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetVisibleBounds(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisibleBounds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterForUpdatesNeeded<P0>(&self, callback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVirtualSurfaceUpdatesCallbackNative>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterForUpdatesNeeded)(windows_core::Interface::as_raw(self), callback.param().abi()) }
    }
    pub unsafe fn Resize(&self, newwidth: i32, newheight: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), newwidth, newheight) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceNative_Vtbl {
    pub base__: ISurfaceImageSourceNative_Vtbl,
    #[cfg(feature = "windef")]
    pub Invalidate: unsafe extern "system" fn(*mut core::ffi::c_void, super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Invalidate: usize,
    pub GetUpdateRectCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetUpdateRects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetUpdateRects: usize,
    #[cfg(feature = "windef")]
    pub GetVisibleBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetVisibleBounds: usize,
    pub RegisterForUpdatesNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "dxgi", feature = "windef"))]
pub trait IVirtualSurfaceImageSourceNative_Impl: ISurfaceImageSourceNative_Impl {
    fn Invalidate(&self, updaterect: &super::RECT) -> windows_core::Result<()>;
    fn GetUpdateRectCount(&self) -> windows_core::Result<u32>;
    fn GetUpdateRects(&self, updates: *mut super::RECT, count: u32) -> windows_core::Result<()>;
    fn GetVisibleBounds(&self) -> windows_core::Result<super::RECT>;
    fn RegisterForUpdatesNeeded(&self, callback: windows_core::Ref<IVirtualSurfaceUpdatesCallbackNative>) -> windows_core::Result<()>;
    fn Resize(&self, newwidth: i32, newheight: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "dxgi", feature = "windef"))]
impl IVirtualSurfaceImageSourceNative_Vtbl {
    pub const fn new<Identity: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invalidate<Identity: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updaterect: super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVirtualSurfaceImageSourceNative_Impl::Invalidate(this, core::mem::transmute(&updaterect)).into()
            }
        }
        unsafe extern "system" fn GetUpdateRectCount<Identity: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVirtualSurfaceImageSourceNative_Impl::GetUpdateRectCount(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUpdateRects<Identity: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updates: *mut super::RECT, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVirtualSurfaceImageSourceNative_Impl::GetUpdateRects(this, core::mem::transmute_copy(&updates), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetVisibleBounds<Identity: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bounds: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVirtualSurfaceImageSourceNative_Impl::GetVisibleBounds(this) {
                    Ok(ok__) => {
                        bounds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterForUpdatesNeeded<Identity: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVirtualSurfaceImageSourceNative_Impl::RegisterForUpdatesNeeded(this, core::mem::transmute_copy(&callback)).into()
            }
        }
        unsafe extern "system" fn Resize<Identity: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newwidth: i32, newheight: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVirtualSurfaceImageSourceNative_Impl::Resize(this, core::mem::transmute_copy(&newwidth), core::mem::transmute_copy(&newheight)).into()
            }
        }
        Self {
            base__: ISurfaceImageSourceNative_Vtbl::new::<Identity, OFFSET>(),
            Invalidate: Invalidate::<Identity, OFFSET>,
            GetUpdateRectCount: GetUpdateRectCount::<Identity, OFFSET>,
            GetUpdateRects: GetUpdateRects::<Identity, OFFSET>,
            GetVisibleBounds: GetVisibleBounds::<Identity, OFFSET>,
            RegisterForUpdatesNeeded: RegisterForUpdatesNeeded::<Identity, OFFSET>,
            Resize: Resize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVirtualSurfaceImageSourceNative as windows_core::Interface>::IID || iid == &<ISurfaceImageSourceNative as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "dxgi", feature = "windef"))]
impl windows_core::RuntimeName for IVirtualSurfaceImageSourceNative {}
windows_core::imp::define_interface!(IVirtualSurfaceUpdatesCallbackNative, IVirtualSurfaceUpdatesCallbackNative_Vtbl, 0xdbf2e947_8e6c_4254_9eee_7738f71386c9);
windows_core::imp::interface_hierarchy!(IVirtualSurfaceUpdatesCallbackNative, windows_core::IUnknown);
impl IVirtualSurfaceUpdatesCallbackNative {
    pub unsafe fn UpdatesNeeded(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdatesNeeded)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceUpdatesCallbackNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UpdatesNeeded: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVirtualSurfaceUpdatesCallbackNative_Impl: windows_core::IUnknownImpl {
    fn UpdatesNeeded(&self) -> windows_core::Result<()>;
}
impl IVirtualSurfaceUpdatesCallbackNative_Vtbl {
    pub const fn new<Identity: IVirtualSurfaceUpdatesCallbackNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdatesNeeded<Identity: IVirtualSurfaceUpdatesCallbackNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVirtualSurfaceUpdatesCallbackNative_Impl::UpdatesNeeded(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), UpdatesNeeded: UpdatesNeeded::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVirtualSurfaceUpdatesCallbackNative as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVirtualSurfaceUpdatesCallbackNative {}
