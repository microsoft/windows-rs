pub trait ICompositionCapabilitiesInteropFactoryImpl: Sized {
    fn GetForWindow();
}
impl ::windows::core::RuntimeName for ICompositionCapabilitiesInteropFactory {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Composition.ICompositionCapabilitiesInteropFactory";
}
impl ICompositionCapabilitiesInteropFactoryVtbl {
    pub const fn new<Impl: ICompositionCapabilitiesInteropFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionCapabilitiesInteropFactoryVtbl {
        unsafe extern "system" fn GetForWindow<Impl: ICompositionCapabilitiesInteropFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionCapabilitiesInteropFactory>, base.5, GetForWindow::<Impl, OFFSET>)
    }
}
pub trait ICompositionDrawingSurfaceInteropImpl: Sized {
    fn BeginDraw();
    fn EndDraw();
    fn Resize();
    fn Scroll();
    fn ResumeDraw();
    fn SuspendDraw();
}
impl ::windows::core::RuntimeName for ICompositionDrawingSurfaceInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Composition.ICompositionDrawingSurfaceInterop";
}
impl ICompositionDrawingSurfaceInteropVtbl {
    pub const fn new<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionDrawingSurfaceInteropVtbl {
        unsafe extern "system" fn BeginDraw<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginDraw(&*(&updaterect as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&updateoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDraw<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resize<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resize(&*(&sizepixels as *const <super::super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scroll<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Scroll(&*(&scrollrect as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&cliprect as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), offsetx, offsety) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeDraw<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResumeDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendDraw<Impl: ICompositionDrawingSurfaceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuspendDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionDrawingSurfaceInterop>, base.5, BeginDraw::<Impl, OFFSET>, EndDraw::<Impl, OFFSET>, Resize::<Impl, OFFSET>, Scroll::<Impl, OFFSET>, ResumeDraw::<Impl, OFFSET>, SuspendDraw::<Impl, OFFSET>)
    }
}
pub trait ICompositionDrawingSurfaceInterop2Impl: Sized + ICompositionDrawingSurfaceInteropImpl {
    fn CopySurface();
}
impl ::windows::core::RuntimeName for ICompositionDrawingSurfaceInterop2 {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Composition.ICompositionDrawingSurfaceInterop2";
}
impl ICompositionDrawingSurfaceInterop2Vtbl {
    pub const fn new<Impl: ICompositionDrawingSurfaceInterop2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionDrawingSurfaceInterop2Vtbl {
        unsafe extern "system" fn CopySurface<Impl: ICompositionDrawingSurfaceInterop2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationresource: *mut ::core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopySurface(&*(&destinationresource as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), destinationoffsetx, destinationoffsety, &*(&sourcerectangle as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionDrawingSurfaceInterop2>, base.5, CopySurface::<Impl, OFFSET>)
    }
}
pub trait ICompositionGraphicsDeviceInteropImpl: Sized {
    fn GetRenderingDevice();
    fn SetRenderingDevice();
}
impl ::windows::core::RuntimeName for ICompositionGraphicsDeviceInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Composition.ICompositionGraphicsDeviceInterop";
}
impl ICompositionGraphicsDeviceInteropVtbl {
    pub const fn new<Impl: ICompositionGraphicsDeviceInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionGraphicsDeviceInteropVtbl {
        unsafe extern "system" fn GetRenderingDevice<Impl: ICompositionGraphicsDeviceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRenderingDevice(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderingDevice<Impl: ICompositionGraphicsDeviceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRenderingDevice(&*(&value as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionGraphicsDeviceInterop>, base.5, GetRenderingDevice::<Impl, OFFSET>, SetRenderingDevice::<Impl, OFFSET>)
    }
}
pub trait ICompositorDesktopInteropImpl: Sized {
    fn CreateDesktopWindowTarget();
    fn EnsureOnThread();
}
impl ::windows::core::RuntimeName for ICompositorDesktopInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Composition.ICompositorDesktopInterop";
}
impl ICompositorDesktopInteropVtbl {
    pub const fn new<Impl: ICompositorDesktopInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositorDesktopInteropVtbl {
        unsafe extern "system" fn CreateDesktopWindowTarget<Impl: ICompositorDesktopInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDesktopWindowTarget(&*(&hwndtarget as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&istopmost as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnsureOnThread<Impl: ICompositorDesktopInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnsureOnThread(threadid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositorDesktopInterop>, base.5, CreateDesktopWindowTarget::<Impl, OFFSET>, EnsureOnThread::<Impl, OFFSET>)
    }
}
pub trait ICompositorInteropImpl: Sized {
    fn CreateCompositionSurfaceForHandle();
    fn CreateCompositionSurfaceForSwapChain();
    fn CreateGraphicsDevice();
}
impl ::windows::core::RuntimeName for ICompositorInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Composition.ICompositorInterop";
}
impl ICompositorInteropVtbl {
    pub const fn new<Impl: ICompositorInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositorInteropVtbl {
        unsafe extern "system" fn CreateCompositionSurfaceForHandle<Impl: ICompositorInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, swapchain: super::super::super::Foundation::HANDLE, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCompositionSurfaceForHandle(&*(&swapchain as *const <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositionSurfaceForSwapChain<Impl: ICompositorInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCompositionSurfaceForSwapChain(&*(&swapchain as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGraphicsDevice<Impl: ICompositorInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGraphicsDevice(&*(&renderingdevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositorInterop>, base.5, CreateCompositionSurfaceForHandle::<Impl, OFFSET>, CreateCompositionSurfaceForSwapChain::<Impl, OFFSET>, CreateGraphicsDevice::<Impl, OFFSET>)
    }
}
pub trait IDesktopWindowTargetInteropImpl: Sized {
    fn Hwnd();
}
impl ::windows::core::RuntimeName for IDesktopWindowTargetInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Composition.IDesktopWindowTargetInterop";
}
impl IDesktopWindowTargetInteropVtbl {
    pub const fn new<Impl: IDesktopWindowTargetInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDesktopWindowTargetInteropVtbl {
        unsafe extern "system" fn Hwnd<Impl: IDesktopWindowTargetInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hwnd(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDesktopWindowTargetInterop>, base.5, Hwnd::<Impl, OFFSET>)
    }
}
pub trait ISwapChainInteropImpl: Sized {
    fn SetSwapChain();
}
impl ::windows::core::RuntimeName for ISwapChainInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Composition.ISwapChainInterop";
}
impl ISwapChainInteropVtbl {
    pub const fn new<Impl: ISwapChainInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISwapChainInteropVtbl {
        unsafe extern "system" fn SetSwapChain<Impl: ISwapChainInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSwapChain(&*(&swapchain as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISwapChainInterop>, base.5, SetSwapChain::<Impl, OFFSET>)
    }
}
pub trait IVisualInteractionSourceInteropImpl: Sized {
    fn TryRedirectForManipulation();
}
impl ::windows::core::RuntimeName for IVisualInteractionSourceInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Composition.IVisualInteractionSourceInterop";
}
impl IVisualInteractionSourceInteropVtbl {
    pub const fn new<Impl: IVisualInteractionSourceInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualInteractionSourceInteropVtbl {
        unsafe extern "system" fn TryRedirectForManipulation<Impl: IVisualInteractionSourceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryRedirectForManipulation(&*(&pointerinfo as *const <super::super::super::UI::Input::Pointer::POINTER_INFO as ::windows::core::Abi>::Abi as *const <super::super::super::UI::Input::Pointer::POINTER_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualInteractionSourceInterop>, base.5, TryRedirectForManipulation::<Impl, OFFSET>)
    }
}
