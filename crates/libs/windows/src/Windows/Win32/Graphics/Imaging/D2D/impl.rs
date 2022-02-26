#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IWICImageEncoder_Impl: Sized {
    fn WriteFrame(&self, pimage: &::core::option::Option<super::super::Direct2D::ID2D1Image>, pframeencode: &::core::option::Option<super::IWICBitmapFrameEncode>, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()>;
    fn WriteFrameThumbnail(&self, pimage: &::core::option::Option<super::super::Direct2D::ID2D1Image>, pframeencode: &::core::option::Option<super::IWICBitmapFrameEncode>, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()>;
    fn WriteThumbnail(&self, pimage: &::core::option::Option<super::super::Direct2D::ID2D1Image>, pencoder: &::core::option::Option<super::IWICBitmapEncoder>, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl IWICImageEncoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICImageEncoder_Impl, const OFFSET: isize>() -> IWICImageEncoder_Vtbl {
        unsafe extern "system" fn WriteFrame<Identity: ::windows::core::IUnknownImpl, Impl: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pframeencode: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteFrame(::core::mem::transmute(&pimage), ::core::mem::transmute(&pframeencode), ::core::mem::transmute_copy(&pimageparameters)).into()
        }
        unsafe extern "system" fn WriteFrameThumbnail<Identity: ::windows::core::IUnknownImpl, Impl: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pframeencode: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteFrameThumbnail(::core::mem::transmute(&pimage), ::core::mem::transmute(&pframeencode), ::core::mem::transmute_copy(&pimageparameters)).into()
        }
        unsafe extern "system" fn WriteThumbnail<Identity: ::windows::core::IUnknownImpl, Impl: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pencoder: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteThumbnail(::core::mem::transmute(&pimage), ::core::mem::transmute(&pencoder), ::core::mem::transmute_copy(&pimageparameters)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            WriteFrame: WriteFrame::<Identity, Impl, OFFSET>,
            WriteFrameThumbnail: WriteFrameThumbnail::<Identity, Impl, OFFSET>,
            WriteThumbnail: WriteThumbnail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICImageEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactory2_Impl: Sized + super::IWICImagingFactory_Impl {
    fn CreateImageEncoder(&self, pd2ddevice: &::core::option::Option<super::super::Direct2D::ID2D1Device>) -> ::windows::core::Result<IWICImageEncoder>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICImagingFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory2_Impl, const OFFSET: isize>() -> IWICImagingFactory2_Vtbl {
        unsafe extern "system" fn CreateImageEncoder<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2ddevice: ::windows::core::RawPtr, ppwicimageencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateImageEncoder(::core::mem::transmute(&pd2ddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwicimageencoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::IWICImagingFactory_Vtbl::new::<Identity, Impl, OFFSET>(), CreateImageEncoder: CreateImageEncoder::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICImagingFactory2 as ::windows::core::Interface>::IID || iid == &<super::IWICImagingFactory as ::windows::core::Interface>::IID
    }
}
