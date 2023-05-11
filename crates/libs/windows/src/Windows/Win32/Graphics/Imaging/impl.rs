#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICBitmap_Impl: Sized + IWICBitmapSource_Impl {
    fn Lock(&self, prclock: *const WICRect, flags: u32) -> ::windows_core::Result<IWICBitmapLock>;
    fn SetPalette(&self, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn SetResolution(&self, dpix: f64, dpiy: f64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICBitmap {}
impl IWICBitmap_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmap_Impl, const OFFSET: isize>() -> IWICBitmap_Vtbl {
        unsafe extern "system" fn Lock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prclock: *const WICRect, flags: u32, ppilock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Lock(::core::mem::transmute_copy(&prclock), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppilock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPalette(::windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn SetResolution<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetResolution(::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        Self {
            base__: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Lock: Lock::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            SetResolution: SetResolution::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmap as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICBitmapClipper_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: ::core::option::Option<&IWICBitmapSource>, prc: *const WICRect) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICBitmapClipper {}
impl IWICBitmapClipper_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapClipper_Impl, const OFFSET: isize>() -> IWICBitmapClipper_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, prc: *const WICRect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pisource), ::core::mem::transmute_copy(&prc)).into()
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapClipper as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapCodecInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetContainerFormat(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetPixelFormats(&self, cformats: u32, pguidpixelformats: *mut ::windows_core::GUID, pcactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetColorManagementVersion(&self, cchcolormanagementversion: u32, wzcolormanagementversion: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceManufacturer(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceModels(&self, cchdevicemodels: u32, wzdevicemodels: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetMimeTypes(&self, cchmimetypes: u32, wzmimetypes: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetFileExtensions(&self, cchfileextensions: u32, wzfileextensions: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn DoesSupportAnimation(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportChromakey(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportLossless(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportMultiframe(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn MatchesMimeType(&self, wzmimetype: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWICBitmapCodecInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapCodecInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>() -> IWICBitmapCodecInfo_Vtbl {
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidcontainerformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormats<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, pguidpixelformats: *mut ::windows_core::GUID, pcactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelFormats(::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&pguidpixelformats), ::core::mem::transmute_copy(&pcactual)).into()
        }
        unsafe extern "system" fn GetColorManagementVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorManagementVersion(::core::mem::transmute_copy(&cchcolormanagementversion), ::core::mem::transmute(&wzcolormanagementversion), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceManufacturer(::core::mem::transmute_copy(&cchdevicemanufacturer), ::core::mem::transmute(&wzdevicemanufacturer), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceModels<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceModels(::core::mem::transmute_copy(&cchdevicemodels), ::core::mem::transmute(&wzdevicemodels), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetMimeTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmimetypes: u32, wzmimetypes: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMimeTypes(::core::mem::transmute_copy(&cchmimetypes), ::core::mem::transmute(&wzmimetypes), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetFileExtensions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchfileextensions: u32, wzfileextensions: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFileExtensions(::core::mem::transmute_copy(&cchfileextensions), ::core::mem::transmute(&wzfileextensions), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn DoesSupportAnimation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportanimation: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoesSupportAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportanimation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportChromakey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportchromakey: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoesSupportChromakey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportchromakey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportLossless<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportlossless: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoesSupportLossless() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportlossless, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportMultiframe<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportmultiframe: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoesSupportMultiframe() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportmultiframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesMimeType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzmimetype: ::windows_core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MatchesMimeType(::core::mem::transmute(&wzmimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmatches, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICComponentInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetPixelFormats: GetPixelFormats::<Identity, Impl, OFFSET>,
            GetColorManagementVersion: GetColorManagementVersion::<Identity, Impl, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, Impl, OFFSET>,
            GetDeviceModels: GetDeviceModels::<Identity, Impl, OFFSET>,
            GetMimeTypes: GetMimeTypes::<Identity, Impl, OFFSET>,
            GetFileExtensions: GetFileExtensions::<Identity, Impl, OFFSET>,
            DoesSupportAnimation: DoesSupportAnimation::<Identity, Impl, OFFSET>,
            DoesSupportChromakey: DoesSupportChromakey::<Identity, Impl, OFFSET>,
            DoesSupportLossless: DoesSupportLossless::<Identity, Impl, OFFSET>,
            DoesSupportMultiframe: DoesSupportMultiframe::<Identity, Impl, OFFSET>,
            MatchesMimeType: MatchesMimeType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapCodecInfo as ::windows_core::ComInterface>::IID || iid == &<IWICComponentInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICBitmapCodecProgressNotification_Impl: Sized {
    fn RegisterProgressNotification(&self, pfnprogressnotification: PFNProgressNotification, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICBitmapCodecProgressNotification {}
impl IWICBitmapCodecProgressNotification_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecProgressNotification_Impl, const OFFSET: isize>() -> IWICBitmapCodecProgressNotification_Vtbl {
        unsafe extern "system" fn RegisterProgressNotification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapCodecProgressNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfnprogressnotification: PFNProgressNotification, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterProgressNotification(::core::mem::transmute_copy(&pfnprogressnotification), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&dwprogressflags)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterProgressNotification: RegisterProgressNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapCodecProgressNotification as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWICBitmapDecoder_Impl: Sized {
    fn QueryCapability(&self, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<u32>;
    fn Initialize(&self, pistream: ::core::option::Option<&super::super::System::Com::IStream>, cacheoptions: WICDecodeOptions) -> ::windows_core::Result<()>;
    fn GetContainerFormat(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDecoderInfo(&self) -> ::windows_core::Result<IWICBitmapDecoderInfo>;
    fn CopyPalette(&self, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn GetMetadataQueryReader(&self) -> ::windows_core::Result<IWICMetadataQueryReader>;
    fn GetPreview(&self) -> ::windows_core::Result<IWICBitmapSource>;
    fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: *mut ::core::option::Option<IWICColorContext>, pcactualcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetThumbnail(&self) -> ::windows_core::Result<IWICBitmapSource>;
    fn GetFrameCount(&self) -> ::windows_core::Result<u32>;
    fn GetFrame(&self, index: u32) -> ::windows_core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IWICBitmapDecoder {}
#[cfg(feature = "Win32_System_Com")]
impl IWICBitmapDecoder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>() -> IWICBitmapDecoder_Vtbl {
        unsafe extern "system" fn QueryCapability<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pdwcapability: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryCapability(::windows_core::from_raw_borrowed(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcapability, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, cacheoptions: WICDecodeOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&cacheoptions)).into()
        }
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidcontainerformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDecoderInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidecoderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDecoderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidecoderinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyPalette(::windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn GetMetadataQueryReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadataQueryReader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadataqueryreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreview<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreview() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapsource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContexts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut *mut ::core::ffi::c_void, pcactualcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontexts), ::core::mem::transmute_copy(&pcactualcount)).into()
        }
        unsafe extern "system" fn GetThumbnail<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppithumbnail, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppibitmapframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrame(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryCapability: QueryCapability::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetDecoderInfo: GetDecoderInfo::<Identity, Impl, OFFSET>,
            CopyPalette: CopyPalette::<Identity, Impl, OFFSET>,
            GetMetadataQueryReader: GetMetadataQueryReader::<Identity, Impl, OFFSET>,
            GetPreview: GetPreview::<Identity, Impl, OFFSET>,
            GetColorContexts: GetColorContexts::<Identity, Impl, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, Impl, OFFSET>,
            GetFrameCount: GetFrameCount::<Identity, Impl, OFFSET>,
            GetFrame: GetFrame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapDecoder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICBitmapDecoderInfo_Impl: Sized + IWICBitmapCodecInfo_Impl {
    fn GetPatterns(&self, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows_core::Result<()>;
    fn MatchesPattern(&self, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CreateInstance(&self) -> ::windows_core::Result<IWICBitmapDecoder>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IWICBitmapDecoderInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICBitmapDecoderInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>() -> IWICBitmapDecoderInfo_Vtbl {
        unsafe extern "system" fn GetPatterns<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPatterns(::core::mem::transmute_copy(&cbsizepatterns), ::core::mem::transmute_copy(&ppatterns), ::core::mem::transmute_copy(&pcpatterns), ::core::mem::transmute_copy(&pcbpatternsactual)).into()
        }
        unsafe extern "system" fn MatchesPattern<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MatchesPattern(::windows_core::from_raw_borrowed(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmatches, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapdecoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapdecoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICBitmapCodecInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPatterns: GetPatterns::<Identity, Impl, OFFSET>,
            MatchesPattern: MatchesPattern::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapDecoderInfo as ::windows_core::ComInterface>::IID || iid == &<IWICComponentInfo as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapCodecInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICBitmapEncoder_Impl: Sized {
    fn Initialize(&self, pistream: ::core::option::Option<&super::super::System::Com::IStream>, cacheoption: WICBitmapEncoderCacheOption) -> ::windows_core::Result<()>;
    fn GetContainerFormat(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetEncoderInfo(&self) -> ::windows_core::Result<IWICBitmapEncoderInfo>;
    fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const ::core::option::Option<IWICColorContext>) -> ::windows_core::Result<()>;
    fn SetPalette(&self, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn SetThumbnail(&self, pithumbnail: ::core::option::Option<&IWICBitmapSource>) -> ::windows_core::Result<()>;
    fn SetPreview(&self, pipreview: ::core::option::Option<&IWICBitmapSource>) -> ::windows_core::Result<()>;
    fn CreateNewFrame(&self, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, ppiencoderoptions: *mut ::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyBag2>) -> ::windows_core::Result<()>;
    fn Commit(&self) -> ::windows_core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> ::windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::RuntimeName for IWICBitmapEncoder {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICBitmapEncoder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>() -> IWICBitmapEncoder_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, cacheoption: WICBitmapEncoderCacheOption) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&cacheoption)).into()
        }
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidcontainerformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEncoderInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiencoderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEncoderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiencoderinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorContexts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontext)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPalette(::windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn SetThumbnail<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pithumbnail: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThumbnail(::windows_core::from_raw_borrowed(&pithumbnail)).into()
        }
        unsafe extern "system" fn SetPreview<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipreview: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPreview(::windows_core::from_raw_borrowed(&pipreview)).into()
        }
        unsafe extern "system" fn CreateNewFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut *mut ::core::ffi::c_void, ppiencoderoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateNewFrame(::core::mem::transmute_copy(&ppiframeencode), ::core::mem::transmute_copy(&ppiencoderoptions)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadataQueryWriter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadataquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetEncoderInfo: GetEncoderInfo::<Identity, Impl, OFFSET>,
            SetColorContexts: SetColorContexts::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, Impl, OFFSET>,
            SetPreview: SetPreview::<Identity, Impl, OFFSET>,
            CreateNewFrame: CreateNewFrame::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapEncoder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapEncoderInfo_Impl: Sized + IWICBitmapCodecInfo_Impl {
    fn CreateInstance(&self) -> ::windows_core::Result<IWICBitmapEncoder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWICBitmapEncoderInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapEncoderInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoderInfo_Impl, const OFFSET: isize>() -> IWICBitmapEncoderInfo_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapEncoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapencoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapencoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IWICBitmapCodecInfo_Vtbl::new::<Identity, Impl, OFFSET>(), CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapEncoderInfo as ::windows_core::ComInterface>::IID || iid == &<IWICComponentInfo as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapCodecInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICBitmapFlipRotator_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: ::core::option::Option<&IWICBitmapSource>, options: WICBitmapTransformOptions) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICBitmapFlipRotator {}
impl IWICBitmapFlipRotator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFlipRotator_Impl, const OFFSET: isize>() -> IWICBitmapFlipRotator_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFlipRotator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, options: WICBitmapTransformOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pisource), ::core::mem::transmute_copy(&options)).into()
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapFlipRotator as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICBitmapFrameDecode_Impl: Sized + IWICBitmapSource_Impl {
    fn GetMetadataQueryReader(&self) -> ::windows_core::Result<IWICMetadataQueryReader>;
    fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: *mut ::core::option::Option<IWICColorContext>, pcactualcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetThumbnail(&self) -> ::windows_core::Result<IWICBitmapSource>;
}
impl ::windows_core::RuntimeName for IWICBitmapFrameDecode {}
impl IWICBitmapFrameDecode_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: isize>() -> IWICBitmapFrameDecode_Vtbl {
        unsafe extern "system" fn GetMetadataQueryReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadataQueryReader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadataqueryreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContexts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut *mut ::core::ffi::c_void, pcactualcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontexts), ::core::mem::transmute_copy(&pcactualcount)).into()
        }
        unsafe extern "system" fn GetThumbnail<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppithumbnail, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMetadataQueryReader: GetMetadataQueryReader::<Identity, Impl, OFFSET>,
            GetColorContexts: GetColorContexts::<Identity, Impl, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapFrameDecode as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICBitmapFrameEncode_Impl: Sized {
    fn Initialize(&self, piencoderoptions: ::core::option::Option<&super::super::System::Com::StructuredStorage::IPropertyBag2>) -> ::windows_core::Result<()>;
    fn SetSize(&self, uiwidth: u32, uiheight: u32) -> ::windows_core::Result<()>;
    fn SetResolution(&self, dpix: f64, dpiy: f64) -> ::windows_core::Result<()>;
    fn SetPixelFormat(&self, ppixelformat: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const ::core::option::Option<IWICColorContext>) -> ::windows_core::Result<()>;
    fn SetPalette(&self, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn SetThumbnail(&self, pithumbnail: ::core::option::Option<&IWICBitmapSource>) -> ::windows_core::Result<()>;
    fn WritePixels(&self, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows_core::Result<()>;
    fn WriteSource(&self, pibitmapsource: ::core::option::Option<&IWICBitmapSource>, prc: *const WICRect) -> ::windows_core::Result<()>;
    fn Commit(&self) -> ::windows_core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> ::windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::RuntimeName for IWICBitmapFrameEncode {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICBitmapFrameEncode_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>() -> IWICBitmapFrameEncode_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piencoderoptions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&piencoderoptions)).into()
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSize(::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight)).into()
        }
        unsafe extern "system" fn SetResolution<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetResolution(::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn SetPixelFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPixelFormat(::core::mem::transmute_copy(&ppixelformat)).into()
        }
        unsafe extern "system" fn SetColorContexts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontext)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPalette(::windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn SetThumbnail<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pithumbnail: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThumbnail(::windows_core::from_raw_borrowed(&pithumbnail)).into()
        }
        unsafe extern "system" fn WritePixels<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WritePixels(::core::mem::transmute_copy(&linecount), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbpixels)).into()
        }
        unsafe extern "system" fn WriteSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, prc: *const WICRect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteSource(::windows_core::from_raw_borrowed(&pibitmapsource), ::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadataQueryWriter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadataquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            SetResolution: SetResolution::<Identity, Impl, OFFSET>,
            SetPixelFormat: SetPixelFormat::<Identity, Impl, OFFSET>,
            SetColorContexts: SetColorContexts::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, Impl, OFFSET>,
            WritePixels: WritePixels::<Identity, Impl, OFFSET>,
            WriteSource: WriteSource::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapFrameEncode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICBitmapLock_Impl: Sized {
    fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::Result<()>;
    fn GetStride(&self) -> ::windows_core::Result<u32>;
    fn GetDataPointer(&self, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows_core::Result<()>;
    fn GetPixelFormat(&self) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::RuntimeName for IWICBitmapLock {}
impl IWICBitmapLock_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapLock_Impl, const OFFSET: isize>() -> IWICBitmapLock_Vtbl {
        unsafe extern "system" fn GetSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSize(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetStride<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbstride: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStride() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbstride, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPointer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataPointer(::core::mem::transmute_copy(&pcbbuffersize), ::core::mem::transmute_copy(&ppbdata)).into()
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixelformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetStride: GetStride::<Identity, Impl, OFFSET>,
            GetDataPointer: GetDataPointer::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapLock as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICBitmapScaler_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: ::core::option::Option<&IWICBitmapSource>, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICBitmapScaler {}
impl IWICBitmapScaler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapScaler_Impl, const OFFSET: isize>() -> IWICBitmapScaler_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapScaler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pisource), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&mode)).into()
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapScaler as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICBitmapSource_Impl: Sized {
    fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::Result<()>;
    fn GetPixelFormat(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows_core::Result<()>;
    fn CopyPalette(&self, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICBitmapSource {}
impl IWICBitmapSource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: isize>() -> IWICBitmapSource_Vtbl {
        unsafe extern "system" fn GetSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSize(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixelformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolution<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResolution(::core::mem::transmute_copy(&pdpix), ::core::mem::transmute_copy(&pdpiy)).into()
        }
        unsafe extern "system" fn CopyPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyPalette(::windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn CopyPixels<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyPixels(::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetResolution: GetResolution::<Identity, Impl, OFFSET>,
            CopyPalette: CopyPalette::<Identity, Impl, OFFSET>,
            CopyPixels: CopyPixels::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapSourceTransform_Impl: Sized {
    fn CopyPixels(&self, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows_core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::Result<()>;
    fn GetClosestSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::Result<()>;
    fn GetClosestPixelFormat(&self, pguiddstformat: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn DoesSupportTransform(&self, dsttransform: WICBitmapTransformOptions) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWICBitmapSourceTransform {}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapSourceTransform_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>() -> IWICBitmapSourceTransform_Vtbl {
        unsafe extern "system" fn CopyPixels<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows_core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyPixels(::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pguiddstformat), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&nstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        unsafe extern "system" fn GetClosestSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClosestSize(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetClosestPixelFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguiddstformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClosestPixelFormat(::core::mem::transmute_copy(&pguiddstformat)).into()
        }
        unsafe extern "system" fn DoesSupportTransform<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dsttransform: WICBitmapTransformOptions, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoesSupportTransform(::core::mem::transmute_copy(&dsttransform)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfissupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CopyPixels: CopyPixels::<Identity, Impl, OFFSET>,
            GetClosestSize: GetClosestSize::<Identity, Impl, OFFSET>,
            GetClosestPixelFormat: GetClosestPixelFormat::<Identity, Impl, OFFSET>,
            DoesSupportTransform: DoesSupportTransform::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICBitmapSourceTransform as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICColorContext_Impl: Sized {
    fn InitializeFromFilename(&self, wzfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows_core::Result<()>;
    fn InitializeFromExifColorSpace(&self, value: u32) -> ::windows_core::Result<()>;
    fn GetType(&self) -> ::windows_core::Result<WICColorContextType>;
    fn GetProfileBytes(&self, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetExifColorSpace(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IWICColorContext {}
impl IWICColorContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: isize>() -> IWICColorContext_Vtbl {
        unsafe extern "system" fn InitializeFromFilename<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFromFilename(::core::mem::transmute(&wzfilename)).into()
        }
        unsafe extern "system" fn InitializeFromMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFromMemory(::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&cbbuffersize)).into()
        }
        unsafe extern "system" fn InitializeFromExifColorSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFromExifColorSpace(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WICColorContextType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProfileBytes(::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn GetExifColorSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExifColorSpace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromFilename: InitializeFromFilename::<Identity, Impl, OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Identity, Impl, OFFSET>,
            InitializeFromExifColorSpace: InitializeFromExifColorSpace::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetProfileBytes: GetProfileBytes::<Identity, Impl, OFFSET>,
            GetExifColorSpace: GetExifColorSpace::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICColorContext as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICColorTransform_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pibitmapsource: ::core::option::Option<&IWICBitmapSource>, picontextsource: ::core::option::Option<&IWICColorContext>, picontextdest: ::core::option::Option<&IWICColorContext>, pixelfmtdest: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICColorTransform {}
impl IWICColorTransform_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICColorTransform_Impl, const OFFSET: isize>() -> IWICColorTransform_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICColorTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, picontextsource: *mut ::core::ffi::c_void, picontextdest: *mut ::core::ffi::c_void, pixelfmtdest: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pibitmapsource), ::windows_core::from_raw_borrowed(&picontextsource), ::windows_core::from_raw_borrowed(&picontextdest), ::core::mem::transmute_copy(&pixelfmtdest)).into()
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICColorTransform as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICComponentFactory_Impl: Sized + IWICImagingFactory_Impl {
    fn CreateMetadataReader(&self, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwoptions: u32, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<IWICMetadataReader>;
    fn CreateMetadataReaderFromContainer(&self, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwoptions: u32, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<IWICMetadataReader>;
    fn CreateMetadataWriter(&self, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwmetadataoptions: u32) -> ::windows_core::Result<IWICMetadataWriter>;
    fn CreateMetadataWriterFromReader(&self, pireader: ::core::option::Option<&IWICMetadataReader>, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<IWICMetadataWriter>;
    fn CreateQueryReaderFromBlockReader(&self, piblockreader: ::core::option::Option<&IWICMetadataBlockReader>) -> ::windows_core::Result<IWICMetadataQueryReader>;
    fn CreateQueryWriterFromBlockWriter(&self, piblockwriter: ::core::option::Option<&IWICMetadataBlockWriter>) -> ::windows_core::Result<IWICMetadataQueryWriter>;
    fn CreateEncoderPropertyBag(&self, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IWICComponentFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICComponentFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: isize>() -> IWICComponentFactory_Vtbl {
        unsafe extern "system" fn CreateMetadataReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwoptions: u32, pistream: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMetadataReader(::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwoptions), ::windows_core::from_raw_borrowed(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppireader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataReaderFromContainer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwoptions: u32, pistream: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMetadataReaderFromContainer(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwoptions), ::windows_core::from_raw_borrowed(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppireader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataWriter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwmetadataoptions: u32, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMetadataWriter(::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwmetadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataWriterFromReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pireader: *mut ::core::ffi::c_void, pguidvendor: *const ::windows_core::GUID, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMetadataWriterFromReader(::windows_core::from_raw_borrowed(&pireader), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryReaderFromBlockReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piblockreader: *mut ::core::ffi::c_void, ppiqueryreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateQueryReaderFromBlockReader(::windows_core::from_raw_borrowed(&piblockreader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiqueryreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromBlockWriter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piblockwriter: *mut ::core::ffi::c_void, ppiquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateQueryWriterFromBlockWriter(::windows_core::from_raw_borrowed(&piblockwriter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncoderPropertyBag<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32, ppipropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEncoderPropertyBag(::core::mem::transmute_copy(&ppropoptions), ::core::mem::transmute_copy(&ccount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICImagingFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateMetadataReader: CreateMetadataReader::<Identity, Impl, OFFSET>,
            CreateMetadataReaderFromContainer: CreateMetadataReaderFromContainer::<Identity, Impl, OFFSET>,
            CreateMetadataWriter: CreateMetadataWriter::<Identity, Impl, OFFSET>,
            CreateMetadataWriterFromReader: CreateMetadataWriterFromReader::<Identity, Impl, OFFSET>,
            CreateQueryReaderFromBlockReader: CreateQueryReaderFromBlockReader::<Identity, Impl, OFFSET>,
            CreateQueryWriterFromBlockWriter: CreateQueryWriterFromBlockWriter::<Identity, Impl, OFFSET>,
            CreateEncoderPropertyBag: CreateEncoderPropertyBag::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICComponentFactory as ::windows_core::ComInterface>::IID || iid == &<IWICImagingFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICComponentInfo_Impl: Sized {
    fn GetComponentType(&self) -> ::windows_core::Result<WICComponentType>;
    fn GetCLSID(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetSigningStatus(&self) -> ::windows_core::Result<u32>;
    fn GetAuthor(&self, cchauthor: u32, wzauthor: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetVendorGUID(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetVersion(&self, cchversion: u32, wzversion: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetSpecVersion(&self, cchspecversion: u32, wzspecversion: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetFriendlyName(&self, cchfriendlyname: u32, wzfriendlyname: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICComponentInfo {}
impl IWICComponentInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: isize>() -> IWICComponentInfo_Vtbl {
        unsafe extern "system" fn GetComponentType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetComponentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCLSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSigningStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAuthor(::core::mem::transmute_copy(&cchauthor), ::core::mem::transmute(&wzauthor), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetVendorGUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVendorGUID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidvendor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVersion(::core::mem::transmute_copy(&cchversion), ::core::mem::transmute(&wzversion), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetSpecVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSpecVersion(::core::mem::transmute_copy(&cchspecversion), ::core::mem::transmute(&wzspecversion), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetFriendlyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFriendlyName(::core::mem::transmute_copy(&cchfriendlyname), ::core::mem::transmute(&wzfriendlyname), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetComponentType: GetComponentType::<Identity, Impl, OFFSET>,
            GetCLSID: GetCLSID::<Identity, Impl, OFFSET>,
            GetSigningStatus: GetSigningStatus::<Identity, Impl, OFFSET>,
            GetAuthor: GetAuthor::<Identity, Impl, OFFSET>,
            GetVendorGUID: GetVendorGUID::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetSpecVersion: GetSpecVersion::<Identity, Impl, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICComponentInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsDecoder_Impl: Sized {
    fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> ::windows_core::Result<()>;
    fn GetFrame(&self, arrayindex: u32, miplevel: u32, sliceindex: u32) -> ::windows_core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::RuntimeName for IWICDdsDecoder {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsDecoder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDdsDecoder_Impl, const OFFSET: isize>() -> IWICDdsDecoder_Vtbl {
        unsafe extern "system" fn GetParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDdsDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParameters(::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn GetFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDdsDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arrayindex: u32, miplevel: u32, sliceindex: u32, ppibitmapframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrame(::core::mem::transmute_copy(&arrayindex), ::core::mem::transmute_copy(&miplevel), ::core::mem::transmute_copy(&sliceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            GetFrame: GetFrame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICDdsDecoder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsEncoder_Impl: Sized {
    fn SetParameters(&self, pparameters: *const WICDdsParameters) -> ::windows_core::Result<()>;
    fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> ::windows_core::Result<()>;
    fn CreateNewFrame(&self, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::RuntimeName for IWICDdsEncoder {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsEncoder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDdsEncoder_Impl, const OFFSET: isize>() -> IWICDdsEncoder_Vtbl {
        unsafe extern "system" fn SetParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const WICDdsParameters) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameters(::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn GetParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParameters(::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn CreateNewFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut *mut ::core::ffi::c_void, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateNewFrame(::core::mem::transmute_copy(&ppiframeencode), ::core::mem::transmute_copy(&parrayindex), ::core::mem::transmute_copy(&pmiplevel), ::core::mem::transmute_copy(&psliceindex)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            CreateNewFrame: CreateNewFrame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICDdsEncoder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsFrameDecode_Impl: Sized {
    fn GetSizeInBlocks(&self, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows_core::Result<()>;
    fn GetFormatInfo(&self) -> ::windows_core::Result<WICDdsFormatInfo>;
    fn CopyBlocks(&self, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::RuntimeName for IWICDdsFrameDecode {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsFrameDecode_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDdsFrameDecode_Impl, const OFFSET: isize>() -> IWICDdsFrameDecode_Vtbl {
        unsafe extern "system" fn GetSizeInBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSizeInBlocks(::core::mem::transmute_copy(&pwidthinblocks), ::core::mem::transmute_copy(&pheightinblocks)).into()
        }
        unsafe extern "system" fn GetFormatInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatinfo: *mut WICDdsFormatInfo) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormatInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformatinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyBlocks(::core::mem::transmute_copy(&prcboundsinblocks), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSizeInBlocks: GetSizeInBlocks::<Identity, Impl, OFFSET>,
            GetFormatInfo: GetFormatInfo::<Identity, Impl, OFFSET>,
            CopyBlocks: CopyBlocks::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICDdsFrameDecode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICDevelopRaw_Impl: Sized + IWICBitmapFrameDecode_Impl {
    fn QueryRawCapabilitiesInfo(&self, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows_core::Result<()>;
    fn LoadParameterSet(&self, parameterset: WICRawParameterSet) -> ::windows_core::Result<()>;
    fn GetCurrentParameterSet(&self) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2>;
    fn SetExposureCompensation(&self, ev: f64) -> ::windows_core::Result<()>;
    fn GetExposureCompensation(&self) -> ::windows_core::Result<f64>;
    fn SetWhitePointRGB(&self, red: u32, green: u32, blue: u32) -> ::windows_core::Result<()>;
    fn GetWhitePointRGB(&self, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows_core::Result<()>;
    fn SetNamedWhitePoint(&self, whitepoint: WICNamedWhitePoint) -> ::windows_core::Result<()>;
    fn GetNamedWhitePoint(&self) -> ::windows_core::Result<WICNamedWhitePoint>;
    fn SetWhitePointKelvin(&self, whitepointkelvin: u32) -> ::windows_core::Result<()>;
    fn GetWhitePointKelvin(&self) -> ::windows_core::Result<u32>;
    fn GetKelvinRangeInfo(&self, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows_core::Result<()>;
    fn SetContrast(&self, contrast: f64) -> ::windows_core::Result<()>;
    fn GetContrast(&self) -> ::windows_core::Result<f64>;
    fn SetGamma(&self, gamma: f64) -> ::windows_core::Result<()>;
    fn GetGamma(&self) -> ::windows_core::Result<f64>;
    fn SetSharpness(&self, sharpness: f64) -> ::windows_core::Result<()>;
    fn GetSharpness(&self) -> ::windows_core::Result<f64>;
    fn SetSaturation(&self, saturation: f64) -> ::windows_core::Result<()>;
    fn GetSaturation(&self) -> ::windows_core::Result<f64>;
    fn SetTint(&self, tint: f64) -> ::windows_core::Result<()>;
    fn GetTint(&self) -> ::windows_core::Result<f64>;
    fn SetNoiseReduction(&self, noisereduction: f64) -> ::windows_core::Result<()>;
    fn GetNoiseReduction(&self) -> ::windows_core::Result<f64>;
    fn SetDestinationColorContext(&self, pcolorcontext: ::core::option::Option<&IWICColorContext>) -> ::windows_core::Result<()>;
    fn SetToneCurve(&self, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows_core::Result<()>;
    fn GetToneCurve(&self, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows_core::Result<()>;
    fn SetRotation(&self, rotation: f64) -> ::windows_core::Result<()>;
    fn GetRotation(&self) -> ::windows_core::Result<f64>;
    fn SetRenderMode(&self, rendermode: WICRawRenderMode) -> ::windows_core::Result<()>;
    fn GetRenderMode(&self) -> ::windows_core::Result<WICRawRenderMode>;
    fn SetNotificationCallback(&self, pcallback: ::core::option::Option<&IWICDevelopRawNotificationCallback>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::RuntimeName for IWICDevelopRaw {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICDevelopRaw_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>() -> IWICDevelopRaw_Vtbl {
        unsafe extern "system" fn QueryRawCapabilitiesInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryRawCapabilitiesInfo(::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn LoadParameterSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterset: WICRawParameterSet) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadParameterSet(::core::mem::transmute_copy(&parameterset)).into()
        }
        unsafe extern "system" fn GetCurrentParameterSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcurrentparameterset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentParameterSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcurrentparameterset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExposureCompensation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ev: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExposureCompensation(::core::mem::transmute_copy(&ev)).into()
        }
        unsafe extern "system" fn GetExposureCompensation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pev: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExposureCompensation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pev, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointRGB<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, red: u32, green: u32, blue: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWhitePointRGB(::core::mem::transmute_copy(&red), ::core::mem::transmute_copy(&green), ::core::mem::transmute_copy(&blue)).into()
        }
        unsafe extern "system" fn GetWhitePointRGB<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWhitePointRGB(::core::mem::transmute_copy(&pred), ::core::mem::transmute_copy(&pgreen), ::core::mem::transmute_copy(&pblue)).into()
        }
        unsafe extern "system" fn SetNamedWhitePoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepoint: WICNamedWhitePoint) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNamedWhitePoint(::core::mem::transmute_copy(&whitepoint)).into()
        }
        unsafe extern "system" fn GetNamedWhitePoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwhitepoint: *mut WICNamedWhitePoint) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNamedWhitePoint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwhitepoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointKelvin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointkelvin: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWhitePointKelvin(::core::mem::transmute_copy(&whitepointkelvin)).into()
        }
        unsafe extern "system" fn GetWhitePointKelvin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwhitepointkelvin: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWhitePointKelvin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwhitepointkelvin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKelvinRangeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetKelvinRangeInfo(::core::mem::transmute_copy(&pminkelvintemp), ::core::mem::transmute_copy(&pmaxkelvintemp), ::core::mem::transmute_copy(&pkelvintempstepvalue)).into()
        }
        unsafe extern "system" fn SetContrast<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contrast: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContrast(::core::mem::transmute_copy(&contrast)).into()
        }
        unsafe extern "system" fn GetContrast<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrast: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContrast() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontrast, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGamma<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGamma(::core::mem::transmute_copy(&gamma)).into()
        }
        unsafe extern "system" fn GetGamma<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgamma: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGamma() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgamma, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharpness<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharpness: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSharpness(::core::mem::transmute_copy(&sharpness)).into()
        }
        unsafe extern "system" fn GetSharpness<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharpness: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSharpness() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psharpness, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaturation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, saturation: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSaturation(::core::mem::transmute_copy(&saturation)).into()
        }
        unsafe extern "system" fn GetSaturation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psaturation: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSaturation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psaturation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tint: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTint(::core::mem::transmute_copy(&tint)).into()
        }
        unsafe extern "system" fn GetTint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptint: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoiseReduction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noisereduction: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNoiseReduction(::core::mem::transmute_copy(&noisereduction)).into()
        }
        unsafe extern "system" fn GetNoiseReduction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnoisereduction: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNoiseReduction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnoisereduction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationColorContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDestinationColorContext(::windows_core::from_raw_borrowed(&pcolorcontext)).into()
        }
        unsafe extern "system" fn SetToneCurve<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetToneCurve(::core::mem::transmute_copy(&cbtonecurvesize), ::core::mem::transmute_copy(&ptonecurve)).into()
        }
        unsafe extern "system" fn GetToneCurve<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetToneCurve(::core::mem::transmute_copy(&cbtonecurvebuffersize), ::core::mem::transmute_copy(&ptonecurve), ::core::mem::transmute_copy(&pcbactualtonecurvebuffersize)).into()
        }
        unsafe extern "system" fn SetRotation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRotation(::core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn GetRotation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protation: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRotation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(protation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendermode: WICRawRenderMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRenderMode(::core::mem::transmute_copy(&rendermode)).into()
        }
        unsafe extern "system" fn GetRenderMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendermode: *mut WICRawRenderMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRenderMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prendermode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNotificationCallback(::windows_core::from_raw_borrowed(&pcallback)).into()
        }
        Self {
            base__: IWICBitmapFrameDecode_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueryRawCapabilitiesInfo: QueryRawCapabilitiesInfo::<Identity, Impl, OFFSET>,
            LoadParameterSet: LoadParameterSet::<Identity, Impl, OFFSET>,
            GetCurrentParameterSet: GetCurrentParameterSet::<Identity, Impl, OFFSET>,
            SetExposureCompensation: SetExposureCompensation::<Identity, Impl, OFFSET>,
            GetExposureCompensation: GetExposureCompensation::<Identity, Impl, OFFSET>,
            SetWhitePointRGB: SetWhitePointRGB::<Identity, Impl, OFFSET>,
            GetWhitePointRGB: GetWhitePointRGB::<Identity, Impl, OFFSET>,
            SetNamedWhitePoint: SetNamedWhitePoint::<Identity, Impl, OFFSET>,
            GetNamedWhitePoint: GetNamedWhitePoint::<Identity, Impl, OFFSET>,
            SetWhitePointKelvin: SetWhitePointKelvin::<Identity, Impl, OFFSET>,
            GetWhitePointKelvin: GetWhitePointKelvin::<Identity, Impl, OFFSET>,
            GetKelvinRangeInfo: GetKelvinRangeInfo::<Identity, Impl, OFFSET>,
            SetContrast: SetContrast::<Identity, Impl, OFFSET>,
            GetContrast: GetContrast::<Identity, Impl, OFFSET>,
            SetGamma: SetGamma::<Identity, Impl, OFFSET>,
            GetGamma: GetGamma::<Identity, Impl, OFFSET>,
            SetSharpness: SetSharpness::<Identity, Impl, OFFSET>,
            GetSharpness: GetSharpness::<Identity, Impl, OFFSET>,
            SetSaturation: SetSaturation::<Identity, Impl, OFFSET>,
            GetSaturation: GetSaturation::<Identity, Impl, OFFSET>,
            SetTint: SetTint::<Identity, Impl, OFFSET>,
            GetTint: GetTint::<Identity, Impl, OFFSET>,
            SetNoiseReduction: SetNoiseReduction::<Identity, Impl, OFFSET>,
            GetNoiseReduction: GetNoiseReduction::<Identity, Impl, OFFSET>,
            SetDestinationColorContext: SetDestinationColorContext::<Identity, Impl, OFFSET>,
            SetToneCurve: SetToneCurve::<Identity, Impl, OFFSET>,
            GetToneCurve: GetToneCurve::<Identity, Impl, OFFSET>,
            SetRotation: SetRotation::<Identity, Impl, OFFSET>,
            GetRotation: GetRotation::<Identity, Impl, OFFSET>,
            SetRenderMode: SetRenderMode::<Identity, Impl, OFFSET>,
            GetRenderMode: GetRenderMode::<Identity, Impl, OFFSET>,
            SetNotificationCallback: SetNotificationCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICDevelopRaw as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapSource as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapFrameDecode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICDevelopRawNotificationCallback_Impl: Sized {
    fn Notify(&self, notificationmask: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICDevelopRawNotificationCallback {}
impl IWICDevelopRawNotificationCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRawNotificationCallback_Impl, const OFFSET: isize>() -> IWICDevelopRawNotificationCallback_Vtbl {
        unsafe extern "system" fn Notify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICDevelopRawNotificationCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationmask: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Notify(::core::mem::transmute_copy(&notificationmask)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICDevelopRawNotificationCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWICEnumMetadataItem_Impl: Sized {
    fn Next(&self, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IWICEnumMetadataItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWICEnumMetadataItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IWICEnumMetadataItem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>() -> IWICEnumMetadataItem_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgeltschema), ::core::mem::transmute_copy(&rgeltid), ::core::mem::transmute_copy(&rgeltvalue), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadataitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienummetadataitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICEnumMetadataItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICFastMetadataEncoder_Impl: Sized {
    fn Commit(&self) -> ::windows_core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> ::windows_core::Result<IWICMetadataQueryWriter>;
}
impl ::windows_core::RuntimeName for IWICFastMetadataEncoder {}
impl IWICFastMetadataEncoder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICFastMetadataEncoder_Impl, const OFFSET: isize>() -> IWICFastMetadataEncoder_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICFastMetadataEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICFastMetadataEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadataQueryWriter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadataquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, Impl, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICFastMetadataEncoder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICFormatConverter_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: ::core::option::Option<&IWICBitmapSource>, dstformat: *const ::windows_core::GUID, dither: WICBitmapDitherType, pipalette: ::core::option::Option<&IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows_core::Result<()>;
    fn CanConvert(&self, srcpixelformat: *const ::windows_core::GUID, dstpixelformat: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWICFormatConverter {}
#[cfg(feature = "Win32_Foundation")]
impl IWICFormatConverter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICFormatConverter_Impl, const OFFSET: isize>() -> IWICFormatConverter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, dstformat: *const ::windows_core::GUID, dither: WICBitmapDitherType, pipalette: *mut ::core::ffi::c_void, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pisource), ::core::mem::transmute_copy(&dstformat), ::core::mem::transmute_copy(&dither), ::windows_core::from_raw_borrowed(&pipalette), ::core::mem::transmute_copy(&alphathresholdpercent), ::core::mem::transmute_copy(&palettetranslate)).into()
        }
        unsafe extern "system" fn CanConvert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcpixelformat: *const ::windows_core::GUID, dstpixelformat: *const ::windows_core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanConvert(::core::mem::transmute_copy(&srcpixelformat), ::core::mem::transmute_copy(&dstpixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanconvert, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CanConvert: CanConvert::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICFormatConverter as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICFormatConverterInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetPixelFormats(&self, cformats: u32, ppixelformatguids: *mut ::windows_core::GUID, pcactual: *mut u32) -> ::windows_core::Result<()>;
    fn CreateInstance(&self) -> ::windows_core::Result<IWICFormatConverter>;
}
impl ::windows_core::RuntimeName for IWICFormatConverterInfo {}
impl IWICFormatConverterInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICFormatConverterInfo_Impl, const OFFSET: isize>() -> IWICFormatConverterInfo_Vtbl {
        unsafe extern "system" fn GetPixelFormats<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICFormatConverterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, ppixelformatguids: *mut ::windows_core::GUID, pcactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelFormats(::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&ppixelformatguids), ::core::mem::transmute_copy(&pcactual)).into()
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICFormatConverterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiconverter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiconverter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICComponentInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPixelFormats: GetPixelFormats::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICFormatConverterInfo as ::windows_core::ComInterface>::IID || iid == &<IWICComponentInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactory_Impl: Sized {
    fn CreateDecoderFromFilename(&self, wzfilename: &::windows_core::PCWSTR, pguidvendor: *const ::windows_core::GUID, dwdesiredaccess: super::super::Foundation::GENERIC_ACCESS_RIGHTS, metadataoptions: WICDecodeOptions) -> ::windows_core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromStream(&self, pistream: ::core::option::Option<&super::super::System::Com::IStream>, pguidvendor: *const ::windows_core::GUID, metadataoptions: WICDecodeOptions) -> ::windows_core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows_core::GUID, metadataoptions: WICDecodeOptions) -> ::windows_core::Result<IWICBitmapDecoder>;
    fn CreateComponentInfo(&self, clsidcomponent: *const ::windows_core::GUID) -> ::windows_core::Result<IWICComponentInfo>;
    fn CreateDecoder(&self, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<IWICBitmapDecoder>;
    fn CreateEncoder(&self, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<IWICBitmapEncoder>;
    fn CreatePalette(&self) -> ::windows_core::Result<IWICPalette>;
    fn CreateFormatConverter(&self) -> ::windows_core::Result<IWICFormatConverter>;
    fn CreateBitmapScaler(&self) -> ::windows_core::Result<IWICBitmapScaler>;
    fn CreateBitmapClipper(&self) -> ::windows_core::Result<IWICBitmapClipper>;
    fn CreateBitmapFlipRotator(&self) -> ::windows_core::Result<IWICBitmapFlipRotator>;
    fn CreateStream(&self) -> ::windows_core::Result<IWICStream>;
    fn CreateColorContext(&self) -> ::windows_core::Result<IWICColorContext>;
    fn CreateColorTransformer(&self) -> ::windows_core::Result<IWICColorTransform>;
    fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows_core::GUID, option: WICBitmapCreateCacheOption) -> ::windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromSource(&self, pibitmapsource: ::core::option::Option<&IWICBitmapSource>, option: WICBitmapCreateCacheOption) -> ::windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromSourceRect(&self, pibitmapsource: ::core::option::Option<&IWICBitmapSource>, x: u32, y: u32, width: u32, height: u32) -> ::windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows_core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8) -> ::windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromHBITMAP(&self, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption) -> ::windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromHICON(&self, hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::Result<IWICBitmap>;
    fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn CreateFastMetadataEncoderFromDecoder(&self, pidecoder: ::core::option::Option<&IWICBitmapDecoder>) -> ::windows_core::Result<IWICFastMetadataEncoder>;
    fn CreateFastMetadataEncoderFromFrameDecode(&self, piframedecoder: ::core::option::Option<&IWICBitmapFrameDecode>) -> ::windows_core::Result<IWICFastMetadataEncoder>;
    fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<IWICMetadataQueryWriter>;
    fn CreateQueryWriterFromReader(&self, piqueryreader: ::core::option::Option<&IWICMetadataQueryReader>, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IWICImagingFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICImagingFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>() -> IWICImagingFactory_Vtbl {
        unsafe extern "system" fn CreateDecoderFromFilename<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: ::windows_core::PCWSTR, pguidvendor: *const ::windows_core::GUID, dwdesiredaccess: super::super::Foundation::GENERIC_ACCESS_RIGHTS, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDecoderFromFilename(::core::mem::transmute(&wzfilename), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidecoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoderFromStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pguidvendor: *const ::windows_core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDecoderFromStream(::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidecoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoderFromFileHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfile: usize, pguidvendor: *const ::windows_core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDecoderFromFileHandle(::core::mem::transmute_copy(&hfile), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidecoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComponentInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcomponent: *const ::windows_core::GUID, ppiinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateComponentInfo(::core::mem::transmute_copy(&clsidcomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDecoder(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidecoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncoder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, ppiencoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEncoder(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiencoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipalette: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePalette() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipalette, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFormatConverter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiformatconverter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFormatConverter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiformatconverter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapScaler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapscaler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBitmapScaler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapscaler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapClipper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapclipper: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBitmapClipper() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapclipper, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFlipRotator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapfliprotator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBitmapFlipRotator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapfliprotator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwicstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwicstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiccolorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateColorContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiccolorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorTransformer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiccolortransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateColorTransformer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiccolortransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows_core::GUID, option: WICBitmapCreateCacheOption, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBitmap(::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, option: WICBitmapCreateCacheOption, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBitmapFromSource(::windows_core::from_raw_borrowed(&pibitmapsource), ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromSourceRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBitmapFromSourceRect(::windows_core::from_raw_borrowed(&pibitmapsource), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows_core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBitmapFromMemory(::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromHBITMAP<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBitmapFromHBITMAP(::core::mem::transmute_copy(&hbitmap), ::core::mem::transmute_copy(&hpalette), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromHICON<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBitmapFromHICON(::core::mem::transmute_copy(&hicon)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComponentEnumerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, componenttypes: u32, options: u32, ppienumunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateComponentEnumerator(::core::mem::transmute_copy(&componenttypes), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromDecoder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidecoder: *mut ::core::ffi::c_void, ppifastencoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFastMetadataEncoderFromDecoder(::windows_core::from_raw_borrowed(&pidecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifastencoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromFrameDecode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piframedecoder: *mut ::core::ffi::c_void, ppifastencoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFastMetadataEncoderFromFrameDecode(::windows_core::from_raw_borrowed(&piframedecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifastencoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, ppiquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateQueryWriter(::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piqueryreader: *mut ::core::ffi::c_void, pguidvendor: *const ::windows_core::GUID, ppiquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateQueryWriterFromReader(::windows_core::from_raw_borrowed(&piqueryreader), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDecoderFromFilename: CreateDecoderFromFilename::<Identity, Impl, OFFSET>,
            CreateDecoderFromStream: CreateDecoderFromStream::<Identity, Impl, OFFSET>,
            CreateDecoderFromFileHandle: CreateDecoderFromFileHandle::<Identity, Impl, OFFSET>,
            CreateComponentInfo: CreateComponentInfo::<Identity, Impl, OFFSET>,
            CreateDecoder: CreateDecoder::<Identity, Impl, OFFSET>,
            CreateEncoder: CreateEncoder::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateFormatConverter: CreateFormatConverter::<Identity, Impl, OFFSET>,
            CreateBitmapScaler: CreateBitmapScaler::<Identity, Impl, OFFSET>,
            CreateBitmapClipper: CreateBitmapClipper::<Identity, Impl, OFFSET>,
            CreateBitmapFlipRotator: CreateBitmapFlipRotator::<Identity, Impl, OFFSET>,
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            CreateColorContext: CreateColorContext::<Identity, Impl, OFFSET>,
            CreateColorTransformer: CreateColorTransformer::<Identity, Impl, OFFSET>,
            CreateBitmap: CreateBitmap::<Identity, Impl, OFFSET>,
            CreateBitmapFromSource: CreateBitmapFromSource::<Identity, Impl, OFFSET>,
            CreateBitmapFromSourceRect: CreateBitmapFromSourceRect::<Identity, Impl, OFFSET>,
            CreateBitmapFromMemory: CreateBitmapFromMemory::<Identity, Impl, OFFSET>,
            CreateBitmapFromHBITMAP: CreateBitmapFromHBITMAP::<Identity, Impl, OFFSET>,
            CreateBitmapFromHICON: CreateBitmapFromHICON::<Identity, Impl, OFFSET>,
            CreateComponentEnumerator: CreateComponentEnumerator::<Identity, Impl, OFFSET>,
            CreateFastMetadataEncoderFromDecoder: CreateFastMetadataEncoderFromDecoder::<Identity, Impl, OFFSET>,
            CreateFastMetadataEncoderFromFrameDecode: CreateFastMetadataEncoderFromFrameDecode::<Identity, Impl, OFFSET>,
            CreateQueryWriter: CreateQueryWriter::<Identity, Impl, OFFSET>,
            CreateQueryWriterFromReader: CreateQueryWriterFromReader::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICImagingFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IWICJpegFrameDecode_Impl: Sized {
    fn DoesSupportIndexing(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIndexing(&self, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows_core::Result<()>;
    fn ClearIndexing(&self) -> ::windows_core::Result<()>;
    fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows_core::Result<()>;
    fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows_core::Result<()>;
    fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows_core::Result<()>;
    fn GetFrameHeader(&self, pframeheader: *mut WICJpegFrameHeader) -> ::windows_core::Result<()>;
    fn GetScanHeader(&self, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> ::windows_core::Result<()>;
    fn CopyScan(&self, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows_core::Result<()>;
    fn CopyMinimalStream(&self, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::RuntimeName for IWICJpegFrameDecode {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IWICJpegFrameDecode_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>() -> IWICJpegFrameDecode_Vtbl {
        unsafe extern "system" fn DoesSupportIndexing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfindexingsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoesSupportIndexing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfindexingsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIndexing(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&horizontalintervalsize)).into()
        }
        unsafe extern "system" fn ClearIndexing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearIndexing().into()
        }
        unsafe extern "system" fn GetAcHuffmanTable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pachuffmantable)).into()
        }
        unsafe extern "system" fn GetDcHuffmanTable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pdchuffmantable)).into()
        }
        unsafe extern "system" fn GetQuantizationTable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuantizationTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pquantizationtable)).into()
        }
        unsafe extern "system" fn GetFrameHeader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframeheader: *mut WICJpegFrameHeader) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrameHeader(::core::mem::transmute_copy(&pframeheader)).into()
        }
        unsafe extern "system" fn GetScanHeader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScanHeader(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&pscanheader)).into()
        }
        unsafe extern "system" fn CopyScan<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyScan(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&scanoffset), ::core::mem::transmute_copy(&cbscandata), ::core::mem::transmute_copy(&pbscandata), ::core::mem::transmute_copy(&pcbscandataactual)).into()
        }
        unsafe extern "system" fn CopyMinimalStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyMinimalStream(::core::mem::transmute_copy(&streamoffset), ::core::mem::transmute_copy(&cbstreamdata), ::core::mem::transmute_copy(&pbstreamdata), ::core::mem::transmute_copy(&pcbstreamdataactual)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DoesSupportIndexing: DoesSupportIndexing::<Identity, Impl, OFFSET>,
            SetIndexing: SetIndexing::<Identity, Impl, OFFSET>,
            ClearIndexing: ClearIndexing::<Identity, Impl, OFFSET>,
            GetAcHuffmanTable: GetAcHuffmanTable::<Identity, Impl, OFFSET>,
            GetDcHuffmanTable: GetDcHuffmanTable::<Identity, Impl, OFFSET>,
            GetQuantizationTable: GetQuantizationTable::<Identity, Impl, OFFSET>,
            GetFrameHeader: GetFrameHeader::<Identity, Impl, OFFSET>,
            GetScanHeader: GetScanHeader::<Identity, Impl, OFFSET>,
            CopyScan: CopyScan::<Identity, Impl, OFFSET>,
            CopyMinimalStream: CopyMinimalStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICJpegFrameDecode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICJpegFrameEncode_Impl: Sized {
    fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows_core::Result<()>;
    fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows_core::Result<()>;
    fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows_core::Result<()>;
    fn WriteScan(&self, cbscandata: u32, pbscandata: *const u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::RuntimeName for IWICJpegFrameEncode {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICJpegFrameEncode_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>() -> IWICJpegFrameEncode_Vtbl {
        unsafe extern "system" fn GetAcHuffmanTable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pachuffmantable)).into()
        }
        unsafe extern "system" fn GetDcHuffmanTable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pdchuffmantable)).into()
        }
        unsafe extern "system" fn GetQuantizationTable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuantizationTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pquantizationtable)).into()
        }
        unsafe extern "system" fn WriteScan<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbscandata: u32, pbscandata: *const u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteScan(::core::mem::transmute_copy(&cbscandata), ::core::mem::transmute_copy(&pbscandata)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAcHuffmanTable: GetAcHuffmanTable::<Identity, Impl, OFFSET>,
            GetDcHuffmanTable: GetDcHuffmanTable::<Identity, Impl, OFFSET>,
            GetQuantizationTable: GetQuantizationTable::<Identity, Impl, OFFSET>,
            WriteScan: WriteScan::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICJpegFrameEncode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockReader_Impl: Sized {
    fn GetContainerFormat(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetReaderByIndex(&self, nindex: u32) -> ::windows_core::Result<IWICMetadataReader>;
    fn GetEnumerator(&self) -> ::windows_core::Result<super::super::System::Com::IEnumUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IWICMetadataBlockReader {}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataBlockReader_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>() -> IWICMetadataBlockReader_Vtbl {
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidcontainerformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReaderByIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatareader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReaderByIndex(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadatareader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienummetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetReaderByIndex: GetReaderByIndex::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICMetadataBlockReader as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockWriter_Impl: Sized + IWICMetadataBlockReader_Impl {
    fn InitializeFromBlockReader(&self, pimdblockreader: ::core::option::Option<&IWICMetadataBlockReader>) -> ::windows_core::Result<()>;
    fn GetWriterByIndex(&self, nindex: u32) -> ::windows_core::Result<IWICMetadataWriter>;
    fn AddWriter(&self, pimetadatawriter: ::core::option::Option<&IWICMetadataWriter>) -> ::windows_core::Result<()>;
    fn SetWriterByIndex(&self, nindex: u32, pimetadatawriter: ::core::option::Option<&IWICMetadataWriter>) -> ::windows_core::Result<()>;
    fn RemoveWriterByIndex(&self, nindex: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IWICMetadataBlockWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataBlockWriter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>() -> IWICMetadataBlockWriter_Vtbl {
        unsafe extern "system" fn InitializeFromBlockReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimdblockreader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFromBlockReader(::windows_core::from_raw_borrowed(&pimdblockreader)).into()
        }
        unsafe extern "system" fn GetWriterByIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatawriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWriterByIndex(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadatawriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWriter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimetadatawriter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddWriter(::windows_core::from_raw_borrowed(&pimetadatawriter)).into()
        }
        unsafe extern "system" fn SetWriterByIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pimetadatawriter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWriterByIndex(::core::mem::transmute_copy(&nindex), ::windows_core::from_raw_borrowed(&pimetadatawriter)).into()
        }
        unsafe extern "system" fn RemoveWriterByIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveWriterByIndex(::core::mem::transmute_copy(&nindex)).into()
        }
        Self {
            base__: IWICMetadataBlockReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitializeFromBlockReader: InitializeFromBlockReader::<Identity, Impl, OFFSET>,
            GetWriterByIndex: GetWriterByIndex::<Identity, Impl, OFFSET>,
            AddWriter: AddWriter::<Identity, Impl, OFFSET>,
            SetWriterByIndex: SetWriterByIndex::<Identity, Impl, OFFSET>,
            RemoveWriterByIndex: RemoveWriterByIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICMetadataBlockWriter as ::windows_core::ComInterface>::IID || iid == &<IWICMetadataBlockReader as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICMetadataHandlerInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetMetadataFormat(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetContainerFormats(&self, ccontainerformats: u32, pguidcontainerformats: *mut ::windows_core::GUID, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceManufacturer(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceModels(&self, cchdevicemodels: u32, wzdevicemodels: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn DoesRequireFullStream(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportPadding(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesRequireFixedSize(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWICMetadataHandlerInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IWICMetadataHandlerInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>() -> IWICMetadataHandlerInfo_Vtbl {
        unsafe extern "system" fn GetMetadataFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadataFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidmetadataformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerFormats<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut ::windows_core::GUID, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContainerFormats(::core::mem::transmute_copy(&ccontainerformats), ::core::mem::transmute_copy(&pguidcontainerformats), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceManufacturer(::core::mem::transmute_copy(&cchdevicemanufacturer), ::core::mem::transmute(&wzdevicemanufacturer), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceModels<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceModels(::core::mem::transmute_copy(&cchdevicemodels), ::core::mem::transmute(&wzdevicemodels), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn DoesRequireFullStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequiresfullstream: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoesRequireFullStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrequiresfullstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportPadding<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportspadding: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoesSupportPadding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportspadding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesRequireFixedSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffixedsize: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoesRequireFixedSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffixedsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICComponentInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMetadataFormat: GetMetadataFormat::<Identity, Impl, OFFSET>,
            GetContainerFormats: GetContainerFormats::<Identity, Impl, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, Impl, OFFSET>,
            GetDeviceModels: GetDeviceModels::<Identity, Impl, OFFSET>,
            DoesRequireFullStream: DoesRequireFullStream::<Identity, Impl, OFFSET>,
            DoesSupportPadding: DoesSupportPadding::<Identity, Impl, OFFSET>,
            DoesRequireFixedSize: DoesRequireFixedSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICMetadataHandlerInfo as ::windows_core::ComInterface>::IID || iid == &<IWICComponentInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWICMetadataQueryReader_Impl: Sized {
    fn GetContainerFormat(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetLocation(&self, cchmaxlength: u32, wznamespace: &::windows_core::PWSTR, pcchactuallength: *mut u32) -> ::windows_core::Result<()>;
    fn GetMetadataByName(&self, wzname: &::windows_core::PCWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetEnumerator(&self) -> ::windows_core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWICMetadataQueryReader {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IWICMetadataQueryReader_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>() -> IWICMetadataQueryReader_Vtbl {
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidcontainerformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmaxlength: u32, wznamespace: ::windows_core::PWSTR, pcchactuallength: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocation(::core::mem::transmute_copy(&cchmaxlength), ::core::mem::transmute(&wznamespace), ::core::mem::transmute_copy(&pcchactuallength)).into()
        }
        unsafe extern "system" fn GetMetadataByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: ::windows_core::PCWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMetadataByName(::core::mem::transmute(&wzname), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienumstring: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetLocation: GetLocation::<Identity, Impl, OFFSET>,
            GetMetadataByName: GetMetadataByName::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICMetadataQueryReader as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWICMetadataQueryWriter_Impl: Sized + IWICMetadataQueryReader_Impl {
    fn SetMetadataByName(&self, wzname: &::windows_core::PCWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn RemoveMetadataByName(&self, wzname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWICMetadataQueryWriter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IWICMetadataQueryWriter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataQueryWriter_Impl, const OFFSET: isize>() -> IWICMetadataQueryWriter_Vtbl {
        unsafe extern "system" fn SetMetadataByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataQueryWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: ::windows_core::PCWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMetadataByName(::core::mem::transmute(&wzname), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn RemoveMetadataByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataQueryWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveMetadataByName(::core::mem::transmute(&wzname)).into()
        }
        Self {
            base__: IWICMetadataQueryReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMetadataByName: SetMetadataByName::<Identity, Impl, OFFSET>,
            RemoveMetadataByName: RemoveMetadataByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICMetadataQueryWriter as ::windows_core::ComInterface>::IID || iid == &<IWICMetadataQueryReader as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWICMetadataReader_Impl: Sized {
    fn GetMetadataFormat(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetMetadataHandlerInfo(&self) -> ::windows_core::Result<IWICMetadataHandlerInfo>;
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetValueByIndex(&self, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetEnumerator(&self) -> ::windows_core::Result<IWICEnumMetadataItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWICMetadataReader {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IWICMetadataReader_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: isize>() -> IWICMetadataReader_Vtbl {
        unsafe extern "system" fn GetMetadataFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadataFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidmetadataformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataHandlerInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppihandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadataHandlerInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppihandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueByIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetValueByIndex(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetValue(::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienummetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMetadataFormat: GetMetadataFormat::<Identity, Impl, OFFSET>,
            GetMetadataHandlerInfo: GetMetadataHandlerInfo::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetValueByIndex: GetValueByIndex::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICMetadataReader as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICMetadataReaderInfo_Impl: Sized + IWICMetadataHandlerInfo_Impl {
    fn GetPatterns(&self, guidcontainerformat: *const ::windows_core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows_core::Result<()>;
    fn MatchesPattern(&self, guidcontainerformat: *const ::windows_core::GUID, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CreateInstance(&self) -> ::windows_core::Result<IWICMetadataReader>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IWICMetadataReaderInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICMetadataReaderInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: isize>() -> IWICMetadataReaderInfo_Vtbl {
        unsafe extern "system" fn GetPatterns<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPatterns(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&ppattern), ::core::mem::transmute_copy(&pccount), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn MatchesPattern<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, pistream: *mut ::core::ffi::c_void, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MatchesPattern(::core::mem::transmute_copy(&guidcontainerformat), ::windows_core::from_raw_borrowed(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmatches, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppireader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICMetadataHandlerInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPatterns: GetPatterns::<Identity, Impl, OFFSET>,
            MatchesPattern: MatchesPattern::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICMetadataReaderInfo as ::windows_core::ComInterface>::IID || iid == &<IWICComponentInfo as ::windows_core::ComInterface>::IID || iid == &<IWICMetadataHandlerInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWICMetadataWriter_Impl: Sized + IWICMetadataReader_Impl {
    fn SetValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn SetValueByIndex(&self, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn RemoveValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn RemoveValueByIndex(&self, nindex: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWICMetadataWriter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IWICMetadataWriter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataWriter_Impl, const OFFSET: isize>() -> IWICMetadataWriter_Vtbl {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn SetValueByIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValueByIndex(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn RemoveValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveValue(::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid)).into()
        }
        unsafe extern "system" fn RemoveValueByIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveValueByIndex(::core::mem::transmute_copy(&nindex)).into()
        }
        Self {
            base__: IWICMetadataReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            SetValueByIndex: SetValueByIndex::<Identity, Impl, OFFSET>,
            RemoveValue: RemoveValue::<Identity, Impl, OFFSET>,
            RemoveValueByIndex: RemoveValueByIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICMetadataWriter as ::windows_core::ComInterface>::IID || iid == &<IWICMetadataReader as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICMetadataWriterInfo_Impl: Sized + IWICMetadataHandlerInfo_Impl {
    fn GetHeader(&self, guidcontainerformat: *const ::windows_core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows_core::Result<()>;
    fn CreateInstance(&self) -> ::windows_core::Result<IWICMetadataWriter>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWICMetadataWriterInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IWICMetadataWriterInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataWriterInfo_Impl, const OFFSET: isize>() -> IWICMetadataWriterInfo_Vtbl {
        unsafe extern "system" fn GetHeader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataWriterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHeader(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pheader), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICMetadataWriterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICMetadataHandlerInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetHeader: GetHeader::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICMetadataWriterInfo as ::windows_core::ComInterface>::IID || iid == &<IWICComponentInfo as ::windows_core::ComInterface>::IID || iid == &<IWICMetadataHandlerInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPalette_Impl: Sized {
    fn InitializePredefined(&self, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn InitializeCustom(&self, pcolors: *const u32, ccount: u32) -> ::windows_core::Result<()>;
    fn InitializeFromBitmap(&self, pisurface: ::core::option::Option<&IWICBitmapSource>, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn InitializeFromPalette(&self, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn GetType(&self) -> ::windows_core::Result<WICBitmapPaletteType>;
    fn GetColorCount(&self) -> ::windows_core::Result<u32>;
    fn GetColors(&self, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows_core::Result<()>;
    fn IsBlackWhite(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsGrayscale(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn HasAlpha(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWICPalette {}
#[cfg(feature = "Win32_Foundation")]
impl IWICPalette_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: isize>() -> IWICPalette_Vtbl {
        unsafe extern "system" fn InitializePredefined<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializePredefined(::core::mem::transmute_copy(&epalettetype), ::core::mem::transmute_copy(&faddtransparentcolor)).into()
        }
        unsafe extern "system" fn InitializeCustom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolors: *const u32, ccount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeCustom(::core::mem::transmute_copy(&pcolors), ::core::mem::transmute_copy(&ccount)).into()
        }
        unsafe extern "system" fn InitializeFromBitmap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisurface: *mut ::core::ffi::c_void, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFromBitmap(::windows_core::from_raw_borrowed(&pisurface), ::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&faddtransparentcolor)).into()
        }
        unsafe extern "system" fn InitializeFromPalette<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFromPalette(::windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pepalettetype: *mut WICBitmapPaletteType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pepalettetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColorCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColors<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColors(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&pcolors), ::core::mem::transmute_copy(&pcactualcolors)).into()
        }
        unsafe extern "system" fn IsBlackWhite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisblackwhite: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsBlackWhite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisblackwhite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGrayscale<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisgrayscale: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsGrayscale() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisgrayscale, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasAlpha<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfhasalpha: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasAlpha() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasalpha, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializePredefined: InitializePredefined::<Identity, Impl, OFFSET>,
            InitializeCustom: InitializeCustom::<Identity, Impl, OFFSET>,
            InitializeFromBitmap: InitializeFromBitmap::<Identity, Impl, OFFSET>,
            InitializeFromPalette: InitializeFromPalette::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetColorCount: GetColorCount::<Identity, Impl, OFFSET>,
            GetColors: GetColors::<Identity, Impl, OFFSET>,
            IsBlackWhite: IsBlackWhite::<Identity, Impl, OFFSET>,
            IsGrayscale: IsGrayscale::<Identity, Impl, OFFSET>,
            HasAlpha: HasAlpha::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICPalette as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICPersistStream_Impl: Sized + super::super::System::Com::IPersistStream_Impl {
    fn LoadEx(&self, pistream: ::core::option::Option<&super::super::System::Com::IStream>, pguidpreferredvendor: *const ::windows_core::GUID, dwpersistoptions: u32) -> ::windows_core::Result<()>;
    fn SaveEx(&self, pistream: ::core::option::Option<&super::super::System::Com::IStream>, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IWICPersistStream {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICPersistStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPersistStream_Impl, const OFFSET: isize>() -> IWICPersistStream_Vtbl {
        unsafe extern "system" fn LoadEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPersistStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pguidpreferredvendor: *const ::windows_core::GUID, dwpersistoptions: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadEx(::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&pguidpreferredvendor), ::core::mem::transmute_copy(&dwpersistoptions)).into()
        }
        unsafe extern "system" fn SaveEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPersistStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveEx(::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&dwpersistoptions), ::core::mem::transmute_copy(&fcleardirty)).into()
        }
        Self {
            base__: super::super::System::Com::IPersistStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            LoadEx: LoadEx::<Identity, Impl, OFFSET>,
            SaveEx: SaveEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICPersistStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IPersist as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IPersistStream as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICPixelFormatInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetFormatGUID(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetColorContext(&self) -> ::windows_core::Result<IWICColorContext>;
    fn GetBitsPerPixel(&self) -> ::windows_core::Result<u32>;
    fn GetChannelCount(&self) -> ::windows_core::Result<u32>;
    fn GetChannelMask(&self, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICPixelFormatInfo {}
impl IWICPixelFormatInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>() -> IWICPixelFormatInfo_Vtbl {
        unsafe extern "system" fn GetFormatGUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormatGUID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicolorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColorContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppicolorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitsPerPixel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puibitsperpixel: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBitsPerPixel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puibitsperpixel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puichannelcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puichannelcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelMask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChannelMask(::core::mem::transmute_copy(&uichannelindex), ::core::mem::transmute_copy(&cbmaskbuffer), ::core::mem::transmute_copy(&pbmaskbuffer), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        Self {
            base__: IWICComponentInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFormatGUID: GetFormatGUID::<Identity, Impl, OFFSET>,
            GetColorContext: GetColorContext::<Identity, Impl, OFFSET>,
            GetBitsPerPixel: GetBitsPerPixel::<Identity, Impl, OFFSET>,
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            GetChannelMask: GetChannelMask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo as ::windows_core::ComInterface>::IID || iid == &<IWICComponentInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPixelFormatInfo2_Impl: Sized + IWICPixelFormatInfo_Impl {
    fn SupportsTransparency(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetNumericRepresentation(&self) -> ::windows_core::Result<WICPixelFormatNumericRepresentation>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWICPixelFormatInfo2 {}
#[cfg(feature = "Win32_Foundation")]
impl IWICPixelFormatInfo2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPixelFormatInfo2_Impl, const OFFSET: isize>() -> IWICPixelFormatInfo2_Vtbl {
        unsafe extern "system" fn SupportsTransparency<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPixelFormatInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportstransparency: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportsTransparency() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportstransparency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericRepresentation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPixelFormatInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumericrepresentation: *mut WICPixelFormatNumericRepresentation) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumericRepresentation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumericrepresentation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICPixelFormatInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            SupportsTransparency: SupportsTransparency::<Identity, Impl, OFFSET>,
            GetNumericRepresentation: GetNumericRepresentation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo2 as ::windows_core::ComInterface>::IID || iid == &<IWICComponentInfo as ::windows_core::ComInterface>::IID || iid == &<IWICPixelFormatInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICPlanarBitmapFrameEncode_Impl: Sized {
    fn WritePixels(&self, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows_core::Result<()>;
    fn WriteSource(&self, ppplanes: *const ::core::option::Option<IWICBitmapSource>, cplanes: u32, prcsource: *const WICRect) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICPlanarBitmapFrameEncode {}
impl IWICPlanarBitmapFrameEncode_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: isize>() -> IWICPlanarBitmapFrameEncode_Vtbl {
        unsafe extern "system" fn WritePixels<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WritePixels(::core::mem::transmute_copy(&linecount), ::core::mem::transmute_copy(&pplanes), ::core::mem::transmute_copy(&cplanes)).into()
        }
        unsafe extern "system" fn WriteSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplanes: *const *mut ::core::ffi::c_void, cplanes: u32, prcsource: *const WICRect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteSource(::core::mem::transmute_copy(&ppplanes), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&prcsource)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WritePixels: WritePixels::<Identity, Impl, OFFSET>,
            WriteSource: WriteSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICPlanarBitmapFrameEncode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPlanarBitmapSourceTransform_Impl: Sized {
    fn DoesSupportTransform(&self, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows_core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CopyPixels(&self, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWICPlanarBitmapSourceTransform {}
#[cfg(feature = "Win32_Foundation")]
impl IWICPlanarBitmapSourceTransform_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: isize>() -> IWICPlanarBitmapSourceTransform_Vtbl {
        unsafe extern "system" fn DoesSupportTransform<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows_core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoesSupportTransform(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&dstplanaroptions), ::core::mem::transmute_copy(&pguiddstformats), ::core::mem::transmute_copy(&pplanedescriptions), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&pfissupported)).into()
        }
        unsafe extern "system" fn CopyPixels<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyPixels(::core::mem::transmute_copy(&prcsource), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&dstplanaroptions), ::core::mem::transmute_copy(&pdstplanes), ::core::mem::transmute_copy(&cplanes)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DoesSupportTransform: DoesSupportTransform::<Identity, Impl, OFFSET>,
            CopyPixels: CopyPixels::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICPlanarBitmapSourceTransform as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPlanarFormatConverter_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, ppplanes: *const ::core::option::Option<IWICBitmapSource>, cplanes: u32, dstformat: *const ::windows_core::GUID, dither: WICBitmapDitherType, pipalette: ::core::option::Option<&IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows_core::Result<()>;
    fn CanConvert(&self, psrcpixelformats: *const ::windows_core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWICPlanarFormatConverter {}
#[cfg(feature = "Win32_Foundation")]
impl IWICPlanarFormatConverter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPlanarFormatConverter_Impl, const OFFSET: isize>() -> IWICPlanarFormatConverter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPlanarFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplanes: *const *mut ::core::ffi::c_void, cplanes: u32, dstformat: *const ::windows_core::GUID, dither: WICBitmapDitherType, pipalette: *mut ::core::ffi::c_void, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&ppplanes), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&dstformat), ::core::mem::transmute_copy(&dither), ::windows_core::from_raw_borrowed(&pipalette), ::core::mem::transmute_copy(&alphathresholdpercent), ::core::mem::transmute_copy(&palettetranslate)).into()
        }
        unsafe extern "system" fn CanConvert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICPlanarFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcpixelformats: *const ::windows_core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows_core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanConvert(::core::mem::transmute_copy(&psrcpixelformats), ::core::mem::transmute_copy(&csrcplanes), ::core::mem::transmute_copy(&dstpixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanconvert, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CanConvert: CanConvert::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICPlanarFormatConverter as ::windows_core::ComInterface>::IID || iid == &<IWICBitmapSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICProgressCallback_Impl: Sized {
    fn Notify(&self, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICProgressCallback {}
impl IWICProgressCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICProgressCallback_Impl, const OFFSET: isize>() -> IWICProgressCallback_Vtbl {
        unsafe extern "system" fn Notify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICProgressCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Notify(::core::mem::transmute_copy(&uframenum), ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&dblprogress)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICProgressCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
pub trait IWICProgressiveLevelControl_Impl: Sized {
    fn GetLevelCount(&self) -> ::windows_core::Result<u32>;
    fn GetCurrentLevel(&self) -> ::windows_core::Result<u32>;
    fn SetCurrentLevel(&self, nlevel: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWICProgressiveLevelControl {}
impl IWICProgressiveLevelControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: isize>() -> IWICProgressiveLevelControl_Vtbl {
        unsafe extern "system" fn GetLevelCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclevels: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLevelCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclevels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlevel: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlevel: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCurrentLevel(::core::mem::transmute_copy(&nlevel)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLevelCount: GetLevelCount::<Identity, Impl, OFFSET>,
            GetCurrentLevel: GetCurrentLevel::<Identity, Impl, OFFSET>,
            SetCurrentLevel: SetCurrentLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICProgressiveLevelControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICStream_Impl: Sized + super::super::System::Com::IStream_Impl {
    fn InitializeFromIStream(&self, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn InitializeFromFilename(&self, wzfilename: &::windows_core::PCWSTR, dwdesiredaccess: u32) -> ::windows_core::Result<()>;
    fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows_core::Result<()>;
    fn InitializeFromIStreamRegion(&self, pistream: ::core::option::Option<&super::super::System::Com::IStream>, uloffset: u64, ulmaxsize: u64) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IWICStream {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICStream_Impl, const OFFSET: isize>() -> IWICStream_Vtbl {
        unsafe extern "system" fn InitializeFromIStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFromIStream(::windows_core::from_raw_borrowed(&pistream)).into()
        }
        unsafe extern "system" fn InitializeFromFilename<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: ::windows_core::PCWSTR, dwdesiredaccess: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFromFilename(::core::mem::transmute(&wzfilename), ::core::mem::transmute_copy(&dwdesiredaccess)).into()
        }
        unsafe extern "system" fn InitializeFromMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFromMemory(::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&cbbuffersize)).into()
        }
        unsafe extern "system" fn InitializeFromIStreamRegion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, uloffset: u64, ulmaxsize: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFromIStreamRegion(::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&ulmaxsize)).into()
        }
        Self {
            base__: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitializeFromIStream: InitializeFromIStream::<Identity, Impl, OFFSET>,
            InitializeFromFilename: InitializeFromFilename::<Identity, Impl, OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Identity, Impl, OFFSET>,
            InitializeFromIStreamRegion: InitializeFromIStreamRegion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IStream as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWICStreamProvider_Impl: Sized {
    fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn GetPersistOptions(&self) -> ::windows_core::Result<u32>;
    fn GetPreferredVendorGUID(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn RefreshStream(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IWICStreamProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IWICStreamProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICStreamProvider_Impl, const OFFSET: isize>() -> IWICStreamProvider_Vtbl {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppistream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppistream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPersistOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpersistoptions: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPersistOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpersistoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredVendorGUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidpreferredvendor: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreferredVendorGUID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidpreferredvendor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshStream().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            GetPersistOptions: GetPersistOptions::<Identity, Impl, OFFSET>,
            GetPreferredVendorGUID: GetPreferredVendorGUID::<Identity, Impl, OFFSET>,
            RefreshStream: RefreshStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWICStreamProvider as ::windows_core::ComInterface>::IID
    }
}
