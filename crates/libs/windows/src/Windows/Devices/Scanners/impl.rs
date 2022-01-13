#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IImageScannerImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultScanSource(&mut self) -> ::windows::core::Result<ImageScannerScanSource>;
    fn IsScanSourceSupported(&mut self, value: ImageScannerScanSource) -> ::windows::core::Result<bool>;
    fn FlatbedConfiguration(&mut self) -> ::windows::core::Result<ImageScannerFlatbedConfiguration>;
    fn FeederConfiguration(&mut self) -> ::windows::core::Result<ImageScannerFeederConfiguration>;
    fn AutoConfiguration(&mut self) -> ::windows::core::Result<ImageScannerAutoConfiguration>;
    fn IsPreviewSupported(&mut self, scansource: ImageScannerScanSource) -> ::windows::core::Result<bool>;
    fn ScanPreviewToStreamAsync(&mut self, scansource: ImageScannerScanSource, targetstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageScannerPreviewResult>>;
    fn ScanFilesToFolderAsync(&mut self, scansource: ImageScannerScanSource, storagefolder: &::core::option::Option<super::super::Storage::StorageFolder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ImageScannerScanResult, u32>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageScanner {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScanner";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IImageScannerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageScannerVtbl {
        unsafe extern "system" fn DeviceId<Impl: IImageScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultScanSource<Impl: IImageScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerScanSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultScanSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScanSourceSupported<Impl: IImageScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerScanSource, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScanSourceSupported(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlatbedConfiguration<Impl: IImageScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlatbedConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeederConfiguration<Impl: IImageScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeederConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoConfiguration<Impl: IImageScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPreviewSupported<Impl: IImageScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPreviewSupported(scansource) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanPreviewToStreamAsync<Impl: IImageScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, targetstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanPreviewToStreamAsync(scansource, &*(&targetstream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanFilesToFolderAsync<Impl: IImageScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, storagefolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanFilesToFolderAsync(scansource, &*(&storagefolder as *const <super::super::Storage::StorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFolder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageScanner, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            DefaultScanSource: DefaultScanSource::<Impl, IMPL_OFFSET>,
            IsScanSourceSupported: IsScanSourceSupported::<Impl, IMPL_OFFSET>,
            FlatbedConfiguration: FlatbedConfiguration::<Impl, IMPL_OFFSET>,
            FeederConfiguration: FeederConfiguration::<Impl, IMPL_OFFSET>,
            AutoConfiguration: AutoConfiguration::<Impl, IMPL_OFFSET>,
            IsPreviewSupported: IsPreviewSupported::<Impl, IMPL_OFFSET>,
            ScanPreviewToStreamAsync: ScanPreviewToStreamAsync::<Impl, IMPL_OFFSET>,
            ScanFilesToFolderAsync: ScanFilesToFolderAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageScanner as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing", feature = "implement_exclusive"))]
pub trait IImageScannerFeederConfigurationImpl: Sized + IImageScannerFormatConfigurationImpl + IImageScannerSourceConfigurationImpl {
    fn CanAutoDetectPageSize(&mut self) -> ::windows::core::Result<bool>;
    fn AutoDetectPageSize(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoDetectPageSize(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PageSize(&mut self) -> ::windows::core::Result<super::super::Graphics::Printing::PrintMediaSize>;
    fn SetPageSize(&mut self, value: super::super::Graphics::Printing::PrintMediaSize) -> ::windows::core::Result<()>;
    fn PageOrientation(&mut self) -> ::windows::core::Result<super::super::Graphics::Printing::PrintOrientation>;
    fn SetPageOrientation(&mut self, value: super::super::Graphics::Printing::PrintOrientation) -> ::windows::core::Result<()>;
    fn PageSizeDimensions(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn IsPageSizeSupported(&mut self, pagesize: super::super::Graphics::Printing::PrintMediaSize, pageorientation: super::super::Graphics::Printing::PrintOrientation) -> ::windows::core::Result<bool>;
    fn MaxNumberOfPages(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxNumberOfPages(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CanScanDuplex(&mut self) -> ::windows::core::Result<bool>;
    fn Duplex(&mut self) -> ::windows::core::Result<bool>;
    fn SetDuplex(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CanScanAhead(&mut self) -> ::windows::core::Result<bool>;
    fn ScanAhead(&mut self) -> ::windows::core::Result<bool>;
    fn SetScanAhead(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageScannerFeederConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerFeederConfiguration";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl IImageScannerFeederConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerFeederConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageScannerFeederConfigurationVtbl {
        unsafe extern "system" fn CanAutoDetectPageSize<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanAutoDetectPageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDetectPageSize<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoDetectPageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDetectPageSize<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoDetectPageSize(value).into()
        }
        unsafe extern "system" fn PageSize<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Printing::PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageSize<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Printing::PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageSize(value).into()
        }
        unsafe extern "system" fn PageOrientation<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Printing::PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageOrientation<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Printing::PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageOrientation(value).into()
        }
        unsafe extern "system" fn PageSizeDimensions<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageSizeDimensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPageSizeSupported<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagesize: super::super::Graphics::Printing::PrintMediaSize, pageorientation: super::super::Graphics::Printing::PrintOrientation, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPageSizeSupported(pagesize, pageorientation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxNumberOfPages<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxNumberOfPages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxNumberOfPages<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxNumberOfPages(value).into()
        }
        unsafe extern "system" fn CanScanDuplex<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanScanDuplex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duplex<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duplex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplex<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuplex(value).into()
        }
        unsafe extern "system" fn CanScanAhead<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanScanAhead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanAhead<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanAhead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScanAhead<Impl: IImageScannerFeederConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScanAhead(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageScannerFeederConfiguration, BASE_OFFSET>(),
            CanAutoDetectPageSize: CanAutoDetectPageSize::<Impl, IMPL_OFFSET>,
            AutoDetectPageSize: AutoDetectPageSize::<Impl, IMPL_OFFSET>,
            SetAutoDetectPageSize: SetAutoDetectPageSize::<Impl, IMPL_OFFSET>,
            PageSize: PageSize::<Impl, IMPL_OFFSET>,
            SetPageSize: SetPageSize::<Impl, IMPL_OFFSET>,
            PageOrientation: PageOrientation::<Impl, IMPL_OFFSET>,
            SetPageOrientation: SetPageOrientation::<Impl, IMPL_OFFSET>,
            PageSizeDimensions: PageSizeDimensions::<Impl, IMPL_OFFSET>,
            IsPageSizeSupported: IsPageSizeSupported::<Impl, IMPL_OFFSET>,
            MaxNumberOfPages: MaxNumberOfPages::<Impl, IMPL_OFFSET>,
            SetMaxNumberOfPages: SetMaxNumberOfPages::<Impl, IMPL_OFFSET>,
            CanScanDuplex: CanScanDuplex::<Impl, IMPL_OFFSET>,
            Duplex: Duplex::<Impl, IMPL_OFFSET>,
            SetDuplex: SetDuplex::<Impl, IMPL_OFFSET>,
            CanScanAhead: CanScanAhead::<Impl, IMPL_OFFSET>,
            ScanAhead: ScanAhead::<Impl, IMPL_OFFSET>,
            SetScanAhead: SetScanAhead::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageScannerFeederConfiguration as ::windows::core::Interface>::IID
    }
}
pub trait IImageScannerFormatConfigurationImpl: Sized {
    fn DefaultFormat(&mut self) -> ::windows::core::Result<ImageScannerFormat>;
    fn Format(&mut self) -> ::windows::core::Result<ImageScannerFormat>;
    fn SetFormat(&mut self, value: ImageScannerFormat) -> ::windows::core::Result<()>;
    fn IsFormatSupported(&mut self, value: ImageScannerFormat) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IImageScannerFormatConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerFormatConfiguration";
}
impl IImageScannerFormatConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerFormatConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageScannerFormatConfigurationVtbl {
        unsafe extern "system" fn DefaultFormat<Impl: IImageScannerFormatConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Impl: IImageScannerFormatConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: IImageScannerFormatConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(value).into()
        }
        unsafe extern "system" fn IsFormatSupported<Impl: IImageScannerFormatConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerFormat, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFormatSupported(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageScannerFormatConfiguration, BASE_OFFSET>(),
            DefaultFormat: DefaultFormat::<Impl, IMPL_OFFSET>,
            Format: Format::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            IsFormatSupported: IsFormatSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageScannerFormatConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageScannerPreviewResultImpl: Sized {
    fn Succeeded(&mut self) -> ::windows::core::Result<bool>;
    fn Format(&mut self) -> ::windows::core::Result<ImageScannerFormat>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageScannerPreviewResult {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerPreviewResult";
}
#[cfg(feature = "implement_exclusive")]
impl IImageScannerPreviewResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerPreviewResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageScannerPreviewResultVtbl {
        unsafe extern "system" fn Succeeded<Impl: IImageScannerPreviewResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Succeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Impl: IImageScannerPreviewResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageScannerPreviewResult, BASE_OFFSET>(),
            Succeeded: Succeeded::<Impl, IMPL_OFFSET>,
            Format: Format::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageScannerPreviewResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IImageScannerScanResultImpl: Sized {
    fn ScannedFiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageScannerScanResult {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerScanResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl IImageScannerScanResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerScanResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageScannerScanResultVtbl {
        unsafe extern "system" fn ScannedFiles<Impl: IImageScannerScanResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScannedFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageScannerScanResult, BASE_OFFSET>(),
            ScannedFiles: ScannedFiles::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageScannerScanResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IImageScannerSourceConfigurationImpl: Sized + IImageScannerFormatConfigurationImpl {
    fn MinScanArea(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MaxScanArea(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SelectedScanRegion(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetSelectedScanRegion(&mut self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn AutoCroppingMode(&mut self) -> ::windows::core::Result<ImageScannerAutoCroppingMode>;
    fn SetAutoCroppingMode(&mut self, value: ImageScannerAutoCroppingMode) -> ::windows::core::Result<()>;
    fn IsAutoCroppingModeSupported(&mut self, value: ImageScannerAutoCroppingMode) -> ::windows::core::Result<bool>;
    fn MinResolution(&mut self) -> ::windows::core::Result<ImageScannerResolution>;
    fn MaxResolution(&mut self) -> ::windows::core::Result<ImageScannerResolution>;
    fn OpticalResolution(&mut self) -> ::windows::core::Result<ImageScannerResolution>;
    fn DesiredResolution(&mut self) -> ::windows::core::Result<ImageScannerResolution>;
    fn SetDesiredResolution(&mut self, value: &ImageScannerResolution) -> ::windows::core::Result<()>;
    fn ActualResolution(&mut self) -> ::windows::core::Result<ImageScannerResolution>;
    fn DefaultColorMode(&mut self) -> ::windows::core::Result<ImageScannerColorMode>;
    fn ColorMode(&mut self) -> ::windows::core::Result<ImageScannerColorMode>;
    fn SetColorMode(&mut self, value: ImageScannerColorMode) -> ::windows::core::Result<()>;
    fn IsColorModeSupported(&mut self, value: ImageScannerColorMode) -> ::windows::core::Result<bool>;
    fn MinBrightness(&mut self) -> ::windows::core::Result<i32>;
    fn MaxBrightness(&mut self) -> ::windows::core::Result<i32>;
    fn BrightnessStep(&mut self) -> ::windows::core::Result<u32>;
    fn DefaultBrightness(&mut self) -> ::windows::core::Result<i32>;
    fn Brightness(&mut self) -> ::windows::core::Result<i32>;
    fn SetBrightness(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn MinContrast(&mut self) -> ::windows::core::Result<i32>;
    fn MaxContrast(&mut self) -> ::windows::core::Result<i32>;
    fn ContrastStep(&mut self) -> ::windows::core::Result<u32>;
    fn DefaultContrast(&mut self) -> ::windows::core::Result<i32>;
    fn Contrast(&mut self) -> ::windows::core::Result<i32>;
    fn SetContrast(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IImageScannerSourceConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerSourceConfiguration";
}
#[cfg(feature = "Foundation")]
impl IImageScannerSourceConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerSourceConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageScannerSourceConfigurationVtbl {
        unsafe extern "system" fn MinScanArea<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinScanArea() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxScanArea<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxScanArea() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedScanRegion<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedScanRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedScanRegion<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedScanRegion(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoCroppingMode<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerAutoCroppingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoCroppingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoCroppingMode<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerAutoCroppingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoCroppingMode(value).into()
        }
        unsafe extern "system" fn IsAutoCroppingModeSupported<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerAutoCroppingMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAutoCroppingModeSupported(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpticalResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpticalResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredResolution(&*(&value as *const <ImageScannerResolution as ::windows::core::Abi>::Abi as *const <ImageScannerResolution as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualResolution<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultColorMode<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultColorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorMode<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorMode<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorMode(value).into()
        }
        unsafe extern "system" fn IsColorModeSupported<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ImageScannerColorMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsColorModeSupported(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinBrightness<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinBrightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBrightness<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBrightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrightnessStep<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrightnessStep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultBrightness<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultBrightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Brightness<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Brightness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrightness<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBrightness(value).into()
        }
        unsafe extern "system" fn MinContrast<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxContrast<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContrastStep<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContrastStep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultContrast<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contrast<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContrast<Impl: IImageScannerSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContrast(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageScannerSourceConfiguration, BASE_OFFSET>(),
            MinScanArea: MinScanArea::<Impl, IMPL_OFFSET>,
            MaxScanArea: MaxScanArea::<Impl, IMPL_OFFSET>,
            SelectedScanRegion: SelectedScanRegion::<Impl, IMPL_OFFSET>,
            SetSelectedScanRegion: SetSelectedScanRegion::<Impl, IMPL_OFFSET>,
            AutoCroppingMode: AutoCroppingMode::<Impl, IMPL_OFFSET>,
            SetAutoCroppingMode: SetAutoCroppingMode::<Impl, IMPL_OFFSET>,
            IsAutoCroppingModeSupported: IsAutoCroppingModeSupported::<Impl, IMPL_OFFSET>,
            MinResolution: MinResolution::<Impl, IMPL_OFFSET>,
            MaxResolution: MaxResolution::<Impl, IMPL_OFFSET>,
            OpticalResolution: OpticalResolution::<Impl, IMPL_OFFSET>,
            DesiredResolution: DesiredResolution::<Impl, IMPL_OFFSET>,
            SetDesiredResolution: SetDesiredResolution::<Impl, IMPL_OFFSET>,
            ActualResolution: ActualResolution::<Impl, IMPL_OFFSET>,
            DefaultColorMode: DefaultColorMode::<Impl, IMPL_OFFSET>,
            ColorMode: ColorMode::<Impl, IMPL_OFFSET>,
            SetColorMode: SetColorMode::<Impl, IMPL_OFFSET>,
            IsColorModeSupported: IsColorModeSupported::<Impl, IMPL_OFFSET>,
            MinBrightness: MinBrightness::<Impl, IMPL_OFFSET>,
            MaxBrightness: MaxBrightness::<Impl, IMPL_OFFSET>,
            BrightnessStep: BrightnessStep::<Impl, IMPL_OFFSET>,
            DefaultBrightness: DefaultBrightness::<Impl, IMPL_OFFSET>,
            Brightness: Brightness::<Impl, IMPL_OFFSET>,
            SetBrightness: SetBrightness::<Impl, IMPL_OFFSET>,
            MinContrast: MinContrast::<Impl, IMPL_OFFSET>,
            MaxContrast: MaxContrast::<Impl, IMPL_OFFSET>,
            ContrastStep: ContrastStep::<Impl, IMPL_OFFSET>,
            DefaultContrast: DefaultContrast::<Impl, IMPL_OFFSET>,
            Contrast: Contrast::<Impl, IMPL_OFFSET>,
            SetContrast: SetContrast::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageScannerSourceConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IImageScannerStaticsImpl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageScanner>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageScannerStatics {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IImageScannerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageScannerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageScannerStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IImageScannerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IImageScannerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageScannerStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageScannerStatics as ::windows::core::Interface>::IID
    }
}
