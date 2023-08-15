#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`, `\"UI_Composition\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
pub trait ICompositionCapabilitiesInteropFactory_Impl: Sized {
    fn GetForWindow(&self, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<super::super::super::super::UI::Composition::CompositionCapabilities>;
}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
impl ::windows_core::RuntimeName for ICompositionCapabilitiesInteropFactory {}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
impl ICompositionCapabilitiesInteropFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionCapabilitiesInteropFactory_Impl, const OFFSET: isize>() -> ICompositionCapabilitiesInteropFactory_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionCapabilitiesInteropFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetForWindow(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ICompositionCapabilitiesInteropFactory, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICompositionCapabilitiesInteropFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICompositionDrawingSurfaceInterop_Impl: Sized {
    fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows_core::Result<()>;
    fn EndDraw(&self) -> ::windows_core::Result<()>;
    fn Resize(&self, sizepixels: &super::super::super::Foundation::SIZE) -> ::windows_core::Result<()>;
    fn Scroll(&self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::Result<()>;
    fn ResumeDraw(&self) -> ::windows_core::Result<()>;
    fn SuspendDraw(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICompositionDrawingSurfaceInterop {}
#[cfg(feature = "Win32_Foundation")]
impl ICompositionDrawingSurfaceInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>() -> ICompositionDrawingSurfaceInterop_Vtbl {
        unsafe extern "system" fn BeginDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDraw(::core::mem::transmute_copy(&updaterect), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&updateoffset)).into()
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDraw().into()
        }
        unsafe extern "system" fn Resize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resize(::core::mem::transmute(&sizepixels)).into()
        }
        unsafe extern "system" fn Scroll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Scroll(::core::mem::transmute_copy(&scrollrect), ::core::mem::transmute_copy(&cliprect), ::core::mem::transmute_copy(&offsetx), ::core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn ResumeDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResumeDraw().into()
        }
        unsafe extern "system" fn SuspendDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspendDraw().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, Impl, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceInterop as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICompositionDrawingSurfaceInterop2_Impl: Sized + ICompositionDrawingSurfaceInterop_Impl {
    fn CopySurface(&self, destinationresource: ::core::option::Option<&::windows_core::IUnknown>, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICompositionDrawingSurfaceInterop2 {}
#[cfg(feature = "Win32_Foundation")]
impl ICompositionDrawingSurfaceInterop2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop2_Impl, const OFFSET: isize>() -> ICompositionDrawingSurfaceInterop2_Vtbl {
        unsafe extern "system" fn CopySurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationresource: *mut ::core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopySurface(::windows_core::from_raw_borrowed(&destinationresource), ::core::mem::transmute_copy(&destinationoffsetx), ::core::mem::transmute_copy(&destinationoffsety), ::core::mem::transmute_copy(&sourcerectangle)).into()
        }
        Self { base__: ICompositionDrawingSurfaceInterop_Vtbl::new::<Identity, Impl, OFFSET>(), CopySurface: CopySurface::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceInterop2 as ::windows_core::ComInterface>::IID || iid == &<ICompositionDrawingSurfaceInterop as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`, `\"implement\"`*"]
pub trait ICompositionGraphicsDeviceInterop_Impl: Sized {
    fn GetRenderingDevice(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetRenderingDevice(&self, value: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICompositionGraphicsDeviceInterop {}
impl ICompositionGraphicsDeviceInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>() -> ICompositionGraphicsDeviceInterop_Vtbl {
        unsafe extern "system" fn GetRenderingDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRenderingDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderingDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRenderingDevice(::windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRenderingDevice: GetRenderingDevice::<Identity, Impl, OFFSET>,
            SetRenderingDevice: SetRenderingDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICompositionGraphicsDeviceInterop as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`, `\"UI_Composition_Desktop\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
pub trait ICompositorDesktopInterop_Impl: Sized {
    fn CreateDesktopWindowTarget(&self, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>;
    fn EnsureOnThread(&self, threadid: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
impl ::windows_core::RuntimeName for ICompositorDesktopInterop {}
#[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
impl ICompositorDesktopInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorDesktopInterop_Impl, const OFFSET: isize>() -> ICompositorDesktopInterop_Vtbl {
        unsafe extern "system" fn CreateDesktopWindowTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorDesktopInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDesktopWindowTarget(::core::mem::transmute_copy(&hwndtarget), ::core::mem::transmute_copy(&istopmost)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnsureOnThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorDesktopInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnsureOnThread(::core::mem::transmute_copy(&threadid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDesktopWindowTarget: CreateDesktopWindowTarget::<Identity, Impl, OFFSET>,
            EnsureOnThread: EnsureOnThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICompositorDesktopInterop as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`, `\"UI_Composition\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
pub trait ICompositorInterop_Impl: Sized {
    fn CreateCompositionSurfaceForHandle(&self, swapchain: super::super::super::Foundation::HANDLE) -> ::windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateCompositionSurfaceForSwapChain(&self, swapchain: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateGraphicsDevice(&self, renderingdevice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice>;
}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
impl ::windows_core::RuntimeName for ICompositorInterop {}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
impl ICompositorInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: isize>() -> ICompositorInterop_Vtbl {
        unsafe extern "system" fn CreateCompositionSurfaceForHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: super::super::super::Foundation::HANDLE, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCompositionSurfaceForHandle(::core::mem::transmute_copy(&swapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositionSurfaceForSwapChain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCompositionSurfaceForSwapChain(::windows_core::from_raw_borrowed(&swapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGraphicsDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGraphicsDevice(::windows_core::from_raw_borrowed(&renderingdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateCompositionSurfaceForHandle: CreateCompositionSurfaceForHandle::<Identity, Impl, OFFSET>,
            CreateCompositionSurfaceForSwapChain: CreateCompositionSurfaceForSwapChain::<Identity, Impl, OFFSET>,
            CreateGraphicsDevice: CreateGraphicsDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICompositorInterop as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDesktopWindowTargetInterop_Impl: Sized {
    fn Hwnd(&self) -> ::windows_core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDesktopWindowTargetInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IDesktopWindowTargetInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDesktopWindowTargetInterop_Impl, const OFFSET: isize>() -> IDesktopWindowTargetInterop_Vtbl {
        unsafe extern "system" fn Hwnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDesktopWindowTargetInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Hwnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Hwnd: Hwnd::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDesktopWindowTargetInterop as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Input_Pointer\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IVisualInteractionSourceInterop_Impl: Sized {
    fn TryRedirectForManipulation(&self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IVisualInteractionSourceInterop {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl IVisualInteractionSourceInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVisualInteractionSourceInterop_Impl, const OFFSET: isize>() -> IVisualInteractionSourceInterop_Vtbl {
        unsafe extern "system" fn TryRedirectForManipulation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVisualInteractionSourceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TryRedirectForManipulation(::core::mem::transmute_copy(&pointerinfo)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TryRedirectForManipulation: TryRedirectForManipulation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVisualInteractionSourceInterop as ::windows_core::ComInterface>::IID
    }
}
