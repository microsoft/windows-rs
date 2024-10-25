#[cfg(feature = "UI_Composition")]
pub trait ICompositionCapabilitiesInteropFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn GetForWindow(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionCapabilities>;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ICompositionCapabilitiesInteropFactory {}
#[cfg(feature = "UI_Composition")]
impl ICompositionCapabilitiesInteropFactory_Vtbl {
    pub const fn new<Identity: ICompositionCapabilitiesInteropFactory_Impl, const OFFSET: isize>() -> ICompositionCapabilitiesInteropFactory_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ICompositionCapabilitiesInteropFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositionCapabilitiesInteropFactory_Impl::GetForWindow(this, core::mem::transmute_copy(&hwnd)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionCapabilitiesInteropFactory, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionCapabilitiesInteropFactory as windows_core::Interface>::IID
    }
}
pub trait ICompositionDrawingSurfaceInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> windows_core::Result<()>;
    fn EndDraw(&self) -> windows_core::Result<()>;
    fn Resize(&self, sizepixels: &super::super::super::Foundation::SIZE) -> windows_core::Result<()>;
    fn Scroll(&self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> windows_core::Result<()>;
    fn ResumeDraw(&self) -> windows_core::Result<()>;
    fn SuspendDraw(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICompositionDrawingSurfaceInterop {}
impl ICompositionDrawingSurfaceInterop_Vtbl {
    pub const fn new<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>() -> ICompositionDrawingSurfaceInterop_Vtbl {
        unsafe extern "system" fn BeginDraw<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICompositionDrawingSurfaceInterop_Impl::BeginDraw(this, core::mem::transmute_copy(&updaterect), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&updateobject), core::mem::transmute_copy(&updateoffset)).into()
        }
        unsafe extern "system" fn EndDraw<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICompositionDrawingSurfaceInterop_Impl::EndDraw(this).into()
        }
        unsafe extern "system" fn Resize<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sizepixels: super::super::super::Foundation::SIZE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICompositionDrawingSurfaceInterop_Impl::Resize(this, core::mem::transmute(&sizepixels)).into()
        }
        unsafe extern "system" fn Scroll<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICompositionDrawingSurfaceInterop_Impl::Scroll(this, core::mem::transmute_copy(&scrollrect), core::mem::transmute_copy(&cliprect), core::mem::transmute_copy(&offsetx), core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn ResumeDraw<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICompositionDrawingSurfaceInterop_Impl::ResumeDraw(this).into()
        }
        unsafe extern "system" fn SuspendDraw<Identity: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICompositionDrawingSurfaceInterop_Impl::SuspendDraw(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginDraw: BeginDraw::<Identity, OFFSET>,
            EndDraw: EndDraw::<Identity, OFFSET>,
            Resize: Resize::<Identity, OFFSET>,
            Scroll: Scroll::<Identity, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceInterop as windows_core::Interface>::IID
    }
}
pub trait ICompositionDrawingSurfaceInterop2_Impl: Sized + ICompositionDrawingSurfaceInterop_Impl {
    fn CopySurface(&self, destinationresource: Option<&windows_core::IUnknown>, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICompositionDrawingSurfaceInterop2 {}
impl ICompositionDrawingSurfaceInterop2_Vtbl {
    pub const fn new<Identity: ICompositionDrawingSurfaceInterop2_Impl, const OFFSET: isize>() -> ICompositionDrawingSurfaceInterop2_Vtbl {
        unsafe extern "system" fn CopySurface<Identity: ICompositionDrawingSurfaceInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationresource: *mut core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICompositionDrawingSurfaceInterop2_Impl::CopySurface(this, windows_core::from_raw_borrowed(&destinationresource), core::mem::transmute_copy(&destinationoffsetx), core::mem::transmute_copy(&destinationoffsety), core::mem::transmute_copy(&sourcerectangle)).into()
        }
        Self { base__: ICompositionDrawingSurfaceInterop_Vtbl::new::<Identity, OFFSET>(), CopySurface: CopySurface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceInterop2 as windows_core::Interface>::IID || iid == &<ICompositionDrawingSurfaceInterop as windows_core::Interface>::IID
    }
}
pub trait ICompositionGraphicsDeviceInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn GetRenderingDevice(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetRenderingDevice(&self, value: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICompositionGraphicsDeviceInterop {}
impl ICompositionGraphicsDeviceInterop_Vtbl {
    pub const fn new<Identity: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>() -> ICompositionGraphicsDeviceInterop_Vtbl {
        unsafe extern "system" fn GetRenderingDevice<Identity: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositionGraphicsDeviceInterop_Impl::GetRenderingDevice(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderingDevice<Identity: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICompositionGraphicsDeviceInterop_Impl::SetRenderingDevice(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRenderingDevice: GetRenderingDevice::<Identity, OFFSET>,
            SetRenderingDevice: SetRenderingDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionGraphicsDeviceInterop as windows_core::Interface>::IID
    }
}
pub trait ICompositionTextureInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn GetAvailableFence(&self, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICompositionTextureInterop {}
impl ICompositionTextureInterop_Vtbl {
    pub const fn new<Identity: ICompositionTextureInterop_Impl, const OFFSET: isize>() -> ICompositionTextureInterop_Vtbl {
        unsafe extern "system" fn GetAvailableFence<Identity: ICompositionTextureInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICompositionTextureInterop_Impl::GetAvailableFence(this, core::mem::transmute_copy(&fencevalue), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&availablefence)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetAvailableFence: GetAvailableFence::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionTextureInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition_Desktop")]
pub trait ICompositorDesktopInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateDesktopWindowTarget(&self, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL) -> windows_core::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>;
    fn EnsureOnThread(&self, threadid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Composition_Desktop")]
impl windows_core::RuntimeName for ICompositorDesktopInterop {}
#[cfg(feature = "UI_Composition_Desktop")]
impl ICompositorDesktopInterop_Vtbl {
    pub const fn new<Identity: ICompositorDesktopInterop_Impl, const OFFSET: isize>() -> ICompositorDesktopInterop_Vtbl {
        unsafe extern "system" fn CreateDesktopWindowTarget<Identity: ICompositorDesktopInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositorDesktopInterop_Impl::CreateDesktopWindowTarget(this, core::mem::transmute_copy(&hwndtarget), core::mem::transmute_copy(&istopmost)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnsureOnThread<Identity: ICompositorDesktopInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICompositorDesktopInterop_Impl::EnsureOnThread(this, core::mem::transmute_copy(&threadid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDesktopWindowTarget: CreateDesktopWindowTarget::<Identity, OFFSET>,
            EnsureOnThread: EnsureOnThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositorDesktopInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition")]
pub trait ICompositorInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateCompositionSurfaceForHandle(&self, swapchain: super::super::super::Foundation::HANDLE) -> windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateCompositionSurfaceForSwapChain(&self, swapchain: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateGraphicsDevice(&self, renderingdevice: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice>;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ICompositorInterop {}
#[cfg(feature = "UI_Composition")]
impl ICompositorInterop_Vtbl {
    pub const fn new<Identity: ICompositorInterop_Impl, const OFFSET: isize>() -> ICompositorInterop_Vtbl {
        unsafe extern "system" fn CreateCompositionSurfaceForHandle<Identity: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, swapchain: super::super::super::Foundation::HANDLE, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositorInterop_Impl::CreateCompositionSurfaceForHandle(this, core::mem::transmute_copy(&swapchain)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositionSurfaceForSwapChain<Identity: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, swapchain: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositorInterop_Impl::CreateCompositionSurfaceForSwapChain(this, windows_core::from_raw_borrowed(&swapchain)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGraphicsDevice<Identity: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingdevice: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositorInterop_Impl::CreateGraphicsDevice(this, windows_core::from_raw_borrowed(&renderingdevice)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateCompositionSurfaceForHandle: CreateCompositionSurfaceForHandle::<Identity, OFFSET>,
            CreateCompositionSurfaceForSwapChain: CreateCompositionSurfaceForSwapChain::<Identity, OFFSET>,
            CreateGraphicsDevice: CreateGraphicsDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositorInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition")]
pub trait ICompositorInterop2_Impl: Sized + windows_core::IUnknownImpl {
    fn CheckCompositionTextureSupport(&self, renderingdevice: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn CreateCompositionTexture(&self, d3dtexture: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::super::super::UI::Composition::CompositionTexture>;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ICompositorInterop2 {}
#[cfg(feature = "UI_Composition")]
impl ICompositorInterop2_Vtbl {
    pub const fn new<Identity: ICompositorInterop2_Impl, const OFFSET: isize>() -> ICompositorInterop2_Vtbl {
        unsafe extern "system" fn CheckCompositionTextureSupport<Identity: ICompositorInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingdevice: *mut core::ffi::c_void, supportscompositiontextures: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositorInterop2_Impl::CheckCompositionTextureSupport(this, windows_core::from_raw_borrowed(&renderingdevice)) {
                Ok(ok__) => {
                    supportscompositiontextures.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositionTexture<Identity: ICompositorInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, d3dtexture: *mut core::ffi::c_void, compositiontexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositorInterop2_Impl::CreateCompositionTexture(this, windows_core::from_raw_borrowed(&d3dtexture)) {
                Ok(ok__) => {
                    compositiontexture.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CheckCompositionTextureSupport: CheckCompositionTextureSupport::<Identity, OFFSET>,
            CreateCompositionTexture: CreateCompositionTexture::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositorInterop2 as windows_core::Interface>::IID
    }
}
pub trait IDesktopWindowTargetInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn Hwnd(&self) -> windows_core::Result<super::super::super::Foundation::HWND>;
}
impl windows_core::RuntimeName for IDesktopWindowTargetInterop {}
impl IDesktopWindowTargetInterop_Vtbl {
    pub const fn new<Identity: IDesktopWindowTargetInterop_Impl, const OFFSET: isize>() -> IDesktopWindowTargetInterop_Vtbl {
        unsafe extern "system" fn Hwnd<Identity: IDesktopWindowTargetInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDesktopWindowTargetInterop_Impl::Hwnd(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Hwnd: Hwnd::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopWindowTargetInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IVisualInteractionSourceInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn TryRedirectForManipulation(&self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IVisualInteractionSourceInterop {}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl IVisualInteractionSourceInterop_Vtbl {
    pub const fn new<Identity: IVisualInteractionSourceInterop_Impl, const OFFSET: isize>() -> IVisualInteractionSourceInterop_Vtbl {
        unsafe extern "system" fn TryRedirectForManipulation<Identity: IVisualInteractionSourceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVisualInteractionSourceInterop_Impl::TryRedirectForManipulation(this, core::mem::transmute_copy(&pointerinfo)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TryRedirectForManipulation: TryRedirectForManipulation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualInteractionSourceInterop as windows_core::Interface>::IID
    }
}
