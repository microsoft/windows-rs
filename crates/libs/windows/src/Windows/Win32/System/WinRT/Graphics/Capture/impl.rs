#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Capture\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IGraphicsCaptureItemInterop_Impl: Sized {
    fn CreateForWindow(&self, window: super::super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateForMonitor(&self, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IGraphicsCaptureItemInterop {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IGraphicsCaptureItemInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGraphicsCaptureItemInterop_Impl, const OFFSET: isize>() -> IGraphicsCaptureItemInterop_Vtbl {
        unsafe extern "system" fn CreateForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGraphicsCaptureItemInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateForWindow(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn CreateForMonitor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGraphicsCaptureItemInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateForMonitor(::core::mem::transmute_copy(&monitor), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&result)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, Impl, OFFSET>,
            CreateForMonitor: CreateForMonitor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureItemInterop as ::windows::core::ComInterface>::IID
    }
}
