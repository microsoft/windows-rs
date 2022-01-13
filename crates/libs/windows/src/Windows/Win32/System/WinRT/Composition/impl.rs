#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
pub trait ICompositionCapabilitiesInteropFactoryImpl: Sized {
    fn GetForWindow(&mut self, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<super::super::super::super::UI::Composition::CompositionCapabilities>;
}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
impl ::windows::core::RuntimeName for ICompositionCapabilitiesInteropFactory {
    const NAME: &'static str = "";
}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
impl ICompositionCapabilitiesInteropFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionCapabilitiesInteropFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionCapabilitiesInteropFactoryVtbl {
        unsafe extern "system" fn GetForWindow<Impl: ICompositionCapabilitiesInteropFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForWindow(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionCapabilitiesInteropFactory, BASE_OFFSET>(),
            GetForWindow: GetForWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionCapabilitiesInteropFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICompositionDrawingSurfaceInteropImpl: Sized {
    fn BeginDraw(&mut self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn EndDraw(&mut self) -> ::windows::core::Result<()>;
    fn Resize(&mut self, sizepixels: super::super::super::Foundation::SIZE) -> ::windows::core::Result<()>;
    fn Scroll(&mut self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()>;
    fn ResumeDraw(&mut self) -> ::windows::core::Result<()>;
    fn SuspendDraw(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICompositionDrawingSurfaceInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDrawingSurfaceInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionDrawingSurfaceInteropVtbl {
        unsafe extern "system" fn BeginDraw<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginDraw(::core::mem::transmute_copy(&updaterect), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&updateoffset)).into()
        }
        unsafe extern "system" fn EndDraw<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndDraw().into()
        }
        unsafe extern "system" fn Resize<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resize(::core::mem::transmute_copy(&sizepixels)).into()
        }
        unsafe extern "system" fn Scroll<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Scroll(::core::mem::transmute_copy(&scrollrect), ::core::mem::transmute_copy(&cliprect), ::core::mem::transmute_copy(&offsetx), ::core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn ResumeDraw<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResumeDraw().into()
        }
        unsafe extern "system" fn SuspendDraw<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuspendDraw().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginDraw: BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw: EndDraw::<Impl, IMPL_OFFSET>,
            Resize: Resize::<Impl, IMPL_OFFSET>,
            Scroll: Scroll::<Impl, IMPL_OFFSET>,
            ResumeDraw: ResumeDraw::<Impl, IMPL_OFFSET>,
            SuspendDraw: SuspendDraw::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICompositionDrawingSurfaceInterop2Impl: Sized + ICompositionDrawingSurfaceInteropImpl {
    fn CopySurface(&mut self, destinationresource: ::core::option::Option<::windows::core::IUnknown>, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICompositionDrawingSurfaceInterop2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDrawingSurfaceInterop2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionDrawingSurfaceInterop2Vtbl {
        unsafe extern "system" fn CopySurface<Impl: ICompositionDrawingSurfaceInterop2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationresource: *mut ::core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopySurface(::core::mem::transmute(&destinationresource), ::core::mem::transmute_copy(&destinationoffsetx), ::core::mem::transmute_copy(&destinationoffsety), ::core::mem::transmute_copy(&sourcerectangle)).into()
        }
        Self { base: ICompositionDrawingSurfaceInteropVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CopySurface: CopySurface::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceInterop2 as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionGraphicsDeviceInteropImpl: Sized {
    fn GetRenderingDevice(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetRenderingDevice(&mut self, value: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ICompositionGraphicsDeviceInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGraphicsDeviceInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGraphicsDeviceInteropVtbl {
        unsafe extern "system" fn GetRenderingDevice<Impl: ICompositionGraphicsDeviceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRenderingDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderingDevice<Impl: ICompositionGraphicsDeviceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRenderingDevice(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRenderingDevice: GetRenderingDevice::<Impl, IMPL_OFFSET>,
            SetRenderingDevice: SetRenderingDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGraphicsDeviceInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
pub trait ICompositorDesktopInteropImpl: Sized {
    fn CreateDesktopWindowTarget(&mut self, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>;
    fn EnsureOnThread(&mut self, threadid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
impl ICompositorDesktopInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorDesktopInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorDesktopInteropVtbl {
        unsafe extern "system" fn CreateDesktopWindowTarget<Impl: ICompositorDesktopInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDesktopWindowTarget(::core::mem::transmute_copy(&hwndtarget), ::core::mem::transmute_copy(&istopmost)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnsureOnThread<Impl: ICompositorDesktopInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnsureOnThread(::core::mem::transmute_copy(&threadid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateDesktopWindowTarget: CreateDesktopWindowTarget::<Impl, IMPL_OFFSET>,
            EnsureOnThread: EnsureOnThread::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositorDesktopInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
pub trait ICompositorInteropImpl: Sized {
    fn CreateCompositionSurfaceForHandle(&mut self, swapchain: super::super::super::Foundation::HANDLE) -> ::windows::core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateCompositionSurfaceForSwapChain(&mut self, swapchain: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateGraphicsDevice(&mut self, renderingdevice: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice>;
}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
impl ICompositorInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorInteropVtbl {
        unsafe extern "system" fn CreateCompositionSurfaceForHandle<Impl: ICompositorInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: super::super::super::Foundation::HANDLE, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCompositionSurfaceForHandle(::core::mem::transmute_copy(&swapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositionSurfaceForSwapChain<Impl: ICompositorInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCompositionSurfaceForSwapChain(::core::mem::transmute(&swapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGraphicsDevice<Impl: ICompositorInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGraphicsDevice(::core::mem::transmute(&renderingdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateCompositionSurfaceForHandle: CreateCompositionSurfaceForHandle::<Impl, IMPL_OFFSET>,
            CreateCompositionSurfaceForSwapChain: CreateCompositionSurfaceForSwapChain::<Impl, IMPL_OFFSET>,
            CreateGraphicsDevice: CreateGraphicsDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositorInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDesktopWindowTargetInteropImpl: Sized {
    fn Hwnd(&mut self) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDesktopWindowTargetInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowTargetInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesktopWindowTargetInteropVtbl {
        unsafe extern "system" fn Hwnd<Impl: IDesktopWindowTargetInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hwnd() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Hwnd: Hwnd::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesktopWindowTargetInterop as ::windows::core::Interface>::IID
    }
}
pub trait ISwapChainInteropImpl: Sized {
    fn SetSwapChain(&mut self, swapchain: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ISwapChainInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwapChainInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwapChainInteropVtbl {
        unsafe extern "system" fn SetSwapChain<Impl: ISwapChainInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSwapChain(::core::mem::transmute(&swapchain)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetSwapChain: SetSwapChain::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwapChainInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IVisualInteractionSourceInteropImpl: Sized {
    fn TryRedirectForManipulation(&mut self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl IVisualInteractionSourceInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSourceInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSourceInteropVtbl {
        unsafe extern "system" fn TryRedirectForManipulation<Impl: IVisualInteractionSourceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TryRedirectForManipulation(::core::mem::transmute_copy(&pointerinfo)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), TryRedirectForManipulation: TryRedirectForManipulation::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualInteractionSourceInterop as ::windows::core::Interface>::IID
    }
}
