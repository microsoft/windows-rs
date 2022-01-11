pub trait IWICBitmapImpl: Sized + IWICBitmapSourceImpl {
    fn Lock();
    fn SetPalette();
    fn SetResolution();
}
impl IWICBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapVtbl {
        unsafe extern "system" fn Lock<Impl: IWICBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prclock: *const WICRect, flags: u32, ppilock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPalette<Impl: IWICBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResolution<Impl: IWICBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetPixelFormat::<Impl, IMPL_OFFSET>, GetResolution::<Impl, IMPL_OFFSET>, CopyPalette::<Impl, IMPL_OFFSET>, CopyPixels::<Impl, IMPL_OFFSET>, Lock::<Impl, IMPL_OFFSET>, SetPalette::<Impl, IMPL_OFFSET>, SetResolution::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmap as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapClipperImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
impl IWICBitmapClipperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapClipperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapClipperVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapClipperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, prc: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetPixelFormat::<Impl, IMPL_OFFSET>, GetResolution::<Impl, IMPL_OFFSET>, CopyPalette::<Impl, IMPL_OFFSET>, CopyPixels::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapClipper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapCodecInfoImpl: Sized + IWICComponentInfoImpl {
    fn GetContainerFormat();
    fn GetPixelFormats();
    fn GetColorManagementVersion();
    fn GetDeviceManufacturer();
    fn GetDeviceModels();
    fn GetMimeTypes();
    fn GetFileExtensions();
    fn DoesSupportAnimation();
    fn DoesSupportChromakey();
    fn DoesSupportLossless();
    fn DoesSupportMultiframe();
    fn MatchesMimeType();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapCodecInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapCodecInfoVtbl {
        unsafe extern "system" fn GetContainerFormat<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelFormats<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorManagementVersion<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceModels<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMimeTypes<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmimetypes: u32, wzmimetypes: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileExtensions<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchfileextensions: u32, wzfileextensions: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoesSupportAnimation<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportanimation: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoesSupportChromakey<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportchromakey: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoesSupportLossless<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportlossless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoesSupportMultiframe<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportmultiframe: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MatchesMimeType<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzmimetype: super::super::Foundation::PWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetCLSID::<Impl, IMPL_OFFSET>,
            GetSigningStatus::<Impl, IMPL_OFFSET>,
            GetAuthor::<Impl, IMPL_OFFSET>,
            GetVendorGUID::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            GetSpecVersion::<Impl, IMPL_OFFSET>,
            GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetContainerFormat::<Impl, IMPL_OFFSET>,
            GetPixelFormats::<Impl, IMPL_OFFSET>,
            GetColorManagementVersion::<Impl, IMPL_OFFSET>,
            GetDeviceManufacturer::<Impl, IMPL_OFFSET>,
            GetDeviceModels::<Impl, IMPL_OFFSET>,
            GetMimeTypes::<Impl, IMPL_OFFSET>,
            GetFileExtensions::<Impl, IMPL_OFFSET>,
            DoesSupportAnimation::<Impl, IMPL_OFFSET>,
            DoesSupportChromakey::<Impl, IMPL_OFFSET>,
            DoesSupportLossless::<Impl, IMPL_OFFSET>,
            DoesSupportMultiframe::<Impl, IMPL_OFFSET>,
            MatchesMimeType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapCodecInfo as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapCodecProgressNotificationImpl: Sized {
    fn RegisterProgressNotification();
}
impl IWICBitmapCodecProgressNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecProgressNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapCodecProgressNotificationVtbl {
        unsafe extern "system" fn RegisterProgressNotification<Impl: IWICBitmapCodecProgressNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfnprogressnotification: ::windows::core::RawPtr, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterProgressNotification::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapCodecProgressNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICBitmapDecoderImpl: Sized {
    fn QueryCapability();
    fn Initialize();
    fn GetContainerFormat();
    fn GetDecoderInfo();
    fn CopyPalette();
    fn GetMetadataQueryReader();
    fn GetPreview();
    fn GetColorContexts();
    fn GetThumbnail();
    fn GetFrameCount();
    fn GetFrame();
}
#[cfg(feature = "Win32_System_Com")]
impl IWICBitmapDecoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapDecoderVtbl {
        unsafe extern "system" fn QueryCapability<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pdwcapability: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, cacheoptions: WICDecodeOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContainerFormat<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDecoderInfo<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidecoderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyPalette<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetadataQueryReader<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreview<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorContexts<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut ::windows::core::RawPtr, pcactualcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetThumbnail<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrameCount<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrame<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppibitmapframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            QueryCapability::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            GetContainerFormat::<Impl, IMPL_OFFSET>,
            GetDecoderInfo::<Impl, IMPL_OFFSET>,
            CopyPalette::<Impl, IMPL_OFFSET>,
            GetMetadataQueryReader::<Impl, IMPL_OFFSET>,
            GetPreview::<Impl, IMPL_OFFSET>,
            GetColorContexts::<Impl, IMPL_OFFSET>,
            GetThumbnail::<Impl, IMPL_OFFSET>,
            GetFrameCount::<Impl, IMPL_OFFSET>,
            GetFrame::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapDecoder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICBitmapDecoderInfoImpl: Sized + IWICBitmapCodecInfoImpl + IWICComponentInfoImpl {
    fn GetPatterns();
    fn MatchesPattern();
    fn CreateInstance();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICBitmapDecoderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoderInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapDecoderInfoVtbl {
        unsafe extern "system" fn GetPatterns<Impl: IWICBitmapDecoderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MatchesPattern<Impl: IWICBitmapDecoderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICBitmapDecoderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapdecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetCLSID::<Impl, IMPL_OFFSET>,
            GetSigningStatus::<Impl, IMPL_OFFSET>,
            GetAuthor::<Impl, IMPL_OFFSET>,
            GetVendorGUID::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            GetSpecVersion::<Impl, IMPL_OFFSET>,
            GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetContainerFormat::<Impl, IMPL_OFFSET>,
            GetPixelFormats::<Impl, IMPL_OFFSET>,
            GetColorManagementVersion::<Impl, IMPL_OFFSET>,
            GetDeviceManufacturer::<Impl, IMPL_OFFSET>,
            GetDeviceModels::<Impl, IMPL_OFFSET>,
            GetMimeTypes::<Impl, IMPL_OFFSET>,
            GetFileExtensions::<Impl, IMPL_OFFSET>,
            DoesSupportAnimation::<Impl, IMPL_OFFSET>,
            DoesSupportChromakey::<Impl, IMPL_OFFSET>,
            DoesSupportLossless::<Impl, IMPL_OFFSET>,
            DoesSupportMultiframe::<Impl, IMPL_OFFSET>,
            MatchesMimeType::<Impl, IMPL_OFFSET>,
            GetPatterns::<Impl, IMPL_OFFSET>,
            MatchesPattern::<Impl, IMPL_OFFSET>,
            CreateInstance::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapDecoderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICBitmapEncoderImpl: Sized {
    fn Initialize();
    fn GetContainerFormat();
    fn GetEncoderInfo();
    fn SetColorContexts();
    fn SetPalette();
    fn SetThumbnail();
    fn SetPreview();
    fn CreateNewFrame();
    fn Commit();
    fn GetMetadataQueryWriter();
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICBitmapEncoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapEncoderVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, cacheoption: WICBitmapEncoderCacheOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContainerFormat<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEncoderInfo<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiencoderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorContexts<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPalette<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetThumbnail<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pithumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreview<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipreview: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateNewFrame<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut ::windows::core::RawPtr, ppiencoderoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            GetContainerFormat::<Impl, IMPL_OFFSET>,
            GetEncoderInfo::<Impl, IMPL_OFFSET>,
            SetColorContexts::<Impl, IMPL_OFFSET>,
            SetPalette::<Impl, IMPL_OFFSET>,
            SetThumbnail::<Impl, IMPL_OFFSET>,
            SetPreview::<Impl, IMPL_OFFSET>,
            CreateNewFrame::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            GetMetadataQueryWriter::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapEncoderInfoImpl: Sized + IWICBitmapCodecInfoImpl + IWICComponentInfoImpl {
    fn CreateInstance();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapEncoderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoderInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapEncoderInfoVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IWICBitmapEncoderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetCLSID::<Impl, IMPL_OFFSET>,
            GetSigningStatus::<Impl, IMPL_OFFSET>,
            GetAuthor::<Impl, IMPL_OFFSET>,
            GetVendorGUID::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            GetSpecVersion::<Impl, IMPL_OFFSET>,
            GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetContainerFormat::<Impl, IMPL_OFFSET>,
            GetPixelFormats::<Impl, IMPL_OFFSET>,
            GetColorManagementVersion::<Impl, IMPL_OFFSET>,
            GetDeviceManufacturer::<Impl, IMPL_OFFSET>,
            GetDeviceModels::<Impl, IMPL_OFFSET>,
            GetMimeTypes::<Impl, IMPL_OFFSET>,
            GetFileExtensions::<Impl, IMPL_OFFSET>,
            DoesSupportAnimation::<Impl, IMPL_OFFSET>,
            DoesSupportChromakey::<Impl, IMPL_OFFSET>,
            DoesSupportLossless::<Impl, IMPL_OFFSET>,
            DoesSupportMultiframe::<Impl, IMPL_OFFSET>,
            MatchesMimeType::<Impl, IMPL_OFFSET>,
            CreateInstance::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapEncoderInfo as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapFlipRotatorImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
impl IWICBitmapFlipRotatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFlipRotatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapFlipRotatorVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapFlipRotatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, options: WICBitmapTransformOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetPixelFormat::<Impl, IMPL_OFFSET>, GetResolution::<Impl, IMPL_OFFSET>, CopyPalette::<Impl, IMPL_OFFSET>, CopyPixels::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapFlipRotator as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapFrameDecodeImpl: Sized + IWICBitmapSourceImpl {
    fn GetMetadataQueryReader();
    fn GetColorContexts();
    fn GetThumbnail();
}
impl IWICBitmapFrameDecodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameDecodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapFrameDecodeVtbl {
        unsafe extern "system" fn GetMetadataQueryReader<Impl: IWICBitmapFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorContexts<Impl: IWICBitmapFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut ::windows::core::RawPtr, pcactualcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetThumbnail<Impl: IWICBitmapFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetPixelFormat::<Impl, IMPL_OFFSET>, GetResolution::<Impl, IMPL_OFFSET>, CopyPalette::<Impl, IMPL_OFFSET>, CopyPixels::<Impl, IMPL_OFFSET>, GetMetadataQueryReader::<Impl, IMPL_OFFSET>, GetColorContexts::<Impl, IMPL_OFFSET>, GetThumbnail::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapFrameDecode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICBitmapFrameEncodeImpl: Sized {
    fn Initialize();
    fn SetSize();
    fn SetResolution();
    fn SetPixelFormat();
    fn SetColorContexts();
    fn SetPalette();
    fn SetThumbnail();
    fn WritePixels();
    fn WriteSource();
    fn Commit();
    fn GetMetadataQueryWriter();
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICBitmapFrameEncodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapFrameEncodeVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piencoderoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSize<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResolution<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPixelFormat<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorContexts<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPalette<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetThumbnail<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pithumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WritePixels<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteSource<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, prc: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            SetSize::<Impl, IMPL_OFFSET>,
            SetResolution::<Impl, IMPL_OFFSET>,
            SetPixelFormat::<Impl, IMPL_OFFSET>,
            SetColorContexts::<Impl, IMPL_OFFSET>,
            SetPalette::<Impl, IMPL_OFFSET>,
            SetThumbnail::<Impl, IMPL_OFFSET>,
            WritePixels::<Impl, IMPL_OFFSET>,
            WriteSource::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            GetMetadataQueryWriter::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapFrameEncode as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapLockImpl: Sized {
    fn GetSize();
    fn GetStride();
    fn GetDataPointer();
    fn GetPixelFormat();
}
impl IWICBitmapLockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapLockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapLockVtbl {
        unsafe extern "system" fn GetSize<Impl: IWICBitmapLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStride<Impl: IWICBitmapLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbstride: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataPointer<Impl: IWICBitmapLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IWICBitmapLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetStride::<Impl, IMPL_OFFSET>, GetDataPointer::<Impl, IMPL_OFFSET>, GetPixelFormat::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapLock as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapScalerImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
impl IWICBitmapScalerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapScalerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapScalerVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapScalerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetPixelFormat::<Impl, IMPL_OFFSET>, GetResolution::<Impl, IMPL_OFFSET>, CopyPalette::<Impl, IMPL_OFFSET>, CopyPixels::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapScaler as ::windows::core::Interface>::IID
    }
}
pub trait IWICBitmapSourceImpl: Sized {
    fn GetSize();
    fn GetPixelFormat();
    fn GetResolution();
    fn CopyPalette();
    fn CopyPixels();
}
impl IWICBitmapSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapSourceVtbl {
        unsafe extern "system" fn GetSize<Impl: IWICBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IWICBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResolution<Impl: IWICBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyPalette<Impl: IWICBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyPixels<Impl: IWICBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetPixelFormat::<Impl, IMPL_OFFSET>, GetResolution::<Impl, IMPL_OFFSET>, CopyPalette::<Impl, IMPL_OFFSET>, CopyPixels::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapSourceTransformImpl: Sized {
    fn CopyPixels();
    fn GetClosestSize();
    fn GetClosestPixelFormat();
    fn DoesSupportTransform();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICBitmapSourceTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSourceTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICBitmapSourceTransformVtbl {
        unsafe extern "system" fn CopyPixels<Impl: IWICBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows::core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClosestSize<Impl: IWICBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClosestPixelFormat<Impl: IWICBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguiddstformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoesSupportTransform<Impl: IWICBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dsttransform: WICBitmapTransformOptions, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CopyPixels::<Impl, IMPL_OFFSET>, GetClosestSize::<Impl, IMPL_OFFSET>, GetClosestPixelFormat::<Impl, IMPL_OFFSET>, DoesSupportTransform::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICBitmapSourceTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICColorContextImpl: Sized {
    fn InitializeFromFilename();
    fn InitializeFromMemory();
    fn InitializeFromExifColorSpace();
    fn GetType();
    fn GetProfileBytes();
    fn GetExifColorSpace();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICColorContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICColorContextVtbl {
        unsafe extern "system" fn InitializeFromFilename<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromMemory<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromExifColorSpace<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WICColorContextType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProfileBytes<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExifColorSpace<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InitializeFromFilename::<Impl, IMPL_OFFSET>, InitializeFromMemory::<Impl, IMPL_OFFSET>, InitializeFromExifColorSpace::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, GetProfileBytes::<Impl, IMPL_OFFSET>, GetExifColorSpace::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICColorContext as ::windows::core::Interface>::IID
    }
}
pub trait IWICColorTransformImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
impl IWICColorTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICColorTransformVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICColorTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, picontextsource: ::windows::core::RawPtr, picontextdest: ::windows::core::RawPtr, pixelfmtdest: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetPixelFormat::<Impl, IMPL_OFFSET>, GetResolution::<Impl, IMPL_OFFSET>, CopyPalette::<Impl, IMPL_OFFSET>, CopyPixels::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICColorTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICComponentFactoryImpl: Sized + IWICImagingFactoryImpl {
    fn CreateMetadataReader();
    fn CreateMetadataReaderFromContainer();
    fn CreateMetadataWriter();
    fn CreateMetadataWriterFromReader();
    fn CreateQueryReaderFromBlockReader();
    fn CreateQueryWriterFromBlockWriter();
    fn CreateEncoderPropertyBag();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICComponentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICComponentFactoryVtbl {
        unsafe extern "system" fn CreateMetadataReader<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: ::windows::core::RawPtr, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMetadataReaderFromContainer<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: ::windows::core::RawPtr, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMetadataWriter<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwmetadataoptions: u32, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMetadataWriterFromReader<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pireader: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateQueryReaderFromBlockReader<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piblockreader: ::windows::core::RawPtr, ppiqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateQueryWriterFromBlockWriter<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piblockwriter: ::windows::core::RawPtr, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEncoderPropertyBag<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32, ppipropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateDecoderFromFilename::<Impl, IMPL_OFFSET>,
            CreateDecoderFromStream::<Impl, IMPL_OFFSET>,
            CreateDecoderFromFileHandle::<Impl, IMPL_OFFSET>,
            CreateComponentInfo::<Impl, IMPL_OFFSET>,
            CreateDecoder::<Impl, IMPL_OFFSET>,
            CreateEncoder::<Impl, IMPL_OFFSET>,
            CreatePalette::<Impl, IMPL_OFFSET>,
            CreateFormatConverter::<Impl, IMPL_OFFSET>,
            CreateBitmapScaler::<Impl, IMPL_OFFSET>,
            CreateBitmapClipper::<Impl, IMPL_OFFSET>,
            CreateBitmapFlipRotator::<Impl, IMPL_OFFSET>,
            CreateStream::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorTransformer::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromSource::<Impl, IMPL_OFFSET>,
            CreateBitmapFromSourceRect::<Impl, IMPL_OFFSET>,
            CreateBitmapFromMemory::<Impl, IMPL_OFFSET>,
            CreateBitmapFromHBITMAP::<Impl, IMPL_OFFSET>,
            CreateBitmapFromHICON::<Impl, IMPL_OFFSET>,
            CreateComponentEnumerator::<Impl, IMPL_OFFSET>,
            CreateFastMetadataEncoderFromDecoder::<Impl, IMPL_OFFSET>,
            CreateFastMetadataEncoderFromFrameDecode::<Impl, IMPL_OFFSET>,
            CreateQueryWriter::<Impl, IMPL_OFFSET>,
            CreateQueryWriterFromReader::<Impl, IMPL_OFFSET>,
            CreateMetadataReader::<Impl, IMPL_OFFSET>,
            CreateMetadataReaderFromContainer::<Impl, IMPL_OFFSET>,
            CreateMetadataWriter::<Impl, IMPL_OFFSET>,
            CreateMetadataWriterFromReader::<Impl, IMPL_OFFSET>,
            CreateQueryReaderFromBlockReader::<Impl, IMPL_OFFSET>,
            CreateQueryWriterFromBlockWriter::<Impl, IMPL_OFFSET>,
            CreateEncoderPropertyBag::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICComponentFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICComponentInfoImpl: Sized {
    fn GetComponentType();
    fn GetCLSID();
    fn GetSigningStatus();
    fn GetAuthor();
    fn GetVendorGUID();
    fn GetVersion();
    fn GetSpecVersion();
    fn GetFriendlyName();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICComponentInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICComponentInfoVtbl {
        unsafe extern "system" fn GetComponentType<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCLSID<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSigningStatus<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAuthor<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVendorGUID<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVersion<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpecVersion<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFriendlyName<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetComponentType::<Impl, IMPL_OFFSET>, GetCLSID::<Impl, IMPL_OFFSET>, GetSigningStatus::<Impl, IMPL_OFFSET>, GetAuthor::<Impl, IMPL_OFFSET>, GetVendorGUID::<Impl, IMPL_OFFSET>, GetVersion::<Impl, IMPL_OFFSET>, GetSpecVersion::<Impl, IMPL_OFFSET>, GetFriendlyName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICComponentInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsDecoderImpl: Sized {
    fn GetParameters();
    fn GetFrame();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsDecoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsDecoderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICDdsDecoderVtbl {
        unsafe extern "system" fn GetParameters<Impl: IWICDdsDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrame<Impl: IWICDdsDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arrayindex: u32, miplevel: u32, sliceindex: u32, ppibitmapframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetParameters::<Impl, IMPL_OFFSET>, GetFrame::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDdsDecoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsEncoderImpl: Sized {
    fn SetParameters();
    fn GetParameters();
    fn CreateNewFrame();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsEncoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsEncoderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICDdsEncoderVtbl {
        unsafe extern "system" fn SetParameters<Impl: IWICDdsEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParameters<Impl: IWICDdsEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateNewFrame<Impl: IWICDdsEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut ::windows::core::RawPtr, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetParameters::<Impl, IMPL_OFFSET>, GetParameters::<Impl, IMPL_OFFSET>, CreateNewFrame::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDdsEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsFrameDecodeImpl: Sized {
    fn GetSizeInBlocks();
    fn GetFormatInfo();
    fn CopyBlocks();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsFrameDecodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsFrameDecodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICDdsFrameDecodeVtbl {
        unsafe extern "system" fn GetSizeInBlocks<Impl: IWICDdsFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormatInfo<Impl: IWICDdsFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatinfo: *mut WICDdsFormatInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyBlocks<Impl: IWICDdsFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSizeInBlocks::<Impl, IMPL_OFFSET>, GetFormatInfo::<Impl, IMPL_OFFSET>, CopyBlocks::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDdsFrameDecode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICDevelopRawImpl: Sized + IWICBitmapFrameDecodeImpl + IWICBitmapSourceImpl {
    fn QueryRawCapabilitiesInfo();
    fn LoadParameterSet();
    fn GetCurrentParameterSet();
    fn SetExposureCompensation();
    fn GetExposureCompensation();
    fn SetWhitePointRGB();
    fn GetWhitePointRGB();
    fn SetNamedWhitePoint();
    fn GetNamedWhitePoint();
    fn SetWhitePointKelvin();
    fn GetWhitePointKelvin();
    fn GetKelvinRangeInfo();
    fn SetContrast();
    fn GetContrast();
    fn SetGamma();
    fn GetGamma();
    fn SetSharpness();
    fn GetSharpness();
    fn SetSaturation();
    fn GetSaturation();
    fn SetTint();
    fn GetTint();
    fn SetNoiseReduction();
    fn GetNoiseReduction();
    fn SetDestinationColorContext();
    fn SetToneCurve();
    fn GetToneCurve();
    fn SetRotation();
    fn GetRotation();
    fn SetRenderMode();
    fn GetRenderMode();
    fn SetNotificationCallback();
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICDevelopRawVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRawImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICDevelopRawVtbl {
        unsafe extern "system" fn QueryRawCapabilitiesInfo<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadParameterSet<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterset: WICRawParameterSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentParameterSet<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcurrentparameterset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExposureCompensation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ev: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExposureCompensation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pev: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWhitePointRGB<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, red: u32, green: u32, blue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWhitePointRGB<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNamedWhitePoint<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepoint: WICNamedWhitePoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNamedWhitePoint<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwhitepoint: *mut WICNamedWhitePoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWhitePointKelvin<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointkelvin: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWhitePointKelvin<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwhitepointkelvin: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKelvinRangeInfo<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContrast<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contrast: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContrast<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrast: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGamma<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGamma<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgamma: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSharpness<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharpness: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSharpness<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharpness: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSaturation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, saturation: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSaturation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psaturation: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTint<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tint: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTint<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptint: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNoiseReduction<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noisereduction: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNoiseReduction<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnoisereduction: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDestinationColorContext<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetToneCurve<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetToneCurve<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRotation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRotation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protation: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRenderMode<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendermode: WICRawRenderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRenderMode<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendermode: *mut WICRawRenderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotificationCallback<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            GetPixelFormat::<Impl, IMPL_OFFSET>,
            GetResolution::<Impl, IMPL_OFFSET>,
            CopyPalette::<Impl, IMPL_OFFSET>,
            CopyPixels::<Impl, IMPL_OFFSET>,
            GetMetadataQueryReader::<Impl, IMPL_OFFSET>,
            GetColorContexts::<Impl, IMPL_OFFSET>,
            GetThumbnail::<Impl, IMPL_OFFSET>,
            QueryRawCapabilitiesInfo::<Impl, IMPL_OFFSET>,
            LoadParameterSet::<Impl, IMPL_OFFSET>,
            GetCurrentParameterSet::<Impl, IMPL_OFFSET>,
            SetExposureCompensation::<Impl, IMPL_OFFSET>,
            GetExposureCompensation::<Impl, IMPL_OFFSET>,
            SetWhitePointRGB::<Impl, IMPL_OFFSET>,
            GetWhitePointRGB::<Impl, IMPL_OFFSET>,
            SetNamedWhitePoint::<Impl, IMPL_OFFSET>,
            GetNamedWhitePoint::<Impl, IMPL_OFFSET>,
            SetWhitePointKelvin::<Impl, IMPL_OFFSET>,
            GetWhitePointKelvin::<Impl, IMPL_OFFSET>,
            GetKelvinRangeInfo::<Impl, IMPL_OFFSET>,
            SetContrast::<Impl, IMPL_OFFSET>,
            GetContrast::<Impl, IMPL_OFFSET>,
            SetGamma::<Impl, IMPL_OFFSET>,
            GetGamma::<Impl, IMPL_OFFSET>,
            SetSharpness::<Impl, IMPL_OFFSET>,
            GetSharpness::<Impl, IMPL_OFFSET>,
            SetSaturation::<Impl, IMPL_OFFSET>,
            GetSaturation::<Impl, IMPL_OFFSET>,
            SetTint::<Impl, IMPL_OFFSET>,
            GetTint::<Impl, IMPL_OFFSET>,
            SetNoiseReduction::<Impl, IMPL_OFFSET>,
            GetNoiseReduction::<Impl, IMPL_OFFSET>,
            SetDestinationColorContext::<Impl, IMPL_OFFSET>,
            SetToneCurve::<Impl, IMPL_OFFSET>,
            GetToneCurve::<Impl, IMPL_OFFSET>,
            SetRotation::<Impl, IMPL_OFFSET>,
            GetRotation::<Impl, IMPL_OFFSET>,
            SetRenderMode::<Impl, IMPL_OFFSET>,
            GetRenderMode::<Impl, IMPL_OFFSET>,
            SetNotificationCallback::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDevelopRaw as ::windows::core::Interface>::IID
    }
}
pub trait IWICDevelopRawNotificationCallbackImpl: Sized {
    fn Notify();
}
impl IWICDevelopRawNotificationCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRawNotificationCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICDevelopRawNotificationCallbackVtbl {
        unsafe extern "system" fn Notify<Impl: IWICDevelopRawNotificationCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Notify::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICDevelopRawNotificationCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICEnumMetadataItemImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICEnumMetadataItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICEnumMetadataItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICEnumMetadataItemVtbl {
        unsafe extern "system" fn Next<Impl: IWICEnumMetadataItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IWICEnumMetadataItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IWICEnumMetadataItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IWICEnumMetadataItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadataitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICEnumMetadataItem as ::windows::core::Interface>::IID
    }
}
pub trait IWICFastMetadataEncoderImpl: Sized {
    fn Commit();
    fn GetMetadataQueryWriter();
}
impl IWICFastMetadataEncoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFastMetadataEncoderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICFastMetadataEncoderVtbl {
        unsafe extern "system" fn Commit<Impl: IWICFastMetadataEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Impl: IWICFastMetadataEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Commit::<Impl, IMPL_OFFSET>, GetMetadataQueryWriter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICFastMetadataEncoder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICFormatConverterImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
    fn CanConvert();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICFormatConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICFormatConverterVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: ::windows::core::RawPtr, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanConvert<Impl: IWICFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcpixelformat: *const ::windows::core::GUID, dstpixelformat: *const ::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetPixelFormat::<Impl, IMPL_OFFSET>, GetResolution::<Impl, IMPL_OFFSET>, CopyPalette::<Impl, IMPL_OFFSET>, CopyPixels::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, CanConvert::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICFormatConverter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICFormatConverterInfoImpl: Sized + IWICComponentInfoImpl {
    fn GetPixelFormats();
    fn CreateInstance();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICFormatConverterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverterInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICFormatConverterInfoVtbl {
        unsafe extern "system" fn GetPixelFormats<Impl: IWICFormatConverterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, ppixelformatguids: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICFormatConverterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetCLSID::<Impl, IMPL_OFFSET>,
            GetSigningStatus::<Impl, IMPL_OFFSET>,
            GetAuthor::<Impl, IMPL_OFFSET>,
            GetVendorGUID::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            GetSpecVersion::<Impl, IMPL_OFFSET>,
            GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetPixelFormats::<Impl, IMPL_OFFSET>,
            CreateInstance::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICFormatConverterInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactoryImpl: Sized {
    fn CreateDecoderFromFilename();
    fn CreateDecoderFromStream();
    fn CreateDecoderFromFileHandle();
    fn CreateComponentInfo();
    fn CreateDecoder();
    fn CreateEncoder();
    fn CreatePalette();
    fn CreateFormatConverter();
    fn CreateBitmapScaler();
    fn CreateBitmapClipper();
    fn CreateBitmapFlipRotator();
    fn CreateStream();
    fn CreateColorContext();
    fn CreateColorTransformer();
    fn CreateBitmap();
    fn CreateBitmapFromSource();
    fn CreateBitmapFromSourceRect();
    fn CreateBitmapFromMemory();
    fn CreateBitmapFromHBITMAP();
    fn CreateBitmapFromHICON();
    fn CreateComponentEnumerator();
    fn CreateFastMetadataEncoderFromDecoder();
    fn CreateFastMetadataEncoderFromFrameDecode();
    fn CreateQueryWriter();
    fn CreateQueryWriterFromReader();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICImagingFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICImagingFactoryVtbl {
        unsafe extern "system" fn CreateDecoderFromFilename<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDecoderFromStream<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDecoderFromFileHandle<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateComponentInfo<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcomponent: *const ::windows::core::GUID, ppiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDecoder<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEncoder<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePalette<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipalette: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFormatConverter<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiformatconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapScaler<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapscaler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapClipper<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapclipper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapFlipRotator<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapfliprotator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStream<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwicstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorContext<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiccolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorTransformer<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiccolortransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmap<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapFromSource<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapFromSourceRect<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapFromMemory<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapFromHBITMAP<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapFromHICON<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateComponentEnumerator<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, componenttypes: u32, options: u32, ppienumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromDecoder<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromFrameDecode<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piframedecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateQueryWriter<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateQueryWriterFromReader<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piqueryreader: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateDecoderFromFilename::<Impl, IMPL_OFFSET>,
            CreateDecoderFromStream::<Impl, IMPL_OFFSET>,
            CreateDecoderFromFileHandle::<Impl, IMPL_OFFSET>,
            CreateComponentInfo::<Impl, IMPL_OFFSET>,
            CreateDecoder::<Impl, IMPL_OFFSET>,
            CreateEncoder::<Impl, IMPL_OFFSET>,
            CreatePalette::<Impl, IMPL_OFFSET>,
            CreateFormatConverter::<Impl, IMPL_OFFSET>,
            CreateBitmapScaler::<Impl, IMPL_OFFSET>,
            CreateBitmapClipper::<Impl, IMPL_OFFSET>,
            CreateBitmapFlipRotator::<Impl, IMPL_OFFSET>,
            CreateStream::<Impl, IMPL_OFFSET>,
            CreateColorContext::<Impl, IMPL_OFFSET>,
            CreateColorTransformer::<Impl, IMPL_OFFSET>,
            CreateBitmap::<Impl, IMPL_OFFSET>,
            CreateBitmapFromSource::<Impl, IMPL_OFFSET>,
            CreateBitmapFromSourceRect::<Impl, IMPL_OFFSET>,
            CreateBitmapFromMemory::<Impl, IMPL_OFFSET>,
            CreateBitmapFromHBITMAP::<Impl, IMPL_OFFSET>,
            CreateBitmapFromHICON::<Impl, IMPL_OFFSET>,
            CreateComponentEnumerator::<Impl, IMPL_OFFSET>,
            CreateFastMetadataEncoderFromDecoder::<Impl, IMPL_OFFSET>,
            CreateFastMetadataEncoderFromFrameDecode::<Impl, IMPL_OFFSET>,
            CreateQueryWriter::<Impl, IMPL_OFFSET>,
            CreateQueryWriterFromReader::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICImagingFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IWICJpegFrameDecodeImpl: Sized {
    fn DoesSupportIndexing();
    fn SetIndexing();
    fn ClearIndexing();
    fn GetAcHuffmanTable();
    fn GetDcHuffmanTable();
    fn GetQuantizationTable();
    fn GetFrameHeader();
    fn GetScanHeader();
    fn CopyScan();
    fn CopyMinimalStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IWICJpegFrameDecodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICJpegFrameDecodeVtbl {
        unsafe extern "system" fn DoesSupportIndexing<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfindexingsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIndexing<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearIndexing<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAcHuffmanTable<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDcHuffmanTable<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuantizationTable<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrameHeader<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframeheader: *mut WICJpegFrameHeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScanHeader<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyScan<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyMinimalStream<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            DoesSupportIndexing::<Impl, IMPL_OFFSET>,
            SetIndexing::<Impl, IMPL_OFFSET>,
            ClearIndexing::<Impl, IMPL_OFFSET>,
            GetAcHuffmanTable::<Impl, IMPL_OFFSET>,
            GetDcHuffmanTable::<Impl, IMPL_OFFSET>,
            GetQuantizationTable::<Impl, IMPL_OFFSET>,
            GetFrameHeader::<Impl, IMPL_OFFSET>,
            GetScanHeader::<Impl, IMPL_OFFSET>,
            CopyScan::<Impl, IMPL_OFFSET>,
            CopyMinimalStream::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICJpegFrameDecode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICJpegFrameEncodeImpl: Sized {
    fn GetAcHuffmanTable();
    fn GetDcHuffmanTable();
    fn GetQuantizationTable();
    fn WriteScan();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICJpegFrameEncodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameEncodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICJpegFrameEncodeVtbl {
        unsafe extern "system" fn GetAcHuffmanTable<Impl: IWICJpegFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDcHuffmanTable<Impl: IWICJpegFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuantizationTable<Impl: IWICJpegFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteScan<Impl: IWICJpegFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbscandata: u32, pbscandata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetAcHuffmanTable::<Impl, IMPL_OFFSET>, GetDcHuffmanTable::<Impl, IMPL_OFFSET>, GetQuantizationTable::<Impl, IMPL_OFFSET>, WriteScan::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICJpegFrameEncode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockReaderImpl: Sized {
    fn GetContainerFormat();
    fn GetCount();
    fn GetReaderByIndex();
    fn GetEnumerator();
}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataBlockReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataBlockReaderVtbl {
        unsafe extern "system" fn GetContainerFormat<Impl: IWICMetadataBlockReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IWICMetadataBlockReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReaderByIndex<Impl: IWICMetadataBlockReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatareader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IWICMetadataBlockReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetContainerFormat::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetReaderByIndex::<Impl, IMPL_OFFSET>, GetEnumerator::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataBlockReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockWriterImpl: Sized + IWICMetadataBlockReaderImpl {
    fn InitializeFromBlockReader();
    fn GetWriterByIndex();
    fn AddWriter();
    fn SetWriterByIndex();
    fn RemoveWriterByIndex();
}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataBlockWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataBlockWriterVtbl {
        unsafe extern "system" fn InitializeFromBlockReader<Impl: IWICMetadataBlockWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimdblockreader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWriterByIndex<Impl: IWICMetadataBlockWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatawriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddWriter<Impl: IWICMetadataBlockWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimetadatawriter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWriterByIndex<Impl: IWICMetadataBlockWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pimetadatawriter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveWriterByIndex<Impl: IWICMetadataBlockWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetContainerFormat::<Impl, IMPL_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            GetReaderByIndex::<Impl, IMPL_OFFSET>,
            GetEnumerator::<Impl, IMPL_OFFSET>,
            InitializeFromBlockReader::<Impl, IMPL_OFFSET>,
            GetWriterByIndex::<Impl, IMPL_OFFSET>,
            AddWriter::<Impl, IMPL_OFFSET>,
            SetWriterByIndex::<Impl, IMPL_OFFSET>,
            RemoveWriterByIndex::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataBlockWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICMetadataHandlerInfoImpl: Sized + IWICComponentInfoImpl {
    fn GetMetadataFormat();
    fn GetContainerFormats();
    fn GetDeviceManufacturer();
    fn GetDeviceModels();
    fn DoesRequireFullStream();
    fn DoesSupportPadding();
    fn DoesRequireFixedSize();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICMetadataHandlerInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataHandlerInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataHandlerInfoVtbl {
        unsafe extern "system" fn GetMetadataFormat<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContainerFormats<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceModels<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoesRequireFullStream<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequiresfullstream: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoesSupportPadding<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportspadding: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoesRequireFixedSize<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffixedsize: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetCLSID::<Impl, IMPL_OFFSET>,
            GetSigningStatus::<Impl, IMPL_OFFSET>,
            GetAuthor::<Impl, IMPL_OFFSET>,
            GetVendorGUID::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            GetSpecVersion::<Impl, IMPL_OFFSET>,
            GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetMetadataFormat::<Impl, IMPL_OFFSET>,
            GetContainerFormats::<Impl, IMPL_OFFSET>,
            GetDeviceManufacturer::<Impl, IMPL_OFFSET>,
            GetDeviceModels::<Impl, IMPL_OFFSET>,
            DoesRequireFullStream::<Impl, IMPL_OFFSET>,
            DoesSupportPadding::<Impl, IMPL_OFFSET>,
            DoesRequireFixedSize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataHandlerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataQueryReaderImpl: Sized {
    fn GetContainerFormat();
    fn GetLocation();
    fn GetMetadataByName();
    fn GetEnumerator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataQueryReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataQueryReaderVtbl {
        unsafe extern "system" fn GetContainerFormat<Impl: IWICMetadataQueryReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocation<Impl: IWICMetadataQueryReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmaxlength: u32, wznamespace: super::super::Foundation::PWSTR, pcchactuallength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetadataByName<Impl: IWICMetadataQueryReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IWICMetadataQueryReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienumstring: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetContainerFormat::<Impl, IMPL_OFFSET>, GetLocation::<Impl, IMPL_OFFSET>, GetMetadataByName::<Impl, IMPL_OFFSET>, GetEnumerator::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataQueryReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataQueryWriterImpl: Sized + IWICMetadataQueryReaderImpl {
    fn SetMetadataByName();
    fn RemoveMetadataByName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataQueryWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataQueryWriterVtbl {
        unsafe extern "system" fn SetMetadataByName<Impl: IWICMetadataQueryWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveMetadataByName<Impl: IWICMetadataQueryWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetContainerFormat::<Impl, IMPL_OFFSET>, GetLocation::<Impl, IMPL_OFFSET>, GetMetadataByName::<Impl, IMPL_OFFSET>, GetEnumerator::<Impl, IMPL_OFFSET>, SetMetadataByName::<Impl, IMPL_OFFSET>, RemoveMetadataByName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataQueryWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataReaderImpl: Sized {
    fn GetMetadataFormat();
    fn GetMetadataHandlerInfo();
    fn GetCount();
    fn GetValueByIndex();
    fn GetValue();
    fn GetEnumerator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataReaderVtbl {
        unsafe extern "system" fn GetMetadataFormat<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetadataHandlerInfo<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppihandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValueByIndex<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetMetadataFormat::<Impl, IMPL_OFFSET>, GetMetadataHandlerInfo::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetValueByIndex::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, GetEnumerator::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICMetadataReaderInfoImpl: Sized + IWICMetadataHandlerInfoImpl + IWICComponentInfoImpl {
    fn GetPatterns();
    fn MatchesPattern();
    fn CreateInstance();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICMetadataReaderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReaderInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataReaderInfoVtbl {
        unsafe extern "system" fn GetPatterns<Impl: IWICMetadataReaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MatchesPattern<Impl: IWICMetadataReaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pistream: ::windows::core::RawPtr, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICMetadataReaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetCLSID::<Impl, IMPL_OFFSET>,
            GetSigningStatus::<Impl, IMPL_OFFSET>,
            GetAuthor::<Impl, IMPL_OFFSET>,
            GetVendorGUID::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            GetSpecVersion::<Impl, IMPL_OFFSET>,
            GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetMetadataFormat::<Impl, IMPL_OFFSET>,
            GetContainerFormats::<Impl, IMPL_OFFSET>,
            GetDeviceManufacturer::<Impl, IMPL_OFFSET>,
            GetDeviceModels::<Impl, IMPL_OFFSET>,
            DoesRequireFullStream::<Impl, IMPL_OFFSET>,
            DoesSupportPadding::<Impl, IMPL_OFFSET>,
            DoesRequireFixedSize::<Impl, IMPL_OFFSET>,
            GetPatterns::<Impl, IMPL_OFFSET>,
            MatchesPattern::<Impl, IMPL_OFFSET>,
            CreateInstance::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataReaderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICMetadataWriterImpl: Sized + IWICMetadataReaderImpl {
    fn SetValue();
    fn SetValueByIndex();
    fn RemoveValue();
    fn RemoveValueByIndex();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICMetadataWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataWriterVtbl {
        unsafe extern "system" fn SetValue<Impl: IWICMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValueByIndex<Impl: IWICMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveValue<Impl: IWICMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveValueByIndex<Impl: IWICMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetMetadataFormat::<Impl, IMPL_OFFSET>,
            GetMetadataHandlerInfo::<Impl, IMPL_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            GetValueByIndex::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            GetEnumerator::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
            SetValueByIndex::<Impl, IMPL_OFFSET>,
            RemoveValue::<Impl, IMPL_OFFSET>,
            RemoveValueByIndex::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICMetadataWriterInfoImpl: Sized + IWICMetadataHandlerInfoImpl + IWICComponentInfoImpl {
    fn GetHeader();
    fn CreateInstance();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICMetadataWriterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriterInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICMetadataWriterInfoVtbl {
        unsafe extern "system" fn GetHeader<Impl: IWICMetadataWriterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICMetadataWriterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetCLSID::<Impl, IMPL_OFFSET>,
            GetSigningStatus::<Impl, IMPL_OFFSET>,
            GetAuthor::<Impl, IMPL_OFFSET>,
            GetVendorGUID::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            GetSpecVersion::<Impl, IMPL_OFFSET>,
            GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetMetadataFormat::<Impl, IMPL_OFFSET>,
            GetContainerFormats::<Impl, IMPL_OFFSET>,
            GetDeviceManufacturer::<Impl, IMPL_OFFSET>,
            GetDeviceModels::<Impl, IMPL_OFFSET>,
            DoesRequireFullStream::<Impl, IMPL_OFFSET>,
            DoesSupportPadding::<Impl, IMPL_OFFSET>,
            DoesRequireFixedSize::<Impl, IMPL_OFFSET>,
            GetHeader::<Impl, IMPL_OFFSET>,
            CreateInstance::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICMetadataWriterInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPaletteImpl: Sized {
    fn InitializePredefined();
    fn InitializeCustom();
    fn InitializeFromBitmap();
    fn InitializeFromPalette();
    fn GetType();
    fn GetColorCount();
    fn GetColors();
    fn IsBlackWhite();
    fn IsGrayscale();
    fn HasAlpha();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPaletteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPaletteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPaletteVtbl {
        unsafe extern "system" fn InitializePredefined<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeCustom<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolors: *const u32, ccount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromBitmap<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisurface: ::windows::core::RawPtr, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromPalette<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pepalettetype: *mut WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorCount<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColors<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsBlackWhite<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisblackwhite: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsGrayscale<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisgrayscale: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasAlpha<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfhasalpha: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            InitializePredefined::<Impl, IMPL_OFFSET>,
            InitializeCustom::<Impl, IMPL_OFFSET>,
            InitializeFromBitmap::<Impl, IMPL_OFFSET>,
            InitializeFromPalette::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetColorCount::<Impl, IMPL_OFFSET>,
            GetColors::<Impl, IMPL_OFFSET>,
            IsBlackWhite::<Impl, IMPL_OFFSET>,
            IsGrayscale::<Impl, IMPL_OFFSET>,
            HasAlpha::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPalette as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICPersistStreamImpl: Sized + IPersistStreamImpl + IPersistImpl {
    fn LoadEx();
    fn SaveEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWICPersistStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPersistStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPersistStreamVtbl {
        unsafe extern "system" fn LoadEx<Impl: IWICPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pguidpreferredvendor: *const ::windows::core::GUID, dwpersistoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveEx<Impl: IWICPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassID::<Impl, IMPL_OFFSET>, IsDirty::<Impl, IMPL_OFFSET>, Load::<Impl, IMPL_OFFSET>, Save::<Impl, IMPL_OFFSET>, GetSizeMax::<Impl, IMPL_OFFSET>, LoadEx::<Impl, IMPL_OFFSET>, SaveEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPersistStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPixelFormatInfoImpl: Sized + IWICComponentInfoImpl {
    fn GetFormatGUID();
    fn GetColorContext();
    fn GetBitsPerPixel();
    fn GetChannelCount();
    fn GetChannelMask();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPixelFormatInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPixelFormatInfoVtbl {
        unsafe extern "system" fn GetFormatGUID<Impl: IWICPixelFormatInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorContext<Impl: IWICPixelFormatInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBitsPerPixel<Impl: IWICPixelFormatInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puibitsperpixel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChannelCount<Impl: IWICPixelFormatInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puichannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChannelMask<Impl: IWICPixelFormatInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetCLSID::<Impl, IMPL_OFFSET>,
            GetSigningStatus::<Impl, IMPL_OFFSET>,
            GetAuthor::<Impl, IMPL_OFFSET>,
            GetVendorGUID::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            GetSpecVersion::<Impl, IMPL_OFFSET>,
            GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetFormatGUID::<Impl, IMPL_OFFSET>,
            GetColorContext::<Impl, IMPL_OFFSET>,
            GetBitsPerPixel::<Impl, IMPL_OFFSET>,
            GetChannelCount::<Impl, IMPL_OFFSET>,
            GetChannelMask::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPixelFormatInfo2Impl: Sized + IWICPixelFormatInfoImpl + IWICComponentInfoImpl {
    fn SupportsTransparency();
    fn GetNumericRepresentation();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPixelFormatInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPixelFormatInfo2Vtbl {
        unsafe extern "system" fn SupportsTransparency<Impl: IWICPixelFormatInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportstransparency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumericRepresentation<Impl: IWICPixelFormatInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumericrepresentation: *mut WICPixelFormatNumericRepresentation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetComponentType::<Impl, IMPL_OFFSET>,
            GetCLSID::<Impl, IMPL_OFFSET>,
            GetSigningStatus::<Impl, IMPL_OFFSET>,
            GetAuthor::<Impl, IMPL_OFFSET>,
            GetVendorGUID::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            GetSpecVersion::<Impl, IMPL_OFFSET>,
            GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetFormatGUID::<Impl, IMPL_OFFSET>,
            GetColorContext::<Impl, IMPL_OFFSET>,
            GetBitsPerPixel::<Impl, IMPL_OFFSET>,
            GetChannelCount::<Impl, IMPL_OFFSET>,
            GetChannelMask::<Impl, IMPL_OFFSET>,
            SupportsTransparency::<Impl, IMPL_OFFSET>,
            GetNumericRepresentation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo2 as ::windows::core::Interface>::IID
    }
}
pub trait IWICPlanarBitmapFrameEncodeImpl: Sized {
    fn WritePixels();
    fn WriteSource();
}
impl IWICPlanarBitmapFrameEncodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapFrameEncodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPlanarBitmapFrameEncodeVtbl {
        unsafe extern "system" fn WritePixels<Impl: IWICPlanarBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteSource<Impl: IWICPlanarBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplanes: *const ::windows::core::RawPtr, cplanes: u32, prcsource: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, WritePixels::<Impl, IMPL_OFFSET>, WriteSource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPlanarBitmapFrameEncode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPlanarBitmapSourceTransformImpl: Sized {
    fn DoesSupportTransform();
    fn CopyPixels();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPlanarBitmapSourceTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapSourceTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPlanarBitmapSourceTransformVtbl {
        unsafe extern "system" fn DoesSupportTransform<Impl: IWICPlanarBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows::core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyPixels<Impl: IWICPlanarBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DoesSupportTransform::<Impl, IMPL_OFFSET>, CopyPixels::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPlanarBitmapSourceTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPlanarFormatConverterImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
    fn CanConvert();
}
#[cfg(feature = "Win32_Foundation")]
impl IWICPlanarFormatConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarFormatConverterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICPlanarFormatConverterVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICPlanarFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplanes: *const ::windows::core::RawPtr, cplanes: u32, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: ::windows::core::RawPtr, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanConvert<Impl: IWICPlanarFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcpixelformats: *const ::windows::core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetPixelFormat::<Impl, IMPL_OFFSET>, GetResolution::<Impl, IMPL_OFFSET>, CopyPalette::<Impl, IMPL_OFFSET>, CopyPixels::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, CanConvert::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICPlanarFormatConverter as ::windows::core::Interface>::IID
    }
}
pub trait IWICProgressCallbackImpl: Sized {
    fn Notify();
}
impl IWICProgressCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICProgressCallbackVtbl {
        unsafe extern "system" fn Notify<Impl: IWICProgressCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Notify::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICProgressCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWICProgressiveLevelControlImpl: Sized {
    fn GetLevelCount();
    fn GetCurrentLevel();
    fn SetCurrentLevel();
}
impl IWICProgressiveLevelControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressiveLevelControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICProgressiveLevelControlVtbl {
        unsafe extern "system" fn GetLevelCount<Impl: IWICProgressiveLevelControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentLevel<Impl: IWICProgressiveLevelControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentLevel<Impl: IWICProgressiveLevelControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetLevelCount::<Impl, IMPL_OFFSET>, GetCurrentLevel::<Impl, IMPL_OFFSET>, SetCurrentLevel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICProgressiveLevelControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWICStreamImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn InitializeFromIStream();
    fn InitializeFromFilename();
    fn InitializeFromMemory();
    fn InitializeFromIStreamRegion();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWICStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICStreamVtbl {
        unsafe extern "system" fn InitializeFromIStream<Impl: IWICStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromFilename<Impl: IWICStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromMemory<Impl: IWICStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromIStreamRegion<Impl: IWICStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, uloffset: u64, ulmaxsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Read::<Impl, IMPL_OFFSET>,
            Write::<Impl, IMPL_OFFSET>,
            Seek::<Impl, IMPL_OFFSET>,
            SetSize::<Impl, IMPL_OFFSET>,
            CopyTo::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Revert::<Impl, IMPL_OFFSET>,
            LockRegion::<Impl, IMPL_OFFSET>,
            UnlockRegion::<Impl, IMPL_OFFSET>,
            Stat::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            InitializeFromIStream::<Impl, IMPL_OFFSET>,
            InitializeFromFilename::<Impl, IMPL_OFFSET>,
            InitializeFromMemory::<Impl, IMPL_OFFSET>,
            InitializeFromIStreamRegion::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICStreamProviderImpl: Sized {
    fn GetStream();
    fn GetPersistOptions();
    fn GetPreferredVendorGUID();
    fn RefreshStream();
}
#[cfg(feature = "Win32_System_Com")]
impl IWICStreamProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICStreamProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWICStreamProviderVtbl {
        unsafe extern "system" fn GetStream<Impl: IWICStreamProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPersistOptions<Impl: IWICStreamProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpersistoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreferredVendorGUID<Impl: IWICStreamProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidpreferredvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshStream<Impl: IWICStreamProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetStream::<Impl, IMPL_OFFSET>, GetPersistOptions::<Impl, IMPL_OFFSET>, GetPreferredVendorGUID::<Impl, IMPL_OFFSET>, RefreshStream::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWICStreamProvider as ::windows::core::Interface>::IID
    }
}
