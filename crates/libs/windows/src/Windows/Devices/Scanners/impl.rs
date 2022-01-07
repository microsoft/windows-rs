#[cfg(feature = "implement_exclusive")]
pub trait IImageScannerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultScanSource(&self) -> ::windows::core::Result<ImageScannerScanSource>;
    fn IsScanSourceSupported(&self, value: ImageScannerScanSource) -> ::windows::core::Result<bool>;
    fn FlatbedConfiguration(&self) -> ::windows::core::Result<ImageScannerFlatbedConfiguration>;
    fn FeederConfiguration(&self) -> ::windows::core::Result<ImageScannerFeederConfiguration>;
    fn AutoConfiguration(&self) -> ::windows::core::Result<ImageScannerAutoConfiguration>;
    fn IsPreviewSupported(&self, scansource: ImageScannerScanSource) -> ::windows::core::Result<bool>;
    fn ScanPreviewToStreamAsync(&self, scansource: ImageScannerScanSource, targetstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageScannerPreviewResult>>;
    fn ScanFilesToFolderAsync(&self, scansource: ImageScannerScanSource, storagefolder: &::core::option::Option<super::super::Storage::StorageFolder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ImageScannerScanResult, u32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageScanner {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScanner";
}
#[cfg(feature = "implement_exclusive")]
impl IImageScannerVtbl {
    pub const fn new<Impl: IImageScannerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageScannerVtbl {
        unsafe extern "system" fn DeviceId<Impl: IImageScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultScanSource<Impl: IImageScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerScanSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultScanSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScanSourceSupported<Impl: IImageScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ImageScannerScanSource, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsScanSourceSupported(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlatbedConfiguration<Impl: IImageScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlatbedConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeederConfiguration<Impl: IImageScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FeederConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoConfiguration<Impl: IImageScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPreviewSupported<Impl: IImageScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPreviewSupported(scansource) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanPreviewToStreamAsync<Impl: IImageScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, targetstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScanPreviewToStreamAsync(scansource, &*(&targetstream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanFilesToFolderAsync<Impl: IImageScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, storagefolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScanFilesToFolderAsync(scansource, &*(&storagefolder as *const <super::super::Storage::StorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFolder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageScanner>, base.5, DeviceId::<Impl, OFFSET>, DefaultScanSource::<Impl, OFFSET>, IsScanSourceSupported::<Impl, OFFSET>, FlatbedConfiguration::<Impl, OFFSET>, FeederConfiguration::<Impl, OFFSET>, AutoConfiguration::<Impl, OFFSET>, IsPreviewSupported::<Impl, OFFSET>, ScanPreviewToStreamAsync::<Impl, OFFSET>, ScanFilesToFolderAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageScannerFeederConfigurationImpl: Sized + IImageScannerFormatConfigurationImpl + IImageScannerSourceConfigurationImpl {
    fn CanAutoDetectPageSize(&self) -> ::windows::core::Result<bool>;
    fn AutoDetectPageSize(&self) -> ::windows::core::Result<bool>;
    fn SetAutoDetectPageSize(&self, value: bool) -> ::windows::core::Result<()>;
    fn PageSize(&self) -> ::windows::core::Result<super::super::Graphics::Printing::PrintMediaSize>;
    fn SetPageSize(&self, value: super::super::Graphics::Printing::PrintMediaSize) -> ::windows::core::Result<()>;
    fn PageOrientation(&self) -> ::windows::core::Result<super::super::Graphics::Printing::PrintOrientation>;
    fn SetPageOrientation(&self, value: super::super::Graphics::Printing::PrintOrientation) -> ::windows::core::Result<()>;
    fn PageSizeDimensions(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn IsPageSizeSupported(&self, pagesize: super::super::Graphics::Printing::PrintMediaSize, pageorientation: super::super::Graphics::Printing::PrintOrientation) -> ::windows::core::Result<bool>;
    fn MaxNumberOfPages(&self) -> ::windows::core::Result<u32>;
    fn SetMaxNumberOfPages(&self, value: u32) -> ::windows::core::Result<()>;
    fn CanScanDuplex(&self) -> ::windows::core::Result<bool>;
    fn Duplex(&self) -> ::windows::core::Result<bool>;
    fn SetDuplex(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanScanAhead(&self) -> ::windows::core::Result<bool>;
    fn ScanAhead(&self) -> ::windows::core::Result<bool>;
    fn SetScanAhead(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageScannerFeederConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerFeederConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IImageScannerFeederConfigurationVtbl {
    pub const fn new<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageScannerFeederConfigurationVtbl {
        unsafe extern "system" fn CanAutoDetectPageSize<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanAutoDetectPageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDetectPageSize<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoDetectPageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDetectPageSize<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoDetectPageSize(value).into()
        }
        unsafe extern "system" fn PageSize<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Printing::PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageSize<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Printing::PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPageSize(value).into()
        }
        unsafe extern "system" fn PageOrientation<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Printing::PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageOrientation<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Printing::PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPageOrientation(value).into()
        }
        unsafe extern "system" fn PageSizeDimensions<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageSizeDimensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPageSizeSupported<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagesize: super::super::Graphics::Printing::PrintMediaSize, pageorientation: super::super::Graphics::Printing::PrintOrientation, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPageSizeSupported(pagesize, pageorientation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxNumberOfPages<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxNumberOfPages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxNumberOfPages<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxNumberOfPages(value).into()
        }
        unsafe extern "system" fn CanScanDuplex<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanScanDuplex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duplex<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duplex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplex<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDuplex(value).into()
        }
        unsafe extern "system" fn CanScanAhead<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanScanAhead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanAhead<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScanAhead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScanAhead<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScanAhead(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IImageScannerFeederConfiguration>,
            base.5,
            CanAutoDetectPageSize::<Impl, OFFSET>,
            AutoDetectPageSize::<Impl, OFFSET>,
            SetAutoDetectPageSize::<Impl, OFFSET>,
            PageSize::<Impl, OFFSET>,
            SetPageSize::<Impl, OFFSET>,
            PageOrientation::<Impl, OFFSET>,
            SetPageOrientation::<Impl, OFFSET>,
            PageSizeDimensions::<Impl, OFFSET>,
            IsPageSizeSupported::<Impl, OFFSET>,
            MaxNumberOfPages::<Impl, OFFSET>,
            SetMaxNumberOfPages::<Impl, OFFSET>,
            CanScanDuplex::<Impl, OFFSET>,
            Duplex::<Impl, OFFSET>,
            SetDuplex::<Impl, OFFSET>,
            CanScanAhead::<Impl, OFFSET>,
            ScanAhead::<Impl, OFFSET>,
            SetScanAhead::<Impl, OFFSET>,
        )
    }
}
pub trait IImageScannerFormatConfigurationImpl: Sized {
    fn DefaultFormat(&self) -> ::windows::core::Result<ImageScannerFormat>;
    fn Format(&self) -> ::windows::core::Result<ImageScannerFormat>;
    fn SetFormat(&self, value: ImageScannerFormat) -> ::windows::core::Result<()>;
    fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IImageScannerFormatConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerFormatConfiguration";
}
impl IImageScannerFormatConfigurationVtbl {
    pub const fn new<Impl: IImageScannerFormatConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageScannerFormatConfigurationVtbl {
        unsafe extern "system" fn DefaultFormat<Impl: IImageScannerFormatConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Impl: IImageScannerFormatConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: IImageScannerFormatConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ImageScannerFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFormat(value).into()
        }
        unsafe extern "system" fn IsFormatSupported<Impl: IImageScannerFormatConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ImageScannerFormat, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFormatSupported(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageScannerFormatConfiguration>, base.5, DefaultFormat::<Impl, OFFSET>, Format::<Impl, OFFSET>, SetFormat::<Impl, OFFSET>, IsFormatSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageScannerPreviewResultImpl: Sized {
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn Format(&self) -> ::windows::core::Result<ImageScannerFormat>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageScannerPreviewResult {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerPreviewResult";
}
#[cfg(feature = "implement_exclusive")]
impl IImageScannerPreviewResultVtbl {
    pub const fn new<Impl: IImageScannerPreviewResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageScannerPreviewResultVtbl {
        unsafe extern "system" fn Succeeded<Impl: IImageScannerPreviewResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Succeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Impl: IImageScannerPreviewResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageScannerPreviewResult>, base.5, Succeeded::<Impl, OFFSET>, Format::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageScannerScanResultImpl: Sized {
    fn ScannedFiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageScannerScanResult {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerScanResult";
}
#[cfg(feature = "implement_exclusive")]
impl IImageScannerScanResultVtbl {
    pub const fn new<Impl: IImageScannerScanResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageScannerScanResultVtbl {
        unsafe extern "system" fn ScannedFiles<Impl: IImageScannerScanResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScannedFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageScannerScanResult>, base.5, ScannedFiles::<Impl, OFFSET>)
    }
}
pub trait IImageScannerSourceConfigurationImpl: Sized + IImageScannerFormatConfigurationImpl {
    fn MinScanArea(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MaxScanArea(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SelectedScanRegion(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetSelectedScanRegion(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn AutoCroppingMode(&self) -> ::windows::core::Result<ImageScannerAutoCroppingMode>;
    fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows::core::Result<()>;
    fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows::core::Result<bool>;
    fn MinResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn MaxResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn OpticalResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn DesiredResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn SetDesiredResolution(&self, value: &ImageScannerResolution) -> ::windows::core::Result<()>;
    fn ActualResolution(&self) -> ::windows::core::Result<ImageScannerResolution>;
    fn DefaultColorMode(&self) -> ::windows::core::Result<ImageScannerColorMode>;
    fn ColorMode(&self) -> ::windows::core::Result<ImageScannerColorMode>;
    fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows::core::Result<()>;
    fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows::core::Result<bool>;
    fn MinBrightness(&self) -> ::windows::core::Result<i32>;
    fn MaxBrightness(&self) -> ::windows::core::Result<i32>;
    fn BrightnessStep(&self) -> ::windows::core::Result<u32>;
    fn DefaultBrightness(&self) -> ::windows::core::Result<i32>;
    fn Brightness(&self) -> ::windows::core::Result<i32>;
    fn SetBrightness(&self, value: i32) -> ::windows::core::Result<()>;
    fn MinContrast(&self) -> ::windows::core::Result<i32>;
    fn MaxContrast(&self) -> ::windows::core::Result<i32>;
    fn ContrastStep(&self) -> ::windows::core::Result<u32>;
    fn DefaultContrast(&self) -> ::windows::core::Result<i32>;
    fn Contrast(&self) -> ::windows::core::Result<i32>;
    fn SetContrast(&self, value: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IImageScannerSourceConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerSourceConfiguration";
}
impl IImageScannerSourceConfigurationVtbl {
    pub const fn new<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageScannerSourceConfigurationVtbl {
        unsafe extern "system" fn MinScanArea<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinScanArea() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxScanArea<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxScanArea() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedScanRegion<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedScanRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedScanRegion<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedScanRegion(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoCroppingMode<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerAutoCroppingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoCroppingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoCroppingMode<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ImageScannerAutoCroppingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoCroppingMode(value).into()
        }
        unsafe extern "system" fn IsAutoCroppingModeSupported<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ImageScannerAutoCroppingMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAutoCroppingModeSupported(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpticalResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpticalResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredResolution(&*(&value as *const <ImageScannerResolution as ::windows::core::Abi>::Abi as *const <ImageScannerResolution as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultColorMode<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultColorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorMode<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorMode<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ImageScannerColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColorMode(value).into()
        }
        unsafe extern "system" fn IsColorModeSupported<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ImageScannerColorMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsColorModeSupported(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinBrightness<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinBrightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBrightness<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxBrightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrightnessStep<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BrightnessStep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultBrightness<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultBrightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Brightness<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Brightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrightness<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBrightness(value).into()
        }
        unsafe extern "system" fn MinContrast<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxContrast<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContrastStep<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContrastStep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultContrast<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contrast<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContrast<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContrast(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IImageScannerSourceConfiguration>,
            base.5,
            MinScanArea::<Impl, OFFSET>,
            MaxScanArea::<Impl, OFFSET>,
            SelectedScanRegion::<Impl, OFFSET>,
            SetSelectedScanRegion::<Impl, OFFSET>,
            AutoCroppingMode::<Impl, OFFSET>,
            SetAutoCroppingMode::<Impl, OFFSET>,
            IsAutoCroppingModeSupported::<Impl, OFFSET>,
            MinResolution::<Impl, OFFSET>,
            MaxResolution::<Impl, OFFSET>,
            OpticalResolution::<Impl, OFFSET>,
            DesiredResolution::<Impl, OFFSET>,
            SetDesiredResolution::<Impl, OFFSET>,
            ActualResolution::<Impl, OFFSET>,
            DefaultColorMode::<Impl, OFFSET>,
            ColorMode::<Impl, OFFSET>,
            SetColorMode::<Impl, OFFSET>,
            IsColorModeSupported::<Impl, OFFSET>,
            MinBrightness::<Impl, OFFSET>,
            MaxBrightness::<Impl, OFFSET>,
            BrightnessStep::<Impl, OFFSET>,
            DefaultBrightness::<Impl, OFFSET>,
            Brightness::<Impl, OFFSET>,
            SetBrightness::<Impl, OFFSET>,
            MinContrast::<Impl, OFFSET>,
            MaxContrast::<Impl, OFFSET>,
            ContrastStep::<Impl, OFFSET>,
            DefaultContrast::<Impl, OFFSET>,
            Contrast::<Impl, OFFSET>,
            SetContrast::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageScannerStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageScanner>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageScannerStatics {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IImageScannerStaticsVtbl {
    pub const fn new<Impl: IImageScannerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageScannerStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IImageScannerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IImageScannerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageScannerStatics>, base.5, FromIdAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>)
    }
}
