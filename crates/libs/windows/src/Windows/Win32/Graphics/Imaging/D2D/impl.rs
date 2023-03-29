#[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IWICImageEncoder_Impl: Sized {
    fn WriteFrame(&self, pimage: ::core::option::Option<&super::super::Direct2D::ID2D1Image>, pframeencode: ::core::option::Option<&super::IWICBitmapFrameEncode>, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()>;
    fn WriteFrameThumbnail(&self, pimage: ::core::option::Option<&super::super::Direct2D::ID2D1Image>, pframeencode: ::core::option::Option<&super::IWICBitmapFrameEncode>, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()>;
    fn WriteThumbnail(&self, pimage: ::core::option::Option<&super::super::Direct2D::ID2D1Image>, pencoder: ::core::option::Option<&super::IWICBitmapEncoder>, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IWICImageEncoder {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl IWICImageEncoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: isize>() -> IWICImageEncoder_Vtbl {
        unsafe extern "system" fn WriteFrame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: *mut ::core::ffi::c_void, pframeencode: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteFrame(::windows::core::from_raw_borrowed(&pimage), ::windows::core::from_raw_borrowed(&pframeencode), ::core::mem::transmute_copy(&pimageparameters)).into()
        }
        unsafe extern "system" fn WriteFrameThumbnail<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: *mut ::core::ffi::c_void, pframeencode: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteFrameThumbnail(::windows::core::from_raw_borrowed(&pimage), ::windows::core::from_raw_borrowed(&pframeencode), ::core::mem::transmute_copy(&pimageparameters)).into()
        }
        unsafe extern "system" fn WriteThumbnail<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimage: *mut ::core::ffi::c_void, pencoder: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteThumbnail(::windows::core::from_raw_borrowed(&pimage), ::windows::core::from_raw_borrowed(&pencoder), ::core::mem::transmute_copy(&pimageparameters)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WriteFrame: WriteFrame::<Identity, Impl, OFFSET>,
            WriteFrameThumbnail: WriteFrameThumbnail::<Identity, Impl, OFFSET>,
            WriteThumbnail: WriteThumbnail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICImageEncoder as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactory2_Impl: Sized + super::IWICImagingFactory_Impl {
    fn CreateImageEncoder(&self, pd2ddevice: ::core::option::Option<&super::super::Direct2D::ID2D1Device>) -> ::windows::core::Result<IWICImageEncoder>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IWICImagingFactory2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICImagingFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory2_Impl, const OFFSET: isize>() -> IWICImagingFactory2_Vtbl {
        unsafe extern "system" fn CreateImageEncoder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2ddevice: *mut ::core::ffi::c_void, ppwicimageencoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateImageEncoder(::windows::core::from_raw_borrowed(&pd2ddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwicimageencoder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::IWICImagingFactory_Vtbl::new::<Identity, Impl, OFFSET>(), CreateImageEncoder: CreateImageEncoder::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICImagingFactory2 as ::windows::core::ComInterface>::IID || iid == &<super::IWICImagingFactory as ::windows::core::ComInterface>::IID
    }
}
