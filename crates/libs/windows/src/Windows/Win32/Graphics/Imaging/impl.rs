pub trait IWICBitmap_Impl: Sized + IWICBitmapSource_Impl {
    fn Lock(&mut self, prclock: *const WICRect, flags: u32) -> ::windows::core::Result<IWICBitmapLock>;
    fn SetPalette(&mut self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn SetResolution(&mut self, dpix: f64, dpiy: f64) -> ::windows::core::Result<()>;
}
impl IWICBitmap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmap_Vtbl {
        unsafe extern "system" fn Lock<Impl: IWICBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prclock: *const WICRect, flags: u32, ppilock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lock(::core::mem::transmute_copy(&prclock), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppilock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Impl: IWICBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn SetResolution<Impl: IWICBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResolution(::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        Self {
            base: IWICBitmapSource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Lock: Lock::<Impl, IMPL_OFFSET>,
            SetPalette: SetPalette::<Impl, IMPL_OFFSET>,
            SetResolution: SetResolution::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmap as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapClipper_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&mut self, pisource: &::core::option::Option<IWICBitmapSource>, prc: *const WICRect) -> ::windows::core::Result<()>;
}
impl IWICBitmapClipper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapClipper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapClipper_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapClipper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, prc: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pisource), ::core::mem::transmute_copy(&prc)).into()
        }
        Self { base: IWICBitmapSource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapClipper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapCodecInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetContainerFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetPixelFormats(&mut self, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetColorManagementVersion(&mut self, cchcolormanagementversion: u32, wzcolormanagementversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceManufacturer(&mut self, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceModels(&mut self, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetMimeTypes(&mut self, cchmimetypes: u32, wzmimetypes: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetFileExtensions(&mut self, cchfileextensions: u32, wzfileextensions: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn DoesSupportAnimation(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportChromakey(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportLossless(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportMultiframe(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn MatchesMimeType(&mut self, wzmimetype: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapCodecInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapCodecInfo_Vtbl {
        unsafe extern "system" fn GetContainerFormat<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidcontainerformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormats<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPixelFormats(::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&pguidpixelformats), ::core::mem::transmute_copy(&pcactual)).into()
        }
        unsafe extern "system" fn GetColorManagementVersion<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorManagementVersion(::core::mem::transmute_copy(&cchcolormanagementversion), ::core::mem::transmute_copy(&wzcolormanagementversion), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceManufacturer(::core::mem::transmute_copy(&cchdevicemanufacturer), ::core::mem::transmute_copy(&wzdevicemanufacturer), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceModels<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceModels(::core::mem::transmute_copy(&cchdevicemodels), ::core::mem::transmute_copy(&wzdevicemodels), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetMimeTypes<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmimetypes: u32, wzmimetypes: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMimeTypes(::core::mem::transmute_copy(&cchmimetypes), ::core::mem::transmute_copy(&wzmimetypes), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetFileExtensions<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchfileextensions: u32, wzfileextensions: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFileExtensions(::core::mem::transmute_copy(&cchfileextensions), ::core::mem::transmute_copy(&wzfileextensions), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn DoesSupportAnimation<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportanimation: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportanimation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportChromakey<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportchromakey: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportChromakey() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportchromakey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportLossless<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportlossless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportLossless() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportlossless = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportMultiframe<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportmultiframe: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportMultiframe() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportmultiframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesMimeType<Impl: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzmimetype: super::super::Foundation::PWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchesMimeType(::core::mem::transmute_copy(&wzmimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmatches = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICComponentInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Impl, IMPL_OFFSET>,
            GetPixelFormats: GetPixelFormats::<Impl, IMPL_OFFSET>,
            GetColorManagementVersion: GetColorManagementVersion::<Impl, IMPL_OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Impl, IMPL_OFFSET>,
            GetDeviceModels: GetDeviceModels::<Impl, IMPL_OFFSET>,
            GetMimeTypes: GetMimeTypes::<Impl, IMPL_OFFSET>,
            GetFileExtensions: GetFileExtensions::<Impl, IMPL_OFFSET>,
            DoesSupportAnimation: DoesSupportAnimation::<Impl, IMPL_OFFSET>,
            DoesSupportChromakey: DoesSupportChromakey::<Impl, IMPL_OFFSET>,
            DoesSupportLossless: DoesSupportLossless::<Impl, IMPL_OFFSET>,
            DoesSupportMultiframe: DoesSupportMultiframe::<Impl, IMPL_OFFSET>,
            MatchesMimeType: MatchesMimeType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapCodecInfo as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapCodecProgressNotification_Impl: Sized {
    fn RegisterProgressNotification(&mut self, pfnprogressnotification: &PFNProgressNotification, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows::core::Result<()>;
}
impl IWICBitmapCodecProgressNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecProgressNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapCodecProgressNotification_Vtbl {
        unsafe extern "system" fn RegisterProgressNotification<Impl: IWICBitmapCodecProgressNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfnprogressnotification: ::windows::core::RawPtr, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterProgressNotification(::core::mem::transmute_copy(&pfnprogressnotification), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&dwprogressflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterProgressNotification: RegisterProgressNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapCodecProgressNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICBitmapDecoder_Impl: Sized {
    fn QueryCapability(&mut self, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<u32>;
    fn Initialize(&mut self, pistream: &::core::option::Option<super::super::System::Com::IStream>, cacheoptions: WICDecodeOptions) -> ::windows::core::Result<()>;
    fn GetContainerFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDecoderInfo(&mut self) -> ::windows::core::Result<IWICBitmapDecoderInfo>;
    fn CopyPalette(&mut self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn GetMetadataQueryReader(&mut self) -> ::windows::core::Result<IWICMetadataQueryReader>;
    fn GetPreview(&mut self) -> ::windows::core::Result<IWICBitmapSource>;
    fn GetColorContexts(&mut self, ccount: u32, ppicolorcontexts: *mut ::core::option::Option<IWICColorContext>, pcactualcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetThumbnail(&mut self) -> ::windows::core::Result<IWICBitmapSource>;
    fn GetFrameCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetFrame(&mut self, index: u32) -> ::windows::core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWICBitmapDecoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapDecoder_Vtbl {
        unsafe extern "system" fn QueryCapability<Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pdwcapability: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryCapability(::core::mem::transmute(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, cacheoptions: WICDecodeOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&cacheoptions)).into()
        }
        unsafe extern "system" fn GetContainerFormat<Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidcontainerformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDecoderInfo<Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidecoderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDecoderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppidecoderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyPalette<Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn GetMetadataQueryReader<Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataQueryReader() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadataqueryreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreview<Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreview() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapsource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContexts<Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut ::windows::core::RawPtr, pcactualcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontexts), ::core::mem::transmute_copy(&pcactualcount)).into()
        }
        unsafe extern "system" fn GetThumbnail<Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *ppithumbnail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameCount<Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrame<Impl: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppibitmapframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrame(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryCapability: QueryCapability::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetContainerFormat: GetContainerFormat::<Impl, IMPL_OFFSET>,
            GetDecoderInfo: GetDecoderInfo::<Impl, IMPL_OFFSET>,
            CopyPalette: CopyPalette::<Impl, IMPL_OFFSET>,
            GetMetadataQueryReader: GetMetadataQueryReader::<Impl, IMPL_OFFSET>,
            GetPreview: GetPreview::<Impl, IMPL_OFFSET>,
            GetColorContexts: GetColorContexts::<Impl, IMPL_OFFSET>,
            GetThumbnail: GetThumbnail::<Impl, IMPL_OFFSET>,
            GetFrameCount: GetFrameCount::<Impl, IMPL_OFFSET>,
            GetFrame: GetFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapDecoder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICBitmapDecoderInfo_Impl: Sized + IWICComponentInfo_Impl + IWICBitmapCodecInfo_Impl {
    fn GetPatterns(&mut self, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows::core::Result<()>;
    fn MatchesPattern(&mut self, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CreateInstance(&mut self) -> ::windows::core::Result<IWICBitmapDecoder>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICBitmapDecoderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoderInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapDecoderInfo_Vtbl {
        unsafe extern "system" fn GetPatterns<Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPatterns(::core::mem::transmute_copy(&cbsizepatterns), ::core::mem::transmute_copy(&ppatterns), ::core::mem::transmute_copy(&pcpatterns), ::core::mem::transmute_copy(&pcbpatternsactual)).into()
        }
        unsafe extern "system" fn MatchesPattern<Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchesPattern(::core::mem::transmute(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmatches = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapdecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapdecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICBitmapCodecInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPatterns: GetPatterns::<Impl, IMPL_OFFSET>,
            MatchesPattern: MatchesPattern::<Impl, IMPL_OFFSET>,
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapDecoderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICBitmapEncoder_Impl: Sized {
    fn Initialize(&mut self, pistream: &::core::option::Option<super::super::System::Com::IStream>, cacheoption: WICBitmapEncoderCacheOption) -> ::windows::core::Result<()>;
    fn GetContainerFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetEncoderInfo(&mut self) -> ::windows::core::Result<IWICBitmapEncoderInfo>;
    fn SetColorContexts(&mut self, ccount: u32, ppicolorcontext: *const ::core::option::Option<IWICColorContext>) -> ::windows::core::Result<()>;
    fn SetPalette(&mut self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn SetThumbnail(&mut self, pithumbnail: &::core::option::Option<IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn SetPreview(&mut self, pipreview: &::core::option::Option<IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn CreateNewFrame(&mut self, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, ppiencoderoptions: *mut ::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyBag2>) -> ::windows::core::Result<()>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
    fn GetMetadataQueryWriter(&mut self) -> ::windows::core::Result<IWICMetadataQueryWriter>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICBitmapEncoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapEncoder_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, cacheoption: WICBitmapEncoderCacheOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&cacheoption)).into()
        }
        unsafe extern "system" fn GetContainerFormat<Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidcontainerformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEncoderInfo<Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiencoderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEncoderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiencoderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorContexts<Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontext)).into()
        }
        unsafe extern "system" fn SetPalette<Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn SetThumbnail<Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pithumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(::core::mem::transmute(&pithumbnail)).into()
        }
        unsafe extern "system" fn SetPreview<Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipreview: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreview(::core::mem::transmute(&pipreview)).into()
        }
        unsafe extern "system" fn CreateNewFrame<Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut ::windows::core::RawPtr, ppiencoderoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateNewFrame(::core::mem::transmute_copy(&ppiframeencode), ::core::mem::transmute_copy(&ppiencoderoptions)).into()
        }
        unsafe extern "system" fn Commit<Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Impl: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataQueryWriter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadataquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetContainerFormat: GetContainerFormat::<Impl, IMPL_OFFSET>,
            GetEncoderInfo: GetEncoderInfo::<Impl, IMPL_OFFSET>,
            SetColorContexts: SetColorContexts::<Impl, IMPL_OFFSET>,
            SetPalette: SetPalette::<Impl, IMPL_OFFSET>,
            SetThumbnail: SetThumbnail::<Impl, IMPL_OFFSET>,
            SetPreview: SetPreview::<Impl, IMPL_OFFSET>,
            CreateNewFrame: CreateNewFrame::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapEncoderInfo_Impl: Sized + IWICComponentInfo_Impl + IWICBitmapCodecInfo_Impl {
    fn CreateInstance(&mut self) -> ::windows::core::Result<IWICBitmapEncoder>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapEncoderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoderInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapEncoderInfo_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IWICBitmapEncoderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapencoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWICBitmapCodecInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapEncoderInfo as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapFlipRotator_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&mut self, pisource: &::core::option::Option<IWICBitmapSource>, options: WICBitmapTransformOptions) -> ::windows::core::Result<()>;
}
impl IWICBitmapFlipRotator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFlipRotator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapFlipRotator_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapFlipRotator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, options: WICBitmapTransformOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pisource), ::core::mem::transmute_copy(&options)).into()
        }
        Self { base: IWICBitmapSource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapFlipRotator as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapFrameDecode_Impl: Sized + IWICBitmapSource_Impl {
    fn GetMetadataQueryReader(&mut self) -> ::windows::core::Result<IWICMetadataQueryReader>;
    fn GetColorContexts(&mut self, ccount: u32, ppicolorcontexts: *mut ::core::option::Option<IWICColorContext>, pcactualcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetThumbnail(&mut self) -> ::windows::core::Result<IWICBitmapSource>;
}
impl IWICBitmapFrameDecode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameDecode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapFrameDecode_Vtbl {
        unsafe extern "system" fn GetMetadataQueryReader<Impl: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataQueryReader() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadataqueryreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContexts<Impl: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut ::windows::core::RawPtr, pcactualcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontexts), ::core::mem::transmute_copy(&pcactualcount)).into()
        }
        unsafe extern "system" fn GetThumbnail<Impl: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *ppithumbnail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICBitmapSource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMetadataQueryReader: GetMetadataQueryReader::<Impl, IMPL_OFFSET>,
            GetColorContexts: GetColorContexts::<Impl, IMPL_OFFSET>,
            GetThumbnail: GetThumbnail::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapFrameDecode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICBitmapFrameEncode_Impl: Sized {
    fn Initialize(&mut self, piencoderoptions: &::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyBag2>) -> ::windows::core::Result<()>;
    fn SetSize(&mut self, uiwidth: u32, uiheight: u32) -> ::windows::core::Result<()>;
    fn SetResolution(&mut self, dpix: f64, dpiy: f64) -> ::windows::core::Result<()>;
    fn SetPixelFormat(&mut self, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetColorContexts(&mut self, ccount: u32, ppicolorcontext: *const ::core::option::Option<IWICColorContext>) -> ::windows::core::Result<()>;
    fn SetPalette(&mut self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn SetThumbnail(&mut self, pithumbnail: &::core::option::Option<IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn WritePixels(&mut self, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows::core::Result<()>;
    fn WriteSource(&mut self, pibitmapsource: &::core::option::Option<IWICBitmapSource>, prc: *const WICRect) -> ::windows::core::Result<()>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
    fn GetMetadataQueryWriter(&mut self) -> ::windows::core::Result<IWICMetadataQueryWriter>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICBitmapFrameEncode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapFrameEncode_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piencoderoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&piencoderoptions)).into()
        }
        unsafe extern "system" fn SetSize<Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight)).into()
        }
        unsafe extern "system" fn SetResolution<Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResolution(::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn SetPixelFormat<Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPixelFormat(::core::mem::transmute_copy(&ppixelformat)).into()
        }
        unsafe extern "system" fn SetColorContexts<Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorContexts(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontext)).into()
        }
        unsafe extern "system" fn SetPalette<Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn SetThumbnail<Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pithumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(::core::mem::transmute(&pithumbnail)).into()
        }
        unsafe extern "system" fn WritePixels<Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WritePixels(::core::mem::transmute_copy(&linecount), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbpixels)).into()
        }
        unsafe extern "system" fn WriteSource<Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, prc: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteSource(::core::mem::transmute(&pibitmapsource), ::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn Commit<Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Impl: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataQueryWriter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadataquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            SetResolution: SetResolution::<Impl, IMPL_OFFSET>,
            SetPixelFormat: SetPixelFormat::<Impl, IMPL_OFFSET>,
            SetColorContexts: SetColorContexts::<Impl, IMPL_OFFSET>,
            SetPalette: SetPalette::<Impl, IMPL_OFFSET>,
            SetThumbnail: SetThumbnail::<Impl, IMPL_OFFSET>,
            WritePixels: WritePixels::<Impl, IMPL_OFFSET>,
            WriteSource: WriteSource::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapFrameEncode as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapLock_Impl: Sized {
    fn GetSize(&mut self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()>;
    fn GetStride(&mut self) -> ::windows::core::Result<u32>;
    fn GetDataPointer(&mut self, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetPixelFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl IWICBitmapLock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapLock_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapLock_Vtbl {
        unsafe extern "system" fn GetSize<Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSize(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetStride<Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbstride: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStride() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbstride = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPointer<Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataPointer(::core::mem::transmute_copy(&pcbbuffersize), ::core::mem::transmute_copy(&ppbdata)).into()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *ppixelformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSize: GetSize::<Impl, IMPL_OFFSET>,
            GetStride: GetStride::<Impl, IMPL_OFFSET>,
            GetDataPointer: GetDataPointer::<Impl, IMPL_OFFSET>,
            GetPixelFormat: GetPixelFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapLock as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapScaler_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&mut self, pisource: &::core::option::Option<IWICBitmapSource>, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows::core::Result<()>;
}
impl IWICBitmapScaler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapScaler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapScaler_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapScaler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pisource), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&mode)).into()
        }
        Self { base: IWICBitmapSource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapScaler as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapSource_Impl: Sized {
    fn GetSize(&mut self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()>;
    fn GetPixelFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetResolution(&mut self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()>;
    fn CopyPalette(&mut self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn CopyPixels(&mut self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()>;
}
impl IWICBitmapSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapSource_Vtbl {
        unsafe extern "system" fn GetSize<Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSize(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *ppixelformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolution<Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResolution(::core::mem::transmute_copy(&pdpix), ::core::mem::transmute_copy(&pdpiy)).into()
        }
        unsafe extern "system" fn CopyPalette<Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn CopyPixels<Impl: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyPixels(::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSize: GetSize::<Impl, IMPL_OFFSET>,
            GetPixelFormat: GetPixelFormat::<Impl, IMPL_OFFSET>,
            GetResolution: GetResolution::<Impl, IMPL_OFFSET>,
            CopyPalette: CopyPalette::<Impl, IMPL_OFFSET>,
            CopyPixels: CopyPixels::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapSourceTransform_Impl: Sized {
    fn CopyPixels(&mut self, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows::core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()>;
    fn GetClosestSize(&mut self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()>;
    fn GetClosestPixelFormat(&mut self, pguiddstformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DoesSupportTransform(&mut self, dsttransform: WICBitmapTransformOptions) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapSourceTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSourceTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapSourceTransform_Vtbl {
        unsafe extern "system" fn CopyPixels<Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows::core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyPixels(::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pguiddstformat), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&nstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        unsafe extern "system" fn GetClosestSize<Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClosestSize(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetClosestPixelFormat<Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguiddstformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClosestPixelFormat(::core::mem::transmute_copy(&pguiddstformat)).into()
        }
        unsafe extern "system" fn DoesSupportTransform<Impl: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dsttransform: WICBitmapTransformOptions, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportTransform(::core::mem::transmute_copy(&dsttransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfissupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CopyPixels: CopyPixels::<Impl, IMPL_OFFSET>,
            GetClosestSize: GetClosestSize::<Impl, IMPL_OFFSET>,
            GetClosestPixelFormat: GetClosestPixelFormat::<Impl, IMPL_OFFSET>,
            DoesSupportTransform: DoesSupportTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapSourceTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICColorContext_Impl: Sized {
    fn InitializeFromFilename(&mut self, wzfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn InitializeFromMemory(&mut self, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::Result<()>;
    fn InitializeFromExifColorSpace(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn GetType(&mut self) -> ::windows::core::Result<WICColorContextType>;
    fn GetProfileBytes(&mut self, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetExifColorSpace(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICColorContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICColorContext_Vtbl {
        unsafe extern "system" fn InitializeFromFilename<Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromFilename(::core::mem::transmute_copy(&wzfilename)).into()
        }
        unsafe extern "system" fn InitializeFromMemory<Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromMemory(::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&cbbuffersize)).into()
        }
        unsafe extern "system" fn InitializeFromExifColorSpace<Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromExifColorSpace(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetType<Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WICColorContextType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileBytes<Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProfileBytes(::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn GetExifColorSpace<Impl: IWICColorContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExifColorSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitializeFromFilename: InitializeFromFilename::<Impl, IMPL_OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Impl, IMPL_OFFSET>,
            InitializeFromExifColorSpace: InitializeFromExifColorSpace::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetProfileBytes: GetProfileBytes::<Impl, IMPL_OFFSET>,
            GetExifColorSpace: GetExifColorSpace::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICColorContext as ::windows::core::Interface>::IID
    }
}
pub trait IWICColorTransform_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&mut self, pibitmapsource: &::core::option::Option<IWICBitmapSource>, picontextsource: &::core::option::Option<IWICColorContext>, picontextdest: &::core::option::Option<IWICColorContext>, pixelfmtdest: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IWICColorTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICColorTransform_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWICColorTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, picontextsource: ::windows::core::RawPtr, picontextdest: ::windows::core::RawPtr, pixelfmtdest: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pibitmapsource), ::core::mem::transmute(&picontextsource), ::core::mem::transmute(&picontextdest), ::core::mem::transmute_copy(&pixelfmtdest)).into()
        }
        Self { base: IWICBitmapSource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICColorTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICComponentFactory_Impl: Sized + IWICImagingFactory_Impl {
    fn CreateMetadataReader(&mut self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<IWICMetadataReader>;
    fn CreateMetadataReaderFromContainer(&mut self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<IWICMetadataReader>;
    fn CreateMetadataWriter(&mut self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwmetadataoptions: u32) -> ::windows::core::Result<IWICMetadataWriter>;
    fn CreateMetadataWriterFromReader(&mut self, pireader: &::core::option::Option<IWICMetadataReader>, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataWriter>;
    fn CreateQueryReaderFromBlockReader(&mut self, piblockreader: &::core::option::Option<IWICMetadataBlockReader>) -> ::windows::core::Result<IWICMetadataQueryReader>;
    fn CreateQueryWriterFromBlockWriter(&mut self, piblockwriter: &::core::option::Option<IWICMetadataBlockWriter>) -> ::windows::core::Result<IWICMetadataQueryWriter>;
    fn CreateEncoderPropertyBag(&mut self, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICComponentFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICComponentFactory_Vtbl {
        unsafe extern "system" fn CreateMetadataReader<Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: ::windows::core::RawPtr, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMetadataReader(::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppireader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataReaderFromContainer<Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: ::windows::core::RawPtr, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMetadataReaderFromContainer(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppireader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataWriter<Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwmetadataoptions: u32, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMetadataWriter(::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwmetadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataWriterFromReader<Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pireader: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMetadataWriterFromReader(::core::mem::transmute(&pireader), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryReaderFromBlockReader<Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piblockreader: ::windows::core::RawPtr, ppiqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQueryReaderFromBlockReader(::core::mem::transmute(&piblockreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiqueryreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromBlockWriter<Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piblockwriter: ::windows::core::RawPtr, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQueryWriterFromBlockWriter(::core::mem::transmute(&piblockwriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncoderPropertyBag<Impl: IWICComponentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32, ppipropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncoderPropertyBag(::core::mem::transmute_copy(&ppropoptions), ::core::mem::transmute_copy(&ccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppipropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICImagingFactory_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateMetadataReader: CreateMetadataReader::<Impl, IMPL_OFFSET>,
            CreateMetadataReaderFromContainer: CreateMetadataReaderFromContainer::<Impl, IMPL_OFFSET>,
            CreateMetadataWriter: CreateMetadataWriter::<Impl, IMPL_OFFSET>,
            CreateMetadataWriterFromReader: CreateMetadataWriterFromReader::<Impl, IMPL_OFFSET>,
            CreateQueryReaderFromBlockReader: CreateQueryReaderFromBlockReader::<Impl, IMPL_OFFSET>,
            CreateQueryWriterFromBlockWriter: CreateQueryWriterFromBlockWriter::<Impl, IMPL_OFFSET>,
            CreateEncoderPropertyBag: CreateEncoderPropertyBag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICComponentFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICComponentInfo_Impl: Sized {
    fn GetComponentType(&mut self) -> ::windows::core::Result<WICComponentType>;
    fn GetCLSID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetSigningStatus(&mut self) -> ::windows::core::Result<u32>;
    fn GetAuthor(&mut self, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetVendorGUID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetVersion(&mut self, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetSpecVersion(&mut self, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetFriendlyName(&mut self, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICComponentInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICComponentInfo_Vtbl {
        unsafe extern "system" fn GetComponentType<Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCLSID<Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningStatus<Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthor<Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAuthor(::core::mem::transmute_copy(&cchauthor), ::core::mem::transmute_copy(&wzauthor), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetVendorGUID<Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVendorGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidvendor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVersion(::core::mem::transmute_copy(&cchversion), ::core::mem::transmute_copy(&wzversion), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetSpecVersion<Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSpecVersion(::core::mem::transmute_copy(&cchspecversion), ::core::mem::transmute_copy(&wzspecversion), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetFriendlyName<Impl: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFriendlyName(::core::mem::transmute_copy(&cchfriendlyname), ::core::mem::transmute_copy(&wzfriendlyname), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetComponentType: GetComponentType::<Impl, IMPL_OFFSET>,
            GetCLSID: GetCLSID::<Impl, IMPL_OFFSET>,
            GetSigningStatus: GetSigningStatus::<Impl, IMPL_OFFSET>,
            GetAuthor: GetAuthor::<Impl, IMPL_OFFSET>,
            GetVendorGUID: GetVendorGUID::<Impl, IMPL_OFFSET>,
            GetVersion: GetVersion::<Impl, IMPL_OFFSET>,
            GetSpecVersion: GetSpecVersion::<Impl, IMPL_OFFSET>,
            GetFriendlyName: GetFriendlyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICComponentInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsDecoder_Impl: Sized {
    fn GetParameters(&mut self) -> ::windows::core::Result<WICDdsParameters>;
    fn GetFrame(&mut self, arrayindex: u32, miplevel: u32, sliceindex: u32) -> ::windows::core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsDecoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsDecoder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICDdsDecoder_Vtbl {
        unsafe extern "system" fn GetParameters<Impl: IWICDdsDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrame<Impl: IWICDdsDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arrayindex: u32, miplevel: u32, sliceindex: u32, ppibitmapframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrame(::core::mem::transmute_copy(&arrayindex), ::core::mem::transmute_copy(&miplevel), ::core::mem::transmute_copy(&sliceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetParameters: GetParameters::<Impl, IMPL_OFFSET>,
            GetFrame: GetFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDdsDecoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsEncoder_Impl: Sized {
    fn SetParameters(&mut self, pparameters: *const WICDdsParameters) -> ::windows::core::Result<()>;
    fn GetParameters(&mut self) -> ::windows::core::Result<WICDdsParameters>;
    fn CreateNewFrame(&mut self, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsEncoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsEncoder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICDdsEncoder_Vtbl {
        unsafe extern "system" fn SetParameters<Impl: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn GetParameters<Impl: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNewFrame<Impl: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut ::windows::core::RawPtr, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateNewFrame(::core::mem::transmute_copy(&ppiframeencode), ::core::mem::transmute_copy(&parrayindex), ::core::mem::transmute_copy(&pmiplevel), ::core::mem::transmute_copy(&psliceindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetParameters: SetParameters::<Impl, IMPL_OFFSET>,
            GetParameters: GetParameters::<Impl, IMPL_OFFSET>,
            CreateNewFrame: CreateNewFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDdsEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsFrameDecode_Impl: Sized {
    fn GetSizeInBlocks(&mut self, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows::core::Result<()>;
    fn GetFormatInfo(&mut self) -> ::windows::core::Result<WICDdsFormatInfo>;
    fn CopyBlocks(&mut self, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsFrameDecode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsFrameDecode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICDdsFrameDecode_Vtbl {
        unsafe extern "system" fn GetSizeInBlocks<Impl: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSizeInBlocks(::core::mem::transmute_copy(&pwidthinblocks), ::core::mem::transmute_copy(&pheightinblocks)).into()
        }
        unsafe extern "system" fn GetFormatInfo<Impl: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatinfo: *mut WICDdsFormatInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pformatinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyBlocks<Impl: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyBlocks(::core::mem::transmute_copy(&prcboundsinblocks), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSizeInBlocks: GetSizeInBlocks::<Impl, IMPL_OFFSET>,
            GetFormatInfo: GetFormatInfo::<Impl, IMPL_OFFSET>,
            CopyBlocks: CopyBlocks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDdsFrameDecode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICDevelopRaw_Impl: Sized + IWICBitmapSource_Impl + IWICBitmapFrameDecode_Impl {
    fn QueryRawCapabilitiesInfo(&mut self, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows::core::Result<()>;
    fn LoadParameterSet(&mut self, parameterset: WICRawParameterSet) -> ::windows::core::Result<()>;
    fn GetCurrentParameterSet(&mut self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2>;
    fn SetExposureCompensation(&mut self, ev: f64) -> ::windows::core::Result<()>;
    fn GetExposureCompensation(&mut self) -> ::windows::core::Result<f64>;
    fn SetWhitePointRGB(&mut self, red: u32, green: u32, blue: u32) -> ::windows::core::Result<()>;
    fn GetWhitePointRGB(&mut self, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows::core::Result<()>;
    fn SetNamedWhitePoint(&mut self, whitepoint: WICNamedWhitePoint) -> ::windows::core::Result<()>;
    fn GetNamedWhitePoint(&mut self) -> ::windows::core::Result<WICNamedWhitePoint>;
    fn SetWhitePointKelvin(&mut self, whitepointkelvin: u32) -> ::windows::core::Result<()>;
    fn GetWhitePointKelvin(&mut self) -> ::windows::core::Result<u32>;
    fn GetKelvinRangeInfo(&mut self, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows::core::Result<()>;
    fn SetContrast(&mut self, contrast: f64) -> ::windows::core::Result<()>;
    fn GetContrast(&mut self) -> ::windows::core::Result<f64>;
    fn SetGamma(&mut self, gamma: f64) -> ::windows::core::Result<()>;
    fn GetGamma(&mut self) -> ::windows::core::Result<f64>;
    fn SetSharpness(&mut self, sharpness: f64) -> ::windows::core::Result<()>;
    fn GetSharpness(&mut self) -> ::windows::core::Result<f64>;
    fn SetSaturation(&mut self, saturation: f64) -> ::windows::core::Result<()>;
    fn GetSaturation(&mut self) -> ::windows::core::Result<f64>;
    fn SetTint(&mut self, tint: f64) -> ::windows::core::Result<()>;
    fn GetTint(&mut self) -> ::windows::core::Result<f64>;
    fn SetNoiseReduction(&mut self, noisereduction: f64) -> ::windows::core::Result<()>;
    fn GetNoiseReduction(&mut self) -> ::windows::core::Result<f64>;
    fn SetDestinationColorContext(&mut self, pcolorcontext: &::core::option::Option<IWICColorContext>) -> ::windows::core::Result<()>;
    fn SetToneCurve(&mut self, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows::core::Result<()>;
    fn GetToneCurve(&mut self, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn SetRotation(&mut self, rotation: f64) -> ::windows::core::Result<()>;
    fn GetRotation(&mut self) -> ::windows::core::Result<f64>;
    fn SetRenderMode(&mut self, rendermode: WICRawRenderMode) -> ::windows::core::Result<()>;
    fn GetRenderMode(&mut self) -> ::windows::core::Result<WICRawRenderMode>;
    fn SetNotificationCallback(&mut self, pcallback: &::core::option::Option<IWICDevelopRawNotificationCallback>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICDevelopRaw_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRaw_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICDevelopRaw_Vtbl {
        unsafe extern "system" fn QueryRawCapabilitiesInfo<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryRawCapabilitiesInfo(::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn LoadParameterSet<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterset: WICRawParameterSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadParameterSet(::core::mem::transmute_copy(&parameterset)).into()
        }
        unsafe extern "system" fn GetCurrentParameterSet<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcurrentparameterset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentParameterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcurrentparameterset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExposureCompensation<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ev: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExposureCompensation(::core::mem::transmute_copy(&ev)).into()
        }
        unsafe extern "system" fn GetExposureCompensation<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pev: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExposureCompensation() {
                ::core::result::Result::Ok(ok__) => {
                    *pev = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointRGB<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, red: u32, green: u32, blue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePointRGB(::core::mem::transmute_copy(&red), ::core::mem::transmute_copy(&green), ::core::mem::transmute_copy(&blue)).into()
        }
        unsafe extern "system" fn GetWhitePointRGB<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWhitePointRGB(::core::mem::transmute_copy(&pred), ::core::mem::transmute_copy(&pgreen), ::core::mem::transmute_copy(&pblue)).into()
        }
        unsafe extern "system" fn SetNamedWhitePoint<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepoint: WICNamedWhitePoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamedWhitePoint(::core::mem::transmute_copy(&whitepoint)).into()
        }
        unsafe extern "system" fn GetNamedWhitePoint<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwhitepoint: *mut WICNamedWhitePoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedWhitePoint() {
                ::core::result::Result::Ok(ok__) => {
                    *pwhitepoint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointKelvin<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointkelvin: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWhitePointKelvin(::core::mem::transmute_copy(&whitepointkelvin)).into()
        }
        unsafe extern "system" fn GetWhitePointKelvin<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwhitepointkelvin: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWhitePointKelvin() {
                ::core::result::Result::Ok(ok__) => {
                    *pwhitepointkelvin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKelvinRangeInfo<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKelvinRangeInfo(::core::mem::transmute_copy(&pminkelvintemp), ::core::mem::transmute_copy(&pmaxkelvintemp), ::core::mem::transmute_copy(&pkelvintempstepvalue)).into()
        }
        unsafe extern "system" fn SetContrast<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contrast: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContrast(::core::mem::transmute_copy(&contrast)).into()
        }
        unsafe extern "system" fn GetContrast<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrast: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontrast = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGamma<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGamma(::core::mem::transmute_copy(&gamma)).into()
        }
        unsafe extern "system" fn GetGamma<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgamma: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGamma() {
                ::core::result::Result::Ok(ok__) => {
                    *pgamma = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharpness<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharpness: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharpness(::core::mem::transmute_copy(&sharpness)).into()
        }
        unsafe extern "system" fn GetSharpness<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharpness: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSharpness() {
                ::core::result::Result::Ok(ok__) => {
                    *psharpness = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaturation<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, saturation: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSaturation(::core::mem::transmute_copy(&saturation)).into()
        }
        unsafe extern "system" fn GetSaturation<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psaturation: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSaturation() {
                ::core::result::Result::Ok(ok__) => {
                    *psaturation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTint<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tint: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTint(::core::mem::transmute_copy(&tint)).into()
        }
        unsafe extern "system" fn GetTint<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptint: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTint() {
                ::core::result::Result::Ok(ok__) => {
                    *ptint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoiseReduction<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noisereduction: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoiseReduction(::core::mem::transmute_copy(&noisereduction)).into()
        }
        unsafe extern "system" fn GetNoiseReduction<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnoisereduction: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNoiseReduction() {
                ::core::result::Result::Ok(ok__) => {
                    *pnoisereduction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationColorContext<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationColorContext(::core::mem::transmute(&pcolorcontext)).into()
        }
        unsafe extern "system" fn SetToneCurve<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToneCurve(::core::mem::transmute_copy(&cbtonecurvesize), ::core::mem::transmute_copy(&ptonecurve)).into()
        }
        unsafe extern "system" fn GetToneCurve<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetToneCurve(::core::mem::transmute_copy(&cbtonecurvebuffersize), ::core::mem::transmute_copy(&ptonecurve), ::core::mem::transmute_copy(&pcbactualtonecurvebuffersize)).into()
        }
        unsafe extern "system" fn SetRotation<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotation(::core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn GetRotation<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protation: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *protation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderMode<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendermode: WICRawRenderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRenderMode(::core::mem::transmute_copy(&rendermode)).into()
        }
        unsafe extern "system" fn GetRenderMode<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendermode: *mut WICRawRenderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRenderMode() {
                ::core::result::Result::Ok(ok__) => {
                    *prendermode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationCallback<Impl: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationCallback(::core::mem::transmute(&pcallback)).into()
        }
        Self {
            base: IWICBitmapFrameDecode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueryRawCapabilitiesInfo: QueryRawCapabilitiesInfo::<Impl, IMPL_OFFSET>,
            LoadParameterSet: LoadParameterSet::<Impl, IMPL_OFFSET>,
            GetCurrentParameterSet: GetCurrentParameterSet::<Impl, IMPL_OFFSET>,
            SetExposureCompensation: SetExposureCompensation::<Impl, IMPL_OFFSET>,
            GetExposureCompensation: GetExposureCompensation::<Impl, IMPL_OFFSET>,
            SetWhitePointRGB: SetWhitePointRGB::<Impl, IMPL_OFFSET>,
            GetWhitePointRGB: GetWhitePointRGB::<Impl, IMPL_OFFSET>,
            SetNamedWhitePoint: SetNamedWhitePoint::<Impl, IMPL_OFFSET>,
            GetNamedWhitePoint: GetNamedWhitePoint::<Impl, IMPL_OFFSET>,
            SetWhitePointKelvin: SetWhitePointKelvin::<Impl, IMPL_OFFSET>,
            GetWhitePointKelvin: GetWhitePointKelvin::<Impl, IMPL_OFFSET>,
            GetKelvinRangeInfo: GetKelvinRangeInfo::<Impl, IMPL_OFFSET>,
            SetContrast: SetContrast::<Impl, IMPL_OFFSET>,
            GetContrast: GetContrast::<Impl, IMPL_OFFSET>,
            SetGamma: SetGamma::<Impl, IMPL_OFFSET>,
            GetGamma: GetGamma::<Impl, IMPL_OFFSET>,
            SetSharpness: SetSharpness::<Impl, IMPL_OFFSET>,
            GetSharpness: GetSharpness::<Impl, IMPL_OFFSET>,
            SetSaturation: SetSaturation::<Impl, IMPL_OFFSET>,
            GetSaturation: GetSaturation::<Impl, IMPL_OFFSET>,
            SetTint: SetTint::<Impl, IMPL_OFFSET>,
            GetTint: GetTint::<Impl, IMPL_OFFSET>,
            SetNoiseReduction: SetNoiseReduction::<Impl, IMPL_OFFSET>,
            GetNoiseReduction: GetNoiseReduction::<Impl, IMPL_OFFSET>,
            SetDestinationColorContext: SetDestinationColorContext::<Impl, IMPL_OFFSET>,
            SetToneCurve: SetToneCurve::<Impl, IMPL_OFFSET>,
            GetToneCurve: GetToneCurve::<Impl, IMPL_OFFSET>,
            SetRotation: SetRotation::<Impl, IMPL_OFFSET>,
            GetRotation: GetRotation::<Impl, IMPL_OFFSET>,
            SetRenderMode: SetRenderMode::<Impl, IMPL_OFFSET>,
            GetRenderMode: GetRenderMode::<Impl, IMPL_OFFSET>,
            SetNotificationCallback: SetNotificationCallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDevelopRaw as ::windows::core::Interface>::IID
    }
}
pub trait IWICDevelopRawNotificationCallback_Impl: Sized {
    fn Notify(&mut self, notificationmask: u32) -> ::windows::core::Result<()>;
}
impl IWICDevelopRawNotificationCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRawNotificationCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICDevelopRawNotificationCallback_Vtbl {
        unsafe extern "system" fn Notify<Impl: IWICDevelopRawNotificationCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&notificationmask)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Notify: Notify::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDevelopRawNotificationCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICEnumMetadataItem_Impl: Sized {
    fn Next(&mut self, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IWICEnumMetadataItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICEnumMetadataItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICEnumMetadataItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICEnumMetadataItem_Vtbl {
        unsafe extern "system" fn Next<Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgeltschema), ::core::mem::transmute_copy(&rgeltid), ::core::mem::transmute_copy(&rgeltvalue), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadataitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienummetadataitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICEnumMetadataItem as ::windows::core::Interface>::IID
    }
}
pub trait IWICFastMetadataEncoder_Impl: Sized {
    fn Commit(&mut self) -> ::windows::core::Result<()>;
    fn GetMetadataQueryWriter(&mut self) -> ::windows::core::Result<IWICMetadataQueryWriter>;
}
impl IWICFastMetadataEncoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFastMetadataEncoder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICFastMetadataEncoder_Vtbl {
        unsafe extern "system" fn Commit<Impl: IWICFastMetadataEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Impl: IWICFastMetadataEncoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataQueryWriter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadataquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Commit: Commit::<Impl, IMPL_OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICFastMetadataEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICFormatConverter_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&mut self, pisource: &::core::option::Option<IWICBitmapSource>, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: &::core::option::Option<IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::Result<()>;
    fn CanConvert(&mut self, srcpixelformat: *const ::windows::core::GUID, dstpixelformat: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICFormatConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICFormatConverter_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWICFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: ::windows::core::RawPtr, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pisource), ::core::mem::transmute_copy(&dstformat), ::core::mem::transmute_copy(&dither), ::core::mem::transmute(&pipalette), ::core::mem::transmute_copy(&alphathresholdpercent), ::core::mem::transmute_copy(&palettetranslate)).into()
        }
        unsafe extern "system" fn CanConvert<Impl: IWICFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcpixelformat: *const ::windows::core::GUID, dstpixelformat: *const ::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanConvert(::core::mem::transmute_copy(&srcpixelformat), ::core::mem::transmute_copy(&dstpixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfcanconvert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICBitmapSource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            CanConvert: CanConvert::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICFormatConverter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICFormatConverterInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetPixelFormats(&mut self, cformats: u32, ppixelformatguids: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::Result<()>;
    fn CreateInstance(&mut self) -> ::windows::core::Result<IWICFormatConverter>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICFormatConverterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverterInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICFormatConverterInfo_Vtbl {
        unsafe extern "system" fn GetPixelFormats<Impl: IWICFormatConverterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, ppixelformatguids: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPixelFormats(::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&ppixelformatguids), ::core::mem::transmute_copy(&pcactual)).into()
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICFormatConverterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiconverter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICComponentInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPixelFormats: GetPixelFormats::<Impl, IMPL_OFFSET>,
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICFormatConverterInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactory_Impl: Sized {
    fn CreateDecoderFromFilename(&mut self, wzfilename: super::super::Foundation::PWSTR, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromStream(&mut self, pistream: &::core::option::Option<super::super::System::Com::IStream>, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromFileHandle(&mut self, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>;
    fn CreateComponentInfo(&mut self, clsidcomponent: *const ::windows::core::GUID) -> ::windows::core::Result<IWICComponentInfo>;
    fn CreateDecoder(&mut self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapDecoder>;
    fn CreateEncoder(&mut self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapEncoder>;
    fn CreatePalette(&mut self) -> ::windows::core::Result<IWICPalette>;
    fn CreateFormatConverter(&mut self) -> ::windows::core::Result<IWICFormatConverter>;
    fn CreateBitmapScaler(&mut self) -> ::windows::core::Result<IWICBitmapScaler>;
    fn CreateBitmapClipper(&mut self) -> ::windows::core::Result<IWICBitmapClipper>;
    fn CreateBitmapFlipRotator(&mut self) -> ::windows::core::Result<IWICBitmapFlipRotator>;
    fn CreateStream(&mut self) -> ::windows::core::Result<IWICStream>;
    fn CreateColorContext(&mut self) -> ::windows::core::Result<IWICColorContext>;
    fn CreateColorTransformer(&mut self) -> ::windows::core::Result<IWICColorTransform>;
    fn CreateBitmap(&mut self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap>;
    fn CreateBitmapFromSource(&mut self, pibitmapsource: &::core::option::Option<IWICBitmapSource>, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap>;
    fn CreateBitmapFromSourceRect(&mut self, pibitmapsource: &::core::option::Option<IWICBitmapSource>, x: u32, y: u32, width: u32, height: u32) -> ::windows::core::Result<IWICBitmap>;
    fn CreateBitmapFromMemory(&mut self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8) -> ::windows::core::Result<IWICBitmap>;
    fn CreateBitmapFromHBITMAP(&mut self, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption) -> ::windows::core::Result<IWICBitmap>;
    fn CreateBitmapFromHICON(&mut self, hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::Result<IWICBitmap>;
    fn CreateComponentEnumerator(&mut self, componenttypes: u32, options: u32) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown>;
    fn CreateFastMetadataEncoderFromDecoder(&mut self, pidecoder: &::core::option::Option<IWICBitmapDecoder>) -> ::windows::core::Result<IWICFastMetadataEncoder>;
    fn CreateFastMetadataEncoderFromFrameDecode(&mut self, piframedecoder: &::core::option::Option<IWICBitmapFrameDecode>) -> ::windows::core::Result<IWICFastMetadataEncoder>;
    fn CreateQueryWriter(&mut self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter>;
    fn CreateQueryWriterFromReader(&mut self, piqueryreader: &::core::option::Option<IWICMetadataQueryReader>, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICImagingFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICImagingFactory_Vtbl {
        unsafe extern "system" fn CreateDecoderFromFilename<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecoderFromFilename(::core::mem::transmute_copy(&wzfilename), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoderFromStream<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecoderFromStream(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoderFromFileHandle<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecoderFromFileHandle(::core::mem::transmute_copy(&hfile), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComponentInfo<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcomponent: *const ::windows::core::GUID, ppiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateComponentInfo(::core::mem::transmute_copy(&clsidcomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoder<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecoder(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppidecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncoder<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncoder(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiencoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePalette<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipalette: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePalette() {
                ::core::result::Result::Ok(ok__) => {
                    *ppipalette = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFormatConverter<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiformatconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFormatConverter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiformatconverter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapScaler<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapscaler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapScaler() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapscaler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapClipper<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapclipper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapClipper() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapclipper = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFlipRotator<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapfliprotator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFlipRotator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmapfliprotator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStream<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwicstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwicstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContext<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiccolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwiccolorcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorTransformer<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiccolortransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorTransformer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwiccolortransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmap<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmap(::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromSource<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromSource(::core::mem::transmute(&pibitmapsource), ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromSourceRect<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromSourceRect(::core::mem::transmute(&pibitmapsource), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromMemory<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromMemory(::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromHBITMAP<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromHBITMAP(::core::mem::transmute_copy(&hbitmap), ::core::mem::transmute_copy(&hpalette), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromHICON<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromHICON(::core::mem::transmute_copy(&hicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppibitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComponentEnumerator<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, componenttypes: u32, options: u32, ppienumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateComponentEnumerator(::core::mem::transmute_copy(&componenttypes), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppienumunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromDecoder<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFastMetadataEncoderFromDecoder(::core::mem::transmute(&pidecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifastencoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromFrameDecode<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piframedecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFastMetadataEncoderFromFrameDecode(::core::mem::transmute(&piframedecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifastencoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriter<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQueryWriter(::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromReader<Impl: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piqueryreader: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQueryWriterFromReader(::core::mem::transmute(&piqueryreader), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiquerywriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateDecoderFromFilename: CreateDecoderFromFilename::<Impl, IMPL_OFFSET>,
            CreateDecoderFromStream: CreateDecoderFromStream::<Impl, IMPL_OFFSET>,
            CreateDecoderFromFileHandle: CreateDecoderFromFileHandle::<Impl, IMPL_OFFSET>,
            CreateComponentInfo: CreateComponentInfo::<Impl, IMPL_OFFSET>,
            CreateDecoder: CreateDecoder::<Impl, IMPL_OFFSET>,
            CreateEncoder: CreateEncoder::<Impl, IMPL_OFFSET>,
            CreatePalette: CreatePalette::<Impl, IMPL_OFFSET>,
            CreateFormatConverter: CreateFormatConverter::<Impl, IMPL_OFFSET>,
            CreateBitmapScaler: CreateBitmapScaler::<Impl, IMPL_OFFSET>,
            CreateBitmapClipper: CreateBitmapClipper::<Impl, IMPL_OFFSET>,
            CreateBitmapFlipRotator: CreateBitmapFlipRotator::<Impl, IMPL_OFFSET>,
            CreateStream: CreateStream::<Impl, IMPL_OFFSET>,
            CreateColorContext: CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorTransformer: CreateColorTransformer::<Impl, IMPL_OFFSET>,
            CreateBitmap: CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromSource: CreateBitmapFromSource::<Impl, IMPL_OFFSET>,
            CreateBitmapFromSourceRect: CreateBitmapFromSourceRect::<Impl, IMPL_OFFSET>,
            CreateBitmapFromMemory: CreateBitmapFromMemory::<Impl, IMPL_OFFSET>,
            CreateBitmapFromHBITMAP: CreateBitmapFromHBITMAP::<Impl, IMPL_OFFSET>,
            CreateBitmapFromHICON: CreateBitmapFromHICON::<Impl, IMPL_OFFSET>,
            CreateComponentEnumerator: CreateComponentEnumerator::<Impl, IMPL_OFFSET>,
            CreateFastMetadataEncoderFromDecoder: CreateFastMetadataEncoderFromDecoder::<Impl, IMPL_OFFSET>,
            CreateFastMetadataEncoderFromFrameDecode: CreateFastMetadataEncoderFromFrameDecode::<Impl, IMPL_OFFSET>,
            CreateQueryWriter: CreateQueryWriter::<Impl, IMPL_OFFSET>,
            CreateQueryWriterFromReader: CreateQueryWriterFromReader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICImagingFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IWICJpegFrameDecode_Impl: Sized {
    fn DoesSupportIndexing(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIndexing(&mut self, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows::core::Result<()>;
    fn ClearIndexing(&mut self) -> ::windows::core::Result<()>;
    fn GetAcHuffmanTable(&mut self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE>;
    fn GetDcHuffmanTable(&mut self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE>;
    fn GetQuantizationTable(&mut self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE>;
    fn GetFrameHeader(&mut self) -> ::windows::core::Result<WICJpegFrameHeader>;
    fn GetScanHeader(&mut self, scanindex: u32) -> ::windows::core::Result<WICJpegScanHeader>;
    fn CopyScan(&mut self, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows::core::Result<()>;
    fn CopyMinimalStream(&mut self, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IWICJpegFrameDecode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICJpegFrameDecode_Vtbl {
        unsafe extern "system" fn DoesSupportIndexing<Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfindexingsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportIndexing() {
                ::core::result::Result::Ok(ok__) => {
                    *pfindexingsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexing<Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndexing(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&horizontalintervalsize)).into()
        }
        unsafe extern "system" fn ClearIndexing<Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearIndexing().into()
        }
        unsafe extern "system" fn GetAcHuffmanTable<Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pachuffmantable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDcHuffmanTable<Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdchuffmantable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuantizationTable<Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQuantizationTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pquantizationtable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameHeader<Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframeheader: *mut WICJpegFrameHeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *pframeheader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScanHeader<Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScanHeader(::core::mem::transmute_copy(&scanindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pscanheader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyScan<Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyScan(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&scanoffset), ::core::mem::transmute_copy(&cbscandata), ::core::mem::transmute_copy(&pbscandata), ::core::mem::transmute_copy(&pcbscandataactual)).into()
        }
        unsafe extern "system" fn CopyMinimalStream<Impl: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyMinimalStream(::core::mem::transmute_copy(&streamoffset), ::core::mem::transmute_copy(&cbstreamdata), ::core::mem::transmute_copy(&pbstreamdata), ::core::mem::transmute_copy(&pcbstreamdataactual)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DoesSupportIndexing: DoesSupportIndexing::<Impl, IMPL_OFFSET>,
            SetIndexing: SetIndexing::<Impl, IMPL_OFFSET>,
            ClearIndexing: ClearIndexing::<Impl, IMPL_OFFSET>,
            GetAcHuffmanTable: GetAcHuffmanTable::<Impl, IMPL_OFFSET>,
            GetDcHuffmanTable: GetDcHuffmanTable::<Impl, IMPL_OFFSET>,
            GetQuantizationTable: GetQuantizationTable::<Impl, IMPL_OFFSET>,
            GetFrameHeader: GetFrameHeader::<Impl, IMPL_OFFSET>,
            GetScanHeader: GetScanHeader::<Impl, IMPL_OFFSET>,
            CopyScan: CopyScan::<Impl, IMPL_OFFSET>,
            CopyMinimalStream: CopyMinimalStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICJpegFrameDecode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICJpegFrameEncode_Impl: Sized {
    fn GetAcHuffmanTable(&mut self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE>;
    fn GetDcHuffmanTable(&mut self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE>;
    fn GetQuantizationTable(&mut self, scanindex: u32, tableindex: u32) -> ::windows::core::Result<super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE>;
    fn WriteScan(&mut self, cbscandata: u32, pbscandata: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICJpegFrameEncode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameEncode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICJpegFrameEncode_Vtbl {
        unsafe extern "system" fn GetAcHuffmanTable<Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pachuffmantable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDcHuffmanTable<Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDcHuffmanTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdchuffmantable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuantizationTable<Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQuantizationTable(::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pquantizationtable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteScan<Impl: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbscandata: u32, pbscandata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteScan(::core::mem::transmute_copy(&cbscandata), ::core::mem::transmute_copy(&pbscandata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAcHuffmanTable: GetAcHuffmanTable::<Impl, IMPL_OFFSET>,
            GetDcHuffmanTable: GetDcHuffmanTable::<Impl, IMPL_OFFSET>,
            GetQuantizationTable: GetQuantizationTable::<Impl, IMPL_OFFSET>,
            WriteScan: WriteScan::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICJpegFrameEncode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockReader_Impl: Sized {
    fn GetContainerFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetReaderByIndex(&mut self, nindex: u32) -> ::windows::core::Result<IWICMetadataReader>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataBlockReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataBlockReader_Vtbl {
        unsafe extern "system" fn GetContainerFormat<Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidcontainerformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReaderByIndex<Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatareader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReaderByIndex(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadatareader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IWICMetadataBlockReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienummetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetReaderByIndex: GetReaderByIndex::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataBlockReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockWriter_Impl: Sized + IWICMetadataBlockReader_Impl {
    fn InitializeFromBlockReader(&mut self, pimdblockreader: &::core::option::Option<IWICMetadataBlockReader>) -> ::windows::core::Result<()>;
    fn GetWriterByIndex(&mut self, nindex: u32) -> ::windows::core::Result<IWICMetadataWriter>;
    fn AddWriter(&mut self, pimetadatawriter: &::core::option::Option<IWICMetadataWriter>) -> ::windows::core::Result<()>;
    fn SetWriterByIndex(&mut self, nindex: u32, pimetadatawriter: &::core::option::Option<IWICMetadataWriter>) -> ::windows::core::Result<()>;
    fn RemoveWriterByIndex(&mut self, nindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataBlockWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataBlockWriter_Vtbl {
        unsafe extern "system" fn InitializeFromBlockReader<Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimdblockreader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromBlockReader(::core::mem::transmute(&pimdblockreader)).into()
        }
        unsafe extern "system" fn GetWriterByIndex<Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatawriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWriterByIndex(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppimetadatawriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWriter<Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimetadatawriter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddWriter(::core::mem::transmute(&pimetadatawriter)).into()
        }
        unsafe extern "system" fn SetWriterByIndex<Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pimetadatawriter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriterByIndex(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&pimetadatawriter)).into()
        }
        unsafe extern "system" fn RemoveWriterByIndex<Impl: IWICMetadataBlockWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveWriterByIndex(::core::mem::transmute_copy(&nindex)).into()
        }
        Self {
            base: IWICMetadataBlockReader_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromBlockReader: InitializeFromBlockReader::<Impl, IMPL_OFFSET>,
            GetWriterByIndex: GetWriterByIndex::<Impl, IMPL_OFFSET>,
            AddWriter: AddWriter::<Impl, IMPL_OFFSET>,
            SetWriterByIndex: SetWriterByIndex::<Impl, IMPL_OFFSET>,
            RemoveWriterByIndex: RemoveWriterByIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataBlockWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICMetadataHandlerInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetMetadataFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetContainerFormats(&mut self, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceManufacturer(&mut self, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceModels(&mut self, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::Result<()>;
    fn DoesRequireFullStream(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportPadding(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DoesRequireFixedSize(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICMetadataHandlerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataHandlerInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataHandlerInfo_Vtbl {
        unsafe extern "system" fn GetMetadataFormat<Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidmetadataformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerFormats<Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContainerFormats(::core::mem::transmute_copy(&ccontainerformats), ::core::mem::transmute_copy(&pguidcontainerformats), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceManufacturer(::core::mem::transmute_copy(&cchdevicemanufacturer), ::core::mem::transmute_copy(&wzdevicemanufacturer), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceModels<Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceModels(::core::mem::transmute_copy(&cchdevicemodels), ::core::mem::transmute_copy(&wzdevicemodels), ::core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn DoesRequireFullStream<Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequiresfullstream: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesRequireFullStream() {
                ::core::result::Result::Ok(ok__) => {
                    *pfrequiresfullstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportPadding<Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportspadding: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportspadding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesRequireFixedSize<Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffixedsize: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesRequireFixedSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pffixedsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICComponentInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMetadataFormat: GetMetadataFormat::<Impl, IMPL_OFFSET>,
            GetContainerFormats: GetContainerFormats::<Impl, IMPL_OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Impl, IMPL_OFFSET>,
            GetDeviceModels: GetDeviceModels::<Impl, IMPL_OFFSET>,
            DoesRequireFullStream: DoesRequireFullStream::<Impl, IMPL_OFFSET>,
            DoesSupportPadding: DoesSupportPadding::<Impl, IMPL_OFFSET>,
            DoesRequireFixedSize: DoesRequireFixedSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataHandlerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataQueryReader_Impl: Sized {
    fn GetContainerFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetLocation(&mut self, cchmaxlength: u32, wznamespace: super::super::Foundation::PWSTR, pcchactuallength: *mut u32) -> ::windows::core::Result<()>;
    fn GetMetadataByName(&mut self, wzname: super::super::Foundation::PWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataQueryReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataQueryReader_Vtbl {
        unsafe extern "system" fn GetContainerFormat<Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidcontainerformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocation<Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmaxlength: u32, wznamespace: super::super::Foundation::PWSTR, pcchactuallength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocation(::core::mem::transmute_copy(&cchmaxlength), ::core::mem::transmute_copy(&wznamespace), ::core::mem::transmute_copy(&pcchactuallength)).into()
        }
        unsafe extern "system" fn GetMetadataByName<Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMetadataByName(::core::mem::transmute_copy(&wzname), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienumstring: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienumstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Impl, IMPL_OFFSET>,
            GetLocation: GetLocation::<Impl, IMPL_OFFSET>,
            GetMetadataByName: GetMetadataByName::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataQueryReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataQueryWriter_Impl: Sized + IWICMetadataQueryReader_Impl {
    fn SetMetadataByName(&mut self, wzname: super::super::Foundation::PWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn RemoveMetadataByName(&mut self, wzname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataQueryWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataQueryWriter_Vtbl {
        unsafe extern "system" fn SetMetadataByName<Impl: IWICMetadataQueryWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMetadataByName(::core::mem::transmute_copy(&wzname), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn RemoveMetadataByName<Impl: IWICMetadataQueryWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMetadataByName(::core::mem::transmute_copy(&wzname)).into()
        }
        Self {
            base: IWICMetadataQueryReader_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMetadataByName: SetMetadataByName::<Impl, IMPL_OFFSET>,
            RemoveMetadataByName: RemoveMetadataByName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataQueryWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataReader_Impl: Sized {
    fn GetMetadataFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetMetadataHandlerInfo(&mut self) -> ::windows::core::Result<IWICMetadataHandlerInfo>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetValueByIndex(&mut self, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetValue(&mut self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<IWICEnumMetadataItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataReader_Vtbl {
        unsafe extern "system" fn GetMetadataFormat<Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidmetadataformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataHandlerInfo<Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppihandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataHandlerInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppihandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueByIndex<Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValueByIndex(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetValue<Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IWICMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienummetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetMetadataFormat: GetMetadataFormat::<Impl, IMPL_OFFSET>,
            GetMetadataHandlerInfo: GetMetadataHandlerInfo::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetValueByIndex: GetValueByIndex::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICMetadataReaderInfo_Impl: Sized + IWICComponentInfo_Impl + IWICMetadataHandlerInfo_Impl {
    fn GetPatterns(&mut self, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows::core::Result<()>;
    fn MatchesPattern(&mut self, guidcontainerformat: *const ::windows::core::GUID, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CreateInstance(&mut self) -> ::windows::core::Result<IWICMetadataReader>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICMetadataReaderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReaderInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataReaderInfo_Vtbl {
        unsafe extern "system" fn GetPatterns<Impl: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPatterns(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&ppattern), ::core::mem::transmute_copy(&pccount), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn MatchesPattern<Impl: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pistream: ::windows::core::RawPtr, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchesPattern(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmatches = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICMetadataReaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *ppireader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICMetadataHandlerInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPatterns: GetPatterns::<Impl, IMPL_OFFSET>,
            MatchesPattern: MatchesPattern::<Impl, IMPL_OFFSET>,
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataReaderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataWriter_Impl: Sized + IWICMetadataReader_Impl {
    fn SetValue(&mut self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn SetValueByIndex(&mut self, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn RemoveValue(&mut self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn RemoveValueByIndex(&mut self, nindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataWriter_Vtbl {
        unsafe extern "system" fn SetValue<Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn SetValueByIndex<Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValueByIndex(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn RemoveValue<Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveValue(::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid)).into()
        }
        unsafe extern "system" fn RemoveValueByIndex<Impl: IWICMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveValueByIndex(::core::mem::transmute_copy(&nindex)).into()
        }
        Self {
            base: IWICMetadataReader_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            SetValueByIndex: SetValueByIndex::<Impl, IMPL_OFFSET>,
            RemoveValue: RemoveValue::<Impl, IMPL_OFFSET>,
            RemoveValueByIndex: RemoveValueByIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICMetadataWriterInfo_Impl: Sized + IWICComponentInfo_Impl + IWICMetadataHandlerInfo_Impl {
    fn GetHeader(&mut self, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows::core::Result<()>;
    fn CreateInstance(&mut self) -> ::windows::core::Result<IWICMetadataWriter>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICMetadataWriterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriterInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataWriterInfo_Vtbl {
        unsafe extern "system" fn GetHeader<Impl: IWICMetadataWriterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHeader(::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pheader), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICMetadataWriterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICMetadataHandlerInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetHeader: GetHeader::<Impl, IMPL_OFFSET>,
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataWriterInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPalette_Impl: Sized {
    fn InitializePredefined(&mut self, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InitializeCustom(&mut self, pcolors: *const u32, ccount: u32) -> ::windows::core::Result<()>;
    fn InitializeFromBitmap(&mut self, pisurface: &::core::option::Option<IWICBitmapSource>, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InitializeFromPalette(&mut self, pipalette: &::core::option::Option<IWICPalette>) -> ::windows::core::Result<()>;
    fn GetType(&mut self) -> ::windows::core::Result<WICBitmapPaletteType>;
    fn GetColorCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetColors(&mut self, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows::core::Result<()>;
    fn IsBlackWhite(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsGrayscale(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn HasAlpha(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPalette_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPalette_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPalette_Vtbl {
        unsafe extern "system" fn InitializePredefined<Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializePredefined(::core::mem::transmute_copy(&epalettetype), ::core::mem::transmute_copy(&faddtransparentcolor)).into()
        }
        unsafe extern "system" fn InitializeCustom<Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolors: *const u32, ccount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeCustom(::core::mem::transmute_copy(&pcolors), ::core::mem::transmute_copy(&ccount)).into()
        }
        unsafe extern "system" fn InitializeFromBitmap<Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisurface: ::windows::core::RawPtr, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromBitmap(::core::mem::transmute(&pisurface), ::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&faddtransparentcolor)).into()
        }
        unsafe extern "system" fn InitializeFromPalette<Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromPalette(::core::mem::transmute(&pipalette)).into()
        }
        unsafe extern "system" fn GetType<Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pepalettetype: *mut WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pepalettetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorCount<Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColors<Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColors(::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&pcolors), ::core::mem::transmute_copy(&pcactualcolors)).into()
        }
        unsafe extern "system" fn IsBlackWhite<Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisblackwhite: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBlackWhite() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisblackwhite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGrayscale<Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisgrayscale: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGrayscale() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisgrayscale = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasAlpha<Impl: IWICPalette_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfhasalpha: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasAlpha() {
                ::core::result::Result::Ok(ok__) => {
                    *pfhasalpha = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitializePredefined: InitializePredefined::<Impl, IMPL_OFFSET>,
            InitializeCustom: InitializeCustom::<Impl, IMPL_OFFSET>,
            InitializeFromBitmap: InitializeFromBitmap::<Impl, IMPL_OFFSET>,
            InitializeFromPalette: InitializeFromPalette::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetColorCount: GetColorCount::<Impl, IMPL_OFFSET>,
            GetColors: GetColors::<Impl, IMPL_OFFSET>,
            IsBlackWhite: IsBlackWhite::<Impl, IMPL_OFFSET>,
            IsGrayscale: IsGrayscale::<Impl, IMPL_OFFSET>,
            HasAlpha: HasAlpha::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPalette as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICPersistStream_Impl: Sized + super::super::System::Com::IPersist_Impl + super::super::System::Com::IPersistStream_Impl {
    fn LoadEx(&mut self, pistream: &::core::option::Option<super::super::System::Com::IStream>, pguidpreferredvendor: *const ::windows::core::GUID, dwpersistoptions: u32) -> ::windows::core::Result<()>;
    fn SaveEx(&mut self, pistream: &::core::option::Option<super::super::System::Com::IStream>, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICPersistStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPersistStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPersistStream_Vtbl {
        unsafe extern "system" fn LoadEx<Impl: IWICPersistStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pguidpreferredvendor: *const ::windows::core::GUID, dwpersistoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadEx(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&pguidpreferredvendor), ::core::mem::transmute_copy(&dwpersistoptions)).into()
        }
        unsafe extern "system" fn SaveEx<Impl: IWICPersistStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveEx(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&dwpersistoptions), ::core::mem::transmute_copy(&fcleardirty)).into()
        }
        Self {
            base: super::super::System::Com::IPersistStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LoadEx: LoadEx::<Impl, IMPL_OFFSET>,
            SaveEx: SaveEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPersistStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPixelFormatInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetFormatGUID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetColorContext(&mut self) -> ::windows::core::Result<IWICColorContext>;
    fn GetBitsPerPixel(&mut self) -> ::windows::core::Result<u32>;
    fn GetChannelCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetChannelMask(&mut self, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPixelFormatInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPixelFormatInfo_Vtbl {
        unsafe extern "system" fn GetFormatGUID<Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContext<Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppicolorcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitsPerPixel<Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puibitsperpixel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitsPerPixel() {
                ::core::result::Result::Ok(ok__) => {
                    *puibitsperpixel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelCount<Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puichannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puichannelcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelMask<Impl: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChannelMask(::core::mem::transmute_copy(&uichannelindex), ::core::mem::transmute_copy(&cbmaskbuffer), ::core::mem::transmute_copy(&pbmaskbuffer), ::core::mem::transmute_copy(&pcbactual)).into()
        }
        Self {
            base: IWICComponentInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFormatGUID: GetFormatGUID::<Impl, IMPL_OFFSET>,
            GetColorContext: GetColorContext::<Impl, IMPL_OFFSET>,
            GetBitsPerPixel: GetBitsPerPixel::<Impl, IMPL_OFFSET>,
            GetChannelCount: GetChannelCount::<Impl, IMPL_OFFSET>,
            GetChannelMask: GetChannelMask::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPixelFormatInfo2_Impl: Sized + IWICComponentInfo_Impl + IWICPixelFormatInfo_Impl {
    fn SupportsTransparency(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetNumericRepresentation(&mut self) -> ::windows::core::Result<WICPixelFormatNumericRepresentation>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPixelFormatInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPixelFormatInfo2_Vtbl {
        unsafe extern "system" fn SupportsTransparency<Impl: IWICPixelFormatInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportstransparency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsTransparency() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupportstransparency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericRepresentation<Impl: IWICPixelFormatInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumericrepresentation: *mut WICPixelFormatNumericRepresentation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumericRepresentation() {
                ::core::result::Result::Ok(ok__) => {
                    *pnumericrepresentation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICPixelFormatInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SupportsTransparency: SupportsTransparency::<Impl, IMPL_OFFSET>,
            GetNumericRepresentation: GetNumericRepresentation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo2 as ::windows::core::Interface>::IID
    }
}
pub trait IWICPlanarBitmapFrameEncode_Impl: Sized {
    fn WritePixels(&mut self, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::Result<()>;
    fn WriteSource(&mut self, ppplanes: *const ::core::option::Option<IWICBitmapSource>, cplanes: u32, prcsource: *const WICRect) -> ::windows::core::Result<()>;
}
impl IWICPlanarBitmapFrameEncode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapFrameEncode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPlanarBitmapFrameEncode_Vtbl {
        unsafe extern "system" fn WritePixels<Impl: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WritePixels(::core::mem::transmute_copy(&linecount), ::core::mem::transmute_copy(&pplanes), ::core::mem::transmute_copy(&cplanes)).into()
        }
        unsafe extern "system" fn WriteSource<Impl: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplanes: *const ::windows::core::RawPtr, cplanes: u32, prcsource: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteSource(::core::mem::transmute_copy(&ppplanes), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&prcsource)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            WritePixels: WritePixels::<Impl, IMPL_OFFSET>,
            WriteSource: WriteSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPlanarBitmapFrameEncode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPlanarBitmapSourceTransform_Impl: Sized {
    fn DoesSupportTransform(&mut self, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows::core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CopyPixels(&mut self, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPlanarBitmapSourceTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapSourceTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPlanarBitmapSourceTransform_Vtbl {
        unsafe extern "system" fn DoesSupportTransform<Impl: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows::core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoesSupportTransform(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&dstplanaroptions), ::core::mem::transmute_copy(&pguiddstformats), ::core::mem::transmute_copy(&pplanedescriptions), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&pfissupported)).into()
        }
        unsafe extern "system" fn CopyPixels<Impl: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyPixels(::core::mem::transmute_copy(&prcsource), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&dstplanaroptions), ::core::mem::transmute_copy(&pdstplanes), ::core::mem::transmute_copy(&cplanes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DoesSupportTransform: DoesSupportTransform::<Impl, IMPL_OFFSET>,
            CopyPixels: CopyPixels::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPlanarBitmapSourceTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPlanarFormatConverter_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&mut self, ppplanes: *const ::core::option::Option<IWICBitmapSource>, cplanes: u32, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: &::core::option::Option<IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::Result<()>;
    fn CanConvert(&mut self, psrcpixelformats: *const ::windows::core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPlanarFormatConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarFormatConverter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPlanarFormatConverter_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWICPlanarFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplanes: *const ::windows::core::RawPtr, cplanes: u32, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: ::windows::core::RawPtr, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&ppplanes), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&dstformat), ::core::mem::transmute_copy(&dither), ::core::mem::transmute(&pipalette), ::core::mem::transmute_copy(&alphathresholdpercent), ::core::mem::transmute_copy(&palettetranslate)).into()
        }
        unsafe extern "system" fn CanConvert<Impl: IWICPlanarFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcpixelformats: *const ::windows::core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanConvert(::core::mem::transmute_copy(&psrcpixelformats), ::core::mem::transmute_copy(&csrcplanes), ::core::mem::transmute_copy(&dstpixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfcanconvert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWICBitmapSource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            CanConvert: CanConvert::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPlanarFormatConverter as ::windows::core::Interface>::IID
    }
}
pub trait IWICProgressCallback_Impl: Sized {
    fn Notify(&mut self, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::Result<()>;
}
impl IWICProgressCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICProgressCallback_Vtbl {
        unsafe extern "system" fn Notify<Impl: IWICProgressCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&uframenum), ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&dblprogress)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Notify: Notify::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICProgressCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWICProgressiveLevelControl_Impl: Sized {
    fn GetLevelCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetCurrentLevel(&mut self) -> ::windows::core::Result<u32>;
    fn SetCurrentLevel(&mut self, nlevel: u32) -> ::windows::core::Result<()>;
}
impl IWICProgressiveLevelControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressiveLevelControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICProgressiveLevelControl_Vtbl {
        unsafe extern "system" fn GetLevelCount<Impl: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pclevels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLevel<Impl: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pnlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentLevel<Impl: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrentLevel(::core::mem::transmute_copy(&nlevel)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLevelCount: GetLevelCount::<Impl, IMPL_OFFSET>,
            GetCurrentLevel: GetCurrentLevel::<Impl, IMPL_OFFSET>,
            SetCurrentLevel: SetCurrentLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICProgressiveLevelControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICStream_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn InitializeFromIStream(&mut self, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn InitializeFromFilename(&mut self, wzfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> ::windows::core::Result<()>;
    fn InitializeFromMemory(&mut self, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::Result<()>;
    fn InitializeFromIStreamRegion(&mut self, pistream: &::core::option::Option<super::super::System::Com::IStream>, uloffset: u64, ulmaxsize: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICStream_Vtbl {
        unsafe extern "system" fn InitializeFromIStream<Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromIStream(::core::mem::transmute(&pistream)).into()
        }
        unsafe extern "system" fn InitializeFromFilename<Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromFilename(::core::mem::transmute_copy(&wzfilename), ::core::mem::transmute_copy(&dwdesiredaccess)).into()
        }
        unsafe extern "system" fn InitializeFromMemory<Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromMemory(::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&cbbuffersize)).into()
        }
        unsafe extern "system" fn InitializeFromIStreamRegion<Impl: IWICStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, uloffset: u64, ulmaxsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromIStreamRegion(::core::mem::transmute(&pistream), ::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&ulmaxsize)).into()
        }
        Self {
            base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromIStream: InitializeFromIStream::<Impl, IMPL_OFFSET>,
            InitializeFromFilename: InitializeFromFilename::<Impl, IMPL_OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Impl, IMPL_OFFSET>,
            InitializeFromIStreamRegion: InitializeFromIStreamRegion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICStreamProvider_Impl: Sized {
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn GetPersistOptions(&mut self) -> ::windows::core::Result<u32>;
    fn GetPreferredVendorGUID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RefreshStream(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWICStreamProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICStreamProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICStreamProvider_Vtbl {
        unsafe extern "system" fn GetStream<Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppistream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPersistOptions<Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpersistoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPersistOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwpersistoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredVendorGUID<Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidpreferredvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredVendorGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidpreferredvendor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshStream<Impl: IWICStreamProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshStream().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            GetPersistOptions: GetPersistOptions::<Impl, IMPL_OFFSET>,
            GetPreferredVendorGUID: GetPreferredVendorGUID::<Impl, IMPL_OFFSET>,
            RefreshStream: RefreshStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICStreamProvider as ::windows::core::Interface>::IID
    }
}
