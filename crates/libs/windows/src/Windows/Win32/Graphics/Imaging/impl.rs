pub trait IWICBitmap_Impl: Sized + IWICBitmapSource_Impl {
    fn Lock(&self, prclock: *const WICRect, flags: u32) -> ::windows::core::Result<IWICBitmapLock>;
    fn SetPalette(&self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn SetResolution(&self, dpix: f64, dpiy: f64) -> ::windows::core::Result<()>;
}
impl IWICBitmap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmap_Impl, const OFFSET: isize>() -> IWICBitmap_Vtbl {
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prclock: *const WICRect, flags: u32, ppilock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Lock(::core::mem::transmute_copy(&prclock), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppilock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn SetResolution<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetResolution(::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        Self {
            base: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Lock: Lock::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            SetResolution: SetResolution::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmap as ::windows::core::Interface>::IID || iid == &<IWICBitmapSource as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapClipper_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: &::core::option::Option<IWICBitmapSource>, prc: *const WICRect) -> ::windows::core::Result<()>;
}
impl IWICBitmapClipper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapClipper_Impl, const OFFSET: isize>() -> IWICBitmapClipper_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, prc: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pisource), ::core::mem::transmute_copy(&prc)).into()
        }
        Self { base: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapClipper as ::windows::core::Interface>::IID || iid == &<IWICBitmapSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapCodecInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetPixelFormats(&self, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetColorManagementVersion(&self, cchcolormanagementversion: u32, wzcolormanagementversion: &::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceManufacturer(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: &::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceModels(&self, cchdevicemodels: u32, wzdevicemodels: &::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetMimeTypes(&self, cchmimetypes: u32, wzmimetypes: &::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetFileExtensions(&self, cchfileextensions: u32, wzfileextensions: &::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn DoesSupportAnimation(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportChromakey(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportLossless(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportMultiframe(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn MatchesMimeType(&self, wzmimetype: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapCodecInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>() -> IWICBitmapCodecInfo_Vtbl {
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidcontainerformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormats<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPixelFormats(::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&pguidpixelformats), ::core::mem::transmute_copy(&pcactual)).into()
        }
        unsafe extern "system" fn GetColorManagementVersion<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorManagementVersion(::core::mem::transmute_copy(&cchcolormanagementversion), ::core::mem::transmute(&wzcolormanagementversion), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceManufacturer(::core::mem::transmute_copy(&cchdevicemanufacturer), ::core::mem::transmute(&wzdevicemanufacturer), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceModels<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceModels(::core::mem::transmute_copy(&cchdevicemodels), ::core::mem::transmute(&wzdevicemodels), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetMimeTypes<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmimetypes: u32, wzmimetypes: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMimeTypes(::core::mem::transmute_copy(&cchmimetypes), ::core::mem::transmute(&wzmimetypes), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetFileExtensions<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchfileextensions: u32, wzfileextensions: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFileExtensions(::core::mem::transmute_copy(&cchfileextensions), ::core::mem::transmute(&wzfileextensions), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn DoesSupportAnimation<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportanimation: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoesSupportAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportanimation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportChromakey<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportchromakey: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoesSupportChromakey() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportchromakey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportLossless<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportlossless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoesSupportLossless() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportlossless = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportMultiframe<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportmultiframe: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoesSupportMultiframe() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportmultiframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesMimeType<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzmimetype: ::windows::core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MatchesMimeType(::core::mem::transmute(&wzmimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmatches = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICComponentInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapCodecInfo as ::windows::core::Interface>::IID || iid == &<IWICComponentInfo as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapCodecProgressNotification_Impl: Sized {
    fn RegisterProgressNotification(&self, pfnprogressnotification: &PFNProgressNotification, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows::core::Result<()>;
}
impl IWICBitmapCodecProgressNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecProgressNotification_Impl, const OFFSET: isize>() -> IWICBitmapCodecProgressNotification_Vtbl {
        unsafe extern "system" fn RegisterProgressNotification<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecProgressNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfnprogressnotification: ::windows::core::RawPtr, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterProgressNotification(::core::mem::transmute(&pfnprogressnotification), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&dwprogressflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterProgressNotification: RegisterProgressNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapCodecProgressNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICBitmapDecoder_Impl: Sized {
    fn QueryCapability(&self, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<u32>;
    fn Initialize(&self, pistream: &::core::option::Option<super::super::System::Com::IStream>, cacheoptions: WICDecodeOptions) -> ::windows::core::Result<()>;
    fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDecoderInfo(&self) -> ::windows::core::Result<IWICBitmapDecoderInfo>;
    fn CopyPalette(&self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn GetMetadataQueryReader(&self) -> ::windows::core::Result<IWICMetadataQueryReader>;
    fn GetPreview(&self) -> ::windows::core::Result<IWICBitmapSource>;
    fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: *mut ::core::option::Option<IWICColorContext>, pcactualcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetThumbnail(&self) -> ::windows::core::Result<IWICBitmapSource>;
    fn GetFrameCount(&self) -> ::windows::core::Result<u32>;
    fn GetFrame(&self, index: u32) -> ::windows::core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWICBitmapDecoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>() -> IWICBitmapDecoder_Vtbl {
        unsafe extern "system" fn QueryCapability<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pdwcapability: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryCapability(::core::mem::transmute(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, cacheoptions: WICDecodeOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&cacheoptions)).into()
        }
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidcontainerformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDecoderInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidecoderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDecoderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppidecoderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyPalette<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn GetMetadataQueryReader<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMetadataQueryReader() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadataqueryreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreview<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreview() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapsource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContexts<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut ::windows::core::RawPtr, pcactualcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontexts), ::core::mem::transmute_copy(&pcactualcount)).into()
        }
        unsafe extern "system" fn GetThumbnail<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetThumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *ppithumbnail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameCount<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrame<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppibitmapframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFrame(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapDecoder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICBitmapDecoderInfo_Impl: Sized + IWICComponentInfo_Impl + IWICBitmapCodecInfo_Impl {
    fn GetPatterns(&self, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows::core::Result<()>;
    fn MatchesPattern(&self, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CreateInstance(&self) -> ::windows::core::Result<IWICBitmapDecoder>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICBitmapDecoderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>() -> IWICBitmapDecoderInfo_Vtbl {
        unsafe extern "system" fn GetPatterns<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPatterns(::core::mem::transmute_copy(&cbsizepatterns), ::core::mem::transmute_copy(&ppatterns), ::core::mem::transmute_copy(&pcpatterns), ::core::mem::transmute_copy(&pcbpatternsactual)).into()
        }
        unsafe extern "system" fn MatchesPattern<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MatchesPattern(::core::mem::transmute(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmatches = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapdecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapdecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICBitmapCodecInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPatterns: GetPatterns::<Identity, Impl, OFFSET>,
            MatchesPattern: MatchesPattern::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapDecoderInfo as ::windows::core::Interface>::IID || iid == &<IWICComponentInfo as ::windows::core::Interface>::IID || iid == &<IWICBitmapCodecInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICBitmapEncoder_Impl: Sized {
    fn Initialize(&self, pistream: &::core::option::Option<super::super::System::Com::IStream>, cacheoption: WICBitmapEncoderCacheOption) -> ::windows::core::Result<()>;
    fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetEncoderInfo(&self) -> ::windows::core::Result<IWICBitmapEncoderInfo>;
    fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const ::core::option::Option<IWICColorContext>) -> ::windows::core::Result<()>;
    fn SetPalette(&self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn SetThumbnail(&self, pithumbnail: &::core::option::Option<IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn SetPreview(&self, pipreview: &::core::option::Option<IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn CreateNewFrame(&self, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, ppiencoderoptions: *mut ::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyBag2>) -> ::windows::core::Result<()>;
    fn Commit(&self) -> ::windows::core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> ::windows::core::Result<IWICMetadataQueryWriter>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICBitmapEncoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>() -> IWICBitmapEncoder_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, cacheoption: WICBitmapEncoderCacheOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&cacheoption)).into()
        }
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidcontainerformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEncoderInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiencoderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEncoderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiencoderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorContexts<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontext)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn SetThumbnail<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pithumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetThumbnail(::core::mem::transmute(&pithumbnail)).into()
        }
        unsafe extern "system" fn SetPreview<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipreview: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPreview(::core::mem::transmute(&pipreview)).into()
        }
        unsafe extern "system" fn CreateNewFrame<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut ::windows::core::RawPtr, ppiencoderoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateNewFrame(::core::mem::transmute_copy(&ppiframeencode), ::core::mem::transmute_copy(&ppiencoderoptions)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMetadataQueryWriter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadataquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapEncoderInfo_Impl: Sized + IWICComponentInfo_Impl + IWICBitmapCodecInfo_Impl {
    fn CreateInstance(&self) -> ::windows::core::Result<IWICBitmapEncoder>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapEncoderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoderInfo_Impl, const OFFSET: isize>() -> IWICBitmapEncoderInfo_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapencoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWICBitmapCodecInfo_Vtbl::new::<Identity, Impl, OFFSET>(), CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapEncoderInfo as ::windows::core::Interface>::IID || iid == &<IWICComponentInfo as ::windows::core::Interface>::IID || iid == &<IWICBitmapCodecInfo as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapFlipRotator_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: &::core::option::Option<IWICBitmapSource>, options: WICBitmapTransformOptions) -> ::windows::core::Result<()>;
}
impl IWICBitmapFlipRotator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFlipRotator_Impl, const OFFSET: isize>() -> IWICBitmapFlipRotator_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFlipRotator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, options: WICBitmapTransformOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pisource), ::core::mem::transmute_copy(&options)).into()
        }
        Self { base: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapFlipRotator as ::windows::core::Interface>::IID || iid == &<IWICBitmapSource as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapFrameDecode_Impl: Sized + IWICBitmapSource_Impl {
    fn GetMetadataQueryReader(&self) -> ::windows::core::Result<IWICMetadataQueryReader>;
    fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: *mut ::core::option::Option<IWICColorContext>, pcactualcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetThumbnail(&self) -> ::windows::core::Result<IWICBitmapSource>;
}
impl IWICBitmapFrameDecode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: isize>() -> IWICBitmapFrameDecode_Vtbl {
        unsafe extern "system" fn GetMetadataQueryReader<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMetadataQueryReader() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadataqueryreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContexts<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut ::windows::core::RawPtr, pcactualcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontexts), ::core::mem::transmute_copy(&pcactualcount)).into()
        }
        unsafe extern "system" fn GetThumbnail<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetThumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *ppithumbnail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMetadataQueryReader: GetMetadataQueryReader::<Identity, Impl, OFFSET>,
            GetColorContexts: GetColorContexts::<Identity, Impl, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapFrameDecode as ::windows::core::Interface>::IID || iid == &<IWICBitmapSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICBitmapFrameEncode_Impl: Sized {
    fn Initialize(&self, piencoderoptions: &::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyBag2>) -> ::windows::core::Result<()>;
    fn SetSize(&self, uiwidth: u32, uiheight: u32) -> ::windows::core::Result<()>;
    fn SetResolution(&self, dpix: f64, dpiy: f64) -> ::windows::core::Result<()>;
    fn SetPixelFormat(&self, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const ::core::option::Option<IWICColorContext>) -> ::windows::core::Result<()>;
    fn SetPalette(&self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn SetThumbnail(&self, pithumbnail: &::core::option::Option<IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn WritePixels(&self, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows::core::Result<()>;
    fn WriteSource(&self, pibitmapsource: &::core::option::Option<IWICBitmapSource>, prc: *const WICRect) -> ::windows::core::Result<()>;
    fn Commit(&self) -> ::windows::core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> ::windows::core::Result<IWICMetadataQueryWriter>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICBitmapFrameEncode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>() -> IWICBitmapFrameEncode_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piencoderoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&piencoderoptions)).into()
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight)).into()
        }
        unsafe extern "system" fn SetResolution<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetResolution(::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn SetPixelFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPixelFormat(::core::mem::transmute_copy(&ppixelformat)).into()
        }
        unsafe extern "system" fn SetColorContexts<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontext)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn SetThumbnail<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pithumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetThumbnail(::core::mem::transmute(&pithumbnail)).into()
        }
        unsafe extern "system" fn WritePixels<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WritePixels(::core::mem::transmute_copy(&linecount), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbpixels)).into()
        }
        unsafe extern "system" fn WriteSource<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, prc: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteSource(::core::mem::transmute(&pibitmapsource), ::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMetadataQueryWriter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadataquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapFrameEncode as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapLock_Impl: Sized {
    fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()>;
    fn GetStride(&self) -> ::windows::core::Result<u32>;
    fn GetDataPointer(&self, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl IWICBitmapLock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapLock_Impl, const OFFSET: isize>() -> IWICBitmapLock_Vtbl {
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSize(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetStride<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbstride: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStride() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbstride = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPointer<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDataPointer(::core::mem::transmute_copy(&pcbbuffersize), ::core::mem::transmute_copy(&ppbdata)).into()
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *ppixelformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetStride: GetStride::<Identity, Impl, OFFSET>,
            GetDataPointer: GetDataPointer::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapLock as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapScaler_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: &::core::option::Option<IWICBitmapSource>, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows::core::Result<()>;
}
impl IWICBitmapScaler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapScaler_Impl, const OFFSET: isize>() -> IWICBitmapScaler_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapScaler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pisource), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&mode)).into()
        }
        Self { base: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapScaler as ::windows::core::Interface>::IID || iid == &<IWICBitmapSource as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapSource_Impl: Sized {
    fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()>;
    fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()>;
    fn CopyPalette(&self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()>;
}
impl IWICBitmapSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSource_Impl, const OFFSET: isize>() -> IWICBitmapSource_Vtbl {
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSize(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *ppixelformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolution<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResolution(::core::mem::transmute_copy(&pdpix), ::core::mem::transmute_copy(&pdpiy)).into()
        }
        unsafe extern "system" fn CopyPalette<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn CopyPixels<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyPixels(::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetResolution: GetResolution::<Identity, Impl, OFFSET>,
            CopyPalette: CopyPalette::<Identity, Impl, OFFSET>,
            CopyPixels: CopyPixels::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapSourceTransform_Impl: Sized {
    fn CopyPixels(&self, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows::core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()>;
    fn GetClosestSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()>;
    fn GetClosestPixelFormat(&self, pguiddstformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DoesSupportTransform(&self, dsttransform: WICBitmapTransformOptions) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapSourceTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>() -> IWICBitmapSourceTransform_Vtbl {
        unsafe extern "system" fn CopyPixels<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows::core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyPixels(::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pguiddstformat), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&nstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        unsafe extern "system" fn GetClosestSize<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClosestSize(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetClosestPixelFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguiddstformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClosestPixelFormat(::core::mem::transmute_copy(&pguiddstformat)).into()
        }
        unsafe extern "system" fn DoesSupportTransform<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dsttransform: WICBitmapTransformOptions, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoesSupportTransform(::core::mem::transmute_copy(&dsttransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfissupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CopyPixels: CopyPixels::<Identity, Impl, OFFSET>,
            GetClosestSize: GetClosestSize::<Identity, Impl, OFFSET>,
            GetClosestPixelFormat: GetClosestPixelFormat::<Identity, Impl, OFFSET>,
            DoesSupportTransform: DoesSupportTransform::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapSourceTransform as ::windows::core::Interface>::IID
    }
}
pub trait IWICColorContext_Impl: Sized {
    fn InitializeFromFilename(&self, wzfilename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::Result<()>;
    fn InitializeFromExifColorSpace(&self, value: u32) -> ::windows::core::Result<()>;
    fn GetType(&self) -> ::windows::core::Result<WICColorContextType>;
    fn GetProfileBytes(&self, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetExifColorSpace(&self) -> ::windows::core::Result<u32>;
}
impl IWICColorContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorContext_Impl, const OFFSET: isize>() -> IWICColorContext_Vtbl {
        unsafe extern "system" fn InitializeFromFilename<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeFromFilename(::core::mem::transmute(&wzfilename)).into()
        }
        unsafe extern "system" fn InitializeFromMemory<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeFromMemory(::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&cbbuffersize)).into()
        }
        unsafe extern "system" fn InitializeFromExifColorSpace<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeFromExifColorSpace(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WICColorContextType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileBytes<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProfileBytes(::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn GetExifColorSpace<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetExifColorSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InitializeFromFilename: InitializeFromFilename::<Identity, Impl, OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Identity, Impl, OFFSET>,
            InitializeFromExifColorSpace: InitializeFromExifColorSpace::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetProfileBytes: GetProfileBytes::<Identity, Impl, OFFSET>,
            GetExifColorSpace: GetExifColorSpace::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICColorContext as ::windows::core::Interface>::IID
    }
}
pub trait IWICColorTransform_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pibitmapsource: &::core::option::Option<IWICBitmapSource>, picontextsource: &::core::option::Option<IWICColorContext>, picontextdest: &::core::option::Option<IWICColorContext>, pixelfmtdest: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IWICColorTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorTransform_Impl, const OFFSET: isize>() -> IWICColorTransform_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, picontextsource: ::windows::core::RawPtr, picontextdest: ::windows::core::RawPtr, pixelfmtdest: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pibitmapsource), ::core::mem::transmute(&picontextsource), ::core::mem::transmute(&picontextdest), ::core::mem::transmute_copy(&pixelfmtdest)).into()
        }
        Self { base: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICColorTransform as ::windows::core::Interface>::IID || iid == &<IWICBitmapSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICComponentFactory_Impl: Sized + IWICImagingFactory_Impl {
    fn CreateMetadataReader(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<IWICMetadataReader>;
    fn CreateMetadataReaderFromContainer(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<IWICMetadataReader>;
    fn CreateMetadataWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwmetadataoptions: u32) -> ::windows::core::Result<IWICMetadataWriter>;
    fn CreateMetadataWriterFromReader(&self, pireader: &::core::option::Option<IWICMetadataReader>, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataWriter>;
    fn CreateQueryReaderFromBlockReader(&self, piblockreader: &::core::option::Option<IWICMetadataBlockReader>) -> ::windows::core::Result<IWICMetadataQueryReader>;
    fn CreateQueryWriterFromBlockWriter(&self, piblockwriter: &::core::option::Option<IWICMetadataBlockWriter>) -> ::windows::core::Result<IWICMetadataQueryWriter>;
    fn CreateEncoderPropertyBag(&self, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICComponentFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentFactory_Impl, const OFFSET: isize>() -> IWICComponentFactory_Vtbl {
        unsafe extern "system" fn CreateMetadataReader<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: ::windows::core::RawPtr, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMetadataReader(::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppireader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataReaderFromContainer<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: ::windows::core::RawPtr, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMetadataReaderFromContainer(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppireader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataWriter<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwmetadataoptions: u32, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMetadataWriter(::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwmetadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataWriterFromReader<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pireader: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMetadataWriterFromReader(::core::mem::transmute(&pireader), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryReaderFromBlockReader<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piblockreader: ::windows::core::RawPtr, ppiqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateQueryReaderFromBlockReader(::core::mem::transmute(&piblockreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiqueryreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromBlockWriter<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piblockwriter: ::windows::core::RawPtr, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateQueryWriterFromBlockWriter(::core::mem::transmute(&piblockwriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncoderPropertyBag<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32, ppipropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEncoderPropertyBag(::core::mem::transmute_copy(&ppropoptions), ::core::mem::transmute_copy(&ccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppipropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICImagingFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateMetadataReader: CreateMetadataReader::<Identity, Impl, OFFSET>,
            CreateMetadataReaderFromContainer: CreateMetadataReaderFromContainer::<Identity, Impl, OFFSET>,
            CreateMetadataWriter: CreateMetadataWriter::<Identity, Impl, OFFSET>,
            CreateMetadataWriterFromReader: CreateMetadataWriterFromReader::<Identity, Impl, OFFSET>,
            CreateQueryReaderFromBlockReader: CreateQueryReaderFromBlockReader::<Identity, Impl, OFFSET>,
            CreateQueryWriterFromBlockWriter: CreateQueryWriterFromBlockWriter::<Identity, Impl, OFFSET>,
            CreateEncoderPropertyBag: CreateEncoderPropertyBag::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICComponentFactory as ::windows::core::Interface>::IID || iid == &<IWICImagingFactory as ::windows::core::Interface>::IID
    }
}
pub trait IWICComponentInfo_Impl: Sized {
    fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType>;
    fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetSigningStatus(&self) -> ::windows::core::Result<u32>;
    fn GetAuthor(&self, cchauthor: u32, wzauthor: &::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetVersion(&self, cchversion: u32, wzversion: &::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetSpecVersion(&self, cchspecversion: u32, wzspecversion: &::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetFriendlyName(&self, cchfriendlyname: u32, wzfriendlyname: &::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
}
impl IWICComponentInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfo_Impl, const OFFSET: isize>() -> IWICComponentInfo_Vtbl {
        unsafe extern "system" fn GetComponentType<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetComponentType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCLSID<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningStatus<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSigningStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthor<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAuthor(::core::mem::transmute_copy(&cchauthor), ::core::mem::transmute(&wzauthor), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetVendorGUID<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVendorGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidvendor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVersion(::core::mem::transmute_copy(&cchversion), ::core::mem::transmute(&wzversion), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetSpecVersion<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSpecVersion(::core::mem::transmute_copy(&cchspecversion), ::core::mem::transmute(&wzspecversion), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetFriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFriendlyName(::core::mem::transmute_copy(&cchfriendlyname), ::core::mem::transmute(&wzfriendlyname), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICComponentInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsDecoder_Impl: Sized {
    fn GetParameters(&self) -> ::windows::core::Result<WICDdsParameters>;
    fn GetFrame(&self, arrayindex: u32, miplevel: u32, sliceindex: u32) -> ::windows::core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsDecoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsDecoder_Impl, const OFFSET: isize>() -> IWICDdsDecoder_Vtbl {
        unsafe extern "system" fn GetParameters<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrame<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arrayindex: u32, miplevel: u32, sliceindex: u32, ppibitmapframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFrame(::core::mem::transmute_copy(&arrayindex), ::core::mem::transmute_copy(&miplevel), ::core::mem::transmute_copy(&sliceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            GetFrame: GetFrame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDdsDecoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsEncoder_Impl: Sized {
    fn SetParameters(&self, pparameters: *const WICDdsParameters) -> ::windows::core::Result<()>;
    fn GetParameters(&self) -> ::windows::core::Result<WICDdsParameters>;
    fn CreateNewFrame(&self, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsEncoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsEncoder_Impl, const OFFSET: isize>() -> IWICDdsEncoder_Vtbl {
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn GetParameters<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNewFrame<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut ::windows::core::RawPtr, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateNewFrame(::core::mem::transmute_copy(&ppiframeencode), ::core::mem::transmute_copy(&parrayindex), ::core::mem::transmute_copy(&pmiplevel), ::core::mem::transmute_copy(&psliceindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            CreateNewFrame: CreateNewFrame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDdsEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsFrameDecode_Impl: Sized {
    fn GetSizeInBlocks(&self, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows::core::Result<()>;
    fn GetFormatInfo(&self) -> ::windows::core::Result<WICDdsFormatInfo>;
    fn CopyBlocks(&self, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsFrameDecode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsFrameDecode_Impl, const OFFSET: isize>() -> IWICDdsFrameDecode_Vtbl {
        unsafe extern "system" fn GetSizeInBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSizeInBlocks(::core::mem::transmute_copy(&pwidthinblocks), ::core::mem::transmute_copy(&pheightinblocks)).into()
        }
        unsafe extern "system" fn GetFormatInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatinfo: *mut WICDdsFormatInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormatInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pformatinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyBlocks(::core::mem::transmute_copy(&prcboundsinblocks), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSizeInBlocks: GetSizeInBlocks::<Identity, Impl, OFFSET>,
            GetFormatInfo: GetFormatInfo::<Identity, Impl, OFFSET>,
            CopyBlocks: CopyBlocks::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDdsFrameDecode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICDevelopRaw_Impl: Sized + IWICBitmapSource_Impl + IWICBitmapFrameDecode_Impl {
    fn QueryRawCapabilitiesInfo(&self, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows::core::Result<()>;
    fn LoadParameterSet(&self, parameterset: WICRawParameterSet) -> ::windows::core::Result<()>;
    fn GetCurrentParameterSet(&self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2>;
    fn SetExposureCompensation(&self, ev: f64) -> ::windows::core::Result<()>;
    fn GetExposureCompensation(&self) -> ::windows::core::Result<f64>;
    fn SetWhitePointRGB(&self, red: u32, green: u32, blue: u32) -> ::windows::core::Result<()>;
    fn GetWhitePointRGB(&self, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows::core::Result<()>;
    fn SetNamedWhitePoint(&self, whitepoint: WICNamedWhitePoint) -> ::windows::core::Result<()>;
    fn GetNamedWhitePoint(&self) -> ::windows::core::Result<WICNamedWhitePoint>;
    fn SetWhitePointKelvin(&self, whitepointkelvin: u32) -> ::windows::core::Result<()>;
    fn GetWhitePointKelvin(&self) -> ::windows::core::Result<u32>;
    fn GetKelvinRangeInfo(&self, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows::core::Result<()>;
    fn SetContrast(&self, contrast: f64) -> ::windows::core::Result<()>;
    fn GetContrast(&self) -> ::windows::core::Result<f64>;
    fn SetGamma(&self, gamma: f64) -> ::windows::core::Result<()>;
    fn GetGamma(&self) -> ::windows::core::Result<f64>;
    fn SetSharpness(&self, sharpness: f64) -> ::windows::core::Result<()>;
    fn GetSharpness(&self) -> ::windows::core::Result<f64>;
    fn SetSaturation(&self, saturation: f64) -> ::windows::core::Result<()>;
    fn GetSaturation(&self) -> ::windows::core::Result<f64>;
    fn SetTint(&self, tint: f64) -> ::windows::core::Result<()>;
    fn GetTint(&self) -> ::windows::core::Result<f64>;
    fn SetNoiseReduction(&self, noisereduction: f64) -> ::windows::core::Result<()>;
    fn GetNoiseReduction(&self) -> ::windows::core::Result<f64>;
    fn SetDestinationColorContext(&self, pcolorcontext: &::core::option::Option<IWICColorContext>) -> ::windows::core::Result<()>;
    fn SetToneCurve(&self, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows::core::Result<()>;
    fn GetToneCurve(&self, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn SetRotation(&self, rotation: f64) -> ::windows::core::Result<()>;
    fn GetRotation(&self) -> ::windows::core::Result<f64>;
    fn SetRenderMode(&self, rendermode: WICRawRenderMode) -> ::windows::core::Result<()>;
    fn GetRenderMode(&self) -> ::windows::core::Result<WICRawRenderMode>;
    fn SetNotificationCallback(&self, pcallback: &::core::option::Option<IWICDevelopRawNotificationCallback>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICDevelopRaw_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>() -> IWICDevelopRaw_Vtbl {
        unsafe extern "system" fn QueryRawCapabilitiesInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryRawCapabilitiesInfo(::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn LoadParameterSet<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterset: WICRawParameterSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadParameterSet(::core::mem::transmute_copy(&parameterset)).into()
        }
        unsafe extern "system" fn GetCurrentParameterSet<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcurrentparameterset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentParameterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcurrentparameterset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExposureCompensation<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ev: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExposureCompensation(::core::mem::transmute_copy(&ev)).into()
        }
        unsafe extern "system" fn GetExposureCompensation<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pev: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetExposureCompensation() {
                ::core::result::Result::Ok(ok__) => {
                    *pev = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointRGB<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, red: u32, green: u32, blue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWhitePointRGB(::core::mem::transmute_copy(&red), ::core::mem::transmute_copy(&green), ::core::mem::transmute_copy(&blue)).into()
        }
        unsafe extern "system" fn GetWhitePointRGB<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWhitePointRGB(::core::mem::transmute_copy(&pred), ::core::mem::transmute_copy(&pgreen), ::core::mem::transmute_copy(&pblue)).into()
        }
        unsafe extern "system" fn SetNamedWhitePoint<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepoint: WICNamedWhitePoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNamedWhitePoint(::core::mem::transmute_copy(&whitepoint)).into()
        }
        unsafe extern "system" fn GetNamedWhitePoint<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwhitepoint: *mut WICNamedWhitePoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNamedWhitePoint() {
                ::core::result::Result::Ok(ok__) => {
                    *pwhitepoint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointKelvin<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointkelvin: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWhitePointKelvin(::core::mem::transmute_copy(&whitepointkelvin)).into()
        }
        unsafe extern "system" fn GetWhitePointKelvin<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwhitepointkelvin: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWhitePointKelvin() {
                ::core::result::Result::Ok(ok__) => {
                    *pwhitepointkelvin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKelvinRangeInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetKelvinRangeInfo(::core::mem::transmute_copy(&pminkelvintemp), ::core::mem::transmute_copy(&pmaxkelvintemp), ::core::mem::transmute_copy(&pkelvintempstepvalue)).into()
        }
        unsafe extern "system" fn SetContrast<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contrast: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContrast(::core::mem::transmute_copy(&contrast)).into()
        }
        unsafe extern "system" fn GetContrast<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrast: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontrast = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGamma<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGamma(::core::mem::transmute_copy(&gamma)).into()
        }
        unsafe extern "system" fn GetGamma<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgamma: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGamma() {
                ::core::result::Result::Ok(ok__) => {
                    *pgamma = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharpness<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharpness: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSharpness(::core::mem::transmute_copy(&sharpness)).into()
        }
        unsafe extern "system" fn GetSharpness<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharpness: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSharpness() {
                ::core::result::Result::Ok(ok__) => {
                    *psharpness = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaturation<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, saturation: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSaturation(::core::mem::transmute_copy(&saturation)).into()
        }
        unsafe extern "system" fn GetSaturation<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psaturation: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSaturation() {
                ::core::result::Result::Ok(ok__) => {
                    *psaturation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTint<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tint: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTint(::core::mem::transmute_copy(&tint)).into()
        }
        unsafe extern "system" fn GetTint<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptint: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTint() {
                ::core::result::Result::Ok(ok__) => {
                    *ptint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoiseReduction<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noisereduction: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNoiseReduction(::core::mem::transmute_copy(&noisereduction)).into()
        }
        unsafe extern "system" fn GetNoiseReduction<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnoisereduction: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNoiseReduction() {
                ::core::result::Result::Ok(ok__) => {
                    *pnoisereduction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationColorContext<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDestinationColorContext(::core::mem::transmute(&pcolorcontext)).into()
        }
        unsafe extern "system" fn SetToneCurve<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetToneCurve(::core::mem::transmute_copy(&cbtonecurvesize), ::core::mem::transmute_copy(&ptonecurve)).into()
        }
        unsafe extern "system" fn GetToneCurve<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetToneCurve(::core::mem::transmute_copy(&cbtonecurvebuffersize), ::core::mem::transmute_copy(&ptonecurve), ::core::mem::transmute_copy(&pcbactualtonecurvebuffersize)).into()
        }
        unsafe extern "system" fn SetRotation<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRotation(::core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn GetRotation<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protation: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *protation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderMode<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendermode: WICRawRenderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRenderMode(::core::mem::transmute_copy(&rendermode)).into()
        }
        unsafe extern "system" fn GetRenderMode<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendermode: *mut WICRawRenderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRenderMode() {
                ::core::result::Result::Ok(ok__) => {
                    *prendermode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationCallback<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotificationCallback(::core::mem::transmute(&pcallback)).into()
        }
        Self {
            base: IWICBitmapFrameDecode_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDevelopRaw as ::windows::core::Interface>::IID || iid == &<IWICBitmapSource as ::windows::core::Interface>::IID || iid == &<IWICBitmapFrameDecode as ::windows::core::Interface>::IID
    }
}
pub trait IWICDevelopRawNotificationCallback_Impl: Sized {
    fn Notify(&self, notificationmask: u32) -> ::windows::core::Result<()>;
}
impl IWICDevelopRawNotificationCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRawNotificationCallback_Impl, const OFFSET: isize>() -> IWICDevelopRawNotificationCallback_Vtbl {
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRawNotificationCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&notificationmask)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDevelopRawNotificationCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICEnumMetadataItem_Impl: Sized {
    fn Next(&self, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IWICEnumMetadataItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICEnumMetadataItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>() -> IWICEnumMetadataItem_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgeltschema), ::core::mem::transmute_copy(&rgeltid), ::core::mem::transmute_copy(&rgeltvalue), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadataitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienummetadataitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICEnumMetadataItem as ::windows::core::Interface>::IID
    }
}
pub trait IWICFastMetadataEncoder_Impl: Sized {
    fn Commit(&self) -> ::windows::core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> ::windows::core::Result<IWICMetadataQueryWriter>;
}
impl IWICFastMetadataEncoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFastMetadataEncoder_Impl, const OFFSET: isize>() -> IWICFastMetadataEncoder_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IWICFastMetadataEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: ::windows::core::IUnknownImpl, Impl: IWICFastMetadataEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMetadataQueryWriter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadataquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, Impl, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICFastMetadataEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICFormatConverter_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: &::core::option::Option<IWICBitmapSource>, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: &::core::option::Option<IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::Result<()>;
    fn CanConvert(&self, srcpixelformat: *const ::windows::core::GUID, dstpixelformat: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICFormatConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverter_Impl, const OFFSET: isize>() -> IWICFormatConverter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: ::windows::core::RawPtr, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pisource), ::core::mem::transmute_copy(&dstformat), ::core::mem::transmute_copy(&dither), ::core::mem::transmute(&pipalette), ::core::mem::transmute_copy(&alphathresholdpercent), ::core::mem::transmute_copy(&palettetranslate)).into()
        }
        unsafe extern "system" fn CanConvert<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcpixelformat: *const ::windows::core::GUID, dstpixelformat: *const ::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanConvert(::core::mem::transmute_copy(&srcpixelformat), ::core::mem::transmute_copy(&dstpixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfcanconvert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CanConvert: CanConvert::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICFormatConverter as ::windows::core::Interface>::IID || iid == &<IWICBitmapSource as ::windows::core::Interface>::IID
    }
}
pub trait IWICFormatConverterInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetPixelFormats(&self, cformats: u32, ppixelformatguids: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::Result<()>;
    fn CreateInstance(&self) -> ::windows::core::Result<IWICFormatConverter>;
}
impl IWICFormatConverterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverterInfo_Impl, const OFFSET: isize>() -> IWICFormatConverterInfo_Vtbl {
        unsafe extern "system" fn GetPixelFormats<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, ppixelformatguids: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPixelFormats(::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&ppixelformatguids), ::core::mem::transmute_copy(&pcactual)).into()
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiconverter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICComponentInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPixelFormats: GetPixelFormats::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICFormatConverterInfo as ::windows::core::Interface>::IID || iid == &<IWICComponentInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactory_Impl: Sized {
    fn CreateDecoderFromFilename(&self, wzfilename: &::windows::core::PCWSTR, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromStream(&self, pistream: &::core::option::Option<super::super::System::Com::IStream>, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>;
    fn CreateComponentInfo(&self, clsidcomponent: *const ::windows::core::GUID) -> ::windows::core::Result<IWICComponentInfo>;
    fn CreateDecoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapDecoder>;
    fn CreateEncoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapEncoder>;
    fn CreatePalette(&self) -> ::windows::core::Result<IWICPalette>;
    fn CreateFormatConverter(&self) -> ::windows::core::Result<IWICFormatConverter>;
    fn CreateBitmapScaler(&self) -> ::windows::core::Result<IWICBitmapScaler>;
    fn CreateBitmapClipper(&self) -> ::windows::core::Result<IWICBitmapClipper>;
    fn CreateBitmapFlipRotator(&self) -> ::windows::core::Result<IWICBitmapFlipRotator>;
    fn CreateStream(&self) -> ::windows::core::Result<IWICStream>;
    fn CreateColorContext(&self) -> ::windows::core::Result<IWICColorContext>;
    fn CreateColorTransformer(&self) -> ::windows::core::Result<IWICColorTransform>;
    fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap>;
    fn CreateBitmapFromSource(&self, pibitmapsource: &::core::option::Option<IWICBitmapSource>, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap>;
    fn CreateBitmapFromSourceRect(&self, pibitmapsource: &::core::option::Option<IWICBitmapSource>, x: u32, y: u32, width: u32, height: u32) -> ::windows::core::Result<IWICBitmap>;
    fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8) -> ::windows::core::Result<IWICBitmap>;
    fn CreateBitmapFromHBITMAP(&self, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption) -> ::windows::core::Result<IWICBitmap>;
    fn CreateBitmapFromHICON(&self, hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::Result<IWICBitmap>;
    fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown>;
    fn CreateFastMetadataEncoderFromDecoder(&self, pidecoder: &::core::option::Option<IWICBitmapDecoder>) -> ::windows::core::Result<IWICFastMetadataEncoder>;
    fn CreateFastMetadataEncoderFromFrameDecode(&self, piframedecoder: &::core::option::Option<IWICBitmapFrameDecode>) -> ::windows::core::Result<IWICFastMetadataEncoder>;
    fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter>;
    fn CreateQueryWriterFromReader(&self, piqueryreader: &::core::option::Option<IWICMetadataQueryReader>, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICImagingFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>() -> IWICImagingFactory_Vtbl {
        unsafe extern "system" fn CreateDecoderFromFilename<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: ::windows::core::PCWSTR, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDecoderFromFilename(::core::mem::transmute(&wzfilename), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoderFromStream<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDecoderFromStream(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoderFromFileHandle<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDecoderFromFileHandle(::core::mem::transmute_copy(&hfile), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComponentInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcomponent: *const ::windows::core::GUID, ppiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateComponentInfo(::core::mem::transmute_copy(&clsidcomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoder<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDecoder(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncoder<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEncoder(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiencoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipalette: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePalette() {
                ::core::result::Result::Ok(ok__) => {
                    *ppipalette = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFormatConverter<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiformatconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFormatConverter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiformatconverter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapScaler<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapscaler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBitmapScaler() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapscaler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapClipper<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapclipper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBitmapClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapclipper = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFlipRotator<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapfliprotator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBitmapFlipRotator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapfliprotator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStream<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwicstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwicstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContext<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiccolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateColorContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwiccolorcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorTransformer<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiccolortransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateColorTransformer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwiccolortransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmap<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBitmap(::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromSource<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBitmapFromSource(::core::mem::transmute(&pibitmapsource), ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromSourceRect<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBitmapFromSourceRect(::core::mem::transmute(&pibitmapsource), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromMemory<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBitmapFromMemory(::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromHBITMAP<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBitmapFromHBITMAP(::core::mem::transmute_copy(&hbitmap), ::core::mem::transmute_copy(&hpalette), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromHICON<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBitmapFromHICON(::core::mem::transmute_copy(&hicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComponentEnumerator<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, componenttypes: u32, options: u32, ppienumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateComponentEnumerator(::core::mem::transmute_copy(&componenttypes), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppienumunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromDecoder<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFastMetadataEncoderFromDecoder(::core::mem::transmute(&pidecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifastencoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromFrameDecode<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piframedecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFastMetadataEncoderFromFrameDecode(::core::mem::transmute(&piframedecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifastencoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriter<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateQueryWriter(::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromReader<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piqueryreader: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateQueryWriterFromReader(::core::mem::transmute(&piqueryreader), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICImagingFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IWICJpegFrameDecode_Impl: Sized {
    fn DoesSupportIndexing(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIndexing(&self, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows::core::Result<()>;
    fn ClearIndexing(&self) -> ::windows::core::Result<()>;
    fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE>;
    fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE>;
    fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE>;
    fn GetFrameHeader(&self) -> ::windows::core::Result<WICJpegFrameHeader>;
    fn GetScanHeader(&self, scanindex: u32) -> ::windows::core::Result<WICJpegScanHeader>;
    fn CopyScan(&self, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows::core::Result<()>;
    fn CopyMinimalStream(&self, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IWICJpegFrameDecode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>() -> IWICJpegFrameDecode_Vtbl {
        unsafe extern "system" fn DoesSupportIndexing<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfindexingsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoesSupportIndexing() {
                ::core::result::Result::Ok(ok__) => {
                    *pfindexingsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexing<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIndexing(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&horizontalintervalsize)).into()
        }
        unsafe extern "system" fn ClearIndexing<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearIndexing().into()
        }
        unsafe extern "system" fn GetAcHuffmanTable<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pachuffmantable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDcHuffmanTable<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdchuffmantable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuantizationTable<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetQuantizationTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pquantizationtable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameHeader<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframeheader: *mut WICJpegFrameHeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFrameHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *pframeheader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScanHeader<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetScanHeader(::core::mem::transmute_copy(&scanindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pscanheader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyScan<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyScan(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&scanoffset), ::core::mem::transmute_copy(&cbscandata), ::core::mem::transmute_copy(&pbscandata), ::core::mem::transmute_copy(&pcbscandataactual)).into()
        }
        unsafe extern "system" fn CopyMinimalStream<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyMinimalStream(::core::mem::transmute_copy(&streamoffset), ::core::mem::transmute_copy(&cbstreamdata), ::core::mem::transmute_copy(&pbstreamdata), ::core::mem::transmute_copy(&pcbstreamdataactual)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICJpegFrameDecode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICJpegFrameEncode_Impl: Sized {
    fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE>;
    fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE>;
    fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE>;
    fn WriteScan(&self, cbscandata: u32, pbscandata: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICJpegFrameEncode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>() -> IWICJpegFrameEncode_Vtbl {
        unsafe extern "system" fn GetAcHuffmanTable<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pachuffmantable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDcHuffmanTable<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdchuffmantable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuantizationTable<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetQuantizationTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pquantizationtable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteScan<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbscandata: u32, pbscandata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteScan(::core::mem::transmute_copy(&cbscandata), ::core::mem::transmute_copy(&pbscandata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAcHuffmanTable: GetAcHuffmanTable::<Identity, Impl, OFFSET>,
            GetDcHuffmanTable: GetDcHuffmanTable::<Identity, Impl, OFFSET>,
            GetQuantizationTable: GetQuantizationTable::<Identity, Impl, OFFSET>,
            WriteScan: WriteScan::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICJpegFrameEncode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockReader_Impl: Sized {
    fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn GetReaderByIndex(&self, nindex: u32) -> ::windows::core::Result<IWICMetadataReader>;
    fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataBlockReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>() -> IWICMetadataBlockReader_Vtbl {
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidcontainerformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReaderByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatareader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReaderByIndex(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadatareader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienummetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetReaderByIndex: GetReaderByIndex::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataBlockReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockWriter_Impl: Sized + IWICMetadataBlockReader_Impl {
    fn InitializeFromBlockReader(&self, pimdblockreader: &::core::option::Option<IWICMetadataBlockReader>) -> ::windows::core::Result<()>;
    fn GetWriterByIndex(&self, nindex: u32) -> ::windows::core::Result<IWICMetadataWriter>;
    fn AddWriter(&self, pimetadatawriter: &::core::option::Option<IWICMetadataWriter>) -> ::windows::core::Result<()>;
    fn SetWriterByIndex(&self, nindex: u32, pimetadatawriter: &::core::option::Option<IWICMetadataWriter>) -> ::windows::core::Result<()>;
    fn RemoveWriterByIndex(&self, nindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataBlockWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>() -> IWICMetadataBlockWriter_Vtbl {
        unsafe extern "system" fn InitializeFromBlockReader<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimdblockreader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeFromBlockReader(::core::mem::transmute(&pimdblockreader)).into()
        }
        unsafe extern "system" fn GetWriterByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatawriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWriterByIndex(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadatawriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWriter<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimetadatawriter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddWriter(::core::mem::transmute(&pimetadatawriter)).into()
        }
        unsafe extern "system" fn SetWriterByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pimetadatawriter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWriterByIndex(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&pimetadatawriter)).into()
        }
        unsafe extern "system" fn RemoveWriterByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveWriterByIndex(::core::mem::transmute_copy(&nindex)).into()
        }
        Self {
            base: IWICMetadataBlockReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitializeFromBlockReader: InitializeFromBlockReader::<Identity, Impl, OFFSET>,
            GetWriterByIndex: GetWriterByIndex::<Identity, Impl, OFFSET>,
            AddWriter: AddWriter::<Identity, Impl, OFFSET>,
            SetWriterByIndex: SetWriterByIndex::<Identity, Impl, OFFSET>,
            RemoveWriterByIndex: RemoveWriterByIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataBlockWriter as ::windows::core::Interface>::IID || iid == &<IWICMetadataBlockReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICMetadataHandlerInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetContainerFormats(&self, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceManufacturer(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: &::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceModels(&self, cchdevicemodels: u32, wzdevicemodels: &::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn DoesRequireFullStream(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DoesRequireFixedSize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICMetadataHandlerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>() -> IWICMetadataHandlerInfo_Vtbl {
        unsafe extern "system" fn GetMetadataFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMetadataFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidmetadataformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerFormats<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContainerFormats(::core::mem::transmute_copy(&ccontainerformats), ::core::mem::transmute_copy(&pguidcontainerformats), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceManufacturer(::core::mem::transmute_copy(&cchdevicemanufacturer), ::core::mem::transmute(&wzdevicemanufacturer), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceModels<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceModels(::core::mem::transmute_copy(&cchdevicemodels), ::core::mem::transmute(&wzdevicemodels), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn DoesRequireFullStream<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequiresfullstream: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoesRequireFullStream() {
                ::core::result::Result::Ok(ok__) => {
                    *pfrequiresfullstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportPadding<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportspadding: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoesSupportPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportspadding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesRequireFixedSize<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffixedsize: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoesRequireFixedSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pffixedsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICComponentInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMetadataFormat: GetMetadataFormat::<Identity, Impl, OFFSET>,
            GetContainerFormats: GetContainerFormats::<Identity, Impl, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, Impl, OFFSET>,
            GetDeviceModels: GetDeviceModels::<Identity, Impl, OFFSET>,
            DoesRequireFullStream: DoesRequireFullStream::<Identity, Impl, OFFSET>,
            DoesSupportPadding: DoesSupportPadding::<Identity, Impl, OFFSET>,
            DoesRequireFixedSize: DoesRequireFixedSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataHandlerInfo as ::windows::core::Interface>::IID || iid == &<IWICComponentInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataQueryReader_Impl: Sized {
    fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetLocation(&self, cchmaxlength: u32, wznamespace: &::windows::core::PWSTR, pcchactuallength: *mut u32) -> ::windows::core::Result<()>;
    fn GetMetadataByName(&self, wzname: &::windows::core::PCWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataQueryReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>() -> IWICMetadataQueryReader_Vtbl {
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidcontainerformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocation<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmaxlength: u32, wznamespace: ::windows::core::PWSTR, pcchactuallength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLocation(::core::mem::transmute_copy(&cchmaxlength), ::core::mem::transmute(&wznamespace), ::core::mem::transmute_copy(&pcchactuallength)).into()
        }
        unsafe extern "system" fn GetMetadataByName<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: ::windows::core::PCWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMetadataByName(::core::mem::transmute(&wzname), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienumstring: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienumstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetLocation: GetLocation::<Identity, Impl, OFFSET>,
            GetMetadataByName: GetMetadataByName::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataQueryReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataQueryWriter_Impl: Sized + IWICMetadataQueryReader_Impl {
    fn SetMetadataByName(&self, wzname: &::windows::core::PCWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn RemoveMetadataByName(&self, wzname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataQueryWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryWriter_Impl, const OFFSET: isize>() -> IWICMetadataQueryWriter_Vtbl {
        unsafe extern "system" fn SetMetadataByName<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: ::windows::core::PCWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMetadataByName(::core::mem::transmute(&wzname), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn RemoveMetadataByName<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveMetadataByName(::core::mem::transmute(&wzname)).into()
        }
        Self {
            base: IWICMetadataQueryReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMetadataByName: SetMetadataByName::<Identity, Impl, OFFSET>,
            RemoveMetadataByName: RemoveMetadataByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataQueryWriter as ::windows::core::Interface>::IID || iid == &<IWICMetadataQueryReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataReader_Impl: Sized {
    fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetMetadataHandlerInfo(&self) -> ::windows::core::Result<IWICMetadataHandlerInfo>;
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn GetValueByIndex(&self, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetEnumerator(&self) -> ::windows::core::Result<IWICEnumMetadataItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReader_Impl, const OFFSET: isize>() -> IWICMetadataReader_Vtbl {
        unsafe extern "system" fn GetMetadataFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMetadataFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidmetadataformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataHandlerInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppihandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMetadataHandlerInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppihandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetValueByIndex(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienummetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetMetadataFormat: GetMetadataFormat::<Identity, Impl, OFFSET>,
            GetMetadataHandlerInfo: GetMetadataHandlerInfo::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetValueByIndex: GetValueByIndex::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICMetadataReaderInfo_Impl: Sized + IWICComponentInfo_Impl + IWICMetadataHandlerInfo_Impl {
    fn GetPatterns(&self, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows::core::Result<()>;
    fn MatchesPattern(&self, guidcontainerformat: *const ::windows::core::GUID, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CreateInstance(&self) -> ::windows::core::Result<IWICMetadataReader>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICMetadataReaderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: isize>() -> IWICMetadataReaderInfo_Vtbl {
        unsafe extern "system" fn GetPatterns<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPatterns(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&ppattern), ::core::mem::transmute_copy(&pccount), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn MatchesPattern<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pistream: ::windows::core::RawPtr, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MatchesPattern(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmatches = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *ppireader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICMetadataHandlerInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPatterns: GetPatterns::<Identity, Impl, OFFSET>,
            MatchesPattern: MatchesPattern::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataReaderInfo as ::windows::core::Interface>::IID || iid == &<IWICComponentInfo as ::windows::core::Interface>::IID || iid == &<IWICMetadataHandlerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataWriter_Impl: Sized + IWICMetadataReader_Impl {
    fn SetValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn SetValueByIndex(&self, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn RemoveValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn RemoveValueByIndex(&self, nindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriter_Impl, const OFFSET: isize>() -> IWICMetadataWriter_Vtbl {
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn SetValueByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValueByIndex(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn RemoveValue<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveValue(::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid)).into()
        }
        unsafe extern "system" fn RemoveValueByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveValueByIndex(::core::mem::transmute_copy(&nindex)).into()
        }
        Self {
            base: IWICMetadataReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            SetValueByIndex: SetValueByIndex::<Identity, Impl, OFFSET>,
            RemoveValue: RemoveValue::<Identity, Impl, OFFSET>,
            RemoveValueByIndex: RemoveValueByIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataWriter as ::windows::core::Interface>::IID || iid == &<IWICMetadataReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICMetadataWriterInfo_Impl: Sized + IWICComponentInfo_Impl + IWICMetadataHandlerInfo_Impl {
    fn GetHeader(&self, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows::core::Result<()>;
    fn CreateInstance(&self) -> ::windows::core::Result<IWICMetadataWriter>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICMetadataWriterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriterInfo_Impl, const OFFSET: isize>() -> IWICMetadataWriterInfo_Vtbl {
        unsafe extern "system" fn GetHeader<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHeader(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pheader), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICMetadataHandlerInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetHeader: GetHeader::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataWriterInfo as ::windows::core::Interface>::IID || iid == &<IWICComponentInfo as ::windows::core::Interface>::IID || iid == &<IWICMetadataHandlerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPalette_Impl: Sized {
    fn InitializePredefined(&self, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InitializeCustom(&self, pcolors: *const u32, ccount: u32) -> ::windows::core::Result<()>;
    fn InitializeFromBitmap(&self, pisurface: &::core::option::Option<IWICBitmapSource>, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InitializeFromPalette(&self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn GetType(&self) -> ::windows::core::Result<WICBitmapPaletteType>;
    fn GetColorCount(&self) -> ::windows::core::Result<u32>;
    fn GetColors(&self, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows::core::Result<()>;
    fn IsBlackWhite(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsGrayscale(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn HasAlpha(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPalette_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const OFFSET: isize>() -> IWICPalette_Vtbl {
        unsafe extern "system" fn InitializePredefined<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializePredefined(::core::mem::transmute_copy(&epalettetype), ::core::mem::transmute_copy(&faddtransparentcolor)).into()
        }
        unsafe extern "system" fn InitializeCustom<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolors: *const u32, ccount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeCustom(::core::mem::transmute_copy(&pcolors), ::core::mem::transmute_copy(&ccount)).into()
        }
        unsafe extern "system" fn InitializeFromBitmap<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisurface: ::windows::core::RawPtr, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeFromBitmap(::core::mem::transmute(&pisurface), ::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&faddtransparentcolor)).into()
        }
        unsafe extern "system" fn InitializeFromPalette<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeFromPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pepalettetype: *mut WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pepalettetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorCount<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetColorCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColors<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColors(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&pcolors), ::core::mem::transmute_copy(&pcactualcolors)).into()
        }
        unsafe extern "system" fn IsBlackWhite<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisblackwhite: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsBlackWhite() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisblackwhite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGrayscale<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisgrayscale: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsGrayscale() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisgrayscale = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasAlpha<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfhasalpha: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HasAlpha() {
                ::core::result::Result::Ok(ok__) => {
                    *pfhasalpha = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPalette as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICPersistStream_Impl: Sized + super::super::System::Com::IPersist_Impl + super::super::System::Com::IPersistStream_Impl {
    fn LoadEx(&self, pistream: &::core::option::Option<super::super::System::Com::IStream>, pguidpreferredvendor: *const ::windows::core::GUID, dwpersistoptions: u32) -> ::windows::core::Result<()>;
    fn SaveEx(&self, pistream: &::core::option::Option<super::super::System::Com::IStream>, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICPersistStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPersistStream_Impl, const OFFSET: isize>() -> IWICPersistStream_Vtbl {
        unsafe extern "system" fn LoadEx<Identity: ::windows::core::IUnknownImpl, Impl: IWICPersistStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pguidpreferredvendor: *const ::windows::core::GUID, dwpersistoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadEx(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&pguidpreferredvendor), ::core::mem::transmute_copy(&dwpersistoptions)).into()
        }
        unsafe extern "system" fn SaveEx<Identity: ::windows::core::IUnknownImpl, Impl: IWICPersistStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveEx(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&dwpersistoptions), ::core::mem::transmute_copy(&fcleardirty)).into()
        }
        Self {
            base: super::super::System::Com::IPersistStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            LoadEx: LoadEx::<Identity, Impl, OFFSET>,
            SaveEx: SaveEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPersistStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IPersist as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IPersistStream as ::windows::core::Interface>::IID
    }
}
pub trait IWICPixelFormatInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetFormatGUID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetColorContext(&self) -> ::windows::core::Result<IWICColorContext>;
    fn GetBitsPerPixel(&self) -> ::windows::core::Result<u32>;
    fn GetChannelCount(&self) -> ::windows::core::Result<u32>;
    fn GetChannelMask(&self, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::Result<()>;
}
impl IWICPixelFormatInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>() -> IWICPixelFormatInfo_Vtbl {
        unsafe extern "system" fn GetFormatGUID<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormatGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContext<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetColorContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppicolorcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitsPerPixel<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puibitsperpixel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBitsPerPixel() {
                ::core::result::Result::Ok(ok__) => {
                    *puibitsperpixel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelCount<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puichannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puichannelcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelMask<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChannelMask(::core::mem::transmute_copy(&uichannelindex), ::core::mem::transmute_copy(&cbmaskbuffer), ::core::mem::transmute_copy(&pbmaskbuffer), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        Self {
            base: IWICComponentInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFormatGUID: GetFormatGUID::<Identity, Impl, OFFSET>,
            GetColorContext: GetColorContext::<Identity, Impl, OFFSET>,
            GetBitsPerPixel: GetBitsPerPixel::<Identity, Impl, OFFSET>,
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            GetChannelMask: GetChannelMask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo as ::windows::core::Interface>::IID || iid == &<IWICComponentInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPixelFormatInfo2_Impl: Sized + IWICComponentInfo_Impl + IWICPixelFormatInfo_Impl {
    fn SupportsTransparency(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetNumericRepresentation(&self) -> ::windows::core::Result<WICPixelFormatNumericRepresentation>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPixelFormatInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo2_Impl, const OFFSET: isize>() -> IWICPixelFormatInfo2_Vtbl {
        unsafe extern "system" fn SupportsTransparency<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportstransparency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportsTransparency() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportstransparency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericRepresentation<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumericrepresentation: *mut WICPixelFormatNumericRepresentation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNumericRepresentation() {
                ::core::result::Result::Ok(ok__) => {
                    *pnumericrepresentation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICPixelFormatInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            SupportsTransparency: SupportsTransparency::<Identity, Impl, OFFSET>,
            GetNumericRepresentation: GetNumericRepresentation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo2 as ::windows::core::Interface>::IID || iid == &<IWICComponentInfo as ::windows::core::Interface>::IID || iid == &<IWICPixelFormatInfo as ::windows::core::Interface>::IID
    }
}
pub trait IWICPlanarBitmapFrameEncode_Impl: Sized {
    fn WritePixels(&self, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::Result<()>;
    fn WriteSource(&self, ppplanes: *const ::core::option::Option<IWICBitmapSource>, cplanes: u32, prcsource: *const WICRect) -> ::windows::core::Result<()>;
}
impl IWICPlanarBitmapFrameEncode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: isize>() -> IWICPlanarBitmapFrameEncode_Vtbl {
        unsafe extern "system" fn WritePixels<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WritePixels(::core::mem::transmute_copy(&linecount), ::core::mem::transmute_copy(&pplanes), ::core::mem::transmute_copy(&cplanes)).into()
        }
        unsafe extern "system" fn WriteSource<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplanes: *const ::windows::core::RawPtr, cplanes: u32, prcsource: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteSource(::core::mem::transmute_copy(&ppplanes), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&prcsource)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            WritePixels: WritePixels::<Identity, Impl, OFFSET>,
            WriteSource: WriteSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPlanarBitmapFrameEncode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPlanarBitmapSourceTransform_Impl: Sized {
    fn DoesSupportTransform(&self, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows::core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CopyPixels(&self, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPlanarBitmapSourceTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: isize>() -> IWICPlanarBitmapSourceTransform_Vtbl {
        unsafe extern "system" fn DoesSupportTransform<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows::core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DoesSupportTransform(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&dstplanaroptions), ::core::mem::transmute_copy(&pguiddstformats), ::core::mem::transmute_copy(&pplanedescriptions), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&pfissupported)).into()
        }
        unsafe extern "system" fn CopyPixels<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyPixels(::core::mem::transmute_copy(&prcsource), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&dstplanaroptions), ::core::mem::transmute_copy(&pdstplanes), ::core::mem::transmute_copy(&cplanes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DoesSupportTransform: DoesSupportTransform::<Identity, Impl, OFFSET>,
            CopyPixels: CopyPixels::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPlanarBitmapSourceTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPlanarFormatConverter_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, ppplanes: *const ::core::option::Option<IWICBitmapSource>, cplanes: u32, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: &::core::option::Option<IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::Result<()>;
    fn CanConvert(&self, psrcpixelformats: *const ::windows::core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPlanarFormatConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarFormatConverter_Impl, const OFFSET: isize>() -> IWICPlanarFormatConverter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplanes: *const ::windows::core::RawPtr, cplanes: u32, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: ::windows::core::RawPtr, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&ppplanes), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&dstformat), ::core::mem::transmute_copy(&dither), ::core::mem::transmute(&pipalette), ::core::mem::transmute_copy(&alphathresholdpercent), ::core::mem::transmute_copy(&palettetranslate)).into()
        }
        unsafe extern "system" fn CanConvert<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcpixelformats: *const ::windows::core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanConvert(::core::mem::transmute_copy(&psrcpixelformats), ::core::mem::transmute_copy(&csrcplanes), ::core::mem::transmute_copy(&dstpixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfcanconvert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICBitmapSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CanConvert: CanConvert::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPlanarFormatConverter as ::windows::core::Interface>::IID || iid == &<IWICBitmapSource as ::windows::core::Interface>::IID
    }
}
pub trait IWICProgressCallback_Impl: Sized {
    fn Notify(&self, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::Result<()>;
}
impl IWICProgressCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressCallback_Impl, const OFFSET: isize>() -> IWICProgressCallback_Vtbl {
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&uframenum), ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&dblprogress)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICProgressCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWICProgressiveLevelControl_Impl: Sized {
    fn GetLevelCount(&self) -> ::windows::core::Result<u32>;
    fn GetCurrentLevel(&self) -> ::windows::core::Result<u32>;
    fn SetCurrentLevel(&self, nlevel: u32) -> ::windows::core::Result<()>;
}
impl IWICProgressiveLevelControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: isize>() -> IWICProgressiveLevelControl_Vtbl {
        unsafe extern "system" fn GetLevelCount<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLevelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pclevels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLevel<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pnlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentLevel<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCurrentLevel(::core::mem::transmute_copy(&nlevel)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLevelCount: GetLevelCount::<Identity, Impl, OFFSET>,
            GetCurrentLevel: GetCurrentLevel::<Identity, Impl, OFFSET>,
            SetCurrentLevel: SetCurrentLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICProgressiveLevelControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICStream_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn InitializeFromIStream(&self, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn InitializeFromFilename(&self, wzfilename: &::windows::core::PCWSTR, dwdesiredaccess: u32) -> ::windows::core::Result<()>;
    fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::Result<()>;
    fn InitializeFromIStreamRegion(&self, pistream: &::core::option::Option<super::super::System::Com::IStream>, uloffset: u64, ulmaxsize: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICStream_Impl, const OFFSET: isize>() -> IWICStream_Vtbl {
        unsafe extern "system" fn InitializeFromIStream<Identity: ::windows::core::IUnknownImpl, Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeFromIStream(::core::mem::transmute(&pistream)).into()
        }
        unsafe extern "system" fn InitializeFromFilename<Identity: ::windows::core::IUnknownImpl, Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: ::windows::core::PCWSTR, dwdesiredaccess: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeFromFilename(::core::mem::transmute(&wzfilename), ::core::mem::transmute_copy(&dwdesiredaccess)).into()
        }
        unsafe extern "system" fn InitializeFromMemory<Identity: ::windows::core::IUnknownImpl, Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeFromMemory(::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&cbbuffersize)).into()
        }
        unsafe extern "system" fn InitializeFromIStreamRegion<Identity: ::windows::core::IUnknownImpl, Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, uloffset: u64, ulmaxsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeFromIStreamRegion(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&ulmaxsize)).into()
        }
        Self {
            base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitializeFromIStream: InitializeFromIStream::<Identity, Impl, OFFSET>,
            InitializeFromFilename: InitializeFromFilename::<Identity, Impl, OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Identity, Impl, OFFSET>,
            InitializeFromIStreamRegion: InitializeFromIStreamRegion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICStreamProvider_Impl: Sized {
    fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn GetPersistOptions(&self) -> ::windows::core::Result<u32>;
    fn GetPreferredVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RefreshStream(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWICStreamProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICStreamProvider_Impl, const OFFSET: isize>() -> IWICStreamProvider_Vtbl {
        unsafe extern "system" fn GetStream<Identity: ::windows::core::IUnknownImpl, Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppistream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPersistOptions<Identity: ::windows::core::IUnknownImpl, Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpersistoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPersistOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwpersistoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredVendorGUID<Identity: ::windows::core::IUnknownImpl, Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidpreferredvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreferredVendorGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidpreferredvendor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshStream<Identity: ::windows::core::IUnknownImpl, Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RefreshStream().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            GetPersistOptions: GetPersistOptions::<Identity, Impl, OFFSET>,
            GetPreferredVendorGUID: GetPreferredVendorGUID::<Identity, Impl, OFFSET>,
            RefreshStream: RefreshStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICStreamProvider as ::windows::core::Interface>::IID
    }
}
