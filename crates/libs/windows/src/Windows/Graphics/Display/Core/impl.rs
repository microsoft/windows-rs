#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHdmiDisplayInformationImpl: Sized {
    fn GetSupportedDisplayModes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HdmiDisplayMode>>;
    fn GetCurrentDisplayMode(&self) -> ::windows::core::Result<HdmiDisplayMode>;
    fn SetDefaultDisplayModeAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn RequestSetCurrentDisplayModeAsync(&self, mode: &::core::option::Option<HdmiDisplayMode>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestSetCurrentDisplayModeWithHdrAsync(&self, mode: &::core::option::Option<HdmiDisplayMode>, hdroption: HdmiDisplayHdrOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestSetCurrentDisplayModeWithHdrAndMetadataAsync(&self, mode: &::core::option::Option<HdmiDisplayMode>, hdroption: HdmiDisplayHdrOption, hdrmetadata: &HdmiDisplayHdr2086Metadata) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn DisplayModesChanged(&self, value: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HdmiDisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayModesChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHdmiDisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.Core.IHdmiDisplayInformation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHdmiDisplayInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHdmiDisplayInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHdmiDisplayInformationVtbl {
        unsafe extern "system" fn GetSupportedDisplayModes<Impl: IHdmiDisplayInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedDisplayModes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentDisplayMode<Impl: IHdmiDisplayInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentDisplayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultDisplayModeAsync<Impl: IHdmiDisplayInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultDisplayModeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestSetCurrentDisplayModeAsync<Impl: IHdmiDisplayInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSetCurrentDisplayModeAsync(&*(&mode as *const <HdmiDisplayMode as ::windows::core::Abi>::Abi as *const <HdmiDisplayMode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestSetCurrentDisplayModeWithHdrAsync<Impl: IHdmiDisplayInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: ::windows::core::RawPtr, hdroption: HdmiDisplayHdrOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSetCurrentDisplayModeWithHdrAsync(&*(&mode as *const <HdmiDisplayMode as ::windows::core::Abi>::Abi as *const <HdmiDisplayMode as ::windows::core::DefaultType>::DefaultType), hdroption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestSetCurrentDisplayModeWithHdrAndMetadataAsync<Impl: IHdmiDisplayInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: ::windows::core::RawPtr, hdroption: HdmiDisplayHdrOption, hdrmetadata: HdmiDisplayHdr2086Metadata, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSetCurrentDisplayModeWithHdrAndMetadataAsync(&*(&mode as *const <HdmiDisplayMode as ::windows::core::Abi>::Abi as *const <HdmiDisplayMode as ::windows::core::DefaultType>::DefaultType), hdroption, &*(&hdrmetadata as *const <HdmiDisplayHdr2086Metadata as ::windows::core::Abi>::Abi as *const <HdmiDisplayHdr2086Metadata as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayModesChanged<Impl: IHdmiDisplayInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayModesChanged(&*(&value as *const <super::super::super::Foundation::TypedEventHandler<HdmiDisplayInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<HdmiDisplayInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisplayModesChanged<Impl: IHdmiDisplayInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisplayModesChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHdmiDisplayInformation, BASE_OFFSET>(),
            GetSupportedDisplayModes: GetSupportedDisplayModes::<Impl, IMPL_OFFSET>,
            GetCurrentDisplayMode: GetCurrentDisplayMode::<Impl, IMPL_OFFSET>,
            SetDefaultDisplayModeAsync: SetDefaultDisplayModeAsync::<Impl, IMPL_OFFSET>,
            RequestSetCurrentDisplayModeAsync: RequestSetCurrentDisplayModeAsync::<Impl, IMPL_OFFSET>,
            RequestSetCurrentDisplayModeWithHdrAsync: RequestSetCurrentDisplayModeWithHdrAsync::<Impl, IMPL_OFFSET>,
            RequestSetCurrentDisplayModeWithHdrAndMetadataAsync: RequestSetCurrentDisplayModeWithHdrAndMetadataAsync::<Impl, IMPL_OFFSET>,
            DisplayModesChanged: DisplayModesChanged::<Impl, IMPL_OFFSET>,
            RemoveDisplayModesChanged: RemoveDisplayModesChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHdmiDisplayInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHdmiDisplayInformationStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<HdmiDisplayInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHdmiDisplayInformationStatics {
    const NAME: &'static str = "Windows.Graphics.Display.Core.IHdmiDisplayInformationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHdmiDisplayInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHdmiDisplayInformationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHdmiDisplayInformationStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IHdmiDisplayInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHdmiDisplayInformationStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHdmiDisplayInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHdmiDisplayModeImpl: Sized {
    fn ResolutionWidthInRawPixels(&self) -> ::windows::core::Result<u32>;
    fn ResolutionHeightInRawPixels(&self) -> ::windows::core::Result<u32>;
    fn RefreshRate(&self) -> ::windows::core::Result<f64>;
    fn StereoEnabled(&self) -> ::windows::core::Result<bool>;
    fn BitsPerPixel(&self) -> ::windows::core::Result<u16>;
    fn IsEqual(&self, mode: &::core::option::Option<HdmiDisplayMode>) -> ::windows::core::Result<bool>;
    fn ColorSpace(&self) -> ::windows::core::Result<HdmiDisplayColorSpace>;
    fn PixelEncoding(&self) -> ::windows::core::Result<HdmiDisplayPixelEncoding>;
    fn IsSdrLuminanceSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSmpte2084Supported(&self) -> ::windows::core::Result<bool>;
    fn Is2086MetadataSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHdmiDisplayMode {
    const NAME: &'static str = "Windows.Graphics.Display.Core.IHdmiDisplayMode";
}
#[cfg(feature = "implement_exclusive")]
impl IHdmiDisplayModeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHdmiDisplayModeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHdmiDisplayModeVtbl {
        unsafe extern "system" fn ResolutionWidthInRawPixels<Impl: IHdmiDisplayModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolutionWidthInRawPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolutionHeightInRawPixels<Impl: IHdmiDisplayModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolutionHeightInRawPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshRate<Impl: IHdmiDisplayModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StereoEnabled<Impl: IHdmiDisplayModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StereoEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitsPerPixel<Impl: IHdmiDisplayModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitsPerPixel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IHdmiDisplayModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&mode as *const <HdmiDisplayMode as ::windows::core::Abi>::Abi as *const <HdmiDisplayMode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorSpace<Impl: IHdmiDisplayModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HdmiDisplayColorSpace) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelEncoding<Impl: IHdmiDisplayModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HdmiDisplayPixelEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSdrLuminanceSupported<Impl: IHdmiDisplayModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSdrLuminanceSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSmpte2084Supported<Impl: IHdmiDisplayModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSmpte2084Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is2086MetadataSupported<Impl: IHdmiDisplayModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Is2086MetadataSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHdmiDisplayMode, BASE_OFFSET>(),
            ResolutionWidthInRawPixels: ResolutionWidthInRawPixels::<Impl, IMPL_OFFSET>,
            ResolutionHeightInRawPixels: ResolutionHeightInRawPixels::<Impl, IMPL_OFFSET>,
            RefreshRate: RefreshRate::<Impl, IMPL_OFFSET>,
            StereoEnabled: StereoEnabled::<Impl, IMPL_OFFSET>,
            BitsPerPixel: BitsPerPixel::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            ColorSpace: ColorSpace::<Impl, IMPL_OFFSET>,
            PixelEncoding: PixelEncoding::<Impl, IMPL_OFFSET>,
            IsSdrLuminanceSupported: IsSdrLuminanceSupported::<Impl, IMPL_OFFSET>,
            IsSmpte2084Supported: IsSmpte2084Supported::<Impl, IMPL_OFFSET>,
            Is2086MetadataSupported: Is2086MetadataSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHdmiDisplayMode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHdmiDisplayMode2Impl: Sized {
    fn IsDolbyVisionLowLatencySupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHdmiDisplayMode2 {
    const NAME: &'static str = "Windows.Graphics.Display.Core.IHdmiDisplayMode2";
}
#[cfg(feature = "implement_exclusive")]
impl IHdmiDisplayMode2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHdmiDisplayMode2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHdmiDisplayMode2Vtbl {
        unsafe extern "system" fn IsDolbyVisionLowLatencySupported<Impl: IHdmiDisplayMode2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDolbyVisionLowLatencySupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHdmiDisplayMode2, BASE_OFFSET>(),
            IsDolbyVisionLowLatencySupported: IsDolbyVisionLowLatencySupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHdmiDisplayMode2 as ::windows::core::Interface>::IID
    }
}
