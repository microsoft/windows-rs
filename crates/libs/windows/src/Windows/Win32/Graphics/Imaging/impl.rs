pub trait IWICBitmapImpl: Sized + IWICBitmapSourceImpl {
    fn Lock();
    fn SetPalette();
    fn SetResolution();
}
impl ::windows::core::RuntimeName for IWICBitmap {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmap";
}
impl IWICBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapImpl, const OFFSET: isize>() -> IWICBitmapVtbl {
        unsafe extern "system" fn Lock<Impl: IWICBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prclock: *const WICRect, flags: u32, ppilock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lock(&*(&prclock as *const <WICRect as ::windows::core::Abi>::Abi as *const <WICRect as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&ppilock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Impl: IWICBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPalette(&*(&pipalette as *const <IWICPalette as ::windows::core::Abi>::Abi as *const <IWICPalette as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResolution<Impl: IWICBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetResolution(dpix, dpiy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICBitmap>, ::windows::core::GetTrustLevel, Lock::<Impl, OFFSET>, SetPalette::<Impl, OFFSET>, SetResolution::<Impl, OFFSET>)
    }
}
pub trait IWICBitmapClipperImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IWICBitmapClipper {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapClipper";
}
impl IWICBitmapClipperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapClipperImpl, const OFFSET: isize>() -> IWICBitmapClipperVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapClipperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, prc: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pisource as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType), &*(&prc as *const <WICRect as ::windows::core::Abi>::Abi as *const <WICRect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICBitmapClipper>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWICBitmapCodecInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapCodecInfo";
}
impl IWICBitmapCodecInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>() -> IWICBitmapCodecInfoVtbl {
        unsafe extern "system" fn GetContainerFormat<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerFormat(::core::mem::transmute_copy(&pguidcontainerformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormats<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelFormats(cformats, &*(&pguidpixelformats as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorManagementVersion<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorManagementVersion(cchcolormanagementversion, &*(&wzcolormanagementversion as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceManufacturer<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceManufacturer(cchdevicemanufacturer, &*(&wzdevicemanufacturer as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceModels<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceModels(cchdevicemodels, &*(&wzdevicemodels as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMimeTypes<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmimetypes: u32, wzmimetypes: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMimeTypes(cchmimetypes, &*(&wzmimetypes as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileExtensions<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchfileextensions: u32, wzfileextensions: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileExtensions(cchfileextensions, &*(&wzfileextensions as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportAnimation<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportanimation: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportAnimation(::core::mem::transmute_copy(&pfsupportanimation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportChromakey<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportchromakey: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportChromakey(::core::mem::transmute_copy(&pfsupportchromakey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportLossless<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportlossless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportLossless(::core::mem::transmute_copy(&pfsupportlossless)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportMultiframe<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportmultiframe: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportMultiframe(::core::mem::transmute_copy(&pfsupportmultiframe)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesMimeType<Impl: IWICBitmapCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzmimetype: super::super::Foundation::PWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchesMimeType(&*(&wzmimetype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfmatches)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWICBitmapCodecInfo>,
            ::windows::core::GetTrustLevel,
            GetContainerFormat::<Impl, OFFSET>,
            GetPixelFormats::<Impl, OFFSET>,
            GetColorManagementVersion::<Impl, OFFSET>,
            GetDeviceManufacturer::<Impl, OFFSET>,
            GetDeviceModels::<Impl, OFFSET>,
            GetMimeTypes::<Impl, OFFSET>,
            GetFileExtensions::<Impl, OFFSET>,
            DoesSupportAnimation::<Impl, OFFSET>,
            DoesSupportChromakey::<Impl, OFFSET>,
            DoesSupportLossless::<Impl, OFFSET>,
            DoesSupportMultiframe::<Impl, OFFSET>,
            MatchesMimeType::<Impl, OFFSET>,
        )
    }
}
pub trait IWICBitmapCodecProgressNotificationImpl: Sized {
    fn RegisterProgressNotification();
}
impl ::windows::core::RuntimeName for IWICBitmapCodecProgressNotification {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapCodecProgressNotification";
}
impl IWICBitmapCodecProgressNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapCodecProgressNotificationImpl, const OFFSET: isize>() -> IWICBitmapCodecProgressNotificationVtbl {
        unsafe extern "system" fn RegisterProgressNotification<Impl: IWICBitmapCodecProgressNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfnprogressnotification: ::windows::core::RawPtr, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterProgressNotification(&*(&pfnprogressnotification as *const <PFNProgressNotification as ::windows::core::Abi>::Abi as *const <PFNProgressNotification as ::windows::core::DefaultType>::DefaultType), &*(&pvdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwprogressflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICBitmapCodecProgressNotification>, ::windows::core::GetTrustLevel, RegisterProgressNotification::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWICBitmapDecoder {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapDecoder";
}
impl IWICBitmapDecoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoderImpl, const OFFSET: isize>() -> IWICBitmapDecoderVtbl {
        unsafe extern "system" fn QueryCapability<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pdwcapability: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryCapability(&*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcapability)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, cacheoptions: WICDecodeOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), cacheoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerFormat<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerFormat(::core::mem::transmute_copy(&pguidcontainerformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDecoderInfo<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidecoderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDecoderInfo(::core::mem::transmute_copy(&ppidecoderinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyPalette<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyPalette(&*(&pipalette as *const <IWICPalette as ::windows::core::Abi>::Abi as *const <IWICPalette as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataQueryReader<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataQueryReader(::core::mem::transmute_copy(&ppimetadataqueryreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreview<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreview(::core::mem::transmute_copy(&ppibitmapsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContexts<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut ::windows::core::RawPtr, pcactualcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorContexts(ccount, &*(&ppicolorcontexts as *const <IWICColorContext as ::windows::core::Abi>::Abi as *const <IWICColorContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcactualcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnail<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnail(::core::mem::transmute_copy(&ppithumbnail)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameCount<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrame<Impl: IWICBitmapDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppibitmapframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrame(index, ::core::mem::transmute_copy(&ppibitmapframe)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWICBitmapDecoder>,
            ::windows::core::GetTrustLevel,
            QueryCapability::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            GetContainerFormat::<Impl, OFFSET>,
            GetDecoderInfo::<Impl, OFFSET>,
            CopyPalette::<Impl, OFFSET>,
            GetMetadataQueryReader::<Impl, OFFSET>,
            GetPreview::<Impl, OFFSET>,
            GetColorContexts::<Impl, OFFSET>,
            GetThumbnail::<Impl, OFFSET>,
            GetFrameCount::<Impl, OFFSET>,
            GetFrame::<Impl, OFFSET>,
        )
    }
}
pub trait IWICBitmapDecoderInfoImpl: Sized + IWICBitmapCodecInfoImpl + IWICComponentInfoImpl {
    fn GetPatterns();
    fn MatchesPattern();
    fn CreateInstance();
}
impl ::windows::core::RuntimeName for IWICBitmapDecoderInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapDecoderInfo";
}
impl IWICBitmapDecoderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapDecoderInfoImpl, const OFFSET: isize>() -> IWICBitmapDecoderInfoVtbl {
        unsafe extern "system" fn GetPatterns<Impl: IWICBitmapDecoderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatterns(cbsizepatterns, ::core::mem::transmute_copy(&ppatterns), ::core::mem::transmute_copy(&pcpatterns), ::core::mem::transmute_copy(&pcbpatternsactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesPattern<Impl: IWICBitmapDecoderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchesPattern(&*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfmatches)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICBitmapDecoderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapdecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&ppibitmapdecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICBitmapDecoderInfo>, ::windows::core::GetTrustLevel, GetPatterns::<Impl, OFFSET>, MatchesPattern::<Impl, OFFSET>, CreateInstance::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWICBitmapEncoder {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapEncoder";
}
impl IWICBitmapEncoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoderImpl, const OFFSET: isize>() -> IWICBitmapEncoderVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, cacheoption: WICBitmapEncoderCacheOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), cacheoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerFormat<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerFormat(::core::mem::transmute_copy(&pguidcontainerformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEncoderInfo<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiencoderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEncoderInfo(::core::mem::transmute_copy(&ppiencoderinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorContexts<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColorContexts(ccount, &*(&ppicolorcontext as *const <IWICColorContext as ::windows::core::Abi>::Abi as *const <IWICColorContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPalette(&*(&pipalette as *const <IWICPalette as ::windows::core::Abi>::Abi as *const <IWICPalette as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnail<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pithumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetThumbnail(&*(&pithumbnail as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreview<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipreview: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreview(&*(&pipreview as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNewFrame<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut ::windows::core::RawPtr, ppiencoderoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewFrame(::core::mem::transmute_copy(&ppiframeencode), &*(&ppiencoderoptions as *const <super::super::System::Com::StructuredStorage::IPropertyBag2 as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::IPropertyBag2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Impl: IWICBitmapEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataQueryWriter(::core::mem::transmute_copy(&ppimetadataquerywriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWICBitmapEncoder>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            GetContainerFormat::<Impl, OFFSET>,
            GetEncoderInfo::<Impl, OFFSET>,
            SetColorContexts::<Impl, OFFSET>,
            SetPalette::<Impl, OFFSET>,
            SetThumbnail::<Impl, OFFSET>,
            SetPreview::<Impl, OFFSET>,
            CreateNewFrame::<Impl, OFFSET>,
            Commit::<Impl, OFFSET>,
            GetMetadataQueryWriter::<Impl, OFFSET>,
        )
    }
}
pub trait IWICBitmapEncoderInfoImpl: Sized + IWICBitmapCodecInfoImpl + IWICComponentInfoImpl {
    fn CreateInstance();
}
impl ::windows::core::RuntimeName for IWICBitmapEncoderInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapEncoderInfo";
}
impl IWICBitmapEncoderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapEncoderInfoImpl, const OFFSET: isize>() -> IWICBitmapEncoderInfoVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IWICBitmapEncoderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&ppibitmapencoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICBitmapEncoderInfo>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
pub trait IWICBitmapFlipRotatorImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IWICBitmapFlipRotator {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapFlipRotator";
}
impl IWICBitmapFlipRotatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFlipRotatorImpl, const OFFSET: isize>() -> IWICBitmapFlipRotatorVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapFlipRotatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, options: WICBitmapTransformOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pisource as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICBitmapFlipRotator>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>)
    }
}
pub trait IWICBitmapFrameDecodeImpl: Sized + IWICBitmapSourceImpl {
    fn GetMetadataQueryReader();
    fn GetColorContexts();
    fn GetThumbnail();
}
impl ::windows::core::RuntimeName for IWICBitmapFrameDecode {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapFrameDecode";
}
impl IWICBitmapFrameDecodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameDecodeImpl, const OFFSET: isize>() -> IWICBitmapFrameDecodeVtbl {
        unsafe extern "system" fn GetMetadataQueryReader<Impl: IWICBitmapFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataQueryReader(::core::mem::transmute_copy(&ppimetadataqueryreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContexts<Impl: IWICBitmapFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut ::windows::core::RawPtr, pcactualcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorContexts(ccount, &*(&ppicolorcontexts as *const <IWICColorContext as ::windows::core::Abi>::Abi as *const <IWICColorContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcactualcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnail<Impl: IWICBitmapFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnail(::core::mem::transmute_copy(&ppithumbnail)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICBitmapFrameDecode>, ::windows::core::GetTrustLevel, GetMetadataQueryReader::<Impl, OFFSET>, GetColorContexts::<Impl, OFFSET>, GetThumbnail::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWICBitmapFrameEncode {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapFrameEncode";
}
impl IWICBitmapFrameEncodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>() -> IWICBitmapFrameEncodeVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piencoderoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&piencoderoptions as *const <super::super::System::Com::StructuredStorage::IPropertyBag2 as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::IPropertyBag2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSize(uiwidth, uiheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResolution<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetResolution(dpix, dpiy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelFormat<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPixelFormat(&*(&ppixelformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorContexts<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColorContexts(ccount, &*(&ppicolorcontext as *const <IWICColorContext as ::windows::core::Abi>::Abi as *const <IWICColorContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPalette(&*(&pipalette as *const <IWICPalette as ::windows::core::Abi>::Abi as *const <IWICPalette as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnail<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pithumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetThumbnail(&*(&pithumbnail as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WritePixels<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WritePixels(linecount, cbstride, cbbuffersize, pbpixels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSource<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, prc: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteSource(&*(&pibitmapsource as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType), &*(&prc as *const <WICRect as ::windows::core::Abi>::Abi as *const <WICRect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Impl: IWICBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataQueryWriter(::core::mem::transmute_copy(&ppimetadataquerywriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWICBitmapFrameEncode>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            SetSize::<Impl, OFFSET>,
            SetResolution::<Impl, OFFSET>,
            SetPixelFormat::<Impl, OFFSET>,
            SetColorContexts::<Impl, OFFSET>,
            SetPalette::<Impl, OFFSET>,
            SetThumbnail::<Impl, OFFSET>,
            WritePixels::<Impl, OFFSET>,
            WriteSource::<Impl, OFFSET>,
            Commit::<Impl, OFFSET>,
            GetMetadataQueryWriter::<Impl, OFFSET>,
        )
    }
}
pub trait IWICBitmapLockImpl: Sized {
    fn GetSize();
    fn GetStride();
    fn GetDataPointer();
    fn GetPixelFormat();
}
impl ::windows::core::RuntimeName for IWICBitmapLock {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapLock";
}
impl IWICBitmapLockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapLockImpl, const OFFSET: isize>() -> IWICBitmapLockVtbl {
        unsafe extern "system" fn GetSize<Impl: IWICBitmapLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStride<Impl: IWICBitmapLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbstride: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStride(::core::mem::transmute_copy(&pcbstride)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPointer<Impl: IWICBitmapLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataPointer(::core::mem::transmute_copy(&pcbbuffersize), ::core::mem::transmute_copy(&ppbdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IWICBitmapLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelFormat(::core::mem::transmute_copy(&ppixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICBitmapLock>, ::windows::core::GetTrustLevel, GetSize::<Impl, OFFSET>, GetStride::<Impl, OFFSET>, GetDataPointer::<Impl, OFFSET>, GetPixelFormat::<Impl, OFFSET>)
    }
}
pub trait IWICBitmapScalerImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IWICBitmapScaler {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapScaler";
}
impl IWICBitmapScalerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapScalerImpl, const OFFSET: isize>() -> IWICBitmapScalerVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICBitmapScalerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pisource as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType), uiwidth, uiheight, mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICBitmapScaler>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>)
    }
}
pub trait IWICBitmapSourceImpl: Sized {
    fn GetSize();
    fn GetPixelFormat();
    fn GetResolution();
    fn CopyPalette();
    fn CopyPixels();
}
impl ::windows::core::RuntimeName for IWICBitmapSource {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapSource";
}
impl IWICBitmapSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSourceImpl, const OFFSET: isize>() -> IWICBitmapSourceVtbl {
        unsafe extern "system" fn GetSize<Impl: IWICBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Impl: IWICBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelFormat(::core::mem::transmute_copy(&ppixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolution<Impl: IWICBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResolution(::core::mem::transmute_copy(&pdpix), ::core::mem::transmute_copy(&pdpiy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyPalette<Impl: IWICBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyPalette(&*(&pipalette as *const <IWICPalette as ::windows::core::Abi>::Abi as *const <IWICPalette as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyPixels<Impl: IWICBitmapSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyPixels(&*(&prc as *const <WICRect as ::windows::core::Abi>::Abi as *const <WICRect as ::windows::core::DefaultType>::DefaultType), cbstride, cbbuffersize, ::core::mem::transmute_copy(&pbbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICBitmapSource>, ::windows::core::GetTrustLevel, GetSize::<Impl, OFFSET>, GetPixelFormat::<Impl, OFFSET>, GetResolution::<Impl, OFFSET>, CopyPalette::<Impl, OFFSET>, CopyPixels::<Impl, OFFSET>)
    }
}
pub trait IWICBitmapSourceTransformImpl: Sized {
    fn CopyPixels();
    fn GetClosestSize();
    fn GetClosestPixelFormat();
    fn DoesSupportTransform();
}
impl ::windows::core::RuntimeName for IWICBitmapSourceTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICBitmapSourceTransform";
}
impl IWICBitmapSourceTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICBitmapSourceTransformImpl, const OFFSET: isize>() -> IWICBitmapSourceTransformVtbl {
        unsafe extern "system" fn CopyPixels<Impl: IWICBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: &::windows::core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyPixels(&*(&prc as *const <WICRect as ::windows::core::Abi>::Abi as *const <WICRect as ::windows::core::DefaultType>::DefaultType), uiwidth, uiheight, &*(&pguiddstformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dsttransform, nstride, cbbuffersize, ::core::mem::transmute_copy(&pbbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClosestSize<Impl: IWICBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClosestSize(puiwidth, puiheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClosestPixelFormat<Impl: IWICBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguiddstformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClosestPixelFormat(&*(&pguiddstformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportTransform<Impl: IWICBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dsttransform: WICBitmapTransformOptions, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportTransform(dsttransform, ::core::mem::transmute_copy(&pfissupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICBitmapSourceTransform>, ::windows::core::GetTrustLevel, CopyPixels::<Impl, OFFSET>, GetClosestSize::<Impl, OFFSET>, GetClosestPixelFormat::<Impl, OFFSET>, DoesSupportTransform::<Impl, OFFSET>)
    }
}
pub trait IWICColorContextImpl: Sized {
    fn InitializeFromFilename();
    fn InitializeFromMemory();
    fn InitializeFromExifColorSpace();
    fn GetType();
    fn GetProfileBytes();
    fn GetExifColorSpace();
}
impl ::windows::core::RuntimeName for IWICColorContext {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICColorContext";
}
impl IWICColorContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorContextImpl, const OFFSET: isize>() -> IWICColorContextVtbl {
        unsafe extern "system" fn InitializeFromFilename<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeFromFilename(&*(&wzfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeFromMemory<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeFromMemory(pbbuffer, cbbuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeFromExifColorSpace<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeFromExifColorSpace(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WICColorContextType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileBytes<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfileBytes(cbbuffer, pbbuffer, ::core::mem::transmute_copy(&pcbactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExifColorSpace<Impl: IWICColorContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExifColorSpace(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICColorContext>, ::windows::core::GetTrustLevel, InitializeFromFilename::<Impl, OFFSET>, InitializeFromMemory::<Impl, OFFSET>, InitializeFromExifColorSpace::<Impl, OFFSET>, GetType::<Impl, OFFSET>, GetProfileBytes::<Impl, OFFSET>, GetExifColorSpace::<Impl, OFFSET>)
    }
}
pub trait IWICColorTransformImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IWICColorTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICColorTransform";
}
impl IWICColorTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICColorTransformImpl, const OFFSET: isize>() -> IWICColorTransformVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICColorTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, picontextsource: ::windows::core::RawPtr, picontextdest: ::windows::core::RawPtr, pixelfmtdest: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&pibitmapsource as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType),
                &*(&picontextsource as *const <IWICColorContext as ::windows::core::Abi>::Abi as *const <IWICColorContext as ::windows::core::DefaultType>::DefaultType),
                &*(&picontextdest as *const <IWICColorContext as ::windows::core::Abi>::Abi as *const <IWICColorContext as ::windows::core::DefaultType>::DefaultType),
                &*(&pixelfmtdest as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICColorTransform>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>)
    }
}
pub trait IWICComponentFactoryImpl: Sized + IWICImagingFactoryImpl {
    fn CreateMetadataReader();
    fn CreateMetadataReaderFromContainer();
    fn CreateMetadataWriter();
    fn CreateMetadataWriterFromReader();
    fn CreateQueryReaderFromBlockReader();
    fn CreateQueryWriterFromBlockWriter();
    fn CreateEncoderPropertyBag();
}
impl ::windows::core::RuntimeName for IWICComponentFactory {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICComponentFactory";
}
impl IWICComponentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentFactoryImpl, const OFFSET: isize>() -> IWICComponentFactoryVtbl {
        unsafe extern "system" fn CreateMetadataReader<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: &::windows::core::GUID, pguidvendor: &::windows::core::GUID, dwoptions: u32, pistream: ::windows::core::RawPtr, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMetadataReader(
                &*(&guidmetadataformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pguidvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                dwoptions,
                &*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppireader),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataReaderFromContainer<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: &::windows::core::GUID, pguidvendor: &::windows::core::GUID, dwoptions: u32, pistream: ::windows::core::RawPtr, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMetadataReaderFromContainer(
                &*(&guidcontainerformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pguidvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                dwoptions,
                &*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppireader),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataWriter<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: &::windows::core::GUID, pguidvendor: &::windows::core::GUID, dwmetadataoptions: u32, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMetadataWriter(&*(&guidmetadataformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pguidvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwmetadataoptions, ::core::mem::transmute_copy(&ppiwriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataWriterFromReader<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pireader: ::windows::core::RawPtr, pguidvendor: &::windows::core::GUID, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMetadataWriterFromReader(&*(&pireader as *const <IWICMetadataReader as ::windows::core::Abi>::Abi as *const <IWICMetadataReader as ::windows::core::DefaultType>::DefaultType), &*(&pguidvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppiwriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryReaderFromBlockReader<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piblockreader: ::windows::core::RawPtr, ppiqueryreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQueryReaderFromBlockReader(&*(&piblockreader as *const <IWICMetadataBlockReader as ::windows::core::Abi>::Abi as *const <IWICMetadataBlockReader as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppiqueryreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromBlockWriter<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piblockwriter: ::windows::core::RawPtr, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQueryWriterFromBlockWriter(&*(&piblockwriter as *const <IWICMetadataBlockWriter as ::windows::core::Abi>::Abi as *const <IWICMetadataBlockWriter as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppiquerywriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncoderPropertyBag<Impl: IWICComponentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32, ppipropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncoderPropertyBag(&*(&ppropoptions as *const <super::super::System::Com::StructuredStorage::PROPBAG2 as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPBAG2 as ::windows::core::DefaultType>::DefaultType), ccount, ::core::mem::transmute_copy(&ppipropertybag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWICComponentFactory>,
            ::windows::core::GetTrustLevel,
            CreateMetadataReader::<Impl, OFFSET>,
            CreateMetadataReaderFromContainer::<Impl, OFFSET>,
            CreateMetadataWriter::<Impl, OFFSET>,
            CreateMetadataWriterFromReader::<Impl, OFFSET>,
            CreateQueryReaderFromBlockReader::<Impl, OFFSET>,
            CreateQueryWriterFromBlockWriter::<Impl, OFFSET>,
            CreateEncoderPropertyBag::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IWICComponentInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICComponentInfo";
}
impl IWICComponentInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICComponentInfoImpl, const OFFSET: isize>() -> IWICComponentInfoVtbl {
        unsafe extern "system" fn GetComponentType<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCLSID<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCLSID(::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningStatus<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningStatus(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthor<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthor(cchauthor, &*(&wzauthor as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVendorGUID<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVendorGUID(::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion(cchversion, &*(&wzversion as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecVersion<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpecVersion(cchspecversion, &*(&wzspecversion as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFriendlyName<Impl: IWICComponentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFriendlyName(cchfriendlyname, &*(&wzfriendlyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWICComponentInfo>,
            ::windows::core::GetTrustLevel,
            GetComponentType::<Impl, OFFSET>,
            GetCLSID::<Impl, OFFSET>,
            GetSigningStatus::<Impl, OFFSET>,
            GetAuthor::<Impl, OFFSET>,
            GetVendorGUID::<Impl, OFFSET>,
            GetVersion::<Impl, OFFSET>,
            GetSpecVersion::<Impl, OFFSET>,
            GetFriendlyName::<Impl, OFFSET>,
        )
    }
}
pub trait IWICDdsDecoderImpl: Sized {
    fn GetParameters();
    fn GetFrame();
}
impl ::windows::core::RuntimeName for IWICDdsDecoder {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICDdsDecoder";
}
impl IWICDdsDecoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsDecoderImpl, const OFFSET: isize>() -> IWICDdsDecoderVtbl {
        unsafe extern "system" fn GetParameters<Impl: IWICDdsDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameters(::core::mem::transmute_copy(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrame<Impl: IWICDdsDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arrayindex: u32, miplevel: u32, sliceindex: u32, ppibitmapframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrame(arrayindex, miplevel, sliceindex, ::core::mem::transmute_copy(&ppibitmapframe)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICDdsDecoder>, ::windows::core::GetTrustLevel, GetParameters::<Impl, OFFSET>, GetFrame::<Impl, OFFSET>)
    }
}
pub trait IWICDdsEncoderImpl: Sized {
    fn SetParameters();
    fn GetParameters();
    fn CreateNewFrame();
}
impl ::windows::core::RuntimeName for IWICDdsEncoder {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICDdsEncoder";
}
impl IWICDdsEncoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsEncoderImpl, const OFFSET: isize>() -> IWICDdsEncoderVtbl {
        unsafe extern "system" fn SetParameters<Impl: IWICDdsEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetParameters(&*(&pparameters as *const <WICDdsParameters as ::windows::core::Abi>::Abi as *const <WICDdsParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameters<Impl: IWICDdsEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameters(::core::mem::transmute_copy(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNewFrame<Impl: IWICDdsEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut ::windows::core::RawPtr, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewFrame(::core::mem::transmute_copy(&ppiframeencode), ::core::mem::transmute_copy(&parrayindex), ::core::mem::transmute_copy(&pmiplevel), ::core::mem::transmute_copy(&psliceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICDdsEncoder>, ::windows::core::GetTrustLevel, SetParameters::<Impl, OFFSET>, GetParameters::<Impl, OFFSET>, CreateNewFrame::<Impl, OFFSET>)
    }
}
pub trait IWICDdsFrameDecodeImpl: Sized {
    fn GetSizeInBlocks();
    fn GetFormatInfo();
    fn CopyBlocks();
}
impl ::windows::core::RuntimeName for IWICDdsFrameDecode {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICDdsFrameDecode";
}
impl IWICDdsFrameDecodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDdsFrameDecodeImpl, const OFFSET: isize>() -> IWICDdsFrameDecodeVtbl {
        unsafe extern "system" fn GetSizeInBlocks<Impl: IWICDdsFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSizeInBlocks(::core::mem::transmute_copy(&pwidthinblocks), ::core::mem::transmute_copy(&pheightinblocks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatInfo<Impl: IWICDdsFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatinfo: *mut WICDdsFormatInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatInfo(::core::mem::transmute_copy(&pformatinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyBlocks<Impl: IWICDdsFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyBlocks(&*(&prcboundsinblocks as *const <WICRect as ::windows::core::Abi>::Abi as *const <WICRect as ::windows::core::DefaultType>::DefaultType), cbstride, cbbuffersize, ::core::mem::transmute_copy(&pbbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICDdsFrameDecode>, ::windows::core::GetTrustLevel, GetSizeInBlocks::<Impl, OFFSET>, GetFormatInfo::<Impl, OFFSET>, CopyBlocks::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWICDevelopRaw {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICDevelopRaw";
}
impl IWICDevelopRawVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRawImpl, const OFFSET: isize>() -> IWICDevelopRawVtbl {
        unsafe extern "system" fn QueryRawCapabilitiesInfo<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryRawCapabilitiesInfo(&*(&pinfo as *const <WICRawCapabilitiesInfo as ::windows::core::Abi>::Abi as *const <WICRawCapabilitiesInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadParameterSet<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterset: WICRawParameterSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadParameterSet(parameterset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentParameterSet<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcurrentparameterset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentParameterSet(::core::mem::transmute_copy(&ppcurrentparameterset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExposureCompensation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ev: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExposureCompensation(ev) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExposureCompensation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pev: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExposureCompensation(::core::mem::transmute_copy(&pev)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointRGB<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, red: u32, green: u32, blue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWhitePointRGB(red, green, blue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWhitePointRGB<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWhitePointRGB(::core::mem::transmute_copy(&pred), ::core::mem::transmute_copy(&pgreen), ::core::mem::transmute_copy(&pblue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamedWhitePoint<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepoint: WICNamedWhitePoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNamedWhitePoint(whitepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedWhitePoint<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwhitepoint: *mut WICNamedWhitePoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedWhitePoint(::core::mem::transmute_copy(&pwhitepoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointKelvin<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitepointkelvin: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWhitePointKelvin(whitepointkelvin) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWhitePointKelvin<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwhitepointkelvin: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWhitePointKelvin(::core::mem::transmute_copy(&pwhitepointkelvin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKelvinRangeInfo<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKelvinRangeInfo(::core::mem::transmute_copy(&pminkelvintemp), ::core::mem::transmute_copy(&pmaxkelvintemp), ::core::mem::transmute_copy(&pkelvintempstepvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContrast<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contrast: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContrast(contrast) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContrast<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrast: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContrast(::core::mem::transmute_copy(&pcontrast)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGamma<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGamma(gamma) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamma<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgamma: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGamma(::core::mem::transmute_copy(&pgamma)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharpness<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharpness: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSharpness(sharpness) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharpness<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharpness: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSharpness(::core::mem::transmute_copy(&psharpness)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaturation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, saturation: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSaturation(saturation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSaturation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psaturation: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSaturation(::core::mem::transmute_copy(&psaturation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTint<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tint: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTint(tint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTint<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptint: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTint(::core::mem::transmute_copy(&ptint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoiseReduction<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, noisereduction: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNoiseReduction(noisereduction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNoiseReduction<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnoisereduction: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNoiseReduction(::core::mem::transmute_copy(&pnoisereduction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationColorContext<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDestinationColorContext(&*(&pcolorcontext as *const <IWICColorContext as ::windows::core::Abi>::Abi as *const <IWICColorContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToneCurve<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetToneCurve(cbtonecurvesize, &*(&ptonecurve as *const <WICRawToneCurve as ::windows::core::Abi>::Abi as *const <WICRawToneCurve as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToneCurve<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetToneCurve(cbtonecurvebuffersize, ::core::mem::transmute_copy(&ptonecurve), pcbactualtonecurvebuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRotation(rotation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRotation<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protation: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRotation(::core::mem::transmute_copy(&protation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderMode<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rendermode: WICRawRenderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRenderMode(rendermode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRenderMode<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendermode: *mut WICRawRenderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRenderMode(::core::mem::transmute_copy(&prendermode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationCallback<Impl: IWICDevelopRawImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotificationCallback(&*(&pcallback as *const <IWICDevelopRawNotificationCallback as ::windows::core::Abi>::Abi as *const <IWICDevelopRawNotificationCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWICDevelopRaw>,
            ::windows::core::GetTrustLevel,
            QueryRawCapabilitiesInfo::<Impl, OFFSET>,
            LoadParameterSet::<Impl, OFFSET>,
            GetCurrentParameterSet::<Impl, OFFSET>,
            SetExposureCompensation::<Impl, OFFSET>,
            GetExposureCompensation::<Impl, OFFSET>,
            SetWhitePointRGB::<Impl, OFFSET>,
            GetWhitePointRGB::<Impl, OFFSET>,
            SetNamedWhitePoint::<Impl, OFFSET>,
            GetNamedWhitePoint::<Impl, OFFSET>,
            SetWhitePointKelvin::<Impl, OFFSET>,
            GetWhitePointKelvin::<Impl, OFFSET>,
            GetKelvinRangeInfo::<Impl, OFFSET>,
            SetContrast::<Impl, OFFSET>,
            GetContrast::<Impl, OFFSET>,
            SetGamma::<Impl, OFFSET>,
            GetGamma::<Impl, OFFSET>,
            SetSharpness::<Impl, OFFSET>,
            GetSharpness::<Impl, OFFSET>,
            SetSaturation::<Impl, OFFSET>,
            GetSaturation::<Impl, OFFSET>,
            SetTint::<Impl, OFFSET>,
            GetTint::<Impl, OFFSET>,
            SetNoiseReduction::<Impl, OFFSET>,
            GetNoiseReduction::<Impl, OFFSET>,
            SetDestinationColorContext::<Impl, OFFSET>,
            SetToneCurve::<Impl, OFFSET>,
            GetToneCurve::<Impl, OFFSET>,
            SetRotation::<Impl, OFFSET>,
            GetRotation::<Impl, OFFSET>,
            SetRenderMode::<Impl, OFFSET>,
            GetRenderMode::<Impl, OFFSET>,
            SetNotificationCallback::<Impl, OFFSET>,
        )
    }
}
pub trait IWICDevelopRawNotificationCallbackImpl: Sized {
    fn Notify();
}
impl ::windows::core::RuntimeName for IWICDevelopRawNotificationCallback {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICDevelopRawNotificationCallback";
}
impl IWICDevelopRawNotificationCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICDevelopRawNotificationCallbackImpl, const OFFSET: isize>() -> IWICDevelopRawNotificationCallbackVtbl {
        unsafe extern "system" fn Notify<Impl: IWICDevelopRawNotificationCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify(notificationmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICDevelopRawNotificationCallback>, ::windows::core::GetTrustLevel, Notify::<Impl, OFFSET>)
    }
}
pub trait IWICEnumMetadataItemImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IWICEnumMetadataItem {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICEnumMetadataItem";
}
impl IWICEnumMetadataItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICEnumMetadataItemImpl, const OFFSET: isize>() -> IWICEnumMetadataItemVtbl {
        unsafe extern "system" fn Next<Impl: IWICEnumMetadataItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(
                celt,
                &*(&rgeltschema as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&rgeltid as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&rgeltvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pceltfetched),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IWICEnumMetadataItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IWICEnumMetadataItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IWICEnumMetadataItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadataitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppienummetadataitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICEnumMetadataItem>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IWICFastMetadataEncoderImpl: Sized {
    fn Commit();
    fn GetMetadataQueryWriter();
}
impl ::windows::core::RuntimeName for IWICFastMetadataEncoder {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICFastMetadataEncoder";
}
impl IWICFastMetadataEncoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFastMetadataEncoderImpl, const OFFSET: isize>() -> IWICFastMetadataEncoderVtbl {
        unsafe extern "system" fn Commit<Impl: IWICFastMetadataEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Impl: IWICFastMetadataEncoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataQueryWriter(::core::mem::transmute_copy(&ppimetadataquerywriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICFastMetadataEncoder>, ::windows::core::GetTrustLevel, Commit::<Impl, OFFSET>, GetMetadataQueryWriter::<Impl, OFFSET>)
    }
}
pub trait IWICFormatConverterImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
    fn CanConvert();
}
impl ::windows::core::RuntimeName for IWICFormatConverter {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICFormatConverter";
}
impl IWICFormatConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverterImpl, const OFFSET: isize>() -> IWICFormatConverterVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisource: ::windows::core::RawPtr, dstformat: &::windows::core::GUID, dither: WICBitmapDitherType, pipalette: ::windows::core::RawPtr, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&pisource as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType),
                &*(&dstformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                dither,
                &*(&pipalette as *const <IWICPalette as ::windows::core::Abi>::Abi as *const <IWICPalette as ::windows::core::DefaultType>::DefaultType),
                alphathresholdpercent,
                palettetranslate,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanConvert<Impl: IWICFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcpixelformat: &::windows::core::GUID, dstpixelformat: &::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanConvert(&*(&srcpixelformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&dstpixelformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfcanconvert)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICFormatConverter>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, CanConvert::<Impl, OFFSET>)
    }
}
pub trait IWICFormatConverterInfoImpl: Sized + IWICComponentInfoImpl {
    fn GetPixelFormats();
    fn CreateInstance();
}
impl ::windows::core::RuntimeName for IWICFormatConverterInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICFormatConverterInfo";
}
impl IWICFormatConverterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICFormatConverterInfoImpl, const OFFSET: isize>() -> IWICFormatConverterInfoVtbl {
        unsafe extern "system" fn GetPixelFormats<Impl: IWICFormatConverterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, ppixelformatguids: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelFormats(cformats, &*(&ppixelformatguids as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICFormatConverterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&ppiconverter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICFormatConverterInfo>, ::windows::core::GetTrustLevel, GetPixelFormats::<Impl, OFFSET>, CreateInstance::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWICImagingFactory {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICImagingFactory";
}
impl IWICImagingFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICImagingFactoryImpl, const OFFSET: isize>() -> IWICImagingFactoryVtbl {
        unsafe extern "system" fn CreateDecoderFromFilename<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR, pguidvendor: &::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecoderFromFilename(&*(&wzfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pguidvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwdesiredaccess, metadataoptions, ::core::mem::transmute_copy(&ppidecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoderFromStream<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pguidvendor: &::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecoderFromStream(&*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&pguidvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), metadataoptions, ::core::mem::transmute_copy(&ppidecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoderFromFileHandle<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfile: usize, pguidvendor: &::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecoderFromFileHandle(hfile, &*(&pguidvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), metadataoptions, ::core::mem::transmute_copy(&ppidecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComponentInfo<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcomponent: &::windows::core::GUID, ppiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateComponentInfo(&*(&clsidcomponent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppiinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoder<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: &::windows::core::GUID, pguidvendor: &::windows::core::GUID, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecoder(&*(&guidcontainerformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pguidvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppidecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncoder<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: &::windows::core::GUID, pguidvendor: &::windows::core::GUID, ppiencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEncoder(&*(&guidcontainerformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pguidvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppiencoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePalette<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipalette: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePalette(::core::mem::transmute_copy(&ppipalette)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFormatConverter<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiformatconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFormatConverter(::core::mem::transmute_copy(&ppiformatconverter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapScaler<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapscaler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapScaler(::core::mem::transmute_copy(&ppibitmapscaler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapClipper<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapclipper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapClipper(::core::mem::transmute_copy(&ppibitmapclipper)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFlipRotator<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppibitmapfliprotator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFlipRotator(::core::mem::transmute_copy(&ppibitmapfliprotator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStream<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwicstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStream(::core::mem::transmute_copy(&ppiwicstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContext<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiccolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorContext(::core::mem::transmute_copy(&ppiwiccolorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorTransformer<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiccolortransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorTransformer(::core::mem::transmute_copy(&ppiwiccolortransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmap<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: &::windows::core::GUID, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmap(uiwidth, uiheight, &*(&pixelformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), option, ::core::mem::transmute_copy(&ppibitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromSource<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, option: WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromSource(&*(&pibitmapsource as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType), option, ::core::mem::transmute_copy(&ppibitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromSourceRect<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pibitmapsource: ::windows::core::RawPtr, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromSourceRect(&*(&pibitmapsource as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType), x, y, width, height, ::core::mem::transmute_copy(&ppibitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromMemory<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: &::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromMemory(uiwidth, uiheight, &*(&pixelformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cbstride, cbbuffersize, pbbuffer, ::core::mem::transmute_copy(&ppibitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromHBITMAP<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromHBITMAP(&*(&hbitmap as *const <super::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType), &*(&hpalette as *const <super::Gdi::HPALETTE as ::windows::core::Abi>::Abi as *const <super::Gdi::HPALETTE as ::windows::core::DefaultType>::DefaultType), options, ::core::mem::transmute_copy(&ppibitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromHICON<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapFromHICON(&*(&hicon as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppibitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComponentEnumerator<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, componenttypes: u32, options: u32, ppienumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateComponentEnumerator(componenttypes, options, ::core::mem::transmute_copy(&ppienumunknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromDecoder<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFastMetadataEncoderFromDecoder(&*(&pidecoder as *const <IWICBitmapDecoder as ::windows::core::Abi>::Abi as *const <IWICBitmapDecoder as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppifastencoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromFrameDecode<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piframedecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFastMetadataEncoderFromFrameDecode(&*(&piframedecoder as *const <IWICBitmapFrameDecode as ::windows::core::Abi>::Abi as *const <IWICBitmapFrameDecode as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppifastencoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriter<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidmetadataformat: &::windows::core::GUID, pguidvendor: &::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQueryWriter(&*(&guidmetadataformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pguidvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppiquerywriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromReader<Impl: IWICImagingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piqueryreader: ::windows::core::RawPtr, pguidvendor: &::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQueryWriterFromReader(&*(&piqueryreader as *const <IWICMetadataQueryReader as ::windows::core::Abi>::Abi as *const <IWICMetadataQueryReader as ::windows::core::DefaultType>::DefaultType), &*(&pguidvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppiquerywriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWICImagingFactory>,
            ::windows::core::GetTrustLevel,
            CreateDecoderFromFilename::<Impl, OFFSET>,
            CreateDecoderFromStream::<Impl, OFFSET>,
            CreateDecoderFromFileHandle::<Impl, OFFSET>,
            CreateComponentInfo::<Impl, OFFSET>,
            CreateDecoder::<Impl, OFFSET>,
            CreateEncoder::<Impl, OFFSET>,
            CreatePalette::<Impl, OFFSET>,
            CreateFormatConverter::<Impl, OFFSET>,
            CreateBitmapScaler::<Impl, OFFSET>,
            CreateBitmapClipper::<Impl, OFFSET>,
            CreateBitmapFlipRotator::<Impl, OFFSET>,
            CreateStream::<Impl, OFFSET>,
            CreateColorContext::<Impl, OFFSET>,
            CreateColorTransformer::<Impl, OFFSET>,
            CreateBitmap::<Impl, OFFSET>,
            CreateBitmapFromSource::<Impl, OFFSET>,
            CreateBitmapFromSourceRect::<Impl, OFFSET>,
            CreateBitmapFromMemory::<Impl, OFFSET>,
            CreateBitmapFromHBITMAP::<Impl, OFFSET>,
            CreateBitmapFromHICON::<Impl, OFFSET>,
            CreateComponentEnumerator::<Impl, OFFSET>,
            CreateFastMetadataEncoderFromDecoder::<Impl, OFFSET>,
            CreateFastMetadataEncoderFromFrameDecode::<Impl, OFFSET>,
            CreateQueryWriter::<Impl, OFFSET>,
            CreateQueryWriterFromReader::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IWICJpegFrameDecode {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICJpegFrameDecode";
}
impl IWICJpegFrameDecodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>() -> IWICJpegFrameDecodeVtbl {
        unsafe extern "system" fn DoesSupportIndexing<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfindexingsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportIndexing(::core::mem::transmute_copy(&pfindexingsupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexing<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIndexing(options, horizontalintervalsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearIndexing<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearIndexing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAcHuffmanTable<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAcHuffmanTable(scanindex, tableindex, ::core::mem::transmute_copy(&pachuffmantable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDcHuffmanTable<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDcHuffmanTable(scanindex, tableindex, ::core::mem::transmute_copy(&pdchuffmantable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuantizationTable<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQuantizationTable(scanindex, tableindex, ::core::mem::transmute_copy(&pquantizationtable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameHeader<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframeheader: *mut WICJpegFrameHeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameHeader(::core::mem::transmute_copy(&pframeheader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScanHeader<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScanHeader(scanindex, ::core::mem::transmute_copy(&pscanheader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyScan<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyScan(scanindex, scanoffset, cbscandata, ::core::mem::transmute_copy(&pbscandata), ::core::mem::transmute_copy(&pcbscandataactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyMinimalStream<Impl: IWICJpegFrameDecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyMinimalStream(streamoffset, cbstreamdata, ::core::mem::transmute_copy(&pbstreamdata), ::core::mem::transmute_copy(&pcbstreamdataactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWICJpegFrameDecode>,
            ::windows::core::GetTrustLevel,
            DoesSupportIndexing::<Impl, OFFSET>,
            SetIndexing::<Impl, OFFSET>,
            ClearIndexing::<Impl, OFFSET>,
            GetAcHuffmanTable::<Impl, OFFSET>,
            GetDcHuffmanTable::<Impl, OFFSET>,
            GetQuantizationTable::<Impl, OFFSET>,
            GetFrameHeader::<Impl, OFFSET>,
            GetScanHeader::<Impl, OFFSET>,
            CopyScan::<Impl, OFFSET>,
            CopyMinimalStream::<Impl, OFFSET>,
        )
    }
}
pub trait IWICJpegFrameEncodeImpl: Sized {
    fn GetAcHuffmanTable();
    fn GetDcHuffmanTable();
    fn GetQuantizationTable();
    fn WriteScan();
}
impl ::windows::core::RuntimeName for IWICJpegFrameEncode {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICJpegFrameEncode";
}
impl IWICJpegFrameEncodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICJpegFrameEncodeImpl, const OFFSET: isize>() -> IWICJpegFrameEncodeVtbl {
        unsafe extern "system" fn GetAcHuffmanTable<Impl: IWICJpegFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAcHuffmanTable(scanindex, tableindex, ::core::mem::transmute_copy(&pachuffmantable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDcHuffmanTable<Impl: IWICJpegFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDcHuffmanTable(scanindex, tableindex, ::core::mem::transmute_copy(&pdchuffmantable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuantizationTable<Impl: IWICJpegFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQuantizationTable(scanindex, tableindex, ::core::mem::transmute_copy(&pquantizationtable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteScan<Impl: IWICJpegFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbscandata: u32, pbscandata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteScan(cbscandata, pbscandata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICJpegFrameEncode>, ::windows::core::GetTrustLevel, GetAcHuffmanTable::<Impl, OFFSET>, GetDcHuffmanTable::<Impl, OFFSET>, GetQuantizationTable::<Impl, OFFSET>, WriteScan::<Impl, OFFSET>)
    }
}
pub trait IWICMetadataBlockReaderImpl: Sized {
    fn GetContainerFormat();
    fn GetCount();
    fn GetReaderByIndex();
    fn GetEnumerator();
}
impl ::windows::core::RuntimeName for IWICMetadataBlockReader {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICMetadataBlockReader";
}
impl IWICMetadataBlockReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockReaderImpl, const OFFSET: isize>() -> IWICMetadataBlockReaderVtbl {
        unsafe extern "system" fn GetContainerFormat<Impl: IWICMetadataBlockReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerFormat(::core::mem::transmute_copy(&pguidcontainerformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IWICMetadataBlockReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReaderByIndex<Impl: IWICMetadataBlockReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatareader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReaderByIndex(nindex, ::core::mem::transmute_copy(&ppimetadatareader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IWICMetadataBlockReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator(::core::mem::transmute_copy(&ppienummetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICMetadataBlockReader>, ::windows::core::GetTrustLevel, GetContainerFormat::<Impl, OFFSET>, GetCount::<Impl, OFFSET>, GetReaderByIndex::<Impl, OFFSET>, GetEnumerator::<Impl, OFFSET>)
    }
}
pub trait IWICMetadataBlockWriterImpl: Sized + IWICMetadataBlockReaderImpl {
    fn InitializeFromBlockReader();
    fn GetWriterByIndex();
    fn AddWriter();
    fn SetWriterByIndex();
    fn RemoveWriterByIndex();
}
impl ::windows::core::RuntimeName for IWICMetadataBlockWriter {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICMetadataBlockWriter";
}
impl IWICMetadataBlockWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataBlockWriterImpl, const OFFSET: isize>() -> IWICMetadataBlockWriterVtbl {
        unsafe extern "system" fn InitializeFromBlockReader<Impl: IWICMetadataBlockWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimdblockreader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeFromBlockReader(&*(&pimdblockreader as *const <IWICMetadataBlockReader as ::windows::core::Abi>::Abi as *const <IWICMetadataBlockReader as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriterByIndex<Impl: IWICMetadataBlockWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatawriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWriterByIndex(nindex, ::core::mem::transmute_copy(&ppimetadatawriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWriter<Impl: IWICMetadataBlockWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimetadatawriter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWriter(&*(&pimetadatawriter as *const <IWICMetadataWriter as ::windows::core::Abi>::Abi as *const <IWICMetadataWriter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriterByIndex<Impl: IWICMetadataBlockWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pimetadatawriter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWriterByIndex(nindex, &*(&pimetadatawriter as *const <IWICMetadataWriter as ::windows::core::Abi>::Abi as *const <IWICMetadataWriter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWriterByIndex<Impl: IWICMetadataBlockWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveWriterByIndex(nindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICMetadataBlockWriter>, ::windows::core::GetTrustLevel, InitializeFromBlockReader::<Impl, OFFSET>, GetWriterByIndex::<Impl, OFFSET>, AddWriter::<Impl, OFFSET>, SetWriterByIndex::<Impl, OFFSET>, RemoveWriterByIndex::<Impl, OFFSET>)
    }
}
pub trait IWICMetadataHandlerInfoImpl: Sized + IWICComponentInfoImpl {
    fn GetMetadataFormat();
    fn GetContainerFormats();
    fn GetDeviceManufacturer();
    fn GetDeviceModels();
    fn DoesRequireFullStream();
    fn DoesSupportPadding();
    fn DoesRequireFixedSize();
}
impl ::windows::core::RuntimeName for IWICMetadataHandlerInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICMetadataHandlerInfo";
}
impl IWICMetadataHandlerInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>() -> IWICMetadataHandlerInfoVtbl {
        unsafe extern "system" fn GetMetadataFormat<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataFormat(::core::mem::transmute_copy(&pguidmetadataformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerFormats<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerFormats(ccontainerformats, &*(&pguidcontainerformats as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceManufacturer<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceManufacturer(cchdevicemanufacturer, &*(&wzdevicemanufacturer as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceModels<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: super::super::Foundation::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceModels(cchdevicemodels, &*(&wzdevicemodels as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesRequireFullStream<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrequiresfullstream: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesRequireFullStream(::core::mem::transmute_copy(&pfrequiresfullstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportPadding<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportspadding: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportPadding(::core::mem::transmute_copy(&pfsupportspadding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesRequireFixedSize<Impl: IWICMetadataHandlerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffixedsize: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesRequireFixedSize(::core::mem::transmute_copy(&pffixedsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWICMetadataHandlerInfo>,
            ::windows::core::GetTrustLevel,
            GetMetadataFormat::<Impl, OFFSET>,
            GetContainerFormats::<Impl, OFFSET>,
            GetDeviceManufacturer::<Impl, OFFSET>,
            GetDeviceModels::<Impl, OFFSET>,
            DoesRequireFullStream::<Impl, OFFSET>,
            DoesSupportPadding::<Impl, OFFSET>,
            DoesRequireFixedSize::<Impl, OFFSET>,
        )
    }
}
pub trait IWICMetadataQueryReaderImpl: Sized {
    fn GetContainerFormat();
    fn GetLocation();
    fn GetMetadataByName();
    fn GetEnumerator();
}
impl ::windows::core::RuntimeName for IWICMetadataQueryReader {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICMetadataQueryReader";
}
impl IWICMetadataQueryReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryReaderImpl, const OFFSET: isize>() -> IWICMetadataQueryReaderVtbl {
        unsafe extern "system" fn GetContainerFormat<Impl: IWICMetadataQueryReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerFormat(::core::mem::transmute_copy(&pguidcontainerformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocation<Impl: IWICMetadataQueryReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchmaxlength: u32, wznamespace: super::super::Foundation::PWSTR, pcchactuallength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocation(cchmaxlength, &*(&wznamespace as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactuallength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataByName<Impl: IWICMetadataQueryReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataByName(&*(&wzname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvarvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IWICMetadataQueryReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienumstring: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator(::core::mem::transmute_copy(&ppienumstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICMetadataQueryReader>, ::windows::core::GetTrustLevel, GetContainerFormat::<Impl, OFFSET>, GetLocation::<Impl, OFFSET>, GetMetadataByName::<Impl, OFFSET>, GetEnumerator::<Impl, OFFSET>)
    }
}
pub trait IWICMetadataQueryWriterImpl: Sized + IWICMetadataQueryReaderImpl {
    fn SetMetadataByName();
    fn RemoveMetadataByName();
}
impl ::windows::core::RuntimeName for IWICMetadataQueryWriter {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICMetadataQueryWriter";
}
impl IWICMetadataQueryWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataQueryWriterImpl, const OFFSET: isize>() -> IWICMetadataQueryWriterVtbl {
        unsafe extern "system" fn SetMetadataByName<Impl: IWICMetadataQueryWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMetadataByName(&*(&wzname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvarvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMetadataByName<Impl: IWICMetadataQueryWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveMetadataByName(&*(&wzname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICMetadataQueryWriter>, ::windows::core::GetTrustLevel, SetMetadataByName::<Impl, OFFSET>, RemoveMetadataByName::<Impl, OFFSET>)
    }
}
pub trait IWICMetadataReaderImpl: Sized {
    fn GetMetadataFormat();
    fn GetMetadataHandlerInfo();
    fn GetCount();
    fn GetValueByIndex();
    fn GetValue();
    fn GetEnumerator();
}
impl ::windows::core::RuntimeName for IWICMetadataReader {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICMetadataReader";
}
impl IWICMetadataReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReaderImpl, const OFFSET: isize>() -> IWICMetadataReaderVtbl {
        unsafe extern "system" fn GetMetadataFormat<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataFormat(::core::mem::transmute_copy(&pguidmetadataformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataHandlerInfo<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppihandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataHandlerInfo(::core::mem::transmute_copy(&ppihandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueByIndex<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValueByIndex(
                nindex,
                &*(&pvarschema as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarid as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(
                &*(&pvarschema as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarid as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IWICMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator(::core::mem::transmute_copy(&ppienummetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICMetadataReader>, ::windows::core::GetTrustLevel, GetMetadataFormat::<Impl, OFFSET>, GetMetadataHandlerInfo::<Impl, OFFSET>, GetCount::<Impl, OFFSET>, GetValueByIndex::<Impl, OFFSET>, GetValue::<Impl, OFFSET>, GetEnumerator::<Impl, OFFSET>)
    }
}
pub trait IWICMetadataReaderInfoImpl: Sized + IWICMetadataHandlerInfoImpl + IWICComponentInfoImpl {
    fn GetPatterns();
    fn MatchesPattern();
    fn CreateInstance();
}
impl ::windows::core::RuntimeName for IWICMetadataReaderInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICMetadataReaderInfo";
}
impl IWICMetadataReaderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataReaderInfoImpl, const OFFSET: isize>() -> IWICMetadataReaderInfoVtbl {
        unsafe extern "system" fn GetPatterns<Impl: IWICMetadataReaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: &::windows::core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatterns(&*(&guidcontainerformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cbsize, ::core::mem::transmute_copy(&ppattern), ::core::mem::transmute_copy(&pccount), ::core::mem::transmute_copy(&pcbactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesPattern<Impl: IWICMetadataReaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: &::windows::core::GUID, pistream: ::windows::core::RawPtr, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchesPattern(&*(&guidcontainerformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfmatches)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICMetadataReaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppireader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&ppireader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICMetadataReaderInfo>, ::windows::core::GetTrustLevel, GetPatterns::<Impl, OFFSET>, MatchesPattern::<Impl, OFFSET>, CreateInstance::<Impl, OFFSET>)
    }
}
pub trait IWICMetadataWriterImpl: Sized + IWICMetadataReaderImpl {
    fn SetValue();
    fn SetValueByIndex();
    fn RemoveValue();
    fn RemoveValueByIndex();
}
impl ::windows::core::RuntimeName for IWICMetadataWriter {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICMetadataWriter";
}
impl IWICMetadataWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriterImpl, const OFFSET: isize>() -> IWICMetadataWriterVtbl {
        unsafe extern "system" fn SetValue<Impl: IWICMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(
                &*(&pvarschema as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarid as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueByIndex<Impl: IWICMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValueByIndex(
                nindex,
                &*(&pvarschema as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarid as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValue<Impl: IWICMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveValue(&*(&pvarschema as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), &*(&pvarid as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValueByIndex<Impl: IWICMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveValueByIndex(nindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICMetadataWriter>, ::windows::core::GetTrustLevel, SetValue::<Impl, OFFSET>, SetValueByIndex::<Impl, OFFSET>, RemoveValue::<Impl, OFFSET>, RemoveValueByIndex::<Impl, OFFSET>)
    }
}
pub trait IWICMetadataWriterInfoImpl: Sized + IWICMetadataHandlerInfoImpl + IWICComponentInfoImpl {
    fn GetHeader();
    fn CreateInstance();
}
impl ::windows::core::RuntimeName for IWICMetadataWriterInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICMetadataWriterInfo";
}
impl IWICMetadataWriterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICMetadataWriterInfoImpl, const OFFSET: isize>() -> IWICMetadataWriterInfoVtbl {
        unsafe extern "system" fn GetHeader<Impl: IWICMetadataWriterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcontainerformat: &::windows::core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeader(&*(&guidcontainerformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cbsize, ::core::mem::transmute_copy(&pheader), ::core::mem::transmute_copy(&pcbactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: IWICMetadataWriterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&ppiwriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICMetadataWriterInfo>, ::windows::core::GetTrustLevel, GetHeader::<Impl, OFFSET>, CreateInstance::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWICPalette {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICPalette";
}
impl IWICPaletteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPaletteImpl, const OFFSET: isize>() -> IWICPaletteVtbl {
        unsafe extern "system" fn InitializePredefined<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializePredefined(epalettetype, &*(&faddtransparentcolor as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeCustom<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolors: *const u32, ccount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeCustom(pcolors, ccount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeFromBitmap<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisurface: ::windows::core::RawPtr, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeFromBitmap(&*(&pisurface as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType), ccount, &*(&faddtransparentcolor as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeFromPalette<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipalette: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeFromPalette(&*(&pipalette as *const <IWICPalette as ::windows::core::Abi>::Abi as *const <IWICPalette as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pepalettetype: *mut WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pepalettetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorCount<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorCount(::core::mem::transmute_copy(&pccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColors<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColors(ccount, ::core::mem::transmute_copy(&pcolors), ::core::mem::transmute_copy(&pcactualcolors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBlackWhite<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisblackwhite: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBlackWhite(::core::mem::transmute_copy(&pfisblackwhite)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGrayscale<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisgrayscale: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGrayscale(::core::mem::transmute_copy(&pfisgrayscale)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasAlpha<Impl: IWICPaletteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfhasalpha: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasAlpha(::core::mem::transmute_copy(&pfhasalpha)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWICPalette>,
            ::windows::core::GetTrustLevel,
            InitializePredefined::<Impl, OFFSET>,
            InitializeCustom::<Impl, OFFSET>,
            InitializeFromBitmap::<Impl, OFFSET>,
            InitializeFromPalette::<Impl, OFFSET>,
            GetType::<Impl, OFFSET>,
            GetColorCount::<Impl, OFFSET>,
            GetColors::<Impl, OFFSET>,
            IsBlackWhite::<Impl, OFFSET>,
            IsGrayscale::<Impl, OFFSET>,
            HasAlpha::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICPersistStreamImpl: Sized + IPersistStreamImpl + IPersistImpl {
    fn LoadEx();
    fn SaveEx();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWICPersistStream {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICPersistStream";
}
#[cfg(feature = "Win32_System_Com")]
impl IWICPersistStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPersistStreamImpl, const OFFSET: isize>() -> IWICPersistStreamVtbl {
        unsafe extern "system" fn LoadEx<Impl: IWICPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pguidpreferredvendor: &::windows::core::GUID, dwpersistoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadEx(&*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&pguidpreferredvendor as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwpersistoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveEx<Impl: IWICPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveEx(&*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), dwpersistoptions, &*(&fcleardirty as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICPersistStream>, ::windows::core::GetTrustLevel, LoadEx::<Impl, OFFSET>, SaveEx::<Impl, OFFSET>)
    }
}
pub trait IWICPixelFormatInfoImpl: Sized + IWICComponentInfoImpl {
    fn GetFormatGUID();
    fn GetColorContext();
    fn GetBitsPerPixel();
    fn GetChannelCount();
    fn GetChannelMask();
}
impl ::windows::core::RuntimeName for IWICPixelFormatInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICPixelFormatInfo";
}
impl IWICPixelFormatInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfoImpl, const OFFSET: isize>() -> IWICPixelFormatInfoVtbl {
        unsafe extern "system" fn GetFormatGUID<Impl: IWICPixelFormatInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatGUID(::core::mem::transmute_copy(&pformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContext<Impl: IWICPixelFormatInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorContext(::core::mem::transmute_copy(&ppicolorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitsPerPixel<Impl: IWICPixelFormatInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puibitsperpixel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitsPerPixel(::core::mem::transmute_copy(&puibitsperpixel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelCount<Impl: IWICPixelFormatInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puichannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount(::core::mem::transmute_copy(&puichannelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelMask<Impl: IWICPixelFormatInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelMask(uichannelindex, cbmaskbuffer, pbmaskbuffer, ::core::mem::transmute_copy(&pcbactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICPixelFormatInfo>, ::windows::core::GetTrustLevel, GetFormatGUID::<Impl, OFFSET>, GetColorContext::<Impl, OFFSET>, GetBitsPerPixel::<Impl, OFFSET>, GetChannelCount::<Impl, OFFSET>, GetChannelMask::<Impl, OFFSET>)
    }
}
pub trait IWICPixelFormatInfo2Impl: Sized + IWICPixelFormatInfoImpl + IWICComponentInfoImpl {
    fn SupportsTransparency();
    fn GetNumericRepresentation();
}
impl ::windows::core::RuntimeName for IWICPixelFormatInfo2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICPixelFormatInfo2";
}
impl IWICPixelFormatInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPixelFormatInfo2Impl, const OFFSET: isize>() -> IWICPixelFormatInfo2Vtbl {
        unsafe extern "system" fn SupportsTransparency<Impl: IWICPixelFormatInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsupportstransparency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsTransparency(::core::mem::transmute_copy(&pfsupportstransparency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericRepresentation<Impl: IWICPixelFormatInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumericrepresentation: *mut WICPixelFormatNumericRepresentation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumericRepresentation(::core::mem::transmute_copy(&pnumericrepresentation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICPixelFormatInfo2>, ::windows::core::GetTrustLevel, SupportsTransparency::<Impl, OFFSET>, GetNumericRepresentation::<Impl, OFFSET>)
    }
}
pub trait IWICPlanarBitmapFrameEncodeImpl: Sized {
    fn WritePixels();
    fn WriteSource();
}
impl ::windows::core::RuntimeName for IWICPlanarBitmapFrameEncode {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICPlanarBitmapFrameEncode";
}
impl IWICPlanarBitmapFrameEncodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapFrameEncodeImpl, const OFFSET: isize>() -> IWICPlanarBitmapFrameEncodeVtbl {
        unsafe extern "system" fn WritePixels<Impl: IWICPlanarBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WritePixels(linecount, &*(&pplanes as *const <WICBitmapPlane as ::windows::core::Abi>::Abi as *const <WICBitmapPlane as ::windows::core::DefaultType>::DefaultType), cplanes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSource<Impl: IWICPlanarBitmapFrameEncodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplanes: *const ::windows::core::RawPtr, cplanes: u32, prcsource: *const WICRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteSource(&*(&ppplanes as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType), cplanes, &*(&prcsource as *const <WICRect as ::windows::core::Abi>::Abi as *const <WICRect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICPlanarBitmapFrameEncode>, ::windows::core::GetTrustLevel, WritePixels::<Impl, OFFSET>, WriteSource::<Impl, OFFSET>)
    }
}
pub trait IWICPlanarBitmapSourceTransformImpl: Sized {
    fn DoesSupportTransform();
    fn CopyPixels();
}
impl ::windows::core::RuntimeName for IWICPlanarBitmapSourceTransform {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICPlanarBitmapSourceTransform";
}
impl IWICPlanarBitmapSourceTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarBitmapSourceTransformImpl, const OFFSET: isize>() -> IWICPlanarBitmapSourceTransformVtbl {
        unsafe extern "system" fn DoesSupportTransform<Impl: IWICPlanarBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: &::windows::core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesSupportTransform(puiwidth, puiheight, dsttransform, dstplanaroptions, &*(&pguiddstformats as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplanedescriptions), cplanes, ::core::mem::transmute_copy(&pfissupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyPixels<Impl: IWICPlanarBitmapSourceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyPixels(&*(&prcsource as *const <WICRect as ::windows::core::Abi>::Abi as *const <WICRect as ::windows::core::DefaultType>::DefaultType), uiwidth, uiheight, dsttransform, dstplanaroptions, &*(&pdstplanes as *const <WICBitmapPlane as ::windows::core::Abi>::Abi as *const <WICBitmapPlane as ::windows::core::DefaultType>::DefaultType), cplanes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICPlanarBitmapSourceTransform>, ::windows::core::GetTrustLevel, DoesSupportTransform::<Impl, OFFSET>, CopyPixels::<Impl, OFFSET>)
    }
}
pub trait IWICPlanarFormatConverterImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
    fn CanConvert();
}
impl ::windows::core::RuntimeName for IWICPlanarFormatConverter {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICPlanarFormatConverter";
}
impl IWICPlanarFormatConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICPlanarFormatConverterImpl, const OFFSET: isize>() -> IWICPlanarFormatConverterVtbl {
        unsafe extern "system" fn Initialize<Impl: IWICPlanarFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplanes: *const ::windows::core::RawPtr, cplanes: u32, dstformat: &::windows::core::GUID, dither: WICBitmapDitherType, pipalette: ::windows::core::RawPtr, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&ppplanes as *const <IWICBitmapSource as ::windows::core::Abi>::Abi as *const <IWICBitmapSource as ::windows::core::DefaultType>::DefaultType),
                cplanes,
                &*(&dstformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                dither,
                &*(&pipalette as *const <IWICPalette as ::windows::core::Abi>::Abi as *const <IWICPalette as ::windows::core::DefaultType>::DefaultType),
                alphathresholdpercent,
                palettetranslate,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanConvert<Impl: IWICPlanarFormatConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcpixelformats: &::windows::core::GUID, csrcplanes: u32, dstpixelformat: &::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanConvert(&*(&psrcpixelformats as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), csrcplanes, &*(&dstpixelformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfcanconvert)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICPlanarFormatConverter>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, CanConvert::<Impl, OFFSET>)
    }
}
pub trait IWICProgressCallbackImpl: Sized {
    fn Notify();
}
impl ::windows::core::RuntimeName for IWICProgressCallback {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICProgressCallback";
}
impl IWICProgressCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressCallbackImpl, const OFFSET: isize>() -> IWICProgressCallbackVtbl {
        unsafe extern "system" fn Notify<Impl: IWICProgressCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify(uframenum, operation, dblprogress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICProgressCallback>, ::windows::core::GetTrustLevel, Notify::<Impl, OFFSET>)
    }
}
pub trait IWICProgressiveLevelControlImpl: Sized {
    fn GetLevelCount();
    fn GetCurrentLevel();
    fn SetCurrentLevel();
}
impl ::windows::core::RuntimeName for IWICProgressiveLevelControl {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICProgressiveLevelControl";
}
impl IWICProgressiveLevelControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICProgressiveLevelControlImpl, const OFFSET: isize>() -> IWICProgressiveLevelControlVtbl {
        unsafe extern "system" fn GetLevelCount<Impl: IWICProgressiveLevelControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevelCount(::core::mem::transmute_copy(&pclevels)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLevel<Impl: IWICProgressiveLevelControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentLevel(::core::mem::transmute_copy(&pnlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentLevel<Impl: IWICProgressiveLevelControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCurrentLevel(nlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICProgressiveLevelControl>, ::windows::core::GetTrustLevel, GetLevelCount::<Impl, OFFSET>, GetCurrentLevel::<Impl, OFFSET>, SetCurrentLevel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICStreamImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn InitializeFromIStream();
    fn InitializeFromFilename();
    fn InitializeFromMemory();
    fn InitializeFromIStreamRegion();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWICStream {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICStream";
}
#[cfg(feature = "Win32_System_Com")]
impl IWICStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICStreamImpl, const OFFSET: isize>() -> IWICStreamVtbl {
        unsafe extern "system" fn InitializeFromIStream<Impl: IWICStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeFromIStream(&*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeFromFilename<Impl: IWICStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wzfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeFromFilename(&*(&wzfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwdesiredaccess) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeFromMemory<Impl: IWICStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeFromMemory(pbbuffer, cbbuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeFromIStreamRegion<Impl: IWICStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, uloffset: u64, ulmaxsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeFromIStreamRegion(&*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), uloffset, ulmaxsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICStream>, ::windows::core::GetTrustLevel, InitializeFromIStream::<Impl, OFFSET>, InitializeFromFilename::<Impl, OFFSET>, InitializeFromMemory::<Impl, OFFSET>, InitializeFromIStreamRegion::<Impl, OFFSET>)
    }
}
pub trait IWICStreamProviderImpl: Sized {
    fn GetStream();
    fn GetPersistOptions();
    fn GetPreferredVendorGUID();
    fn RefreshStream();
}
impl ::windows::core::RuntimeName for IWICStreamProvider {
    const NAME: &'static str = "Windows.Win32.Graphics.Imaging.IWICStreamProvider";
}
impl IWICStreamProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWICStreamProviderImpl, const OFFSET: isize>() -> IWICStreamProviderVtbl {
        unsafe extern "system" fn GetStream<Impl: IWICStreamProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&ppistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPersistOptions<Impl: IWICStreamProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpersistoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPersistOptions(::core::mem::transmute_copy(&pdwpersistoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredVendorGUID<Impl: IWICStreamProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidpreferredvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredVendorGUID(::core::mem::transmute_copy(&pguidpreferredvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshStream<Impl: IWICStreamProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWICStreamProvider>, ::windows::core::GetTrustLevel, GetStream::<Impl, OFFSET>, GetPersistOptions::<Impl, OFFSET>, GetPreferredVendorGUID::<Impl, OFFSET>, RefreshStream::<Impl, OFFSET>)
    }
}
