#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
pub trait IPdfRendererNativeImpl: Sized {
    fn RenderPageToSurface();
    fn RenderPageToDeviceContext();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
impl IPdfRendererNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfRendererNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPdfRendererNativeVtbl {
        unsafe extern "system" fn RenderPageToSurface<Impl: IPdfRendererNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, psurface: ::windows::core::RawPtr, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenderPageToDeviceContext<Impl: IPdfRendererNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, pd2ddevicecontext: ::windows::core::RawPtr, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RenderPageToSurface::<Impl, IMPL_OFFSET>, RenderPageToDeviceContext::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPdfRendererNative as ::windows::core::Interface>::IID
    }
}
