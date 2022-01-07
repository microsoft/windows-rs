#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<BarcodeScannerCapabilities>;
    fn ClaimScannerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedBarcodeScanner>>;
    fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetSupportedSymbologiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>;
    fn IsSymbologySupportedAsync(&self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RetrieveStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn GetSupportedProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn IsProfileSupported(&self, profile: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn StatusUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScanner";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerVtbl {
    pub const fn new<Impl: IBarcodeScannerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerVtbl {
        unsafe extern "system" fn DeviceId<Impl: IBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: IBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClaimScannerAsync<Impl: IBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClaimScannerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHealthAsync<Impl: IBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckHealthAsync(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedSymbologiesAsync<Impl: IBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSupportedSymbologiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSymbologySupportedAsync<Impl: IBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, barcodesymbology: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSymbologySupportedAsync(barcodesymbology) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveStatisticsAsync<Impl: IBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetrieveStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedProfiles<Impl: IBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSupportedProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsProfileSupported<Impl: IBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsProfileSupported(&*(&profile as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUpdated<Impl: IBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUpdated<Impl: IBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStatusUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBarcodeScanner>,
            base.5,
            DeviceId::<Impl, OFFSET>,
            Capabilities::<Impl, OFFSET>,
            ClaimScannerAsync::<Impl, OFFSET>,
            CheckHealthAsync::<Impl, OFFSET>,
            GetSupportedSymbologiesAsync::<Impl, OFFSET>,
            IsSymbologySupportedAsync::<Impl, OFFSET>,
            RetrieveStatisticsAsync::<Impl, OFFSET>,
            GetSupportedProfiles::<Impl, OFFSET>,
            IsProfileSupported::<Impl, OFFSET>,
            StatusUpdated::<Impl, OFFSET>,
            RemoveStatusUpdated::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScanner2Impl: Sized {
    fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScanner2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScanner2";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScanner2Vtbl {
    pub const fn new<Impl: IBarcodeScanner2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScanner2Vtbl {
        unsafe extern "system" fn VideoDeviceId<Impl: IBarcodeScanner2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScanner2>, base.5, VideoDeviceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerCapabilitiesImpl: Sized {
    fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsImagePreviewSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerCapabilitiesVtbl {
    pub const fn new<Impl: IBarcodeScannerCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerCapabilitiesVtbl {
        unsafe extern "system" fn PowerReportingType<Impl: IBarcodeScannerCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsReportingSupported<Impl: IBarcodeScannerCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatisticsReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsUpdatingSupported<Impl: IBarcodeScannerCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatisticsUpdatingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsImagePreviewSupported<Impl: IBarcodeScannerCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsImagePreviewSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerCapabilities>, base.5, PowerReportingType::<Impl, OFFSET>, IsStatisticsReportingSupported::<Impl, OFFSET>, IsStatisticsUpdatingSupported::<Impl, OFFSET>, IsImagePreviewSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerCapabilities1Impl: Sized {
    fn IsSoftwareTriggerSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerCapabilities1 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerCapabilities1";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerCapabilities1Vtbl {
    pub const fn new<Impl: IBarcodeScannerCapabilities1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerCapabilities1Vtbl {
        unsafe extern "system" fn IsSoftwareTriggerSupported<Impl: IBarcodeScannerCapabilities1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSoftwareTriggerSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerCapabilities1>, base.5, IsSoftwareTriggerSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerCapabilities2Impl: Sized {
    fn IsVideoPreviewSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerCapabilities2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerCapabilities2";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerCapabilities2Vtbl {
    pub const fn new<Impl: IBarcodeScannerCapabilities2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerCapabilities2Vtbl {
        unsafe extern "system" fn IsVideoPreviewSupported<Impl: IBarcodeScannerCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsVideoPreviewSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerCapabilities2>, base.5, IsVideoPreviewSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerDataReceivedEventArgsImpl: Sized {
    fn Report(&self) -> ::windows::core::Result<BarcodeScannerReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerDataReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerDataReceivedEventArgsVtbl {
    pub const fn new<Impl: IBarcodeScannerDataReceivedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerDataReceivedEventArgsVtbl {
        unsafe extern "system" fn Report<Impl: IBarcodeScannerDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Report() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerDataReceivedEventArgs>, base.5, Report::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerErrorOccurredEventArgsImpl: Sized {
    fn PartialInputData(&self) -> ::windows::core::Result<BarcodeScannerReport>;
    fn IsRetriable(&self) -> ::windows::core::Result<bool>;
    fn ErrorData(&self) -> ::windows::core::Result<UnifiedPosErrorData>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerErrorOccurredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerErrorOccurredEventArgsVtbl {
    pub const fn new<Impl: IBarcodeScannerErrorOccurredEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerErrorOccurredEventArgsVtbl {
        unsafe extern "system" fn PartialInputData<Impl: IBarcodeScannerErrorOccurredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PartialInputData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRetriable<Impl: IBarcodeScannerErrorOccurredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRetriable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorData<Impl: IBarcodeScannerErrorOccurredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerErrorOccurredEventArgs>, base.5, PartialInputData::<Impl, OFFSET>, IsRetriable::<Impl, OFFSET>, ErrorData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerImagePreviewReceivedEventArgsImpl: Sized {
    fn Preview(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerImagePreviewReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerImagePreviewReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerImagePreviewReceivedEventArgsVtbl {
    pub const fn new<Impl: IBarcodeScannerImagePreviewReceivedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerImagePreviewReceivedEventArgsVtbl {
        unsafe extern "system" fn Preview<Impl: IBarcodeScannerImagePreviewReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Preview() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerImagePreviewReceivedEventArgs>, base.5, Preview::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerReportImpl: Sized {
    fn ScanDataType(&self) -> ::windows::core::Result<u32>;
    fn ScanData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ScanDataLabel(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerReport";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerReportVtbl {
    pub const fn new<Impl: IBarcodeScannerReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerReportVtbl {
        unsafe extern "system" fn ScanDataType<Impl: IBarcodeScannerReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScanDataType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanData<Impl: IBarcodeScannerReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScanData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanDataLabel<Impl: IBarcodeScannerReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScanDataLabel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerReport>, base.5, ScanDataType::<Impl, OFFSET>, ScanData::<Impl, OFFSET>, ScanDataLabel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerReportFactoryImpl: Sized {
    fn CreateInstance(&self, scandatatype: u32, scandata: &::core::option::Option<super::super::Storage::Streams::IBuffer>, scandatalabel: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<BarcodeScannerReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerReportFactory {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerReportFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerReportFactoryVtbl {
    pub const fn new<Impl: IBarcodeScannerReportFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerReportFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBarcodeScannerReportFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scandatatype: u32, scandata: ::windows::core::RawPtr, scandatalabel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(scandatatype, &*(&scandata as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&scandatalabel as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerReportFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerStaticsVtbl {
    pub const fn new<Impl: IBarcodeScannerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerStaticsVtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: IBarcodeScannerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IBarcodeScannerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IBarcodeScannerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerStatics>, base.5, GetDefaultAsync::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerStatics2Vtbl {
    pub const fn new<Impl: IBarcodeScannerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: IBarcodeScannerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerStatics2>, base.5, GetDeviceSelectorWithConnectionTypes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStatusUpdatedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<BarcodeScannerStatus>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerStatusUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerStatusUpdatedEventArgsVtbl {
    pub const fn new<Impl: IBarcodeScannerStatusUpdatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeScannerStatusUpdatedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IBarcodeScannerStatusUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BarcodeScannerStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IBarcodeScannerStatusUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeScannerStatusUpdatedEventArgs>, base.5, Status::<Impl, OFFSET>, ExtendedStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeSymbologiesStaticsImpl: Sized {
    fn Unknown(&self) -> ::windows::core::Result<u32>;
    fn Ean8(&self) -> ::windows::core::Result<u32>;
    fn Ean8Add2(&self) -> ::windows::core::Result<u32>;
    fn Ean8Add5(&self) -> ::windows::core::Result<u32>;
    fn Eanv(&self) -> ::windows::core::Result<u32>;
    fn EanvAdd2(&self) -> ::windows::core::Result<u32>;
    fn EanvAdd5(&self) -> ::windows::core::Result<u32>;
    fn Ean13(&self) -> ::windows::core::Result<u32>;
    fn Ean13Add2(&self) -> ::windows::core::Result<u32>;
    fn Ean13Add5(&self) -> ::windows::core::Result<u32>;
    fn Isbn(&self) -> ::windows::core::Result<u32>;
    fn IsbnAdd5(&self) -> ::windows::core::Result<u32>;
    fn Ismn(&self) -> ::windows::core::Result<u32>;
    fn IsmnAdd2(&self) -> ::windows::core::Result<u32>;
    fn IsmnAdd5(&self) -> ::windows::core::Result<u32>;
    fn Issn(&self) -> ::windows::core::Result<u32>;
    fn IssnAdd2(&self) -> ::windows::core::Result<u32>;
    fn IssnAdd5(&self) -> ::windows::core::Result<u32>;
    fn Ean99(&self) -> ::windows::core::Result<u32>;
    fn Ean99Add2(&self) -> ::windows::core::Result<u32>;
    fn Ean99Add5(&self) -> ::windows::core::Result<u32>;
    fn Upca(&self) -> ::windows::core::Result<u32>;
    fn UpcaAdd2(&self) -> ::windows::core::Result<u32>;
    fn UpcaAdd5(&self) -> ::windows::core::Result<u32>;
    fn Upce(&self) -> ::windows::core::Result<u32>;
    fn UpceAdd2(&self) -> ::windows::core::Result<u32>;
    fn UpceAdd5(&self) -> ::windows::core::Result<u32>;
    fn UpcCoupon(&self) -> ::windows::core::Result<u32>;
    fn TfStd(&self) -> ::windows::core::Result<u32>;
    fn TfDis(&self) -> ::windows::core::Result<u32>;
    fn TfInt(&self) -> ::windows::core::Result<u32>;
    fn TfInd(&self) -> ::windows::core::Result<u32>;
    fn TfMat(&self) -> ::windows::core::Result<u32>;
    fn TfIata(&self) -> ::windows::core::Result<u32>;
    fn Gs1DatabarType1(&self) -> ::windows::core::Result<u32>;
    fn Gs1DatabarType2(&self) -> ::windows::core::Result<u32>;
    fn Gs1DatabarType3(&self) -> ::windows::core::Result<u32>;
    fn Code39(&self) -> ::windows::core::Result<u32>;
    fn Code39Ex(&self) -> ::windows::core::Result<u32>;
    fn Trioptic39(&self) -> ::windows::core::Result<u32>;
    fn Code32(&self) -> ::windows::core::Result<u32>;
    fn Pzn(&self) -> ::windows::core::Result<u32>;
    fn Code93(&self) -> ::windows::core::Result<u32>;
    fn Code93Ex(&self) -> ::windows::core::Result<u32>;
    fn Code128(&self) -> ::windows::core::Result<u32>;
    fn Gs1128(&self) -> ::windows::core::Result<u32>;
    fn Gs1128Coupon(&self) -> ::windows::core::Result<u32>;
    fn UccEan128(&self) -> ::windows::core::Result<u32>;
    fn Sisac(&self) -> ::windows::core::Result<u32>;
    fn Isbt(&self) -> ::windows::core::Result<u32>;
    fn Codabar(&self) -> ::windows::core::Result<u32>;
    fn Code11(&self) -> ::windows::core::Result<u32>;
    fn Msi(&self) -> ::windows::core::Result<u32>;
    fn Plessey(&self) -> ::windows::core::Result<u32>;
    fn Telepen(&self) -> ::windows::core::Result<u32>;
    fn Code16k(&self) -> ::windows::core::Result<u32>;
    fn CodablockA(&self) -> ::windows::core::Result<u32>;
    fn CodablockF(&self) -> ::windows::core::Result<u32>;
    fn Codablock128(&self) -> ::windows::core::Result<u32>;
    fn Code49(&self) -> ::windows::core::Result<u32>;
    fn Aztec(&self) -> ::windows::core::Result<u32>;
    fn DataCode(&self) -> ::windows::core::Result<u32>;
    fn DataMatrix(&self) -> ::windows::core::Result<u32>;
    fn HanXin(&self) -> ::windows::core::Result<u32>;
    fn Maxicode(&self) -> ::windows::core::Result<u32>;
    fn MicroPdf417(&self) -> ::windows::core::Result<u32>;
    fn MicroQr(&self) -> ::windows::core::Result<u32>;
    fn Pdf417(&self) -> ::windows::core::Result<u32>;
    fn Qr(&self) -> ::windows::core::Result<u32>;
    fn MsTag(&self) -> ::windows::core::Result<u32>;
    fn Ccab(&self) -> ::windows::core::Result<u32>;
    fn Ccc(&self) -> ::windows::core::Result<u32>;
    fn Tlc39(&self) -> ::windows::core::Result<u32>;
    fn AusPost(&self) -> ::windows::core::Result<u32>;
    fn CanPost(&self) -> ::windows::core::Result<u32>;
    fn ChinaPost(&self) -> ::windows::core::Result<u32>;
    fn DutchKix(&self) -> ::windows::core::Result<u32>;
    fn InfoMail(&self) -> ::windows::core::Result<u32>;
    fn ItalianPost25(&self) -> ::windows::core::Result<u32>;
    fn ItalianPost39(&self) -> ::windows::core::Result<u32>;
    fn JapanPost(&self) -> ::windows::core::Result<u32>;
    fn KoreanPost(&self) -> ::windows::core::Result<u32>;
    fn SwedenPost(&self) -> ::windows::core::Result<u32>;
    fn UkPost(&self) -> ::windows::core::Result<u32>;
    fn UsIntelligent(&self) -> ::windows::core::Result<u32>;
    fn UsIntelligentPkg(&self) -> ::windows::core::Result<u32>;
    fn UsPlanet(&self) -> ::windows::core::Result<u32>;
    fn UsPostNet(&self) -> ::windows::core::Result<u32>;
    fn Us4StateFics(&self) -> ::windows::core::Result<u32>;
    fn OcrA(&self) -> ::windows::core::Result<u32>;
    fn OcrB(&self) -> ::windows::core::Result<u32>;
    fn Micr(&self) -> ::windows::core::Result<u32>;
    fn ExtendedBase(&self) -> ::windows::core::Result<u32>;
    fn GetName(&self, scandatatype: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeSymbologiesStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeSymbologiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeSymbologiesStaticsVtbl {
    pub const fn new<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeSymbologiesStaticsVtbl {
        unsafe extern "system" fn Unknown<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unknown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean8<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ean8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean8Add2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ean8Add2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean8Add5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ean8Add5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Eanv<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Eanv() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EanvAdd2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EanvAdd2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EanvAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EanvAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean13<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ean13() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean13Add2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ean13Add2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean13Add5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ean13Add5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Isbn<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Isbn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsbnAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsbnAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ismn<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ismn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsmnAdd2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsmnAdd2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsmnAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsmnAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Issn<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Issn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssnAdd2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IssnAdd2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssnAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IssnAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean99<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ean99() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean99Add2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ean99Add2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean99Add5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ean99Add5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Upca<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Upca() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpcaAdd2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpcaAdd2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpcaAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpcaAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Upce<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Upce() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpceAdd2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpceAdd2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpceAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpceAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpcCoupon<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpcCoupon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfStd<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TfStd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfDis<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TfDis() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfInt<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TfInt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfInd<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TfInd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfMat<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TfMat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfIata<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TfIata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gs1DatabarType1<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gs1DatabarType1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gs1DatabarType2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gs1DatabarType2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gs1DatabarType3<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gs1DatabarType3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code39<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Code39() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code39Ex<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Code39Ex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trioptic39<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Trioptic39() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code32<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Code32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pzn<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pzn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code93<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Code93() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code93Ex<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Code93Ex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code128<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Code128() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gs1128<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gs1128() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gs1128Coupon<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gs1128Coupon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UccEan128<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UccEan128() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sisac<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Sisac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Isbt<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Isbt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Codabar<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Codabar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code11<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Code11() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Msi<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Msi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Plessey<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Plessey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Telepen<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Telepen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code16k<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Code16k() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CodablockA<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CodablockA() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CodablockF<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CodablockF() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Codablock128<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Codablock128() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code49<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Code49() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Aztec<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Aztec() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataCode<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataMatrix<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HanXin<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HanXin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maxicode<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Maxicode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicroPdf417<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MicroPdf417() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicroQr<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MicroQr() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pdf417<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pdf417() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qr<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Qr() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MsTag<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MsTag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ccab<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ccab() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ccc<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ccc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tlc39<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tlc39() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AusPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AusPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChinaPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChinaPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DutchKix<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DutchKix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoMail<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InfoMail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItalianPost25<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItalianPost25() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItalianPost39<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItalianPost39() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JapanPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JapanPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KoreanPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KoreanPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwedenPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SwedenPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UkPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UkPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsIntelligent<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UsIntelligent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsIntelligentPkg<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UsIntelligentPkg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsPlanet<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UsPlanet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsPostNet<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UsPostNet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Us4StateFics<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Us4StateFics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OcrA<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OcrA() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OcrB<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OcrB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Micr<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Micr() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedBase<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedBase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scandatatype: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetName(scandatatype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBarcodeSymbologiesStatics>,
            base.5,
            Unknown::<Impl, OFFSET>,
            Ean8::<Impl, OFFSET>,
            Ean8Add2::<Impl, OFFSET>,
            Ean8Add5::<Impl, OFFSET>,
            Eanv::<Impl, OFFSET>,
            EanvAdd2::<Impl, OFFSET>,
            EanvAdd5::<Impl, OFFSET>,
            Ean13::<Impl, OFFSET>,
            Ean13Add2::<Impl, OFFSET>,
            Ean13Add5::<Impl, OFFSET>,
            Isbn::<Impl, OFFSET>,
            IsbnAdd5::<Impl, OFFSET>,
            Ismn::<Impl, OFFSET>,
            IsmnAdd2::<Impl, OFFSET>,
            IsmnAdd5::<Impl, OFFSET>,
            Issn::<Impl, OFFSET>,
            IssnAdd2::<Impl, OFFSET>,
            IssnAdd5::<Impl, OFFSET>,
            Ean99::<Impl, OFFSET>,
            Ean99Add2::<Impl, OFFSET>,
            Ean99Add5::<Impl, OFFSET>,
            Upca::<Impl, OFFSET>,
            UpcaAdd2::<Impl, OFFSET>,
            UpcaAdd5::<Impl, OFFSET>,
            Upce::<Impl, OFFSET>,
            UpceAdd2::<Impl, OFFSET>,
            UpceAdd5::<Impl, OFFSET>,
            UpcCoupon::<Impl, OFFSET>,
            TfStd::<Impl, OFFSET>,
            TfDis::<Impl, OFFSET>,
            TfInt::<Impl, OFFSET>,
            TfInd::<Impl, OFFSET>,
            TfMat::<Impl, OFFSET>,
            TfIata::<Impl, OFFSET>,
            Gs1DatabarType1::<Impl, OFFSET>,
            Gs1DatabarType2::<Impl, OFFSET>,
            Gs1DatabarType3::<Impl, OFFSET>,
            Code39::<Impl, OFFSET>,
            Code39Ex::<Impl, OFFSET>,
            Trioptic39::<Impl, OFFSET>,
            Code32::<Impl, OFFSET>,
            Pzn::<Impl, OFFSET>,
            Code93::<Impl, OFFSET>,
            Code93Ex::<Impl, OFFSET>,
            Code128::<Impl, OFFSET>,
            Gs1128::<Impl, OFFSET>,
            Gs1128Coupon::<Impl, OFFSET>,
            UccEan128::<Impl, OFFSET>,
            Sisac::<Impl, OFFSET>,
            Isbt::<Impl, OFFSET>,
            Codabar::<Impl, OFFSET>,
            Code11::<Impl, OFFSET>,
            Msi::<Impl, OFFSET>,
            Plessey::<Impl, OFFSET>,
            Telepen::<Impl, OFFSET>,
            Code16k::<Impl, OFFSET>,
            CodablockA::<Impl, OFFSET>,
            CodablockF::<Impl, OFFSET>,
            Codablock128::<Impl, OFFSET>,
            Code49::<Impl, OFFSET>,
            Aztec::<Impl, OFFSET>,
            DataCode::<Impl, OFFSET>,
            DataMatrix::<Impl, OFFSET>,
            HanXin::<Impl, OFFSET>,
            Maxicode::<Impl, OFFSET>,
            MicroPdf417::<Impl, OFFSET>,
            MicroQr::<Impl, OFFSET>,
            Pdf417::<Impl, OFFSET>,
            Qr::<Impl, OFFSET>,
            MsTag::<Impl, OFFSET>,
            Ccab::<Impl, OFFSET>,
            Ccc::<Impl, OFFSET>,
            Tlc39::<Impl, OFFSET>,
            AusPost::<Impl, OFFSET>,
            CanPost::<Impl, OFFSET>,
            ChinaPost::<Impl, OFFSET>,
            DutchKix::<Impl, OFFSET>,
            InfoMail::<Impl, OFFSET>,
            ItalianPost25::<Impl, OFFSET>,
            ItalianPost39::<Impl, OFFSET>,
            JapanPost::<Impl, OFFSET>,
            KoreanPost::<Impl, OFFSET>,
            SwedenPost::<Impl, OFFSET>,
            UkPost::<Impl, OFFSET>,
            UsIntelligent::<Impl, OFFSET>,
            UsIntelligentPkg::<Impl, OFFSET>,
            UsPlanet::<Impl, OFFSET>,
            UsPostNet::<Impl, OFFSET>,
            Us4StateFics::<Impl, OFFSET>,
            OcrA::<Impl, OFFSET>,
            OcrB::<Impl, OFFSET>,
            Micr::<Impl, OFFSET>,
            ExtendedBase::<Impl, OFFSET>,
            GetName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeSymbologiesStatics2Impl: Sized {
    fn Gs1DWCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeSymbologiesStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeSymbologiesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeSymbologiesStatics2Vtbl {
    pub const fn new<Impl: IBarcodeSymbologiesStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeSymbologiesStatics2Vtbl {
        unsafe extern "system" fn Gs1DWCode<Impl: IBarcodeSymbologiesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gs1DWCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBarcodeSymbologiesStatics2>, base.5, Gs1DWCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeSymbologyAttributesImpl: Sized {
    fn IsCheckDigitValidationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCheckDigitValidationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCheckDigitValidationSupported(&self) -> ::windows::core::Result<bool>;
    fn IsCheckDigitTransmissionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCheckDigitTransmissionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCheckDigitTransmissionSupported(&self) -> ::windows::core::Result<bool>;
    fn DecodeLength1(&self) -> ::windows::core::Result<u32>;
    fn SetDecodeLength1(&self, value: u32) -> ::windows::core::Result<()>;
    fn DecodeLength2(&self) -> ::windows::core::Result<u32>;
    fn SetDecodeLength2(&self, value: u32) -> ::windows::core::Result<()>;
    fn DecodeLengthKind(&self) -> ::windows::core::Result<BarcodeSymbologyDecodeLengthKind>;
    fn SetDecodeLengthKind(&self, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::core::Result<()>;
    fn IsDecodeLengthSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeSymbologyAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeSymbologyAttributes";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeSymbologyAttributesVtbl {
    pub const fn new<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBarcodeSymbologyAttributesVtbl {
        unsafe extern "system" fn IsCheckDigitValidationEnabled<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCheckDigitValidationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCheckDigitValidationEnabled<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsCheckDigitValidationEnabled(value).into()
        }
        unsafe extern "system" fn IsCheckDigitValidationSupported<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCheckDigitValidationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCheckDigitTransmissionEnabled<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCheckDigitTransmissionEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCheckDigitTransmissionEnabled<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsCheckDigitTransmissionEnabled(value).into()
        }
        unsafe extern "system" fn IsCheckDigitTransmissionSupported<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCheckDigitTransmissionSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecodeLength1<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecodeLength1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecodeLength1<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDecodeLength1(value).into()
        }
        unsafe extern "system" fn DecodeLength2<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecodeLength2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecodeLength2<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDecodeLength2(value).into()
        }
        unsafe extern "system" fn DecodeLengthKind<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BarcodeSymbologyDecodeLengthKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecodeLengthKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecodeLengthKind<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDecodeLengthKind(value).into()
        }
        unsafe extern "system" fn IsDecodeLengthSupported<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDecodeLengthSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBarcodeSymbologyAttributes>,
            base.5,
            IsCheckDigitValidationEnabled::<Impl, OFFSET>,
            SetIsCheckDigitValidationEnabled::<Impl, OFFSET>,
            IsCheckDigitValidationSupported::<Impl, OFFSET>,
            IsCheckDigitTransmissionEnabled::<Impl, OFFSET>,
            SetIsCheckDigitTransmissionEnabled::<Impl, OFFSET>,
            IsCheckDigitTransmissionSupported::<Impl, OFFSET>,
            DecodeLength1::<Impl, OFFSET>,
            SetDecodeLength1::<Impl, OFFSET>,
            DecodeLength2::<Impl, OFFSET>,
            SetDecodeLength2::<Impl, OFFSET>,
            DecodeLengthKind::<Impl, OFFSET>,
            SetDecodeLengthKind::<Impl, OFFSET>,
            IsDecodeLengthSupported::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<CashDrawerCapabilities>;
    fn Status(&self) -> ::windows::core::Result<CashDrawerStatus>;
    fn IsDrawerOpen(&self) -> ::windows::core::Result<bool>;
    fn DrawerEventSource(&self) -> ::windows::core::Result<CashDrawerEventSource>;
    fn ClaimDrawerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedCashDrawer>>;
    fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn StatusUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawer";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerVtbl {
    pub const fn new<Impl: ICashDrawerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICashDrawerVtbl {
        unsafe extern "system" fn DeviceId<Impl: ICashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: ICashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: ICashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDrawerOpen<Impl: ICashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDrawerOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawerEventSource<Impl: ICashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawerEventSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClaimDrawerAsync<Impl: ICashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClaimDrawerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHealthAsync<Impl: ICashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckHealthAsync(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatisticsAsync<Impl: ICashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUpdated<Impl: ICashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUpdated<Impl: ICashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStatusUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICashDrawer>, base.5, DeviceId::<Impl, OFFSET>, Capabilities::<Impl, OFFSET>, Status::<Impl, OFFSET>, IsDrawerOpen::<Impl, OFFSET>, DrawerEventSource::<Impl, OFFSET>, ClaimDrawerAsync::<Impl, OFFSET>, CheckHealthAsync::<Impl, OFFSET>, GetStatisticsAsync::<Impl, OFFSET>, StatusUpdated::<Impl, OFFSET>, RemoveStatusUpdated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerCapabilitiesImpl: Sized {
    fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatusReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatusMultiDrawerDetectSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDrawerOpenSensorAvailable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerCapabilitiesVtbl {
    pub const fn new<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICashDrawerCapabilitiesVtbl {
        unsafe extern "system" fn PowerReportingType<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsReportingSupported<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatisticsReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsUpdatingSupported<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatisticsUpdatingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatusReportingSupported<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatusReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatusMultiDrawerDetectSupported<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatusMultiDrawerDetectSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDrawerOpenSensorAvailable<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDrawerOpenSensorAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICashDrawerCapabilities>, base.5, PowerReportingType::<Impl, OFFSET>, IsStatisticsReportingSupported::<Impl, OFFSET>, IsStatisticsUpdatingSupported::<Impl, OFFSET>, IsStatusReportingSupported::<Impl, OFFSET>, IsStatusMultiDrawerDetectSupported::<Impl, OFFSET>, IsDrawerOpenSensorAvailable::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerCloseAlarmImpl: Sized {
    fn SetAlarmTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AlarmTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBeepFrequency(&self, value: u32) -> ::windows::core::Result<()>;
    fn BeepFrequency(&self) -> ::windows::core::Result<u32>;
    fn SetBeepDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BeepDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBeepDelay(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BeepDelay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn AlarmTimeoutExpired(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAlarmTimeoutExpired(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawerCloseAlarm {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerCloseAlarm";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerCloseAlarmVtbl {
    pub const fn new<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICashDrawerCloseAlarmVtbl {
        unsafe extern "system" fn SetAlarmTimeout<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlarmTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlarmTimeout<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlarmTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBeepFrequency<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBeepFrequency(value).into()
        }
        unsafe extern "system" fn BeepFrequency<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeepFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBeepDuration<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBeepDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BeepDuration<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeepDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBeepDelay<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBeepDelay(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BeepDelay<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeepDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlarmTimeoutExpired<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlarmTimeoutExpired(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAlarmTimeoutExpired<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAlarmTimeoutExpired(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartAsync<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICashDrawerCloseAlarm>, base.5, SetAlarmTimeout::<Impl, OFFSET>, AlarmTimeout::<Impl, OFFSET>, SetBeepFrequency::<Impl, OFFSET>, BeepFrequency::<Impl, OFFSET>, SetBeepDuration::<Impl, OFFSET>, BeepDuration::<Impl, OFFSET>, SetBeepDelay::<Impl, OFFSET>, BeepDelay::<Impl, OFFSET>, AlarmTimeoutExpired::<Impl, OFFSET>, RemoveAlarmTimeoutExpired::<Impl, OFFSET>, StartAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerEventSourceImpl: Sized {
    fn DrawerClosed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDrawerClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DrawerOpened(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDrawerOpened(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawerEventSource {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerEventSource";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerEventSourceVtbl {
    pub const fn new<Impl: ICashDrawerEventSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICashDrawerEventSourceVtbl {
        unsafe extern "system" fn DrawerClosed<Impl: ICashDrawerEventSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawerClosed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDrawerClosed<Impl: ICashDrawerEventSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDrawerClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawerOpened<Impl: ICashDrawerEventSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawerOpened(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDrawerOpened<Impl: ICashDrawerEventSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDrawerOpened(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICashDrawerEventSource>, base.5, DrawerClosed::<Impl, OFFSET>, RemoveDrawerClosed::<Impl, OFFSET>, DrawerOpened::<Impl, OFFSET>, RemoveDrawerOpened::<Impl, OFFSET>)
    }
}
pub trait ICashDrawerEventSourceEventArgsImpl: Sized {
    fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer>;
}
impl ::windows::core::RuntimeName for ICashDrawerEventSourceEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerEventSourceEventArgs";
}
impl ICashDrawerEventSourceEventArgsVtbl {
    pub const fn new<Impl: ICashDrawerEventSourceEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICashDrawerEventSourceEventArgsVtbl {
        unsafe extern "system" fn CashDrawer<Impl: ICashDrawerEventSourceEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CashDrawer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICashDrawerEventSourceEventArgs>, base.5, CashDrawer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawerStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerStaticsVtbl {
    pub const fn new<Impl: ICashDrawerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICashDrawerStaticsVtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: ICashDrawerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: ICashDrawerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: ICashDrawerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICashDrawerStatics>, base.5, GetDefaultAsync::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawerStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerStatics2Vtbl {
    pub const fn new<Impl: ICashDrawerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICashDrawerStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: ICashDrawerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICashDrawerStatics2>, base.5, GetDeviceSelectorWithConnectionTypes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerStatusImpl: Sized {
    fn StatusKind(&self) -> ::windows::core::Result<CashDrawerStatusKind>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawerStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerStatus";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerStatusVtbl {
    pub const fn new<Impl: ICashDrawerStatusImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICashDrawerStatusVtbl {
        unsafe extern "system" fn StatusKind<Impl: ICashDrawerStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CashDrawerStatusKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: ICashDrawerStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICashDrawerStatus>, base.5, StatusKind::<Impl, OFFSET>, ExtendedStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerStatusUpdatedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<CashDrawerStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerStatusUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerStatusUpdatedEventArgsVtbl {
    pub const fn new<Impl: ICashDrawerStatusUpdatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICashDrawerStatusUpdatedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: ICashDrawerStatusUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICashDrawerStatusUpdatedEventArgs>, base.5, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScannerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDisabledOnDataReceived(&self) -> ::windows::core::Result<bool>;
    fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDecodeDataEnabled(&self) -> ::windows::core::Result<bool>;
    fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RetainDevice(&self) -> ::windows::core::Result<()>;
    fn SetActiveSymbologiesAsync(&self, symbologies: &::core::option::Option<super::super::Foundation::Collections::IIterable<u32>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateStatisticsAsync(&self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetActiveProfileAsync(&self, profile: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DataReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TriggerPressed(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTriggerPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TriggerReleased(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTriggerReleased(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReleaseDeviceRequested(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImagePreviewReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveImagePreviewReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedBarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedBarcodeScanner";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedBarcodeScannerVtbl {
    pub const fn new<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedBarcodeScannerVtbl {
        unsafe extern "system" fn DeviceId<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDisabledOnDataReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsDisabledOnDataReceived(value).into()
        }
        unsafe extern "system" fn IsDisabledOnDataReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDisabledOnDataReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDecodeDataEnabled<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsDecodeDataEnabled(value).into()
        }
        unsafe extern "system" fn IsDecodeDataEnabled<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDecodeDataEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainDevice<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RetainDevice().into()
        }
        unsafe extern "system" fn SetActiveSymbologiesAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, symbologies: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetActiveSymbologiesAsync(&*(&symbologies as *const <super::super::Foundation::Collections::IIterable<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<u32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetStatisticsAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateStatisticsAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateStatisticsAsync(&*(&statistics as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveProfileAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetActiveProfileAsync(&*(&profile as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDataReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDataReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TriggerPressed<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriggerPressed(&*(&handler as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTriggerPressed<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTriggerPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TriggerReleased<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriggerReleased(&*(&handler as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTriggerReleased<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTriggerReleased(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReleaseDeviceRequested<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDeviceRequested(&*(&handler as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReleaseDeviceRequested<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveReleaseDeviceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImagePreviewReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImagePreviewReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveImagePreviewReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveImagePreviewReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorOccurred<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorOccurred(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveErrorOccurred<Impl: IClaimedBarcodeScannerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveErrorOccurred(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IClaimedBarcodeScanner>,
            base.5,
            DeviceId::<Impl, OFFSET>,
            IsEnabled::<Impl, OFFSET>,
            SetIsDisabledOnDataReceived::<Impl, OFFSET>,
            IsDisabledOnDataReceived::<Impl, OFFSET>,
            SetIsDecodeDataEnabled::<Impl, OFFSET>,
            IsDecodeDataEnabled::<Impl, OFFSET>,
            EnableAsync::<Impl, OFFSET>,
            DisableAsync::<Impl, OFFSET>,
            RetainDevice::<Impl, OFFSET>,
            SetActiveSymbologiesAsync::<Impl, OFFSET>,
            ResetStatisticsAsync::<Impl, OFFSET>,
            UpdateStatisticsAsync::<Impl, OFFSET>,
            SetActiveProfileAsync::<Impl, OFFSET>,
            DataReceived::<Impl, OFFSET>,
            RemoveDataReceived::<Impl, OFFSET>,
            TriggerPressed::<Impl, OFFSET>,
            RemoveTriggerPressed::<Impl, OFFSET>,
            TriggerReleased::<Impl, OFFSET>,
            RemoveTriggerReleased::<Impl, OFFSET>,
            ReleaseDeviceRequested::<Impl, OFFSET>,
            RemoveReleaseDeviceRequested::<Impl, OFFSET>,
            ImagePreviewReceived::<Impl, OFFSET>,
            RemoveImagePreviewReceived::<Impl, OFFSET>,
            ErrorOccurred::<Impl, OFFSET>,
            RemoveErrorOccurred::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScanner1Impl: Sized {
    fn StartSoftwareTriggerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopSoftwareTriggerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedBarcodeScanner1 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedBarcodeScanner1";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedBarcodeScanner1Vtbl {
    pub const fn new<Impl: IClaimedBarcodeScanner1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedBarcodeScanner1Vtbl {
        unsafe extern "system" fn StartSoftwareTriggerAsync<Impl: IClaimedBarcodeScanner1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartSoftwareTriggerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopSoftwareTriggerAsync<Impl: IClaimedBarcodeScanner1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopSoftwareTriggerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedBarcodeScanner1>, base.5, StartSoftwareTriggerAsync::<Impl, OFFSET>, StopSoftwareTriggerAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScanner2Impl: Sized {
    fn GetSymbologyAttributesAsync(&self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeSymbologyAttributes>>;
    fn SetSymbologyAttributesAsync(&self, barcodesymbology: u32, attributes: &::core::option::Option<BarcodeSymbologyAttributes>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedBarcodeScanner2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedBarcodeScanner2";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedBarcodeScanner2Vtbl {
    pub const fn new<Impl: IClaimedBarcodeScanner2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedBarcodeScanner2Vtbl {
        unsafe extern "system" fn GetSymbologyAttributesAsync<Impl: IClaimedBarcodeScanner2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, barcodesymbology: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSymbologyAttributesAsync(barcodesymbology) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSymbologyAttributesAsync<Impl: IClaimedBarcodeScanner2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, barcodesymbology: u32, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSymbologyAttributesAsync(barcodesymbology, &*(&attributes as *const <BarcodeSymbologyAttributes as ::windows::core::Abi>::Abi as *const <BarcodeSymbologyAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedBarcodeScanner2>, base.5, GetSymbologyAttributesAsync::<Impl, OFFSET>, SetSymbologyAttributesAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScanner3Impl: Sized {
    fn ShowVideoPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn HideVideoPreview(&self) -> ::windows::core::Result<()>;
    fn SetIsVideoPreviewShownOnEnable(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsVideoPreviewShownOnEnable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedBarcodeScanner3 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedBarcodeScanner3";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedBarcodeScanner3Vtbl {
    pub const fn new<Impl: IClaimedBarcodeScanner3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedBarcodeScanner3Vtbl {
        unsafe extern "system" fn ShowVideoPreviewAsync<Impl: IClaimedBarcodeScanner3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowVideoPreviewAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HideVideoPreview<Impl: IClaimedBarcodeScanner3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).HideVideoPreview().into()
        }
        unsafe extern "system" fn SetIsVideoPreviewShownOnEnable<Impl: IClaimedBarcodeScanner3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsVideoPreviewShownOnEnable(value).into()
        }
        unsafe extern "system" fn IsVideoPreviewShownOnEnable<Impl: IClaimedBarcodeScanner3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsVideoPreviewShownOnEnable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedBarcodeScanner3>, base.5, ShowVideoPreviewAsync::<Impl, OFFSET>, HideVideoPreview::<Impl, OFFSET>, SetIsVideoPreviewShownOnEnable::<Impl, OFFSET>, IsVideoPreviewShownOnEnable::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScanner4Impl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedBarcodeScanner4 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedBarcodeScanner4";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedBarcodeScanner4Vtbl {
    pub const fn new<Impl: IClaimedBarcodeScanner4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedBarcodeScanner4Vtbl {
        unsafe extern "system" fn Closed<Impl: IClaimedBarcodeScanner4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IClaimedBarcodeScanner4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedBarcodeScanner4>, base.5, Closed::<Impl, OFFSET>, RemoveClosed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScannerClosedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedBarcodeScannerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedBarcodeScannerClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedBarcodeScannerClosedEventArgsVtbl {
    pub const fn new<Impl: IClaimedBarcodeScannerClosedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedBarcodeScannerClosedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedBarcodeScannerClosedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedCashDrawerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsDrawerOpen(&self) -> ::windows::core::Result<bool>;
    fn CloseAlarm(&self) -> ::windows::core::Result<CashDrawerCloseAlarm>;
    fn OpenDrawerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RetainDeviceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ResetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UpdateStatisticsAsync(&self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ReleaseDeviceRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedCashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedCashDrawer";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedCashDrawerVtbl {
    pub const fn new<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedCashDrawerVtbl {
        unsafe extern "system" fn DeviceId<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDrawerOpen<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDrawerOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseAlarm<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseAlarm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenDrawerAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenDrawerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainDeviceAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetainDeviceAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetStatisticsAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateStatisticsAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateStatisticsAsync(&*(&statistics as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDeviceRequested<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDeviceRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReleaseDeviceRequested<Impl: IClaimedCashDrawerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveReleaseDeviceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IClaimedCashDrawer>,
            base.5,
            DeviceId::<Impl, OFFSET>,
            IsEnabled::<Impl, OFFSET>,
            IsDrawerOpen::<Impl, OFFSET>,
            CloseAlarm::<Impl, OFFSET>,
            OpenDrawerAsync::<Impl, OFFSET>,
            EnableAsync::<Impl, OFFSET>,
            DisableAsync::<Impl, OFFSET>,
            RetainDeviceAsync::<Impl, OFFSET>,
            ResetStatisticsAsync::<Impl, OFFSET>,
            UpdateStatisticsAsync::<Impl, OFFSET>,
            ReleaseDeviceRequested::<Impl, OFFSET>,
            RemoveReleaseDeviceRequested::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedCashDrawer2Impl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedCashDrawer2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedCashDrawer2";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedCashDrawer2Vtbl {
    pub const fn new<Impl: IClaimedCashDrawer2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedCashDrawer2Vtbl {
        unsafe extern "system" fn Closed<Impl: IClaimedCashDrawer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IClaimedCashDrawer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedCashDrawer2>, base.5, Closed::<Impl, OFFSET>, RemoveClosed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedCashDrawerClosedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedCashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedCashDrawerClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedCashDrawerClosedEventArgsVtbl {
    pub const fn new<Impl: IClaimedCashDrawerClosedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedCashDrawerClosedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedCashDrawerClosedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedJournalPrinterImpl: Sized {
    fn CreateJob(&self) -> ::windows::core::Result<JournalPrintJob>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedJournalPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedJournalPrinter";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedJournalPrinterVtbl {
    pub const fn new<Impl: IClaimedJournalPrinterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedJournalPrinterVtbl {
        unsafe extern "system" fn CreateJob<Impl: IClaimedJournalPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateJob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedJournalPrinter>, base.5, CreateJob::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedLineDisplayImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<LineDisplayCapabilities>;
    fn PhysicalDeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PhysicalDeviceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceServiceVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultWindow(&self) -> ::windows::core::Result<LineDisplayWindow>;
    fn RetainDevice(&self) -> ::windows::core::Result<()>;
    fn ReleaseDeviceRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedLineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedLineDisplay";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedLineDisplayVtbl {
    pub const fn new<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedLineDisplayVtbl {
        unsafe extern "system" fn DeviceId<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalDeviceName<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhysicalDeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalDeviceDescription<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhysicalDeviceDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceControlDescription<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceControlDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceControlVersion<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceControlVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceServiceVersion<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceServiceVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultWindow<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainDevice<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RetainDevice().into()
        }
        unsafe extern "system" fn ReleaseDeviceRequested<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDeviceRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReleaseDeviceRequested<Impl: IClaimedLineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveReleaseDeviceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IClaimedLineDisplay>,
            base.5,
            DeviceId::<Impl, OFFSET>,
            Capabilities::<Impl, OFFSET>,
            PhysicalDeviceName::<Impl, OFFSET>,
            PhysicalDeviceDescription::<Impl, OFFSET>,
            DeviceControlDescription::<Impl, OFFSET>,
            DeviceControlVersion::<Impl, OFFSET>,
            DeviceServiceVersion::<Impl, OFFSET>,
            DefaultWindow::<Impl, OFFSET>,
            RetainDevice::<Impl, OFFSET>,
            ReleaseDeviceRequested::<Impl, OFFSET>,
            RemoveReleaseDeviceRequested::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedLineDisplay2Impl: Sized {
    fn GetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn CheckPowerStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>;
    fn StatusUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SupportedScreenSizesInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Size>>;
    fn MaxBitmapSizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SupportedCharacterSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>>;
    fn CustomGlyphs(&self) -> ::windows::core::Result<LineDisplayCustomGlyphs>;
    fn GetAttributes(&self) -> ::windows::core::Result<LineDisplayAttributes>;
    fn TryUpdateAttributesAsync(&self, attributes: &::core::option::Option<LineDisplayAttributes>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetDescriptorAsync(&self, descriptor: u32, descriptorstate: LineDisplayDescriptorState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryClearDescriptorsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryCreateWindowAsync(&self, viewport: &super::super::Foundation::Rect, windowsize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayWindow>>;
    fn TryStoreStorageFileBitmapAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>;
    fn TryStoreStorageFileBitmapWithAlignmentAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>;
    fn TryStoreStorageFileBitmapWithAlignmentAndWidthAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedLineDisplay2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedLineDisplay2";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedLineDisplay2Vtbl {
    pub const fn new<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedLineDisplay2Vtbl {
        unsafe extern "system" fn GetStatisticsAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHealthAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckHealthAsync(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckPowerStatusAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckPowerStatusAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUpdated<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUpdated<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStatusUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportedScreenSizesInCharacters<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedScreenSizesInCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBitmapSizeInPixels<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxBitmapSizeInPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCharacterSets<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedCharacterSets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomGlyphs<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomGlyphs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateAttributesAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdateAttributesAsync(&*(&attributes as *const <LineDisplayAttributes as ::windows::core::Abi>::Abi as *const <LineDisplayAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetDescriptorAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, descriptor: u32, descriptorstate: LineDisplayDescriptorState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetDescriptorAsync(descriptor, descriptorstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryClearDescriptorsAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryClearDescriptorsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateWindowAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: super::super::Foundation::Rect, windowsize: super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateWindowAsync(&*(&viewport as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&windowsize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryStoreStorageFileBitmapAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryStoreStorageFileBitmapAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryStoreStorageFileBitmapWithAlignmentAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryStoreStorageFileBitmapWithAlignmentAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), horizontalalignment, verticalalignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryStoreStorageFileBitmapWithAlignmentAndWidthAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryStoreStorageFileBitmapWithAlignmentAndWidthAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), horizontalalignment, verticalalignment, widthinpixels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IClaimedLineDisplay2>,
            base.5,
            GetStatisticsAsync::<Impl, OFFSET>,
            CheckHealthAsync::<Impl, OFFSET>,
            CheckPowerStatusAsync::<Impl, OFFSET>,
            StatusUpdated::<Impl, OFFSET>,
            RemoveStatusUpdated::<Impl, OFFSET>,
            SupportedScreenSizesInCharacters::<Impl, OFFSET>,
            MaxBitmapSizeInPixels::<Impl, OFFSET>,
            SupportedCharacterSets::<Impl, OFFSET>,
            CustomGlyphs::<Impl, OFFSET>,
            GetAttributes::<Impl, OFFSET>,
            TryUpdateAttributesAsync::<Impl, OFFSET>,
            TrySetDescriptorAsync::<Impl, OFFSET>,
            TryClearDescriptorsAsync::<Impl, OFFSET>,
            TryCreateWindowAsync::<Impl, OFFSET>,
            TryStoreStorageFileBitmapAsync::<Impl, OFFSET>,
            TryStoreStorageFileBitmapWithAlignmentAsync::<Impl, OFFSET>,
            TryStoreStorageFileBitmapWithAlignmentAndWidthAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedLineDisplay3Impl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedLineDisplay3 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedLineDisplay3";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedLineDisplay3Vtbl {
    pub const fn new<Impl: IClaimedLineDisplay3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedLineDisplay3Vtbl {
        unsafe extern "system" fn Closed<Impl: IClaimedLineDisplay3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IClaimedLineDisplay3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedLineDisplay3>, base.5, Closed::<Impl, OFFSET>, RemoveClosed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedLineDisplayClosedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedLineDisplayClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedLineDisplayClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedLineDisplayClosedEventArgsVtbl {
    pub const fn new<Impl: IClaimedLineDisplayClosedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedLineDisplayClosedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedLineDisplayClosedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedLineDisplayStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedLineDisplayStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedLineDisplayStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedLineDisplayStaticsVtbl {
    pub const fn new<Impl: IClaimedLineDisplayStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedLineDisplayStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IClaimedLineDisplayStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IClaimedLineDisplayStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: IClaimedLineDisplayStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedLineDisplayStatics>, base.5, FromIdAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>, GetDeviceSelectorWithConnectionTypes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedMagneticStripeReaderImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDisabledOnDataReceived(&self) -> ::windows::core::Result<bool>;
    fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDecodeDataEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsDeviceAuthenticated(&self) -> ::windows::core::Result<bool>;
    fn SetDataEncryptionAlgorithm(&self, value: u32) -> ::windows::core::Result<()>;
    fn DataEncryptionAlgorithm(&self) -> ::windows::core::Result<u32>;
    fn SetTracksToRead(&self, value: MagneticStripeReaderTrackIds) -> ::windows::core::Result<()>;
    fn TracksToRead(&self) -> ::windows::core::Result<MagneticStripeReaderTrackIds>;
    fn SetIsTransmitSentinelsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsTransmitSentinelsEnabled(&self) -> ::windows::core::Result<bool>;
    fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RetainDevice(&self) -> ::windows::core::Result<()>;
    fn SetErrorReportingType(&self, value: MagneticStripeReaderErrorReportingType) -> ::windows::core::Result<()>;
    fn RetrieveDeviceAuthenticationDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn AuthenticateDeviceAsync(&self, responsetoken: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeAuthenticateDeviceAsync(&self, responsetoken: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateKeyAsync(&self, key: &::windows::core::HSTRING, keyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateStatisticsAsync(&self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BankCardDataReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBankCardDataReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AamvaCardDataReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAamvaCardDataReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VendorSpecificDataReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVendorSpecificDataReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReleaseDeviceRequested(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedMagneticStripeReader>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedMagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedMagneticStripeReader";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedMagneticStripeReaderVtbl {
    pub const fn new<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedMagneticStripeReaderVtbl {
        unsafe extern "system" fn DeviceId<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDisabledOnDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsDisabledOnDataReceived(value).into()
        }
        unsafe extern "system" fn IsDisabledOnDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDisabledOnDataReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDecodeDataEnabled<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsDecodeDataEnabled(value).into()
        }
        unsafe extern "system" fn IsDecodeDataEnabled<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDecodeDataEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDeviceAuthenticated<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDeviceAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataEncryptionAlgorithm<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDataEncryptionAlgorithm(value).into()
        }
        unsafe extern "system" fn DataEncryptionAlgorithm<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataEncryptionAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTracksToRead<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MagneticStripeReaderTrackIds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTracksToRead(value).into()
        }
        unsafe extern "system" fn TracksToRead<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackIds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TracksToRead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTransmitSentinelsEnabled<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsTransmitSentinelsEnabled(value).into()
        }
        unsafe extern "system" fn IsTransmitSentinelsEnabled<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTransmitSentinelsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainDevice<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RetainDevice().into()
        }
        unsafe extern "system" fn SetErrorReportingType<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MagneticStripeReaderErrorReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetErrorReportingType(value).into()
        }
        unsafe extern "system" fn RetrieveDeviceAuthenticationDataAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetrieveDeviceAuthenticationDataAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticateDeviceAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthenticateDeviceAsync(::core::slice::from_raw_parts(::core::mem::transmute_copy(&responsetoken), responseToken_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeAuthenticateDeviceAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeAuthenticateDeviceAsync(::core::slice::from_raw_parts(::core::mem::transmute_copy(&responsetoken), responseToken_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateKeyAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateKeyAsync(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&keyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetStatisticsAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateStatisticsAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateStatisticsAsync(&*(&statistics as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BankCardDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BankCardDataReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBankCardDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBankCardDataReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AamvaCardDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AamvaCardDataReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAamvaCardDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAamvaCardDataReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VendorSpecificDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VendorSpecificDataReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVendorSpecificDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveVendorSpecificDataReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReleaseDeviceRequested<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDeviceRequested(&*(&handler as *const <super::super::Foundation::EventHandler<ClaimedMagneticStripeReader> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ClaimedMagneticStripeReader> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReleaseDeviceRequested<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveReleaseDeviceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorOccurred<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorOccurred(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveErrorOccurred<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveErrorOccurred(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IClaimedMagneticStripeReader>,
            base.5,
            DeviceId::<Impl, OFFSET>,
            IsEnabled::<Impl, OFFSET>,
            SetIsDisabledOnDataReceived::<Impl, OFFSET>,
            IsDisabledOnDataReceived::<Impl, OFFSET>,
            SetIsDecodeDataEnabled::<Impl, OFFSET>,
            IsDecodeDataEnabled::<Impl, OFFSET>,
            IsDeviceAuthenticated::<Impl, OFFSET>,
            SetDataEncryptionAlgorithm::<Impl, OFFSET>,
            DataEncryptionAlgorithm::<Impl, OFFSET>,
            SetTracksToRead::<Impl, OFFSET>,
            TracksToRead::<Impl, OFFSET>,
            SetIsTransmitSentinelsEnabled::<Impl, OFFSET>,
            IsTransmitSentinelsEnabled::<Impl, OFFSET>,
            EnableAsync::<Impl, OFFSET>,
            DisableAsync::<Impl, OFFSET>,
            RetainDevice::<Impl, OFFSET>,
            SetErrorReportingType::<Impl, OFFSET>,
            RetrieveDeviceAuthenticationDataAsync::<Impl, OFFSET>,
            AuthenticateDeviceAsync::<Impl, OFFSET>,
            DeAuthenticateDeviceAsync::<Impl, OFFSET>,
            UpdateKeyAsync::<Impl, OFFSET>,
            ResetStatisticsAsync::<Impl, OFFSET>,
            UpdateStatisticsAsync::<Impl, OFFSET>,
            BankCardDataReceived::<Impl, OFFSET>,
            RemoveBankCardDataReceived::<Impl, OFFSET>,
            AamvaCardDataReceived::<Impl, OFFSET>,
            RemoveAamvaCardDataReceived::<Impl, OFFSET>,
            VendorSpecificDataReceived::<Impl, OFFSET>,
            RemoveVendorSpecificDataReceived::<Impl, OFFSET>,
            ReleaseDeviceRequested::<Impl, OFFSET>,
            RemoveReleaseDeviceRequested::<Impl, OFFSET>,
            ErrorOccurred::<Impl, OFFSET>,
            RemoveErrorOccurred::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedMagneticStripeReader2Impl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedMagneticStripeReader2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedMagneticStripeReader2";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedMagneticStripeReader2Vtbl {
    pub const fn new<Impl: IClaimedMagneticStripeReader2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedMagneticStripeReader2Vtbl {
        unsafe extern "system" fn Closed<Impl: IClaimedMagneticStripeReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IClaimedMagneticStripeReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedMagneticStripeReader2>, base.5, Closed::<Impl, OFFSET>, RemoveClosed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedMagneticStripeReaderClosedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedMagneticStripeReaderClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedMagneticStripeReaderClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedMagneticStripeReaderClosedEventArgsVtbl {
    pub const fn new<Impl: IClaimedMagneticStripeReaderClosedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedMagneticStripeReaderClosedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedMagneticStripeReaderClosedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedPosPrinterImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetCharacterSet(&self, value: u32) -> ::windows::core::Result<()>;
    fn CharacterSet(&self) -> ::windows::core::Result<u32>;
    fn IsCoverOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCharacterSetMappingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetMapMode(&self, value: PosPrinterMapMode) -> ::windows::core::Result<()>;
    fn MapMode(&self) -> ::windows::core::Result<PosPrinterMapMode>;
    fn Receipt(&self) -> ::windows::core::Result<ClaimedReceiptPrinter>;
    fn Slip(&self) -> ::windows::core::Result<ClaimedSlipPrinter>;
    fn Journal(&self) -> ::windows::core::Result<ClaimedJournalPrinter>;
    fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RetainDeviceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ResetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UpdateStatisticsAsync(&self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ReleaseDeviceRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedPosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedPosPrinter";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedPosPrinterVtbl {
    pub const fn new<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedPosPrinterVtbl {
        unsafe extern "system" fn DeviceId<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterSet<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCharacterSet(value).into()
        }
        unsafe extern "system" fn CharacterSet<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCoverOpen<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCoverOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCharacterSetMappingEnabled<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsCharacterSetMappingEnabled(value).into()
        }
        unsafe extern "system" fn IsCharacterSetMappingEnabled<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCharacterSetMappingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapMode<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PosPrinterMapMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMapMode(value).into()
        }
        unsafe extern "system" fn MapMode<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterMapMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receipt<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Receipt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Slip<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Slip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAsync<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableAsync<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainDeviceAsync<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetainDeviceAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetStatisticsAsync<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateStatisticsAsync<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateStatisticsAsync(&*(&statistics as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDeviceRequested<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDeviceRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReleaseDeviceRequested<Impl: IClaimedPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveReleaseDeviceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IClaimedPosPrinter>,
            base.5,
            DeviceId::<Impl, OFFSET>,
            IsEnabled::<Impl, OFFSET>,
            SetCharacterSet::<Impl, OFFSET>,
            CharacterSet::<Impl, OFFSET>,
            IsCoverOpen::<Impl, OFFSET>,
            SetIsCharacterSetMappingEnabled::<Impl, OFFSET>,
            IsCharacterSetMappingEnabled::<Impl, OFFSET>,
            SetMapMode::<Impl, OFFSET>,
            MapMode::<Impl, OFFSET>,
            Receipt::<Impl, OFFSET>,
            Slip::<Impl, OFFSET>,
            Journal::<Impl, OFFSET>,
            EnableAsync::<Impl, OFFSET>,
            DisableAsync::<Impl, OFFSET>,
            RetainDeviceAsync::<Impl, OFFSET>,
            ResetStatisticsAsync::<Impl, OFFSET>,
            UpdateStatisticsAsync::<Impl, OFFSET>,
            ReleaseDeviceRequested::<Impl, OFFSET>,
            RemoveReleaseDeviceRequested::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedPosPrinter2Impl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedPosPrinter2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedPosPrinter2";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedPosPrinter2Vtbl {
    pub const fn new<Impl: IClaimedPosPrinter2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedPosPrinter2Vtbl {
        unsafe extern "system" fn Closed<Impl: IClaimedPosPrinter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IClaimedPosPrinter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedPosPrinter2>, base.5, Closed::<Impl, OFFSET>, RemoveClosed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedPosPrinterClosedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedPosPrinterClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedPosPrinterClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedPosPrinterClosedEventArgsVtbl {
    pub const fn new<Impl: IClaimedPosPrinterClosedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedPosPrinterClosedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedPosPrinterClosedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedReceiptPrinterImpl: Sized {
    fn SidewaysMaxLines(&self) -> ::windows::core::Result<u32>;
    fn SidewaysMaxChars(&self) -> ::windows::core::Result<u32>;
    fn LinesToPaperCut(&self) -> ::windows::core::Result<u32>;
    fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn PrintArea(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn CreateJob(&self) -> ::windows::core::Result<ReceiptPrintJob>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedReceiptPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedReceiptPrinter";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedReceiptPrinterVtbl {
    pub const fn new<Impl: IClaimedReceiptPrinterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedReceiptPrinterVtbl {
        unsafe extern "system" fn SidewaysMaxLines<Impl: IClaimedReceiptPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SidewaysMaxLines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SidewaysMaxChars<Impl: IClaimedReceiptPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SidewaysMaxChars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinesToPaperCut<Impl: IClaimedReceiptPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LinesToPaperCut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageSize<Impl: IClaimedReceiptPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrintArea<Impl: IClaimedReceiptPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrintArea() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJob<Impl: IClaimedReceiptPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateJob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClaimedReceiptPrinter>, base.5, SidewaysMaxLines::<Impl, OFFSET>, SidewaysMaxChars::<Impl, OFFSET>, LinesToPaperCut::<Impl, OFFSET>, PageSize::<Impl, OFFSET>, PrintArea::<Impl, OFFSET>, CreateJob::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedSlipPrinterImpl: Sized {
    fn SidewaysMaxLines(&self) -> ::windows::core::Result<u32>;
    fn SidewaysMaxChars(&self) -> ::windows::core::Result<u32>;
    fn MaxLines(&self) -> ::windows::core::Result<u32>;
    fn LinesNearEndToEnd(&self) -> ::windows::core::Result<u32>;
    fn PrintSide(&self) -> ::windows::core::Result<PosPrinterPrintSide>;
    fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn PrintArea(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn OpenJaws(&self) -> ::windows::core::Result<()>;
    fn CloseJaws(&self) -> ::windows::core::Result<()>;
    fn InsertSlipAsync(&self, timeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RemoveSlipAsync(&self, timeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ChangePrintSide(&self, printside: PosPrinterPrintSide) -> ::windows::core::Result<()>;
    fn CreateJob(&self) -> ::windows::core::Result<SlipPrintJob>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedSlipPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedSlipPrinter";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedSlipPrinterVtbl {
    pub const fn new<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClaimedSlipPrinterVtbl {
        unsafe extern "system" fn SidewaysMaxLines<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SidewaysMaxLines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SidewaysMaxChars<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SidewaysMaxChars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxLines<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxLines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinesNearEndToEnd<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LinesNearEndToEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintSide<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterPrintSide) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrintSide() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageSize<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrintArea<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrintArea() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenJaws<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OpenJaws().into()
        }
        unsafe extern "system" fn CloseJaws<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CloseJaws().into()
        }
        unsafe extern "system" fn InsertSlipAsync<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertSlipAsync(&*(&timeout as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSlipAsync<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveSlipAsync(&*(&timeout as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangePrintSide<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printside: PosPrinterPrintSide) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ChangePrintSide(printside).into()
        }
        unsafe extern "system" fn CreateJob<Impl: IClaimedSlipPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateJob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IClaimedSlipPrinter>,
            base.5,
            SidewaysMaxLines::<Impl, OFFSET>,
            SidewaysMaxChars::<Impl, OFFSET>,
            MaxLines::<Impl, OFFSET>,
            LinesNearEndToEnd::<Impl, OFFSET>,
            PrintSide::<Impl, OFFSET>,
            PageSize::<Impl, OFFSET>,
            PrintArea::<Impl, OFFSET>,
            OpenJaws::<Impl, OFFSET>,
            CloseJaws::<Impl, OFFSET>,
            InsertSlipAsync::<Impl, OFFSET>,
            RemoveSlipAsync::<Impl, OFFSET>,
            ChangePrintSide::<Impl, OFFSET>,
            CreateJob::<Impl, OFFSET>,
        )
    }
}
pub trait ICommonClaimedPosPrinterStationImpl: Sized {
    fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()>;
    fn CharactersPerLine(&self) -> ::windows::core::Result<u32>;
    fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn LineHeight(&self) -> ::windows::core::Result<u32>;
    fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()>;
    fn LineSpacing(&self) -> ::windows::core::Result<u32>;
    fn LineWidth(&self) -> ::windows::core::Result<u32>;
    fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsLetterQuality(&self) -> ::windows::core::Result<bool>;
    fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool>;
    fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()>;
    fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge>;
    fn IsCoverOpen(&self) -> ::windows::core::Result<bool>;
    fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool>;
    fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool>;
    fn IsHeadCleaning(&self) -> ::windows::core::Result<bool>;
    fn IsPaperEmpty(&self) -> ::windows::core::Result<bool>;
    fn IsReadyToPrint(&self) -> ::windows::core::Result<bool>;
    fn ValidateData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICommonClaimedPosPrinterStation {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonClaimedPosPrinterStation";
}
impl ICommonClaimedPosPrinterStationVtbl {
    pub const fn new<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommonClaimedPosPrinterStationVtbl {
        unsafe extern "system" fn SetCharactersPerLine<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCharactersPerLine(value).into()
        }
        unsafe extern "system" fn CharactersPerLine<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CharactersPerLine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLineHeight(value).into()
        }
        unsafe extern "system" fn LineHeight<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LineHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineSpacing<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLineSpacing(value).into()
        }
        unsafe extern "system" fn LineSpacing<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LineSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineWidth<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LineWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLetterQuality<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsLetterQuality(value).into()
        }
        unsafe extern "system" fn IsLetterQuality<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLetterQuality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEnd<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPaperNearEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorCartridge<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PosPrinterColorCartridge) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColorCartridge(value).into()
        }
        unsafe extern "system" fn ColorCartridge<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCartridge) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorCartridge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCoverOpen<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCoverOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeRemoved<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCartridgeRemoved() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeEmpty<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCartridgeEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHeadCleaning<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHeadCleaning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmpty<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPaperEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadyToPrint<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReadyToPrint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateData<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidateData(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ICommonClaimedPosPrinterStation>,
            base.5,
            SetCharactersPerLine::<Impl, OFFSET>,
            CharactersPerLine::<Impl, OFFSET>,
            SetLineHeight::<Impl, OFFSET>,
            LineHeight::<Impl, OFFSET>,
            SetLineSpacing::<Impl, OFFSET>,
            LineSpacing::<Impl, OFFSET>,
            LineWidth::<Impl, OFFSET>,
            SetIsLetterQuality::<Impl, OFFSET>,
            IsLetterQuality::<Impl, OFFSET>,
            IsPaperNearEnd::<Impl, OFFSET>,
            SetColorCartridge::<Impl, OFFSET>,
            ColorCartridge::<Impl, OFFSET>,
            IsCoverOpen::<Impl, OFFSET>,
            IsCartridgeRemoved::<Impl, OFFSET>,
            IsCartridgeEmpty::<Impl, OFFSET>,
            IsHeadCleaning::<Impl, OFFSET>,
            IsPaperEmpty::<Impl, OFFSET>,
            IsReadyToPrint::<Impl, OFFSET>,
            ValidateData::<Impl, OFFSET>,
        )
    }
}
pub trait ICommonPosPrintStationCapabilitiesImpl: Sized {
    fn IsPrinterPresent(&self) -> ::windows::core::Result<bool>;
    fn IsDualColorSupported(&self) -> ::windows::core::Result<bool>;
    fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities>;
    fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors>;
    fn IsBoldSupported(&self) -> ::windows::core::Result<bool>;
    fn IsItalicSupported(&self) -> ::windows::core::Result<bool>;
    fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool>;
    fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
impl ::windows::core::RuntimeName for ICommonPosPrintStationCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonPosPrintStationCapabilities";
}
impl ICommonPosPrintStationCapabilitiesVtbl {
    pub const fn new<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommonPosPrintStationCapabilitiesVtbl {
        unsafe extern "system" fn IsPrinterPresent<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPrinterPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDualColorSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDualColorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorCartridgeCapabilities<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorCartridgeCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CartridgeSensors<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterCartridgeSensors) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CartridgeSensors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBoldSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBoldSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsItalicSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsItalicSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUnderlineSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUnderlineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighPrintSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDoubleHighPrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleWidePrintSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDoubleWidePrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighDoubleWidePrintSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDoubleHighDoubleWidePrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmptySensorSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPaperEmptySensorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEndSensorSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPaperNearEndSensorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCharactersPerLine<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedCharactersPerLine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ICommonPosPrintStationCapabilities>,
            base.5,
            IsPrinterPresent::<Impl, OFFSET>,
            IsDualColorSupported::<Impl, OFFSET>,
            ColorCartridgeCapabilities::<Impl, OFFSET>,
            CartridgeSensors::<Impl, OFFSET>,
            IsBoldSupported::<Impl, OFFSET>,
            IsItalicSupported::<Impl, OFFSET>,
            IsUnderlineSupported::<Impl, OFFSET>,
            IsDoubleHighPrintSupported::<Impl, OFFSET>,
            IsDoubleWidePrintSupported::<Impl, OFFSET>,
            IsDoubleHighDoubleWidePrintSupported::<Impl, OFFSET>,
            IsPaperEmptySensorSupported::<Impl, OFFSET>,
            IsPaperNearEndSensorSupported::<Impl, OFFSET>,
            SupportedCharactersPerLine::<Impl, OFFSET>,
        )
    }
}
pub trait ICommonReceiptSlipCapabilitiesImpl: Sized + ICommonPosPrintStationCapabilitiesImpl {
    fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool>;
    fn IsBitmapSupported(&self) -> ::windows::core::Result<bool>;
    fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool>;
    fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool>;
    fn Is180RotationSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool>;
    fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities>;
    fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
    fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
}
impl ::windows::core::RuntimeName for ICommonReceiptSlipCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonReceiptSlipCapabilities";
}
impl ICommonReceiptSlipCapabilitiesVtbl {
    pub const fn new<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommonReceiptSlipCapabilitiesVtbl {
        unsafe extern "system" fn IsBarcodeSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBarcodeSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBitmapSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBitmapSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLeft90RotationSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLeft90RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRight90RotationSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRight90RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is180RotationSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Is180RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrintAreaSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPrintAreaSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuledLineCapabilities<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterRuledLineCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RuledLineCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBarcodeRotations<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedBarcodeRotations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBitmapRotations<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedBitmapRotations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommonReceiptSlipCapabilities>, base.5, IsBarcodeSupported::<Impl, OFFSET>, IsBitmapSupported::<Impl, OFFSET>, IsLeft90RotationSupported::<Impl, OFFSET>, IsRight90RotationSupported::<Impl, OFFSET>, Is180RotationSupported::<Impl, OFFSET>, IsPrintAreaSupported::<Impl, OFFSET>, RuledLineCapabilities::<Impl, OFFSET>, SupportedBarcodeRotations::<Impl, OFFSET>, SupportedBitmapRotations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJournalPrintJobImpl: Sized {
    fn Print(&self, data: &::windows::core::HSTRING, printoptions: &::core::option::Option<PosPrinterPrintOptions>) -> ::windows::core::Result<()>;
    fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()>;
    fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJournalPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IJournalPrintJob";
}
#[cfg(feature = "implement_exclusive")]
impl IJournalPrintJobVtbl {
    pub const fn new<Impl: IJournalPrintJobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IJournalPrintJobVtbl {
        unsafe extern "system" fn Print<Impl: IJournalPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Print(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&printoptions as *const <PosPrinterPrintOptions as ::windows::core::Abi>::Abi as *const <PosPrinterPrintOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FeedPaperByLine<Impl: IJournalPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FeedPaperByLine(linecount).into()
        }
        unsafe extern "system" fn FeedPaperByMapModeUnit<Impl: IJournalPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FeedPaperByMapModeUnit(distance).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IJournalPrintJob>, base.5, Print::<Impl, OFFSET>, FeedPaperByLine::<Impl, OFFSET>, FeedPaperByMapModeUnit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJournalPrinterCapabilitiesImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJournalPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.IJournalPrinterCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IJournalPrinterCapabilitiesVtbl {
    pub const fn new<Impl: IJournalPrinterCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IJournalPrinterCapabilitiesVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IJournalPrinterCapabilities>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJournalPrinterCapabilities2Impl: Sized {
    fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJournalPrinterCapabilities2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IJournalPrinterCapabilities2";
}
#[cfg(feature = "implement_exclusive")]
impl IJournalPrinterCapabilities2Vtbl {
    pub const fn new<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IJournalPrinterCapabilities2Vtbl {
        unsafe extern "system" fn IsReverseVideoSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReverseVideoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStrikethroughSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStrikethroughSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSuperscriptSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSuperscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscriptSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSubscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByLineSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByLineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByMapModeUnitSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByMapModeUnitSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IJournalPrinterCapabilities2>, base.5, IsReverseVideoSupported::<Impl, OFFSET>, IsStrikethroughSupported::<Impl, OFFSET>, IsSuperscriptSupported::<Impl, OFFSET>, IsSubscriptSupported::<Impl, OFFSET>, IsReversePaperFeedByLineSupported::<Impl, OFFSET>, IsReversePaperFeedByMapModeUnitSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<LineDisplayCapabilities>;
    fn PhysicalDeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PhysicalDeviceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceServiceVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ClaimAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplay";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayVtbl {
    pub const fn new<Impl: ILineDisplayImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayVtbl {
        unsafe extern "system" fn DeviceId<Impl: ILineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: ILineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalDeviceName<Impl: ILineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhysicalDeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalDeviceDescription<Impl: ILineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhysicalDeviceDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceControlDescription<Impl: ILineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceControlDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceControlVersion<Impl: ILineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceControlVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceServiceVersion<Impl: ILineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceServiceVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClaimAsync<Impl: ILineDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClaimAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplay>, base.5, DeviceId::<Impl, OFFSET>, Capabilities::<Impl, OFFSET>, PhysicalDeviceName::<Impl, OFFSET>, PhysicalDeviceDescription::<Impl, OFFSET>, DeviceControlDescription::<Impl, OFFSET>, DeviceControlVersion::<Impl, OFFSET>, DeviceServiceVersion::<Impl, OFFSET>, ClaimAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplay2Impl: Sized {
    fn CheckPowerStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplay2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplay2";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplay2Vtbl {
    pub const fn new<Impl: ILineDisplay2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplay2Vtbl {
        unsafe extern "system" fn CheckPowerStatusAsync<Impl: ILineDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckPowerStatusAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplay2>, base.5, CheckPowerStatusAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayAttributesImpl: Sized {
    fn IsPowerNotifyEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPowerNotifyEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Brightness(&self) -> ::windows::core::Result<i32>;
    fn SetBrightness(&self, value: i32) -> ::windows::core::Result<()>;
    fn BlinkRate(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBlinkRate(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ScreenSizeInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetScreenSizeInCharacters(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn CharacterSet(&self) -> ::windows::core::Result<i32>;
    fn SetCharacterSet(&self, value: i32) -> ::windows::core::Result<()>;
    fn IsCharacterSetMappingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentWindow(&self) -> ::windows::core::Result<LineDisplayWindow>;
    fn SetCurrentWindow(&self, value: &::core::option::Option<LineDisplayWindow>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayAttributes";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayAttributesVtbl {
    pub const fn new<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayAttributesVtbl {
        unsafe extern "system" fn IsPowerNotifyEnabled<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPowerNotifyEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPowerNotifyEnabled<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsPowerNotifyEnabled(value).into()
        }
        unsafe extern "system" fn Brightness<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBrightness<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBrightness(value).into()
        }
        unsafe extern "system" fn BlinkRate<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BlinkRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlinkRate<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBlinkRate(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScreenSizeInCharacters<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScreenSizeInCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScreenSizeInCharacters<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScreenSizeInCharacters(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CharacterSet<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterSet<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCharacterSet(value).into()
        }
        unsafe extern "system" fn IsCharacterSetMappingEnabled<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCharacterSetMappingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCharacterSetMappingEnabled<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsCharacterSetMappingEnabled(value).into()
        }
        unsafe extern "system" fn CurrentWindow<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentWindow<Impl: ILineDisplayAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCurrentWindow(&*(&value as *const <LineDisplayWindow as ::windows::core::Abi>::Abi as *const <LineDisplayWindow as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ILineDisplayAttributes>,
            base.5,
            IsPowerNotifyEnabled::<Impl, OFFSET>,
            SetIsPowerNotifyEnabled::<Impl, OFFSET>,
            Brightness::<Impl, OFFSET>,
            SetBrightness::<Impl, OFFSET>,
            BlinkRate::<Impl, OFFSET>,
            SetBlinkRate::<Impl, OFFSET>,
            ScreenSizeInCharacters::<Impl, OFFSET>,
            SetScreenSizeInCharacters::<Impl, OFFSET>,
            CharacterSet::<Impl, OFFSET>,
            SetCharacterSet::<Impl, OFFSET>,
            IsCharacterSetMappingEnabled::<Impl, OFFSET>,
            SetIsCharacterSetMappingEnabled::<Impl, OFFSET>,
            CurrentWindow::<Impl, OFFSET>,
            SetCurrentWindow::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayCapabilitiesImpl: Sized {
    fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool>;
    fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn CanChangeScreenSize(&self) -> ::windows::core::Result<bool>;
    fn CanDisplayBitmaps(&self) -> ::windows::core::Result<bool>;
    fn CanReadCharacterAtCursor(&self) -> ::windows::core::Result<bool>;
    fn CanMapCharacterSets(&self) -> ::windows::core::Result<bool>;
    fn CanDisplayCustomGlyphs(&self) -> ::windows::core::Result<bool>;
    fn CanReverse(&self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity>;
    fn CanBlink(&self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity>;
    fn CanChangeBlinkRate(&self) -> ::windows::core::Result<bool>;
    fn IsBrightnessSupported(&self) -> ::windows::core::Result<bool>;
    fn IsCursorSupported(&self) -> ::windows::core::Result<bool>;
    fn IsHorizontalMarqueeSupported(&self) -> ::windows::core::Result<bool>;
    fn IsVerticalMarqueeSupported(&self) -> ::windows::core::Result<bool>;
    fn IsInterCharacterWaitSupported(&self) -> ::windows::core::Result<bool>;
    fn SupportedDescriptors(&self) -> ::windows::core::Result<u32>;
    fn SupportedWindows(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayCapabilitiesVtbl {
    pub const fn new<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayCapabilitiesVtbl {
        unsafe extern "system" fn IsStatisticsReportingSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatisticsReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsUpdatingSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatisticsUpdatingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerReportingType<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanChangeScreenSize<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanChangeScreenSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDisplayBitmaps<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanDisplayBitmaps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanReadCharacterAtCursor<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanReadCharacterAtCursor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMapCharacterSets<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanMapCharacterSets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDisplayCustomGlyphs<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanDisplayCustomGlyphs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanReverse<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanReverse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanBlink<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanBlink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanChangeBlinkRate<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanChangeBlinkRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBrightnessSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBrightnessSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCursorSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCursorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHorizontalMarqueeSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHorizontalMarqueeSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVerticalMarqueeSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsVerticalMarqueeSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInterCharacterWaitSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInterCharacterWaitSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedDescriptors<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWindows<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedWindows() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ILineDisplayCapabilities>,
            base.5,
            IsStatisticsReportingSupported::<Impl, OFFSET>,
            IsStatisticsUpdatingSupported::<Impl, OFFSET>,
            PowerReportingType::<Impl, OFFSET>,
            CanChangeScreenSize::<Impl, OFFSET>,
            CanDisplayBitmaps::<Impl, OFFSET>,
            CanReadCharacterAtCursor::<Impl, OFFSET>,
            CanMapCharacterSets::<Impl, OFFSET>,
            CanDisplayCustomGlyphs::<Impl, OFFSET>,
            CanReverse::<Impl, OFFSET>,
            CanBlink::<Impl, OFFSET>,
            CanChangeBlinkRate::<Impl, OFFSET>,
            IsBrightnessSupported::<Impl, OFFSET>,
            IsCursorSupported::<Impl, OFFSET>,
            IsHorizontalMarqueeSupported::<Impl, OFFSET>,
            IsVerticalMarqueeSupported::<Impl, OFFSET>,
            IsInterCharacterWaitSupported::<Impl, OFFSET>,
            SupportedDescriptors::<Impl, OFFSET>,
            SupportedWindows::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayCursorImpl: Sized {
    fn CanCustomize(&self) -> ::windows::core::Result<bool>;
    fn IsBlinkSupported(&self) -> ::windows::core::Result<bool>;
    fn IsBlockSupported(&self) -> ::windows::core::Result<bool>;
    fn IsHalfBlockSupported(&self) -> ::windows::core::Result<bool>;
    fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReverseSupported(&self) -> ::windows::core::Result<bool>;
    fn IsOtherSupported(&self) -> ::windows::core::Result<bool>;
    fn GetAttributes(&self) -> ::windows::core::Result<LineDisplayCursorAttributes>;
    fn TryUpdateAttributesAsync(&self, attributes: &::core::option::Option<LineDisplayCursorAttributes>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayCursor {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayCursor";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayCursorVtbl {
    pub const fn new<Impl: ILineDisplayCursorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayCursorVtbl {
        unsafe extern "system" fn CanCustomize<Impl: ILineDisplayCursorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanCustomize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBlinkSupported<Impl: ILineDisplayCursorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBlinkSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBlockSupported<Impl: ILineDisplayCursorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBlockSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHalfBlockSupported<Impl: ILineDisplayCursorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHalfBlockSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUnderlineSupported<Impl: ILineDisplayCursorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUnderlineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReverseSupported<Impl: ILineDisplayCursorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReverseSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOtherSupported<Impl: ILineDisplayCursorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOtherSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: ILineDisplayCursorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateAttributesAsync<Impl: ILineDisplayCursorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdateAttributesAsync(&*(&attributes as *const <LineDisplayCursorAttributes as ::windows::core::Abi>::Abi as *const <LineDisplayCursorAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplayCursor>, base.5, CanCustomize::<Impl, OFFSET>, IsBlinkSupported::<Impl, OFFSET>, IsBlockSupported::<Impl, OFFSET>, IsHalfBlockSupported::<Impl, OFFSET>, IsUnderlineSupported::<Impl, OFFSET>, IsReverseSupported::<Impl, OFFSET>, IsOtherSupported::<Impl, OFFSET>, GetAttributes::<Impl, OFFSET>, TryUpdateAttributesAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayCursorAttributesImpl: Sized {
    fn IsBlinkEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsBlinkEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CursorType(&self) -> ::windows::core::Result<LineDisplayCursorType>;
    fn SetCursorType(&self, value: LineDisplayCursorType) -> ::windows::core::Result<()>;
    fn IsAutoAdvanceEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsAutoAdvanceEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetPosition(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayCursorAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayCursorAttributes";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayCursorAttributesVtbl {
    pub const fn new<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayCursorAttributesVtbl {
        unsafe extern "system" fn IsBlinkEnabled<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBlinkEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsBlinkEnabled<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsBlinkEnabled(value).into()
        }
        unsafe extern "system" fn CursorType<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayCursorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CursorType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCursorType<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: LineDisplayCursorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCursorType(value).into()
        }
        unsafe extern "system" fn IsAutoAdvanceEnabled<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAutoAdvanceEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAutoAdvanceEnabled<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsAutoAdvanceEnabled(value).into()
        }
        unsafe extern "system" fn Position<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplayCursorAttributes>, base.5, IsBlinkEnabled::<Impl, OFFSET>, SetIsBlinkEnabled::<Impl, OFFSET>, CursorType::<Impl, OFFSET>, SetCursorType::<Impl, OFFSET>, IsAutoAdvanceEnabled::<Impl, OFFSET>, SetIsAutoAdvanceEnabled::<Impl, OFFSET>, Position::<Impl, OFFSET>, SetPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayCustomGlyphsImpl: Sized {
    fn SizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SupportedGlyphCodes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn TryRedefineAsync(&self, glyphcode: u32, glyphdata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayCustomGlyphs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayCustomGlyphs";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayCustomGlyphsVtbl {
    pub const fn new<Impl: ILineDisplayCustomGlyphsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayCustomGlyphsVtbl {
        unsafe extern "system" fn SizeInPixels<Impl: ILineDisplayCustomGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SizeInPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedGlyphCodes<Impl: ILineDisplayCustomGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedGlyphCodes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRedefineAsync<Impl: ILineDisplayCustomGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphcode: u32, glyphdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryRedefineAsync(glyphcode, &*(&glyphdata as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplayCustomGlyphs>, base.5, SizeInPixels::<Impl, OFFSET>, SupportedGlyphCodes::<Impl, OFFSET>, TryRedefineAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayMarqueeImpl: Sized {
    fn Format(&self) -> ::windows::core::Result<LineDisplayMarqueeFormat>;
    fn SetFormat(&self, value: LineDisplayMarqueeFormat) -> ::windows::core::Result<()>;
    fn RepeatWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetRepeatWaitInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ScrollWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetScrollWaitInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TryStartScrollingAsync(&self, direction: LineDisplayScrollDirection) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryStopScrollingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayMarquee {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayMarquee";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayMarqueeVtbl {
    pub const fn new<Impl: ILineDisplayMarqueeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayMarqueeVtbl {
        unsafe extern "system" fn Format<Impl: ILineDisplayMarqueeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayMarqueeFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFormat<Impl: ILineDisplayMarqueeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: LineDisplayMarqueeFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFormat(value).into()
        }
        unsafe extern "system" fn RepeatWaitInterval<Impl: ILineDisplayMarqueeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RepeatWaitInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepeatWaitInterval<Impl: ILineDisplayMarqueeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRepeatWaitInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScrollWaitInterval<Impl: ILineDisplayMarqueeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScrollWaitInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollWaitInterval<Impl: ILineDisplayMarqueeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScrollWaitInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryStartScrollingAsync<Impl: ILineDisplayMarqueeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, direction: LineDisplayScrollDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryStartScrollingAsync(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryStopScrollingAsync<Impl: ILineDisplayMarqueeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryStopScrollingAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplayMarquee>, base.5, Format::<Impl, OFFSET>, SetFormat::<Impl, OFFSET>, RepeatWaitInterval::<Impl, OFFSET>, SetRepeatWaitInterval::<Impl, OFFSET>, ScrollWaitInterval::<Impl, OFFSET>, SetScrollWaitInterval::<Impl, OFFSET>, TryStartScrollingAsync::<Impl, OFFSET>, TryStopScrollingAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayStaticsVtbl {
    pub const fn new<Impl: ILineDisplayStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: ILineDisplayStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefaultAsync<Impl: ILineDisplayStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: ILineDisplayStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: ILineDisplayStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplayStatics>, base.5, FromIdAsync::<Impl, OFFSET>, GetDefaultAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>, GetDeviceSelectorWithConnectionTypes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStatics2Impl: Sized {
    fn StatisticsCategorySelector(&self) -> ::windows::core::Result<LineDisplayStatisticsCategorySelector>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayStatics2Vtbl {
    pub const fn new<Impl: ILineDisplayStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayStatics2Vtbl {
        unsafe extern "system" fn StatisticsCategorySelector<Impl: ILineDisplayStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatisticsCategorySelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplayStatics2>, base.5, StatisticsCategorySelector::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStatisticsCategorySelectorImpl: Sized {
    fn AllStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UnifiedPosStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ManufacturerStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayStatisticsCategorySelector {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayStatisticsCategorySelector";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayStatisticsCategorySelectorVtbl {
    pub const fn new<Impl: ILineDisplayStatisticsCategorySelectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayStatisticsCategorySelectorVtbl {
        unsafe extern "system" fn AllStatistics<Impl: ILineDisplayStatisticsCategorySelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnifiedPosStatistics<Impl: ILineDisplayStatisticsCategorySelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnifiedPosStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerStatistics<Impl: ILineDisplayStatisticsCategorySelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManufacturerStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplayStatisticsCategorySelector>, base.5, AllStatistics::<Impl, OFFSET>, UnifiedPosStatistics::<Impl, OFFSET>, ManufacturerStatistics::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStatusUpdatedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<LineDisplayPowerStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayStatusUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayStatusUpdatedEventArgsVtbl {
    pub const fn new<Impl: ILineDisplayStatusUpdatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayStatusUpdatedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: ILineDisplayStatusUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayPowerStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplayStatusUpdatedEventArgs>, base.5, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStoredBitmapImpl: Sized {
    fn EscapeSequence(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TryDeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayStoredBitmap {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayStoredBitmap";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayStoredBitmapVtbl {
    pub const fn new<Impl: ILineDisplayStoredBitmapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayStoredBitmapVtbl {
        unsafe extern "system" fn EscapeSequence<Impl: ILineDisplayStoredBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EscapeSequence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDeleteAsync<Impl: ILineDisplayStoredBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryDeleteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplayStoredBitmap>, base.5, EscapeSequence::<Impl, OFFSET>, TryDeleteAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayWindowImpl: Sized {
    fn SizeInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn InterCharacterWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetInterCharacterWaitInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TryRefreshAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayTextAsync(&self, text: &::windows::core::HSTRING, displayattribute: LineDisplayTextAttribute) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayTextAtPositionAsync(&self, text: &::windows::core::HSTRING, displayattribute: LineDisplayTextAttribute, startposition: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayTextNormalAsync(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryScrollTextAsync(&self, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryClearTextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayWindow {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayWindow";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayWindowVtbl {
    pub const fn new<Impl: ILineDisplayWindowImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayWindowVtbl {
        unsafe extern "system" fn SizeInCharacters<Impl: ILineDisplayWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SizeInCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterCharacterWaitInterval<Impl: ILineDisplayWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InterCharacterWaitInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterCharacterWaitInterval<Impl: ILineDisplayWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInterCharacterWaitInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryRefreshAsync<Impl: ILineDisplayWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryRefreshAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayTextAsync<Impl: ILineDisplayWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayattribute: LineDisplayTextAttribute, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryDisplayTextAsync(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), displayattribute) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayTextAtPositionAsync<Impl: ILineDisplayWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayattribute: LineDisplayTextAttribute, startposition: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryDisplayTextAtPositionAsync(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), displayattribute, &*(&startposition as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayTextNormalAsync<Impl: ILineDisplayWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryDisplayTextNormalAsync(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryScrollTextAsync<Impl: ILineDisplayWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryScrollTextAsync(direction, numberofcolumnsorrows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryClearTextAsync<Impl: ILineDisplayWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryClearTextAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILineDisplayWindow>, base.5, SizeInCharacters::<Impl, OFFSET>, InterCharacterWaitInterval::<Impl, OFFSET>, SetInterCharacterWaitInterval::<Impl, OFFSET>, TryRefreshAsync::<Impl, OFFSET>, TryDisplayTextAsync::<Impl, OFFSET>, TryDisplayTextAtPositionAsync::<Impl, OFFSET>, TryDisplayTextNormalAsync::<Impl, OFFSET>, TryScrollTextAsync::<Impl, OFFSET>, TryClearTextAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayWindow2Impl: Sized {
    fn Cursor(&self) -> ::windows::core::Result<LineDisplayCursor>;
    fn Marquee(&self) -> ::windows::core::Result<LineDisplayMarquee>;
    fn ReadCharacterAtCursorAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn TryDisplayStoredBitmapAtCursorAsync(&self, bitmap: &::core::option::Option<LineDisplayStoredBitmap>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtCursorAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtPointAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, offsetinpixels: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtPointWithWidthAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, offsetinpixels: &super::super::Foundation::Point, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayWindow2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayWindow2";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayWindow2Vtbl {
    pub const fn new<Impl: ILineDisplayWindow2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILineDisplayWindow2Vtbl {
        unsafe extern "system" fn Cursor<Impl: ILineDisplayWindow2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cursor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Marquee<Impl: ILineDisplayWindow2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Marquee() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadCharacterAtCursorAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadCharacterAtCursorAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStoredBitmapAtCursorAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryDisplayStoredBitmapAtCursorAsync(&*(&bitmap as *const <LineDisplayStoredBitmap as ::windows::core::Abi>::Abi as *const <LineDisplayStoredBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStorageFileBitmapAtCursorAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryDisplayStorageFileBitmapAtCursorAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), horizontalalignment, verticalalignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), horizontalalignment, verticalalignment, widthinpixels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStorageFileBitmapAtPointAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, offsetinpixels: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryDisplayStorageFileBitmapAtPointAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), &*(&offsetinpixels as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStorageFileBitmapAtPointWithWidthAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, offsetinpixels: super::super::Foundation::Point, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryDisplayStorageFileBitmapAtPointWithWidthAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), &*(&offsetinpixels as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), widthinpixels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ILineDisplayWindow2>,
            base.5,
            Cursor::<Impl, OFFSET>,
            Marquee::<Impl, OFFSET>,
            ReadCharacterAtCursorAsync::<Impl, OFFSET>,
            TryDisplayStoredBitmapAtCursorAsync::<Impl, OFFSET>,
            TryDisplayStorageFileBitmapAtCursorAsync::<Impl, OFFSET>,
            TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync::<Impl, OFFSET>,
            TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync::<Impl, OFFSET>,
            TryDisplayStorageFileBitmapAtPointAsync::<Impl, OFFSET>,
            TryDisplayStorageFileBitmapAtPointWithWidthAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<MagneticStripeReaderCapabilities>;
    fn SupportedCardTypes(&self) -> ::windows::core::Result<::windows::core::Array<u32>>;
    fn DeviceAuthenticationProtocol(&self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationProtocol>;
    fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ClaimReaderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedMagneticStripeReader>>;
    fn RetrieveStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn GetErrorReportingType(&self) -> ::windows::core::Result<MagneticStripeReaderErrorReportingType>;
    fn StatusUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReader";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderVtbl {
    pub const fn new<Impl: IMagneticStripeReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderVtbl {
        unsafe extern "system" fn DeviceId<Impl: IMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: IMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCardTypes<Impl: IMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedCardTypes() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceAuthenticationProtocol<Impl: IMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderAuthenticationProtocol) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceAuthenticationProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHealthAsync<Impl: IMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckHealthAsync(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClaimReaderAsync<Impl: IMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClaimReaderAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveStatisticsAsync<Impl: IMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetrieveStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorReportingType<Impl: IMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderErrorReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUpdated<Impl: IMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUpdated<Impl: IMagneticStripeReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStatusUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMagneticStripeReader>, base.5, DeviceId::<Impl, OFFSET>, Capabilities::<Impl, OFFSET>, SupportedCardTypes::<Impl, OFFSET>, DeviceAuthenticationProtocol::<Impl, OFFSET>, CheckHealthAsync::<Impl, OFFSET>, ClaimReaderAsync::<Impl, OFFSET>, RetrieveStatisticsAsync::<Impl, OFFSET>, GetErrorReportingType::<Impl, OFFSET>, StatusUpdated::<Impl, OFFSET>, RemoveStatusUpdated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl: Sized {
    fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport>;
    fn LicenseNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExpirationDate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Restrictions(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Class(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Endorsements(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BirthDate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Surname(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Suffix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Gender(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HairColor(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EyeColor(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Height(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Weight(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderAamvaCardDataReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderAamvaCardDataReceivedEventArgsVtbl {
    pub const fn new<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderAamvaCardDataReceivedEventArgsVtbl {
        unsafe extern "system" fn Report<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Report() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LicenseNumber<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LicenseNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restrictions<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Restrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Class<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Endorsements<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Endorsements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BirthDate<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BirthDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstName<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FirstName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Surname<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Surname() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suffix<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Suffix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gender<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gender() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HairColor<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HairColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EyeColor<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EyeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Weight<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn City<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).City() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostalCode<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PostalCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderAamvaCardDataReceivedEventArgs>,
            base.5,
            Report::<Impl, OFFSET>,
            LicenseNumber::<Impl, OFFSET>,
            ExpirationDate::<Impl, OFFSET>,
            Restrictions::<Impl, OFFSET>,
            Class::<Impl, OFFSET>,
            Endorsements::<Impl, OFFSET>,
            BirthDate::<Impl, OFFSET>,
            FirstName::<Impl, OFFSET>,
            Surname::<Impl, OFFSET>,
            Suffix::<Impl, OFFSET>,
            Gender::<Impl, OFFSET>,
            HairColor::<Impl, OFFSET>,
            EyeColor::<Impl, OFFSET>,
            Height::<Impl, OFFSET>,
            Weight::<Impl, OFFSET>,
            Address::<Impl, OFFSET>,
            City::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            PostalCode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderBankCardDataReceivedEventArgsImpl: Sized {
    fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport>;
    fn AccountNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExpirationDate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MiddleInitial(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Surname(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Suffix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderBankCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderBankCardDataReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderBankCardDataReceivedEventArgsVtbl {
    pub const fn new<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderBankCardDataReceivedEventArgsVtbl {
        unsafe extern "system" fn Report<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Report() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccountNumber<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccountNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceCode<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstName<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FirstName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MiddleInitial<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MiddleInitial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Surname<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Surname() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suffix<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Suffix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderBankCardDataReceivedEventArgs>, base.5, Report::<Impl, OFFSET>, AccountNumber::<Impl, OFFSET>, ExpirationDate::<Impl, OFFSET>, ServiceCode::<Impl, OFFSET>, Title::<Impl, OFFSET>, FirstName::<Impl, OFFSET>, MiddleInitial::<Impl, OFFSET>, Surname::<Impl, OFFSET>, Suffix::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderCapabilitiesImpl: Sized {
    fn CardAuthentication(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedEncryptionAlgorithms(&self) -> ::windows::core::Result<u32>;
    fn AuthenticationLevel(&self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationLevel>;
    fn IsIsoSupported(&self) -> ::windows::core::Result<bool>;
    fn IsJisOneSupported(&self) -> ::windows::core::Result<bool>;
    fn IsJisTwoSupported(&self) -> ::windows::core::Result<bool>;
    fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsTrackDataMaskingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsTransmitSentinelsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderCapabilitiesVtbl {
    pub const fn new<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderCapabilitiesVtbl {
        unsafe extern "system" fn CardAuthentication<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CardAuthentication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedEncryptionAlgorithms<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedEncryptionAlgorithms() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationLevel<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderAuthenticationLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIsoSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsIsoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsJisOneSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsJisOneSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsJisTwoSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsJisTwoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerReportingType<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsReportingSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatisticsReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsUpdatingSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatisticsUpdatingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTrackDataMaskingSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTrackDataMaskingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransmitSentinelsSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTransmitSentinelsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderCapabilities>,
            base.5,
            CardAuthentication::<Impl, OFFSET>,
            SupportedEncryptionAlgorithms::<Impl, OFFSET>,
            AuthenticationLevel::<Impl, OFFSET>,
            IsIsoSupported::<Impl, OFFSET>,
            IsJisOneSupported::<Impl, OFFSET>,
            IsJisTwoSupported::<Impl, OFFSET>,
            PowerReportingType::<Impl, OFFSET>,
            IsStatisticsReportingSupported::<Impl, OFFSET>,
            IsStatisticsUpdatingSupported::<Impl, OFFSET>,
            IsTrackDataMaskingSupported::<Impl, OFFSET>,
            IsTransmitSentinelsSupported::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderCardTypesStaticsImpl: Sized {
    fn Unknown(&self) -> ::windows::core::Result<u32>;
    fn Bank(&self) -> ::windows::core::Result<u32>;
    fn Aamva(&self) -> ::windows::core::Result<u32>;
    fn ExtendedBase(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderCardTypesStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderCardTypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderCardTypesStaticsVtbl {
    pub const fn new<Impl: IMagneticStripeReaderCardTypesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderCardTypesStaticsVtbl {
        unsafe extern "system" fn Unknown<Impl: IMagneticStripeReaderCardTypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unknown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bank<Impl: IMagneticStripeReaderCardTypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bank() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Aamva<Impl: IMagneticStripeReaderCardTypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Aamva() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedBase<Impl: IMagneticStripeReaderCardTypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedBase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderCardTypesStatics>, base.5, Unknown::<Impl, OFFSET>, Bank::<Impl, OFFSET>, Aamva::<Impl, OFFSET>, ExtendedBase::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderEncryptionAlgorithmsStaticsImpl: Sized {
    fn None(&self) -> ::windows::core::Result<u32>;
    fn TripleDesDukpt(&self) -> ::windows::core::Result<u32>;
    fn ExtendedBase(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderEncryptionAlgorithmsStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderEncryptionAlgorithmsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderEncryptionAlgorithmsStaticsVtbl {
    pub const fn new<Impl: IMagneticStripeReaderEncryptionAlgorithmsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderEncryptionAlgorithmsStaticsVtbl {
        unsafe extern "system" fn None<Impl: IMagneticStripeReaderEncryptionAlgorithmsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).None() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TripleDesDukpt<Impl: IMagneticStripeReaderEncryptionAlgorithmsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TripleDesDukpt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedBase<Impl: IMagneticStripeReaderEncryptionAlgorithmsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedBase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderEncryptionAlgorithmsStatics>, base.5, None::<Impl, OFFSET>, TripleDesDukpt::<Impl, OFFSET>, ExtendedBase::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderErrorOccurredEventArgsImpl: Sized {
    fn Track1Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn Track2Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn Track3Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn Track4Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn ErrorData(&self) -> ::windows::core::Result<UnifiedPosErrorData>;
    fn PartialInputData(&self) -> ::windows::core::Result<MagneticStripeReaderReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderErrorOccurredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderErrorOccurredEventArgsVtbl {
    pub const fn new<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderErrorOccurredEventArgsVtbl {
        unsafe extern "system" fn Track1Status<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Track1Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track2Status<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Track2Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track3Status<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Track3Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track4Status<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Track4Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorData<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartialInputData<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PartialInputData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderErrorOccurredEventArgs>, base.5, Track1Status::<Impl, OFFSET>, Track2Status::<Impl, OFFSET>, Track3Status::<Impl, OFFSET>, Track4Status::<Impl, OFFSET>, ErrorData::<Impl, OFFSET>, PartialInputData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderReportImpl: Sized {
    fn CardType(&self) -> ::windows::core::Result<u32>;
    fn Track1(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Track2(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Track3(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Track4(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn CardAuthenticationData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CardAuthenticationDataLength(&self) -> ::windows::core::Result<u32>;
    fn AdditionalSecurityInformation(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderReport";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderReportVtbl {
    pub const fn new<Impl: IMagneticStripeReaderReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderReportVtbl {
        unsafe extern "system" fn CardType<Impl: IMagneticStripeReaderReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CardType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track1<Impl: IMagneticStripeReaderReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Track1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track2<Impl: IMagneticStripeReaderReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Track2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track3<Impl: IMagneticStripeReaderReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Track3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track4<Impl: IMagneticStripeReaderReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Track4() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMagneticStripeReaderReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardAuthenticationData<Impl: IMagneticStripeReaderReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CardAuthenticationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardAuthenticationDataLength<Impl: IMagneticStripeReaderReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CardAuthenticationDataLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdditionalSecurityInformation<Impl: IMagneticStripeReaderReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdditionalSecurityInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderReport>, base.5, CardType::<Impl, OFFSET>, Track1::<Impl, OFFSET>, Track2::<Impl, OFFSET>, Track3::<Impl, OFFSET>, Track4::<Impl, OFFSET>, Properties::<Impl, OFFSET>, CardAuthenticationData::<Impl, OFFSET>, CardAuthenticationDataLength::<Impl, OFFSET>, AdditionalSecurityInformation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderStaticsVtbl {
    pub const fn new<Impl: IMagneticStripeReaderStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderStaticsVtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: IMagneticStripeReaderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IMagneticStripeReaderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IMagneticStripeReaderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderStatics>, base.5, GetDefaultAsync::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderStatics2Vtbl {
    pub const fn new<Impl: IMagneticStripeReaderStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: IMagneticStripeReaderStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderStatics2>, base.5, GetDeviceSelectorWithConnectionTypes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderStatusUpdatedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MagneticStripeReaderStatus>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderStatusUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderStatusUpdatedEventArgsVtbl {
    pub const fn new<Impl: IMagneticStripeReaderStatusUpdatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderStatusUpdatedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IMagneticStripeReaderStatusUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IMagneticStripeReaderStatusUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderStatusUpdatedEventArgs>, base.5, Status::<Impl, OFFSET>, ExtendedStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderTrackDataImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn DiscretionaryData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn EncryptedData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderTrackData {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderTrackData";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderTrackDataVtbl {
    pub const fn new<Impl: IMagneticStripeReaderTrackDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderTrackDataVtbl {
        unsafe extern "system" fn Data<Impl: IMagneticStripeReaderTrackDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscretionaryData<Impl: IMagneticStripeReaderTrackDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DiscretionaryData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptedData<Impl: IMagneticStripeReaderTrackDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncryptedData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderTrackData>, base.5, Data::<Impl, OFFSET>, DiscretionaryData::<Impl, OFFSET>, EncryptedData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgsImpl: Sized {
    fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgsVtbl {
    pub const fn new<Impl: IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgsVtbl {
        unsafe extern "system" fn Report<Impl: IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Report() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs>, base.5, Report::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<PosPrinterCapabilities>;
    fn SupportedCharacterSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn SupportedTypeFaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Status(&self) -> ::windows::core::Result<PosPrinterStatus>;
    fn ClaimPrinterAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedPosPrinter>>;
    fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn StatusUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinter";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterVtbl {
    pub const fn new<Impl: IPosPrinterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinterVtbl {
        unsafe extern "system" fn DeviceId<Impl: IPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: IPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCharacterSets<Impl: IPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedCharacterSets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTypeFaces<Impl: IPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedTypeFaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClaimPrinterAsync<Impl: IPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClaimPrinterAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHealthAsync<Impl: IPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckHealthAsync(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatisticsAsync<Impl: IPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUpdated<Impl: IPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUpdated<Impl: IPosPrinterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStatusUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPosPrinter>, base.5, DeviceId::<Impl, OFFSET>, Capabilities::<Impl, OFFSET>, SupportedCharacterSets::<Impl, OFFSET>, SupportedTypeFaces::<Impl, OFFSET>, Status::<Impl, OFFSET>, ClaimPrinterAsync::<Impl, OFFSET>, CheckHealthAsync::<Impl, OFFSET>, GetStatisticsAsync::<Impl, OFFSET>, StatusUpdated::<Impl, OFFSET>, RemoveStatusUpdated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinter2Impl: Sized {
    fn SupportedBarcodeSymbologies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn GetFontProperty(&self, typeface: &::windows::core::HSTRING) -> ::windows::core::Result<PosPrinterFontProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinter2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinter2";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinter2Vtbl {
    pub const fn new<Impl: IPosPrinter2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinter2Vtbl {
        unsafe extern "system" fn SupportedBarcodeSymbologies<Impl: IPosPrinter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedBarcodeSymbologies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontProperty<Impl: IPosPrinter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typeface: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFontProperty(&*(&typeface as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPosPrinter2>, base.5, SupportedBarcodeSymbologies::<Impl, OFFSET>, GetFontProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterCapabilitiesImpl: Sized {
    fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool>;
    fn DefaultCharacterSet(&self) -> ::windows::core::Result<u32>;
    fn HasCoverSensor(&self) -> ::windows::core::Result<bool>;
    fn CanMapCharacterSet(&self) -> ::windows::core::Result<bool>;
    fn IsTransactionSupported(&self) -> ::windows::core::Result<bool>;
    fn Receipt(&self) -> ::windows::core::Result<ReceiptPrinterCapabilities>;
    fn Slip(&self) -> ::windows::core::Result<SlipPrinterCapabilities>;
    fn Journal(&self) -> ::windows::core::Result<JournalPrinterCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterCapabilitiesVtbl {
    pub const fn new<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinterCapabilitiesVtbl {
        unsafe extern "system" fn PowerReportingType<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsReportingSupported<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatisticsReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsUpdatingSupported<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStatisticsUpdatingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultCharacterSet<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultCharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCoverSensor<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasCoverSensor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMapCharacterSet<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanMapCharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransactionSupported<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTransactionSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receipt<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Receipt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Slip<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Slip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPosPrinterCapabilities>, base.5, PowerReportingType::<Impl, OFFSET>, IsStatisticsReportingSupported::<Impl, OFFSET>, IsStatisticsUpdatingSupported::<Impl, OFFSET>, DefaultCharacterSet::<Impl, OFFSET>, HasCoverSensor::<Impl, OFFSET>, CanMapCharacterSet::<Impl, OFFSET>, IsTransactionSupported::<Impl, OFFSET>, Receipt::<Impl, OFFSET>, Slip::<Impl, OFFSET>, Journal::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterCharacterSetIdsStaticsImpl: Sized {
    fn Utf16LE(&self) -> ::windows::core::Result<u32>;
    fn Ascii(&self) -> ::windows::core::Result<u32>;
    fn Ansi(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterCharacterSetIdsStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterCharacterSetIdsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterCharacterSetIdsStaticsVtbl {
    pub const fn new<Impl: IPosPrinterCharacterSetIdsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinterCharacterSetIdsStaticsVtbl {
        unsafe extern "system" fn Utf16LE<Impl: IPosPrinterCharacterSetIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Utf16LE() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ascii<Impl: IPosPrinterCharacterSetIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ascii() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ansi<Impl: IPosPrinterCharacterSetIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ansi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPosPrinterCharacterSetIdsStatics>, base.5, Utf16LE::<Impl, OFFSET>, Ascii::<Impl, OFFSET>, Ansi::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterFontPropertyImpl: Sized {
    fn TypeFace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsScalableToAnySize(&self) -> ::windows::core::Result<bool>;
    fn CharacterSizes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SizeUInt32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterFontProperty {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterFontProperty";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterFontPropertyVtbl {
    pub const fn new<Impl: IPosPrinterFontPropertyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinterFontPropertyVtbl {
        unsafe extern "system" fn TypeFace<Impl: IPosPrinterFontPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TypeFace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScalableToAnySize<Impl: IPosPrinterFontPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsScalableToAnySize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacterSizes<Impl: IPosPrinterFontPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CharacterSizes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPosPrinterFontProperty>, base.5, TypeFace::<Impl, OFFSET>, IsScalableToAnySize::<Impl, OFFSET>, CharacterSizes::<Impl, OFFSET>)
    }
}
pub trait IPosPrinterJobImpl: Sized {
    fn Print(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrintLine(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrintNewline(&self) -> ::windows::core::Result<()>;
    fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
impl ::windows::core::RuntimeName for IPosPrinterJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterJob";
}
impl IPosPrinterJobVtbl {
    pub const fn new<Impl: IPosPrinterJobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinterJobVtbl {
        unsafe extern "system" fn Print<Impl: IPosPrinterJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Print(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrintLine<Impl: IPosPrinterJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PrintLine(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrintNewline<Impl: IPosPrinterJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PrintNewline().into()
        }
        unsafe extern "system" fn ExecuteAsync<Impl: IPosPrinterJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecuteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPosPrinterJob>, base.5, Print::<Impl, OFFSET>, PrintLine::<Impl, OFFSET>, PrintNewline::<Impl, OFFSET>, ExecuteAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterPrintOptionsImpl: Sized {
    fn TypeFace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTypeFace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CharacterHeight(&self) -> ::windows::core::Result<u32>;
    fn SetCharacterHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn Bold(&self) -> ::windows::core::Result<bool>;
    fn SetBold(&self, value: bool) -> ::windows::core::Result<()>;
    fn Italic(&self) -> ::windows::core::Result<bool>;
    fn SetItalic(&self, value: bool) -> ::windows::core::Result<()>;
    fn Underline(&self) -> ::windows::core::Result<bool>;
    fn SetUnderline(&self, value: bool) -> ::windows::core::Result<()>;
    fn ReverseVideo(&self) -> ::windows::core::Result<bool>;
    fn SetReverseVideo(&self, value: bool) -> ::windows::core::Result<()>;
    fn Strikethrough(&self) -> ::windows::core::Result<bool>;
    fn SetStrikethrough(&self, value: bool) -> ::windows::core::Result<()>;
    fn Superscript(&self) -> ::windows::core::Result<bool>;
    fn SetSuperscript(&self, value: bool) -> ::windows::core::Result<()>;
    fn Subscript(&self) -> ::windows::core::Result<bool>;
    fn SetSubscript(&self, value: bool) -> ::windows::core::Result<()>;
    fn DoubleWide(&self) -> ::windows::core::Result<bool>;
    fn SetDoubleWide(&self, value: bool) -> ::windows::core::Result<()>;
    fn DoubleHigh(&self) -> ::windows::core::Result<bool>;
    fn SetDoubleHigh(&self, value: bool) -> ::windows::core::Result<()>;
    fn Alignment(&self) -> ::windows::core::Result<PosPrinterAlignment>;
    fn SetAlignment(&self, value: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn CharacterSet(&self) -> ::windows::core::Result<u32>;
    fn SetCharacterSet(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterPrintOptions {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterPrintOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterPrintOptionsVtbl {
    pub const fn new<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinterPrintOptionsVtbl {
        unsafe extern "system" fn TypeFace<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TypeFace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeFace<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTypeFace(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CharacterHeight<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CharacterHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterHeight<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCharacterHeight(value).into()
        }
        unsafe extern "system" fn Bold<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBold(value).into()
        }
        unsafe extern "system" fn Italic<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Italic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetItalic(value).into()
        }
        unsafe extern "system" fn Underline<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Underline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUnderline(value).into()
        }
        unsafe extern "system" fn ReverseVideo<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReverseVideo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReverseVideo<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReverseVideo(value).into()
        }
        unsafe extern "system" fn Strikethrough<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Strikethrough() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStrikethrough(value).into()
        }
        unsafe extern "system" fn Superscript<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Superscript() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuperscript<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSuperscript(value).into()
        }
        unsafe extern "system" fn Subscript<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subscript() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscript<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSubscript(value).into()
        }
        unsafe extern "system" fn DoubleWide<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DoubleWide() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoubleWide<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDoubleWide(value).into()
        }
        unsafe extern "system" fn DoubleHigh<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DoubleHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoubleHigh<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDoubleHigh(value).into()
        }
        unsafe extern "system" fn Alignment<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Alignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlignment(value).into()
        }
        unsafe extern "system" fn CharacterSet<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterSet<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCharacterSet(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPosPrinterPrintOptions>,
            base.5,
            TypeFace::<Impl, OFFSET>,
            SetTypeFace::<Impl, OFFSET>,
            CharacterHeight::<Impl, OFFSET>,
            SetCharacterHeight::<Impl, OFFSET>,
            Bold::<Impl, OFFSET>,
            SetBold::<Impl, OFFSET>,
            Italic::<Impl, OFFSET>,
            SetItalic::<Impl, OFFSET>,
            Underline::<Impl, OFFSET>,
            SetUnderline::<Impl, OFFSET>,
            ReverseVideo::<Impl, OFFSET>,
            SetReverseVideo::<Impl, OFFSET>,
            Strikethrough::<Impl, OFFSET>,
            SetStrikethrough::<Impl, OFFSET>,
            Superscript::<Impl, OFFSET>,
            SetSuperscript::<Impl, OFFSET>,
            Subscript::<Impl, OFFSET>,
            SetSubscript::<Impl, OFFSET>,
            DoubleWide::<Impl, OFFSET>,
            SetDoubleWide::<Impl, OFFSET>,
            DoubleHigh::<Impl, OFFSET>,
            SetDoubleHigh::<Impl, OFFSET>,
            Alignment::<Impl, OFFSET>,
            SetAlignment::<Impl, OFFSET>,
            CharacterSet::<Impl, OFFSET>,
            SetCharacterSet::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterReleaseDeviceRequestedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterReleaseDeviceRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterReleaseDeviceRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterReleaseDeviceRequestedEventArgsVtbl {
    pub const fn new<Impl: IPosPrinterReleaseDeviceRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinterReleaseDeviceRequestedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPosPrinterReleaseDeviceRequestedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterStaticsVtbl {
    pub const fn new<Impl: IPosPrinterStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinterStaticsVtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: IPosPrinterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IPosPrinterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IPosPrinterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPosPrinterStatics>, base.5, GetDefaultAsync::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterStatics2Vtbl {
    pub const fn new<Impl: IPosPrinterStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinterStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: IPosPrinterStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPosPrinterStatics2>, base.5, GetDeviceSelectorWithConnectionTypes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterStatusImpl: Sized {
    fn StatusKind(&self) -> ::windows::core::Result<PosPrinterStatusKind>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterStatus";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterStatusVtbl {
    pub const fn new<Impl: IPosPrinterStatusImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinterStatusVtbl {
        unsafe extern "system" fn StatusKind<Impl: IPosPrinterStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterStatusKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IPosPrinterStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPosPrinterStatus>, base.5, StatusKind::<Impl, OFFSET>, ExtendedStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterStatusUpdatedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PosPrinterStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterStatusUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterStatusUpdatedEventArgsVtbl {
    pub const fn new<Impl: IPosPrinterStatusUpdatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPosPrinterStatusUpdatedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IPosPrinterStatusUpdatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPosPrinterStatusUpdatedEventArgs>, base.5, Status::<Impl, OFFSET>)
    }
}
pub trait IReceiptOrSlipJobImpl: Sized + IPosPrinterJobImpl {
    fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()>;
    fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()>;
    fn SetPrintArea(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetBitmap(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn SetBitmapCustomWidthStandardAlign(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()>;
    fn SetCustomAlignedBitmap(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn SetBitmapCustomWidthCustomAlign(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()>;
    fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()>;
    fn DrawRuledLine(&self, positionlist: &::windows::core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()>;
    fn PrintBarcode(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn PrintBarcodeCustomAlign(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn PrintBitmap(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn PrintBitmapCustomWidthStandardAlign(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()>;
    fn PrintCustomAlignedBitmap(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn PrintBitmapCustomWidthCustomAlign(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IReceiptOrSlipJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptOrSlipJob";
}
impl IReceiptOrSlipJobVtbl {
    pub const fn new<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReceiptOrSlipJobVtbl {
        unsafe extern "system" fn SetBarcodeRotation<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PosPrinterRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBarcodeRotation(value).into()
        }
        unsafe extern "system" fn SetPrintRotation<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPrintRotation(value, includebitmaps).into()
        }
        unsafe extern "system" fn SetPrintArea<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPrintArea(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetBitmap<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBitmap(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthStandardAlign<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBitmapCustomWidthStandardAlign(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment, width).into()
        }
        unsafe extern "system" fn SetCustomAlignedBitmap<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCustomAlignedBitmap(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthCustomAlign<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBitmapCustomWidthCustomAlign(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance, width).into()
        }
        unsafe extern "system" fn PrintSavedBitmap<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PrintSavedBitmap(bitmapnumber).into()
        }
        unsafe extern "system" fn DrawRuledLine<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, positionlist: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DrawRuledLine(&*(&positionlist as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), linedirection, linewidth, linestyle, linecolor).into()
        }
        unsafe extern "system" fn PrintBarcode<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PrintBarcode(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), symbology, height, width, textposition, alignment).into()
        }
        unsafe extern "system" fn PrintBarcodeCustomAlign<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PrintBarcodeCustomAlign(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), symbology, height, width, textposition, alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmap<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PrintBitmap(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthStandardAlign<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PrintBitmapCustomWidthStandardAlign(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment, width).into()
        }
        unsafe extern "system" fn PrintCustomAlignedBitmap<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PrintCustomAlignedBitmap(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthCustomAlign<Impl: IReceiptOrSlipJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PrintBitmapCustomWidthCustomAlign(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance, width).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IReceiptOrSlipJob>,
            base.5,
            SetBarcodeRotation::<Impl, OFFSET>,
            SetPrintRotation::<Impl, OFFSET>,
            SetPrintArea::<Impl, OFFSET>,
            SetBitmap::<Impl, OFFSET>,
            SetBitmapCustomWidthStandardAlign::<Impl, OFFSET>,
            SetCustomAlignedBitmap::<Impl, OFFSET>,
            SetBitmapCustomWidthCustomAlign::<Impl, OFFSET>,
            PrintSavedBitmap::<Impl, OFFSET>,
            DrawRuledLine::<Impl, OFFSET>,
            PrintBarcode::<Impl, OFFSET>,
            PrintBarcodeCustomAlign::<Impl, OFFSET>,
            PrintBitmap::<Impl, OFFSET>,
            PrintBitmapCustomWidthStandardAlign::<Impl, OFFSET>,
            PrintCustomAlignedBitmap::<Impl, OFFSET>,
            PrintBitmapCustomWidthCustomAlign::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrintJobImpl: Sized {
    fn MarkFeed(&self, kind: PosPrinterMarkFeedKind) -> ::windows::core::Result<()>;
    fn CutPaper(&self, percentage: f64) -> ::windows::core::Result<()>;
    fn CutPaperDefault(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReceiptPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptPrintJob";
}
#[cfg(feature = "implement_exclusive")]
impl IReceiptPrintJobVtbl {
    pub const fn new<Impl: IReceiptPrintJobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReceiptPrintJobVtbl {
        unsafe extern "system" fn MarkFeed<Impl: IReceiptPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: PosPrinterMarkFeedKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).MarkFeed(kind).into()
        }
        unsafe extern "system" fn CutPaper<Impl: IReceiptPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, percentage: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CutPaper(percentage).into()
        }
        unsafe extern "system" fn CutPaperDefault<Impl: IReceiptPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CutPaperDefault().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReceiptPrintJob>, base.5, MarkFeed::<Impl, OFFSET>, CutPaper::<Impl, OFFSET>, CutPaperDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrintJob2Impl: Sized {
    fn StampPaper(&self) -> ::windows::core::Result<()>;
    fn Print(&self, data: &::windows::core::HSTRING, printoptions: &::core::option::Option<PosPrinterPrintOptions>) -> ::windows::core::Result<()>;
    fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()>;
    fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReceiptPrintJob2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptPrintJob2";
}
#[cfg(feature = "implement_exclusive")]
impl IReceiptPrintJob2Vtbl {
    pub const fn new<Impl: IReceiptPrintJob2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReceiptPrintJob2Vtbl {
        unsafe extern "system" fn StampPaper<Impl: IReceiptPrintJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StampPaper().into()
        }
        unsafe extern "system" fn Print<Impl: IReceiptPrintJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Print(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&printoptions as *const <PosPrinterPrintOptions as ::windows::core::Abi>::Abi as *const <PosPrinterPrintOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FeedPaperByLine<Impl: IReceiptPrintJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FeedPaperByLine(linecount).into()
        }
        unsafe extern "system" fn FeedPaperByMapModeUnit<Impl: IReceiptPrintJob2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FeedPaperByMapModeUnit(distance).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReceiptPrintJob2>, base.5, StampPaper::<Impl, OFFSET>, Print::<Impl, OFFSET>, FeedPaperByLine::<Impl, OFFSET>, FeedPaperByMapModeUnit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrinterCapabilitiesImpl: Sized {
    fn CanCutPaper(&self) -> ::windows::core::Result<bool>;
    fn IsStampSupported(&self) -> ::windows::core::Result<bool>;
    fn MarkFeedCapabilities(&self) -> ::windows::core::Result<PosPrinterMarkFeedCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReceiptPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptPrinterCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IReceiptPrinterCapabilitiesVtbl {
    pub const fn new<Impl: IReceiptPrinterCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReceiptPrinterCapabilitiesVtbl {
        unsafe extern "system" fn CanCutPaper<Impl: IReceiptPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanCutPaper() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStampSupported<Impl: IReceiptPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStampSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkFeedCapabilities<Impl: IReceiptPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterMarkFeedCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MarkFeedCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReceiptPrinterCapabilities>, base.5, CanCutPaper::<Impl, OFFSET>, IsStampSupported::<Impl, OFFSET>, MarkFeedCapabilities::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrinterCapabilities2Impl: Sized {
    fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReceiptPrinterCapabilities2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptPrinterCapabilities2";
}
#[cfg(feature = "implement_exclusive")]
impl IReceiptPrinterCapabilities2Vtbl {
    pub const fn new<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReceiptPrinterCapabilities2Vtbl {
        unsafe extern "system" fn IsReverseVideoSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReverseVideoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStrikethroughSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStrikethroughSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSuperscriptSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSuperscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscriptSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSubscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByLineSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByLineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByMapModeUnitSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByMapModeUnitSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReceiptPrinterCapabilities2>, base.5, IsReverseVideoSupported::<Impl, OFFSET>, IsStrikethroughSupported::<Impl, OFFSET>, IsSuperscriptSupported::<Impl, OFFSET>, IsSubscriptSupported::<Impl, OFFSET>, IsReversePaperFeedByLineSupported::<Impl, OFFSET>, IsReversePaperFeedByMapModeUnitSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlipPrintJobImpl: Sized {
    fn Print(&self, data: &::windows::core::HSTRING, printoptions: &::core::option::Option<PosPrinterPrintOptions>) -> ::windows::core::Result<()>;
    fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()>;
    fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlipPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.ISlipPrintJob";
}
#[cfg(feature = "implement_exclusive")]
impl ISlipPrintJobVtbl {
    pub const fn new<Impl: ISlipPrintJobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISlipPrintJobVtbl {
        unsafe extern "system" fn Print<Impl: ISlipPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Print(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&printoptions as *const <PosPrinterPrintOptions as ::windows::core::Abi>::Abi as *const <PosPrinterPrintOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FeedPaperByLine<Impl: ISlipPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FeedPaperByLine(linecount).into()
        }
        unsafe extern "system" fn FeedPaperByMapModeUnit<Impl: ISlipPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FeedPaperByMapModeUnit(distance).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISlipPrintJob>, base.5, Print::<Impl, OFFSET>, FeedPaperByLine::<Impl, OFFSET>, FeedPaperByMapModeUnit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlipPrinterCapabilitiesImpl: Sized {
    fn IsFullLengthSupported(&self) -> ::windows::core::Result<bool>;
    fn IsBothSidesPrintingSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlipPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ISlipPrinterCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl ISlipPrinterCapabilitiesVtbl {
    pub const fn new<Impl: ISlipPrinterCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISlipPrinterCapabilitiesVtbl {
        unsafe extern "system" fn IsFullLengthSupported<Impl: ISlipPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFullLengthSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBothSidesPrintingSupported<Impl: ISlipPrinterCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBothSidesPrintingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISlipPrinterCapabilities>, base.5, IsFullLengthSupported::<Impl, OFFSET>, IsBothSidesPrintingSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlipPrinterCapabilities2Impl: Sized {
    fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlipPrinterCapabilities2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.ISlipPrinterCapabilities2";
}
#[cfg(feature = "implement_exclusive")]
impl ISlipPrinterCapabilities2Vtbl {
    pub const fn new<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISlipPrinterCapabilities2Vtbl {
        unsafe extern "system" fn IsReverseVideoSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReverseVideoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStrikethroughSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStrikethroughSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSuperscriptSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSuperscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscriptSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSubscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByLineSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByLineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByMapModeUnitSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByMapModeUnitSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISlipPrinterCapabilities2>, base.5, IsReverseVideoSupported::<Impl, OFFSET>, IsStrikethroughSupported::<Impl, OFFSET>, IsSuperscriptSupported::<Impl, OFFSET>, IsSubscriptSupported::<Impl, OFFSET>, IsReversePaperFeedByLineSupported::<Impl, OFFSET>, IsReversePaperFeedByMapModeUnitSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnifiedPosErrorDataImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Severity(&self) -> ::windows::core::Result<UnifiedPosErrorSeverity>;
    fn Reason(&self) -> ::windows::core::Result<UnifiedPosErrorReason>;
    fn ExtendedReason(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnifiedPosErrorData {
    const NAME: &'static str = "Windows.Devices.PointOfService.IUnifiedPosErrorData";
}
#[cfg(feature = "implement_exclusive")]
impl IUnifiedPosErrorDataVtbl {
    pub const fn new<Impl: IUnifiedPosErrorDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUnifiedPosErrorDataVtbl {
        unsafe extern "system" fn Message<Impl: IUnifiedPosErrorDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Severity<Impl: IUnifiedPosErrorDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosErrorSeverity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Severity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Impl: IUnifiedPosErrorDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosErrorReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedReason<Impl: IUnifiedPosErrorDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUnifiedPosErrorData>, base.5, Message::<Impl, OFFSET>, Severity::<Impl, OFFSET>, Reason::<Impl, OFFSET>, ExtendedReason::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnifiedPosErrorDataFactoryImpl: Sized {
    fn CreateInstance(&self, message: &::windows::core::HSTRING, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32) -> ::windows::core::Result<UnifiedPosErrorData>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnifiedPosErrorDataFactory {
    const NAME: &'static str = "Windows.Devices.PointOfService.IUnifiedPosErrorDataFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUnifiedPosErrorDataFactoryVtbl {
    pub const fn new<Impl: IUnifiedPosErrorDataFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUnifiedPosErrorDataFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IUnifiedPosErrorDataFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), severity, reason, extendedreason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUnifiedPosErrorDataFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
