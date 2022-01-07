pub trait IPdfRendererNativeImpl: Sized {
    fn RenderPageToSurface();
    fn RenderPageToDeviceContext();
}
impl ::windows::core::RuntimeName for IPdfRendererNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Pdf.IPdfRendererNative";
}
impl IPdfRendererNativeVtbl {
    pub const fn new<Impl: IPdfRendererNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPdfRendererNativeVtbl {
        unsafe extern "system" fn RenderPageToSurface<Impl: IPdfRendererNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, psurface: ::windows::core::RawPtr, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderPageToSurface(
                &*(&pdfpage as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&psurface as *const <super::super::super::Graphics::Dxgi::IDXGISurface as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Dxgi::IDXGISurface as ::windows::core::DefaultType>::DefaultType),
                &*(&offset as *const <super::super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType),
                &*(&prenderparams as *const <PDF_RENDER_PARAMS as ::windows::core::Abi>::Abi as *const <PDF_RENDER_PARAMS as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderPageToDeviceContext<Impl: IPdfRendererNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, pd2ddevicecontext: ::windows::core::RawPtr, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderPageToDeviceContext(
                &*(&pdfpage as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pd2ddevicecontext as *const <super::super::super::Graphics::Direct2D::ID2D1DeviceContext as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct2D::ID2D1DeviceContext as ::windows::core::DefaultType>::DefaultType),
                &*(&prenderparams as *const <PDF_RENDER_PARAMS as ::windows::core::Abi>::Abi as *const <PDF_RENDER_PARAMS as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPdfRendererNative>, base.5, RenderPageToSurface::<Impl, OFFSET>, RenderPageToDeviceContext::<Impl, OFFSET>)
    }
}
