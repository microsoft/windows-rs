#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IGraphicsCaptureItemInteropImpl: Sized {
    fn CreateForWindow(&mut self, window: super::super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateForMonitor(&mut self, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IGraphicsCaptureItemInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureItemInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureItemInteropVtbl {
        unsafe extern "system" fn CreateForWindow<Impl: IGraphicsCaptureItemInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateForWindow(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn CreateForMonitor<Impl: IGraphicsCaptureItemInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateForMonitor(::core::mem::transmute_copy(&monitor), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&result)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateForWindow: CreateForWindow::<Impl, IMPL_OFFSET>,
            CreateForMonitor: CreateForMonitor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureItemInterop as ::windows::core::Interface>::IID
    }
}
