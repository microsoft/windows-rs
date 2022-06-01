#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
pub trait IPdfRendererNative_Impl: Sized {
    fn RenderPageToSurface(&self, pdfpage: &::core::option::Option<::windows::core::IUnknown>, psurface: &::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISurface>, offset: &super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::Result<()>;
    fn RenderPageToDeviceContext(&self, pdfpage: &::core::option::Option<::windows::core::IUnknown>, pd2ddevicecontext: &::core::option::Option<super::super::super::Graphics::Direct2D::ID2D1DeviceContext>, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
impl ::windows::core::RuntimeName for IPdfRendererNative {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
impl IPdfRendererNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPdfRendererNative_Impl, const OFFSET: isize>() -> IPdfRendererNative_Vtbl {
        unsafe extern "system" fn RenderPageToSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPdfRendererNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, psurface: *mut ::core::ffi::c_void, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenderPageToSurface(::core::mem::transmute(&pdfpage), ::core::mem::transmute(&psurface), ::core::mem::transmute(&offset), ::core::mem::transmute_copy(&prenderparams)).into()
        }
        unsafe extern "system" fn RenderPageToDeviceContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPdfRendererNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, pd2ddevicecontext: *mut ::core::ffi::c_void, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenderPageToDeviceContext(::core::mem::transmute(&pdfpage), ::core::mem::transmute(&pd2ddevicecontext), ::core::mem::transmute_copy(&prenderparams)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RenderPageToSurface: RenderPageToSurface::<Identity, Impl, OFFSET>,
            RenderPageToDeviceContext: RenderPageToDeviceContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPdfRendererNative as ::windows::core::Interface>::IID
    }
}
