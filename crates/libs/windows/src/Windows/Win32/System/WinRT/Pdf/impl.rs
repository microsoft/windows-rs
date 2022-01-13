#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
pub trait IPdfRendererNativeImpl: Sized {
    fn RenderPageToSurface(&mut self, pdfpage: ::core::option::Option<::windows::core::IUnknown>, psurface: ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISurface>, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::Result<()>;
    fn RenderPageToDeviceContext(&mut self, pdfpage: ::core::option::Option<::windows::core::IUnknown>, pd2ddevicecontext: ::core::option::Option<super::super::super::Graphics::Direct2D::ID2D1DeviceContext>, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
impl IPdfRendererNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPdfRendererNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPdfRendererNativeVtbl {
        unsafe extern "system" fn RenderPageToSurface<Impl: IPdfRendererNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, psurface: ::windows::core::RawPtr, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenderPageToSurface(::core::mem::transmute(&pdfpage), ::core::mem::transmute(&psurface), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&prenderparams)).into()
        }
        unsafe extern "system" fn RenderPageToDeviceContext<Impl: IPdfRendererNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, pd2ddevicecontext: ::windows::core::RawPtr, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenderPageToDeviceContext(::core::mem::transmute(&pdfpage), ::core::mem::transmute(&pd2ddevicecontext), ::core::mem::transmute_copy(&prenderparams)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RenderPageToSurface: RenderPageToSurface::<Impl, IMPL_OFFSET>,
            RenderPageToDeviceContext: RenderPageToDeviceContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPdfRendererNative as ::windows::core::Interface>::IID
    }
}
