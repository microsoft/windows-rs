#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBarcodeScannerImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&mut self) -> ::windows::core::Result<BarcodeScannerCapabilities>;
    fn ClaimScannerAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedBarcodeScanner>>;
    fn CheckHealthAsync(&mut self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetSupportedSymbologiesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>;
    fn IsSymbologySupportedAsync(&mut self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RetrieveStatisticsAsync(&mut self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn GetSupportedProfiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn IsProfileSupported(&mut self, profile: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn StatusUpdated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScanner";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBarcodeScannerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerVtbl {
        unsafe extern "system" fn DeviceId<Impl: IBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: IBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClaimScannerAsync<Impl: IBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClaimScannerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHealthAsync<Impl: IBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckHealthAsync(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedSymbologiesAsync<Impl: IBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedSymbologiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSymbologySupportedAsync<Impl: IBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, barcodesymbology: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSymbologySupportedAsync(barcodesymbology) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveStatisticsAsync<Impl: IBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedProfiles<Impl: IBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsProfileSupported<Impl: IBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsProfileSupported(&*(&profile as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUpdated<Impl: IBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUpdated<Impl: IBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScanner, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
            ClaimScannerAsync: ClaimScannerAsync::<Impl, IMPL_OFFSET>,
            CheckHealthAsync: CheckHealthAsync::<Impl, IMPL_OFFSET>,
            GetSupportedSymbologiesAsync: GetSupportedSymbologiesAsync::<Impl, IMPL_OFFSET>,
            IsSymbologySupportedAsync: IsSymbologySupportedAsync::<Impl, IMPL_OFFSET>,
            RetrieveStatisticsAsync: RetrieveStatisticsAsync::<Impl, IMPL_OFFSET>,
            GetSupportedProfiles: GetSupportedProfiles::<Impl, IMPL_OFFSET>,
            IsProfileSupported: IsProfileSupported::<Impl, IMPL_OFFSET>,
            StatusUpdated: StatusUpdated::<Impl, IMPL_OFFSET>,
            RemoveStatusUpdated: RemoveStatusUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScanner as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScanner2Impl: Sized {
    fn VideoDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScanner2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScanner2";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScanner2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScanner2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScanner2Vtbl {
        unsafe extern "system" fn VideoDeviceId<Impl: IBarcodeScanner2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScanner2, BASE_OFFSET>(), VideoDeviceId: VideoDeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScanner2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerCapabilitiesImpl: Sized {
    fn PowerReportingType(&mut self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsImagePreviewSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerCapabilitiesVtbl {
        unsafe extern "system" fn PowerReportingType<Impl: IBarcodeScannerCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsReportingSupported<Impl: IBarcodeScannerCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatisticsReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsUpdatingSupported<Impl: IBarcodeScannerCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatisticsUpdatingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsImagePreviewSupported<Impl: IBarcodeScannerCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsImagePreviewSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerCapabilities, BASE_OFFSET>(),
            PowerReportingType: PowerReportingType::<Impl, IMPL_OFFSET>,
            IsStatisticsReportingSupported: IsStatisticsReportingSupported::<Impl, IMPL_OFFSET>,
            IsStatisticsUpdatingSupported: IsStatisticsUpdatingSupported::<Impl, IMPL_OFFSET>,
            IsImagePreviewSupported: IsImagePreviewSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerCapabilities1Impl: Sized {
    fn IsSoftwareTriggerSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerCapabilities1 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerCapabilities1";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerCapabilities1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerCapabilities1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerCapabilities1Vtbl {
        unsafe extern "system" fn IsSoftwareTriggerSupported<Impl: IBarcodeScannerCapabilities1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSoftwareTriggerSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerCapabilities1, BASE_OFFSET>(),
            IsSoftwareTriggerSupported: IsSoftwareTriggerSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerCapabilities1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerCapabilities2Impl: Sized {
    fn IsVideoPreviewSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerCapabilities2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerCapabilities2";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerCapabilities2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerCapabilities2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerCapabilities2Vtbl {
        unsafe extern "system" fn IsVideoPreviewSupported<Impl: IBarcodeScannerCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVideoPreviewSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerCapabilities2, BASE_OFFSET>(),
            IsVideoPreviewSupported: IsVideoPreviewSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerCapabilities2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerDataReceivedEventArgsImpl: Sized {
    fn Report(&mut self) -> ::windows::core::Result<BarcodeScannerReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerDataReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerDataReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerDataReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerDataReceivedEventArgsVtbl {
        unsafe extern "system" fn Report<Impl: IBarcodeScannerDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Report() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerDataReceivedEventArgs, BASE_OFFSET>(),
            Report: Report::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerDataReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerErrorOccurredEventArgsImpl: Sized {
    fn PartialInputData(&mut self) -> ::windows::core::Result<BarcodeScannerReport>;
    fn IsRetriable(&mut self) -> ::windows::core::Result<bool>;
    fn ErrorData(&mut self) -> ::windows::core::Result<UnifiedPosErrorData>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerErrorOccurredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerErrorOccurredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerErrorOccurredEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerErrorOccurredEventArgsVtbl {
        unsafe extern "system" fn PartialInputData<Impl: IBarcodeScannerErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PartialInputData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRetriable<Impl: IBarcodeScannerErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRetriable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorData<Impl: IBarcodeScannerErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerErrorOccurredEventArgs, BASE_OFFSET>(),
            PartialInputData: PartialInputData::<Impl, IMPL_OFFSET>,
            IsRetriable: IsRetriable::<Impl, IMPL_OFFSET>,
            ErrorData: ErrorData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerErrorOccurredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBarcodeScannerImagePreviewReceivedEventArgsImpl: Sized {
    fn Preview(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerImagePreviewReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerImagePreviewReceivedEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBarcodeScannerImagePreviewReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerImagePreviewReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerImagePreviewReceivedEventArgsVtbl {
        unsafe extern "system" fn Preview<Impl: IBarcodeScannerImagePreviewReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Preview() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerImagePreviewReceivedEventArgs, BASE_OFFSET>(),
            Preview: Preview::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerImagePreviewReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBarcodeScannerReportImpl: Sized {
    fn ScanDataType(&mut self) -> ::windows::core::Result<u32>;
    fn ScanData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ScanDataLabel(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerReport";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBarcodeScannerReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerReportVtbl {
        unsafe extern "system" fn ScanDataType<Impl: IBarcodeScannerReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanDataType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanData<Impl: IBarcodeScannerReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanDataLabel<Impl: IBarcodeScannerReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanDataLabel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerReport, BASE_OFFSET>(),
            ScanDataType: ScanDataType::<Impl, IMPL_OFFSET>,
            ScanData: ScanData::<Impl, IMPL_OFFSET>,
            ScanDataLabel: ScanDataLabel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBarcodeScannerReportFactoryImpl: Sized {
    fn CreateInstance(&mut self, scandatatype: u32, scandata: &::core::option::Option<super::super::Storage::Streams::IBuffer>, scandatalabel: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<BarcodeScannerReport>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerReportFactory {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerReportFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBarcodeScannerReportFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerReportFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerReportFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBarcodeScannerReportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scandatatype: u32, scandata: ::windows::core::RawPtr, scandatalabel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(scandatatype, &*(&scandata as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&scandatalabel as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerReportFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerReportFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerStaticsImpl: Sized {
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerStaticsVtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: IBarcodeScannerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IBarcodeScannerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IBarcodeScannerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerStatics, BASE_OFFSET>(),
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&mut self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: IBarcodeScannerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerStatics2, BASE_OFFSET>(),
            GetDeviceSelectorWithConnectionTypes: GetDeviceSelectorWithConnectionTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStatusUpdatedEventArgsImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<BarcodeScannerStatus>;
    fn ExtendedStatus(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeScannerStatusUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerStatusUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerStatusUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerStatusUpdatedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IBarcodeScannerStatusUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BarcodeScannerStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IBarcodeScannerStatusUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeScannerStatusUpdatedEventArgs, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedStatus: ExtendedStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerStatusUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeSymbologiesStaticsImpl: Sized {
    fn Unknown(&mut self) -> ::windows::core::Result<u32>;
    fn Ean8(&mut self) -> ::windows::core::Result<u32>;
    fn Ean8Add2(&mut self) -> ::windows::core::Result<u32>;
    fn Ean8Add5(&mut self) -> ::windows::core::Result<u32>;
    fn Eanv(&mut self) -> ::windows::core::Result<u32>;
    fn EanvAdd2(&mut self) -> ::windows::core::Result<u32>;
    fn EanvAdd5(&mut self) -> ::windows::core::Result<u32>;
    fn Ean13(&mut self) -> ::windows::core::Result<u32>;
    fn Ean13Add2(&mut self) -> ::windows::core::Result<u32>;
    fn Ean13Add5(&mut self) -> ::windows::core::Result<u32>;
    fn Isbn(&mut self) -> ::windows::core::Result<u32>;
    fn IsbnAdd5(&mut self) -> ::windows::core::Result<u32>;
    fn Ismn(&mut self) -> ::windows::core::Result<u32>;
    fn IsmnAdd2(&mut self) -> ::windows::core::Result<u32>;
    fn IsmnAdd5(&mut self) -> ::windows::core::Result<u32>;
    fn Issn(&mut self) -> ::windows::core::Result<u32>;
    fn IssnAdd2(&mut self) -> ::windows::core::Result<u32>;
    fn IssnAdd5(&mut self) -> ::windows::core::Result<u32>;
    fn Ean99(&mut self) -> ::windows::core::Result<u32>;
    fn Ean99Add2(&mut self) -> ::windows::core::Result<u32>;
    fn Ean99Add5(&mut self) -> ::windows::core::Result<u32>;
    fn Upca(&mut self) -> ::windows::core::Result<u32>;
    fn UpcaAdd2(&mut self) -> ::windows::core::Result<u32>;
    fn UpcaAdd5(&mut self) -> ::windows::core::Result<u32>;
    fn Upce(&mut self) -> ::windows::core::Result<u32>;
    fn UpceAdd2(&mut self) -> ::windows::core::Result<u32>;
    fn UpceAdd5(&mut self) -> ::windows::core::Result<u32>;
    fn UpcCoupon(&mut self) -> ::windows::core::Result<u32>;
    fn TfStd(&mut self) -> ::windows::core::Result<u32>;
    fn TfDis(&mut self) -> ::windows::core::Result<u32>;
    fn TfInt(&mut self) -> ::windows::core::Result<u32>;
    fn TfInd(&mut self) -> ::windows::core::Result<u32>;
    fn TfMat(&mut self) -> ::windows::core::Result<u32>;
    fn TfIata(&mut self) -> ::windows::core::Result<u32>;
    fn Gs1DatabarType1(&mut self) -> ::windows::core::Result<u32>;
    fn Gs1DatabarType2(&mut self) -> ::windows::core::Result<u32>;
    fn Gs1DatabarType3(&mut self) -> ::windows::core::Result<u32>;
    fn Code39(&mut self) -> ::windows::core::Result<u32>;
    fn Code39Ex(&mut self) -> ::windows::core::Result<u32>;
    fn Trioptic39(&mut self) -> ::windows::core::Result<u32>;
    fn Code32(&mut self) -> ::windows::core::Result<u32>;
    fn Pzn(&mut self) -> ::windows::core::Result<u32>;
    fn Code93(&mut self) -> ::windows::core::Result<u32>;
    fn Code93Ex(&mut self) -> ::windows::core::Result<u32>;
    fn Code128(&mut self) -> ::windows::core::Result<u32>;
    fn Gs1128(&mut self) -> ::windows::core::Result<u32>;
    fn Gs1128Coupon(&mut self) -> ::windows::core::Result<u32>;
    fn UccEan128(&mut self) -> ::windows::core::Result<u32>;
    fn Sisac(&mut self) -> ::windows::core::Result<u32>;
    fn Isbt(&mut self) -> ::windows::core::Result<u32>;
    fn Codabar(&mut self) -> ::windows::core::Result<u32>;
    fn Code11(&mut self) -> ::windows::core::Result<u32>;
    fn Msi(&mut self) -> ::windows::core::Result<u32>;
    fn Plessey(&mut self) -> ::windows::core::Result<u32>;
    fn Telepen(&mut self) -> ::windows::core::Result<u32>;
    fn Code16k(&mut self) -> ::windows::core::Result<u32>;
    fn CodablockA(&mut self) -> ::windows::core::Result<u32>;
    fn CodablockF(&mut self) -> ::windows::core::Result<u32>;
    fn Codablock128(&mut self) -> ::windows::core::Result<u32>;
    fn Code49(&mut self) -> ::windows::core::Result<u32>;
    fn Aztec(&mut self) -> ::windows::core::Result<u32>;
    fn DataCode(&mut self) -> ::windows::core::Result<u32>;
    fn DataMatrix(&mut self) -> ::windows::core::Result<u32>;
    fn HanXin(&mut self) -> ::windows::core::Result<u32>;
    fn Maxicode(&mut self) -> ::windows::core::Result<u32>;
    fn MicroPdf417(&mut self) -> ::windows::core::Result<u32>;
    fn MicroQr(&mut self) -> ::windows::core::Result<u32>;
    fn Pdf417(&mut self) -> ::windows::core::Result<u32>;
    fn Qr(&mut self) -> ::windows::core::Result<u32>;
    fn MsTag(&mut self) -> ::windows::core::Result<u32>;
    fn Ccab(&mut self) -> ::windows::core::Result<u32>;
    fn Ccc(&mut self) -> ::windows::core::Result<u32>;
    fn Tlc39(&mut self) -> ::windows::core::Result<u32>;
    fn AusPost(&mut self) -> ::windows::core::Result<u32>;
    fn CanPost(&mut self) -> ::windows::core::Result<u32>;
    fn ChinaPost(&mut self) -> ::windows::core::Result<u32>;
    fn DutchKix(&mut self) -> ::windows::core::Result<u32>;
    fn InfoMail(&mut self) -> ::windows::core::Result<u32>;
    fn ItalianPost25(&mut self) -> ::windows::core::Result<u32>;
    fn ItalianPost39(&mut self) -> ::windows::core::Result<u32>;
    fn JapanPost(&mut self) -> ::windows::core::Result<u32>;
    fn KoreanPost(&mut self) -> ::windows::core::Result<u32>;
    fn SwedenPost(&mut self) -> ::windows::core::Result<u32>;
    fn UkPost(&mut self) -> ::windows::core::Result<u32>;
    fn UsIntelligent(&mut self) -> ::windows::core::Result<u32>;
    fn UsIntelligentPkg(&mut self) -> ::windows::core::Result<u32>;
    fn UsPlanet(&mut self) -> ::windows::core::Result<u32>;
    fn UsPostNet(&mut self) -> ::windows::core::Result<u32>;
    fn Us4StateFics(&mut self) -> ::windows::core::Result<u32>;
    fn OcrA(&mut self) -> ::windows::core::Result<u32>;
    fn OcrB(&mut self) -> ::windows::core::Result<u32>;
    fn Micr(&mut self) -> ::windows::core::Result<u32>;
    fn ExtendedBase(&mut self) -> ::windows::core::Result<u32>;
    fn GetName(&mut self, scandatatype: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeSymbologiesStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeSymbologiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeSymbologiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeSymbologiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeSymbologiesStaticsVtbl {
        unsafe extern "system" fn Unknown<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unknown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean8<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ean8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean8Add2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ean8Add2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean8Add5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ean8Add5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Eanv<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Eanv() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EanvAdd2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EanvAdd2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EanvAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EanvAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean13<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ean13() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean13Add2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ean13Add2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean13Add5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ean13Add5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Isbn<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Isbn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsbnAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsbnAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ismn<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ismn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsmnAdd2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsmnAdd2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsmnAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsmnAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Issn<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Issn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssnAdd2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IssnAdd2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssnAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IssnAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean99<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ean99() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean99Add2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ean99Add2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ean99Add5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ean99Add5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Upca<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Upca() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpcaAdd2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpcaAdd2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpcaAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpcaAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Upce<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Upce() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpceAdd2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpceAdd2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpceAdd5<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpceAdd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpcCoupon<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpcCoupon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfStd<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TfStd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfDis<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TfDis() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfInt<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TfInt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfInd<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TfInd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfMat<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TfMat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TfIata<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TfIata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gs1DatabarType1<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gs1DatabarType1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gs1DatabarType2<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gs1DatabarType2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gs1DatabarType3<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gs1DatabarType3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code39<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code39() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code39Ex<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code39Ex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trioptic39<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trioptic39() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code32<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pzn<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pzn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code93<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code93() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code93Ex<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code93Ex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code128<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code128() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gs1128<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gs1128() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gs1128Coupon<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gs1128Coupon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UccEan128<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UccEan128() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sisac<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sisac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Isbt<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Isbt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Codabar<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Codabar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code11<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code11() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Msi<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Msi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Plessey<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Plessey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Telepen<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Telepen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code16k<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code16k() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CodablockA<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CodablockA() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CodablockF<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CodablockF() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Codablock128<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Codablock128() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code49<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code49() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Aztec<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Aztec() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataCode<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataMatrix<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HanXin<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HanXin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maxicode<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Maxicode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicroPdf417<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MicroPdf417() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicroQr<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MicroQr() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pdf417<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pdf417() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qr<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Qr() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MsTag<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MsTag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ccab<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ccab() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ccc<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ccc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tlc39<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tlc39() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AusPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AusPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChinaPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChinaPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DutchKix<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DutchKix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoMail<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InfoMail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItalianPost25<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItalianPost25() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItalianPost39<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItalianPost39() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JapanPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JapanPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KoreanPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KoreanPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwedenPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SwedenPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UkPost<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UkPost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsIntelligent<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsIntelligent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsIntelligentPkg<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsIntelligentPkg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsPlanet<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsPlanet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsPostNet<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsPostNet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Us4StateFics<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Us4StateFics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OcrA<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OcrA() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OcrB<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OcrB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Micr<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Micr() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedBase<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedBase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IBarcodeSymbologiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scandatatype: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(scandatatype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeSymbologiesStatics, BASE_OFFSET>(),
            Unknown: Unknown::<Impl, IMPL_OFFSET>,
            Ean8: Ean8::<Impl, IMPL_OFFSET>,
            Ean8Add2: Ean8Add2::<Impl, IMPL_OFFSET>,
            Ean8Add5: Ean8Add5::<Impl, IMPL_OFFSET>,
            Eanv: Eanv::<Impl, IMPL_OFFSET>,
            EanvAdd2: EanvAdd2::<Impl, IMPL_OFFSET>,
            EanvAdd5: EanvAdd5::<Impl, IMPL_OFFSET>,
            Ean13: Ean13::<Impl, IMPL_OFFSET>,
            Ean13Add2: Ean13Add2::<Impl, IMPL_OFFSET>,
            Ean13Add5: Ean13Add5::<Impl, IMPL_OFFSET>,
            Isbn: Isbn::<Impl, IMPL_OFFSET>,
            IsbnAdd5: IsbnAdd5::<Impl, IMPL_OFFSET>,
            Ismn: Ismn::<Impl, IMPL_OFFSET>,
            IsmnAdd2: IsmnAdd2::<Impl, IMPL_OFFSET>,
            IsmnAdd5: IsmnAdd5::<Impl, IMPL_OFFSET>,
            Issn: Issn::<Impl, IMPL_OFFSET>,
            IssnAdd2: IssnAdd2::<Impl, IMPL_OFFSET>,
            IssnAdd5: IssnAdd5::<Impl, IMPL_OFFSET>,
            Ean99: Ean99::<Impl, IMPL_OFFSET>,
            Ean99Add2: Ean99Add2::<Impl, IMPL_OFFSET>,
            Ean99Add5: Ean99Add5::<Impl, IMPL_OFFSET>,
            Upca: Upca::<Impl, IMPL_OFFSET>,
            UpcaAdd2: UpcaAdd2::<Impl, IMPL_OFFSET>,
            UpcaAdd5: UpcaAdd5::<Impl, IMPL_OFFSET>,
            Upce: Upce::<Impl, IMPL_OFFSET>,
            UpceAdd2: UpceAdd2::<Impl, IMPL_OFFSET>,
            UpceAdd5: UpceAdd5::<Impl, IMPL_OFFSET>,
            UpcCoupon: UpcCoupon::<Impl, IMPL_OFFSET>,
            TfStd: TfStd::<Impl, IMPL_OFFSET>,
            TfDis: TfDis::<Impl, IMPL_OFFSET>,
            TfInt: TfInt::<Impl, IMPL_OFFSET>,
            TfInd: TfInd::<Impl, IMPL_OFFSET>,
            TfMat: TfMat::<Impl, IMPL_OFFSET>,
            TfIata: TfIata::<Impl, IMPL_OFFSET>,
            Gs1DatabarType1: Gs1DatabarType1::<Impl, IMPL_OFFSET>,
            Gs1DatabarType2: Gs1DatabarType2::<Impl, IMPL_OFFSET>,
            Gs1DatabarType3: Gs1DatabarType3::<Impl, IMPL_OFFSET>,
            Code39: Code39::<Impl, IMPL_OFFSET>,
            Code39Ex: Code39Ex::<Impl, IMPL_OFFSET>,
            Trioptic39: Trioptic39::<Impl, IMPL_OFFSET>,
            Code32: Code32::<Impl, IMPL_OFFSET>,
            Pzn: Pzn::<Impl, IMPL_OFFSET>,
            Code93: Code93::<Impl, IMPL_OFFSET>,
            Code93Ex: Code93Ex::<Impl, IMPL_OFFSET>,
            Code128: Code128::<Impl, IMPL_OFFSET>,
            Gs1128: Gs1128::<Impl, IMPL_OFFSET>,
            Gs1128Coupon: Gs1128Coupon::<Impl, IMPL_OFFSET>,
            UccEan128: UccEan128::<Impl, IMPL_OFFSET>,
            Sisac: Sisac::<Impl, IMPL_OFFSET>,
            Isbt: Isbt::<Impl, IMPL_OFFSET>,
            Codabar: Codabar::<Impl, IMPL_OFFSET>,
            Code11: Code11::<Impl, IMPL_OFFSET>,
            Msi: Msi::<Impl, IMPL_OFFSET>,
            Plessey: Plessey::<Impl, IMPL_OFFSET>,
            Telepen: Telepen::<Impl, IMPL_OFFSET>,
            Code16k: Code16k::<Impl, IMPL_OFFSET>,
            CodablockA: CodablockA::<Impl, IMPL_OFFSET>,
            CodablockF: CodablockF::<Impl, IMPL_OFFSET>,
            Codablock128: Codablock128::<Impl, IMPL_OFFSET>,
            Code49: Code49::<Impl, IMPL_OFFSET>,
            Aztec: Aztec::<Impl, IMPL_OFFSET>,
            DataCode: DataCode::<Impl, IMPL_OFFSET>,
            DataMatrix: DataMatrix::<Impl, IMPL_OFFSET>,
            HanXin: HanXin::<Impl, IMPL_OFFSET>,
            Maxicode: Maxicode::<Impl, IMPL_OFFSET>,
            MicroPdf417: MicroPdf417::<Impl, IMPL_OFFSET>,
            MicroQr: MicroQr::<Impl, IMPL_OFFSET>,
            Pdf417: Pdf417::<Impl, IMPL_OFFSET>,
            Qr: Qr::<Impl, IMPL_OFFSET>,
            MsTag: MsTag::<Impl, IMPL_OFFSET>,
            Ccab: Ccab::<Impl, IMPL_OFFSET>,
            Ccc: Ccc::<Impl, IMPL_OFFSET>,
            Tlc39: Tlc39::<Impl, IMPL_OFFSET>,
            AusPost: AusPost::<Impl, IMPL_OFFSET>,
            CanPost: CanPost::<Impl, IMPL_OFFSET>,
            ChinaPost: ChinaPost::<Impl, IMPL_OFFSET>,
            DutchKix: DutchKix::<Impl, IMPL_OFFSET>,
            InfoMail: InfoMail::<Impl, IMPL_OFFSET>,
            ItalianPost25: ItalianPost25::<Impl, IMPL_OFFSET>,
            ItalianPost39: ItalianPost39::<Impl, IMPL_OFFSET>,
            JapanPost: JapanPost::<Impl, IMPL_OFFSET>,
            KoreanPost: KoreanPost::<Impl, IMPL_OFFSET>,
            SwedenPost: SwedenPost::<Impl, IMPL_OFFSET>,
            UkPost: UkPost::<Impl, IMPL_OFFSET>,
            UsIntelligent: UsIntelligent::<Impl, IMPL_OFFSET>,
            UsIntelligentPkg: UsIntelligentPkg::<Impl, IMPL_OFFSET>,
            UsPlanet: UsPlanet::<Impl, IMPL_OFFSET>,
            UsPostNet: UsPostNet::<Impl, IMPL_OFFSET>,
            Us4StateFics: Us4StateFics::<Impl, IMPL_OFFSET>,
            OcrA: OcrA::<Impl, IMPL_OFFSET>,
            OcrB: OcrB::<Impl, IMPL_OFFSET>,
            Micr: Micr::<Impl, IMPL_OFFSET>,
            ExtendedBase: ExtendedBase::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeSymbologiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeSymbologiesStatics2Impl: Sized {
    fn Gs1DWCode(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeSymbologiesStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeSymbologiesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeSymbologiesStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeSymbologiesStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeSymbologiesStatics2Vtbl {
        unsafe extern "system" fn Gs1DWCode<Impl: IBarcodeSymbologiesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gs1DWCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeSymbologiesStatics2, BASE_OFFSET>(), Gs1DWCode: Gs1DWCode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeSymbologiesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeSymbologyAttributesImpl: Sized {
    fn IsCheckDigitValidationEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCheckDigitValidationEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsCheckDigitValidationSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsCheckDigitTransmissionEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCheckDigitTransmissionEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsCheckDigitTransmissionSupported(&mut self) -> ::windows::core::Result<bool>;
    fn DecodeLength1(&mut self) -> ::windows::core::Result<u32>;
    fn SetDecodeLength1(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn DecodeLength2(&mut self) -> ::windows::core::Result<u32>;
    fn SetDecodeLength2(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn DecodeLengthKind(&mut self) -> ::windows::core::Result<BarcodeSymbologyDecodeLengthKind>;
    fn SetDecodeLengthKind(&mut self, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::core::Result<()>;
    fn IsDecodeLengthSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeSymbologyAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.IBarcodeSymbologyAttributes";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeSymbologyAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeSymbologyAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeSymbologyAttributesVtbl {
        unsafe extern "system" fn IsCheckDigitValidationEnabled<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCheckDigitValidationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCheckDigitValidationEnabled<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCheckDigitValidationEnabled(value).into()
        }
        unsafe extern "system" fn IsCheckDigitValidationSupported<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCheckDigitValidationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCheckDigitTransmissionEnabled<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCheckDigitTransmissionEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCheckDigitTransmissionEnabled<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCheckDigitTransmissionEnabled(value).into()
        }
        unsafe extern "system" fn IsCheckDigitTransmissionSupported<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCheckDigitTransmissionSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecodeLength1<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodeLength1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecodeLength1<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDecodeLength1(value).into()
        }
        unsafe extern "system" fn DecodeLength2<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodeLength2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecodeLength2<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDecodeLength2(value).into()
        }
        unsafe extern "system" fn DecodeLengthKind<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BarcodeSymbologyDecodeLengthKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodeLengthKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecodeLengthKind<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDecodeLengthKind(value).into()
        }
        unsafe extern "system" fn IsDecodeLengthSupported<Impl: IBarcodeSymbologyAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDecodeLengthSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBarcodeSymbologyAttributes, BASE_OFFSET>(),
            IsCheckDigitValidationEnabled: IsCheckDigitValidationEnabled::<Impl, IMPL_OFFSET>,
            SetIsCheckDigitValidationEnabled: SetIsCheckDigitValidationEnabled::<Impl, IMPL_OFFSET>,
            IsCheckDigitValidationSupported: IsCheckDigitValidationSupported::<Impl, IMPL_OFFSET>,
            IsCheckDigitTransmissionEnabled: IsCheckDigitTransmissionEnabled::<Impl, IMPL_OFFSET>,
            SetIsCheckDigitTransmissionEnabled: SetIsCheckDigitTransmissionEnabled::<Impl, IMPL_OFFSET>,
            IsCheckDigitTransmissionSupported: IsCheckDigitTransmissionSupported::<Impl, IMPL_OFFSET>,
            DecodeLength1: DecodeLength1::<Impl, IMPL_OFFSET>,
            SetDecodeLength1: SetDecodeLength1::<Impl, IMPL_OFFSET>,
            DecodeLength2: DecodeLength2::<Impl, IMPL_OFFSET>,
            SetDecodeLength2: SetDecodeLength2::<Impl, IMPL_OFFSET>,
            DecodeLengthKind: DecodeLengthKind::<Impl, IMPL_OFFSET>,
            SetDecodeLengthKind: SetDecodeLengthKind::<Impl, IMPL_OFFSET>,
            IsDecodeLengthSupported: IsDecodeLengthSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeSymbologyAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICashDrawerImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&mut self) -> ::windows::core::Result<CashDrawerCapabilities>;
    fn Status(&mut self) -> ::windows::core::Result<CashDrawerStatus>;
    fn IsDrawerOpen(&mut self) -> ::windows::core::Result<bool>;
    fn DrawerEventSource(&mut self) -> ::windows::core::Result<CashDrawerEventSource>;
    fn ClaimDrawerAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedCashDrawer>>;
    fn CheckHealthAsync(&mut self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetStatisticsAsync(&mut self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn StatusUpdated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICashDrawerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICashDrawerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICashDrawerVtbl {
        unsafe extern "system" fn DeviceId<Impl: ICashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: ICashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: ICashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDrawerOpen<Impl: ICashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDrawerOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawerEventSource<Impl: ICashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawerEventSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClaimDrawerAsync<Impl: ICashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClaimDrawerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHealthAsync<Impl: ICashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckHealthAsync(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatisticsAsync<Impl: ICashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUpdated<Impl: ICashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUpdated<Impl: ICashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICashDrawer, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            IsDrawerOpen: IsDrawerOpen::<Impl, IMPL_OFFSET>,
            DrawerEventSource: DrawerEventSource::<Impl, IMPL_OFFSET>,
            ClaimDrawerAsync: ClaimDrawerAsync::<Impl, IMPL_OFFSET>,
            CheckHealthAsync: CheckHealthAsync::<Impl, IMPL_OFFSET>,
            GetStatisticsAsync: GetStatisticsAsync::<Impl, IMPL_OFFSET>,
            StatusUpdated: StatusUpdated::<Impl, IMPL_OFFSET>,
            RemoveStatusUpdated: RemoveStatusUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICashDrawer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerCapabilitiesImpl: Sized {
    fn PowerReportingType(&mut self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsStatusReportingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsStatusMultiDrawerDetectSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsDrawerOpenSensorAvailable(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICashDrawerCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICashDrawerCapabilitiesVtbl {
        unsafe extern "system" fn PowerReportingType<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsReportingSupported<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatisticsReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsUpdatingSupported<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatisticsUpdatingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatusReportingSupported<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatusReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatusMultiDrawerDetectSupported<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatusMultiDrawerDetectSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDrawerOpenSensorAvailable<Impl: ICashDrawerCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDrawerOpenSensorAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICashDrawerCapabilities, BASE_OFFSET>(),
            PowerReportingType: PowerReportingType::<Impl, IMPL_OFFSET>,
            IsStatisticsReportingSupported: IsStatisticsReportingSupported::<Impl, IMPL_OFFSET>,
            IsStatisticsUpdatingSupported: IsStatisticsUpdatingSupported::<Impl, IMPL_OFFSET>,
            IsStatusReportingSupported: IsStatusReportingSupported::<Impl, IMPL_OFFSET>,
            IsStatusMultiDrawerDetectSupported: IsStatusMultiDrawerDetectSupported::<Impl, IMPL_OFFSET>,
            IsDrawerOpenSensorAvailable: IsDrawerOpenSensorAvailable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICashDrawerCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICashDrawerCloseAlarmImpl: Sized {
    fn SetAlarmTimeout(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AlarmTimeout(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBeepFrequency(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn BeepFrequency(&mut self) -> ::windows::core::Result<u32>;
    fn SetBeepDuration(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BeepDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBeepDelay(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BeepDelay(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn AlarmTimeoutExpired(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAlarmTimeoutExpired(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICashDrawerCloseAlarm {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerCloseAlarm";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICashDrawerCloseAlarmVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICashDrawerCloseAlarmImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICashDrawerCloseAlarmVtbl {
        unsafe extern "system" fn SetAlarmTimeout<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlarmTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlarmTimeout<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlarmTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBeepFrequency<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBeepFrequency(value).into()
        }
        unsafe extern "system" fn BeepFrequency<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeepFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBeepDuration<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBeepDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BeepDuration<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeepDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBeepDelay<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBeepDelay(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BeepDelay<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeepDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlarmTimeoutExpired<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlarmTimeoutExpired(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAlarmTimeoutExpired<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAlarmTimeoutExpired(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartAsync<Impl: ICashDrawerCloseAlarmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICashDrawerCloseAlarm, BASE_OFFSET>(),
            SetAlarmTimeout: SetAlarmTimeout::<Impl, IMPL_OFFSET>,
            AlarmTimeout: AlarmTimeout::<Impl, IMPL_OFFSET>,
            SetBeepFrequency: SetBeepFrequency::<Impl, IMPL_OFFSET>,
            BeepFrequency: BeepFrequency::<Impl, IMPL_OFFSET>,
            SetBeepDuration: SetBeepDuration::<Impl, IMPL_OFFSET>,
            BeepDuration: BeepDuration::<Impl, IMPL_OFFSET>,
            SetBeepDelay: SetBeepDelay::<Impl, IMPL_OFFSET>,
            BeepDelay: BeepDelay::<Impl, IMPL_OFFSET>,
            AlarmTimeoutExpired: AlarmTimeoutExpired::<Impl, IMPL_OFFSET>,
            RemoveAlarmTimeoutExpired: RemoveAlarmTimeoutExpired::<Impl, IMPL_OFFSET>,
            StartAsync: StartAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICashDrawerCloseAlarm as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICashDrawerEventSourceImpl: Sized {
    fn DrawerClosed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDrawerClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DrawerOpened(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDrawerOpened(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICashDrawerEventSource {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerEventSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICashDrawerEventSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICashDrawerEventSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICashDrawerEventSourceVtbl {
        unsafe extern "system" fn DrawerClosed<Impl: ICashDrawerEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawerClosed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDrawerClosed<Impl: ICashDrawerEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDrawerClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DrawerOpened<Impl: ICashDrawerEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawerOpened(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDrawerOpened<Impl: ICashDrawerEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDrawerOpened(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICashDrawerEventSource, BASE_OFFSET>(),
            DrawerClosed: DrawerClosed::<Impl, IMPL_OFFSET>,
            RemoveDrawerClosed: RemoveDrawerClosed::<Impl, IMPL_OFFSET>,
            DrawerOpened: DrawerOpened::<Impl, IMPL_OFFSET>,
            RemoveDrawerOpened: RemoveDrawerOpened::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICashDrawerEventSource as ::windows::core::Interface>::IID
    }
}
pub trait ICashDrawerEventSourceEventArgsImpl: Sized {
    fn CashDrawer(&mut self) -> ::windows::core::Result<CashDrawer>;
}
impl ::windows::core::RuntimeName for ICashDrawerEventSourceEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerEventSourceEventArgs";
}
impl ICashDrawerEventSourceEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICashDrawerEventSourceEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICashDrawerEventSourceEventArgsVtbl {
        unsafe extern "system" fn CashDrawer<Impl: ICashDrawerEventSourceEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CashDrawer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICashDrawerEventSourceEventArgs, BASE_OFFSET>(),
            CashDrawer: CashDrawer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICashDrawerEventSourceEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICashDrawerStaticsImpl: Sized {
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICashDrawerStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICashDrawerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICashDrawerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICashDrawerStaticsVtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: ICashDrawerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: ICashDrawerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: ICashDrawerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICashDrawerStatics, BASE_OFFSET>(),
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICashDrawerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&mut self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawerStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICashDrawerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICashDrawerStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: ICashDrawerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICashDrawerStatics2, BASE_OFFSET>(),
            GetDeviceSelectorWithConnectionTypes: GetDeviceSelectorWithConnectionTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICashDrawerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerStatusImpl: Sized {
    fn StatusKind(&mut self) -> ::windows::core::Result<CashDrawerStatusKind>;
    fn ExtendedStatus(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawerStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerStatus";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICashDrawerStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICashDrawerStatusVtbl {
        unsafe extern "system" fn StatusKind<Impl: ICashDrawerStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CashDrawerStatusKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: ICashDrawerStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICashDrawerStatus, BASE_OFFSET>(),
            StatusKind: StatusKind::<Impl, IMPL_OFFSET>,
            ExtendedStatus: ExtendedStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICashDrawerStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerStatusUpdatedEventArgsImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<CashDrawerStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICashDrawerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerStatusUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICashDrawerStatusUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICashDrawerStatusUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICashDrawerStatusUpdatedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: ICashDrawerStatusUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICashDrawerStatusUpdatedEventArgs, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICashDrawerStatusUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IClaimedBarcodeScannerImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsDisabledOnDataReceived(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsDisabledOnDataReceived(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsDecodeDataEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsDecodeDataEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn EnableAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RetainDevice(&mut self) -> ::windows::core::Result<()>;
    fn SetActiveSymbologiesAsync(&mut self, symbologies: &::core::option::Option<super::super::Foundation::Collections::IIterable<u32>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResetStatisticsAsync(&mut self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateStatisticsAsync(&mut self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetActiveProfileAsync(&mut self, profile: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DataReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TriggerPressed(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTriggerPressed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TriggerReleased(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTriggerReleased(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReleaseDeviceRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImagePreviewReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveImagePreviewReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedBarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedBarcodeScanner";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IClaimedBarcodeScannerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedBarcodeScannerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedBarcodeScannerVtbl {
        unsafe extern "system" fn DeviceId<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDisabledOnDataReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDisabledOnDataReceived(value).into()
        }
        unsafe extern "system" fn IsDisabledOnDataReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisabledOnDataReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDecodeDataEnabled<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDecodeDataEnabled(value).into()
        }
        unsafe extern "system" fn IsDecodeDataEnabled<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDecodeDataEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainDevice<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetainDevice().into()
        }
        unsafe extern "system" fn SetActiveSymbologiesAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symbologies: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveSymbologiesAsync(&*(&symbologies as *const <super::super::Foundation::Collections::IIterable<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<u32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetStatisticsAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateStatisticsAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateStatisticsAsync(&*(&statistics as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveProfileAsync<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveProfileAsync(&*(&profile as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDataReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TriggerPressed<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerPressed(&*(&handler as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTriggerPressed<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTriggerPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TriggerReleased<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerReleased(&*(&handler as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTriggerReleased<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTriggerReleased(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReleaseDeviceRequested<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseDeviceRequested(&*(&handler as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ClaimedBarcodeScanner> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReleaseDeviceRequested<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReleaseDeviceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImagePreviewReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImagePreviewReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveImagePreviewReceived<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveImagePreviewReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorOccurred<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorOccurred(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveErrorOccurred<Impl: IClaimedBarcodeScannerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveErrorOccurred(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedBarcodeScanner, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsDisabledOnDataReceived: SetIsDisabledOnDataReceived::<Impl, IMPL_OFFSET>,
            IsDisabledOnDataReceived: IsDisabledOnDataReceived::<Impl, IMPL_OFFSET>,
            SetIsDecodeDataEnabled: SetIsDecodeDataEnabled::<Impl, IMPL_OFFSET>,
            IsDecodeDataEnabled: IsDecodeDataEnabled::<Impl, IMPL_OFFSET>,
            EnableAsync: EnableAsync::<Impl, IMPL_OFFSET>,
            DisableAsync: DisableAsync::<Impl, IMPL_OFFSET>,
            RetainDevice: RetainDevice::<Impl, IMPL_OFFSET>,
            SetActiveSymbologiesAsync: SetActiveSymbologiesAsync::<Impl, IMPL_OFFSET>,
            ResetStatisticsAsync: ResetStatisticsAsync::<Impl, IMPL_OFFSET>,
            UpdateStatisticsAsync: UpdateStatisticsAsync::<Impl, IMPL_OFFSET>,
            SetActiveProfileAsync: SetActiveProfileAsync::<Impl, IMPL_OFFSET>,
            DataReceived: DataReceived::<Impl, IMPL_OFFSET>,
            RemoveDataReceived: RemoveDataReceived::<Impl, IMPL_OFFSET>,
            TriggerPressed: TriggerPressed::<Impl, IMPL_OFFSET>,
            RemoveTriggerPressed: RemoveTriggerPressed::<Impl, IMPL_OFFSET>,
            TriggerReleased: TriggerReleased::<Impl, IMPL_OFFSET>,
            RemoveTriggerReleased: RemoveTriggerReleased::<Impl, IMPL_OFFSET>,
            ReleaseDeviceRequested: ReleaseDeviceRequested::<Impl, IMPL_OFFSET>,
            RemoveReleaseDeviceRequested: RemoveReleaseDeviceRequested::<Impl, IMPL_OFFSET>,
            ImagePreviewReceived: ImagePreviewReceived::<Impl, IMPL_OFFSET>,
            RemoveImagePreviewReceived: RemoveImagePreviewReceived::<Impl, IMPL_OFFSET>,
            ErrorOccurred: ErrorOccurred::<Impl, IMPL_OFFSET>,
            RemoveErrorOccurred: RemoveErrorOccurred::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedBarcodeScanner as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedBarcodeScanner1Impl: Sized {
    fn StartSoftwareTriggerAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopSoftwareTriggerAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedBarcodeScanner1 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedBarcodeScanner1";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedBarcodeScanner1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedBarcodeScanner1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedBarcodeScanner1Vtbl {
        unsafe extern "system" fn StartSoftwareTriggerAsync<Impl: IClaimedBarcodeScanner1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartSoftwareTriggerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopSoftwareTriggerAsync<Impl: IClaimedBarcodeScanner1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopSoftwareTriggerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedBarcodeScanner1, BASE_OFFSET>(),
            StartSoftwareTriggerAsync: StartSoftwareTriggerAsync::<Impl, IMPL_OFFSET>,
            StopSoftwareTriggerAsync: StopSoftwareTriggerAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedBarcodeScanner1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedBarcodeScanner2Impl: Sized {
    fn GetSymbologyAttributesAsync(&mut self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeSymbologyAttributes>>;
    fn SetSymbologyAttributesAsync(&mut self, barcodesymbology: u32, attributes: &::core::option::Option<BarcodeSymbologyAttributes>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedBarcodeScanner2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedBarcodeScanner2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedBarcodeScanner2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedBarcodeScanner2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedBarcodeScanner2Vtbl {
        unsafe extern "system" fn GetSymbologyAttributesAsync<Impl: IClaimedBarcodeScanner2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, barcodesymbology: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSymbologyAttributesAsync(barcodesymbology) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSymbologyAttributesAsync<Impl: IClaimedBarcodeScanner2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, barcodesymbology: u32, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSymbologyAttributesAsync(barcodesymbology, &*(&attributes as *const <BarcodeSymbologyAttributes as ::windows::core::Abi>::Abi as *const <BarcodeSymbologyAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedBarcodeScanner2, BASE_OFFSET>(),
            GetSymbologyAttributesAsync: GetSymbologyAttributesAsync::<Impl, IMPL_OFFSET>,
            SetSymbologyAttributesAsync: SetSymbologyAttributesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedBarcodeScanner2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedBarcodeScanner3Impl: Sized {
    fn ShowVideoPreviewAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn HideVideoPreview(&mut self) -> ::windows::core::Result<()>;
    fn SetIsVideoPreviewShownOnEnable(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsVideoPreviewShownOnEnable(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedBarcodeScanner3 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedBarcodeScanner3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedBarcodeScanner3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedBarcodeScanner3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedBarcodeScanner3Vtbl {
        unsafe extern "system" fn ShowVideoPreviewAsync<Impl: IClaimedBarcodeScanner3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowVideoPreviewAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HideVideoPreview<Impl: IClaimedBarcodeScanner3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HideVideoPreview().into()
        }
        unsafe extern "system" fn SetIsVideoPreviewShownOnEnable<Impl: IClaimedBarcodeScanner3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsVideoPreviewShownOnEnable(value).into()
        }
        unsafe extern "system" fn IsVideoPreviewShownOnEnable<Impl: IClaimedBarcodeScanner3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVideoPreviewShownOnEnable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedBarcodeScanner3, BASE_OFFSET>(),
            ShowVideoPreviewAsync: ShowVideoPreviewAsync::<Impl, IMPL_OFFSET>,
            HideVideoPreview: HideVideoPreview::<Impl, IMPL_OFFSET>,
            SetIsVideoPreviewShownOnEnable: SetIsVideoPreviewShownOnEnable::<Impl, IMPL_OFFSET>,
            IsVideoPreviewShownOnEnable: IsVideoPreviewShownOnEnable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedBarcodeScanner3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedBarcodeScanner4Impl: Sized {
    fn Closed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedBarcodeScanner4 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedBarcodeScanner4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedBarcodeScanner4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedBarcodeScanner4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedBarcodeScanner4Vtbl {
        unsafe extern "system" fn Closed<Impl: IClaimedBarcodeScanner4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IClaimedBarcodeScanner4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedBarcodeScanner4, BASE_OFFSET>(),
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedBarcodeScanner4 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedBarcodeScannerClosedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedBarcodeScannerClosedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedBarcodeScannerClosedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedBarcodeScannerClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IClaimedCashDrawerImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsDrawerOpen(&mut self) -> ::windows::core::Result<bool>;
    fn CloseAlarm(&mut self) -> ::windows::core::Result<CashDrawerCloseAlarm>;
    fn OpenDrawerAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn EnableAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DisableAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RetainDeviceAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ResetStatisticsAsync(&mut self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UpdateStatisticsAsync(&mut self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ReleaseDeviceRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedCashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedCashDrawer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IClaimedCashDrawerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedCashDrawerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedCashDrawerVtbl {
        unsafe extern "system" fn DeviceId<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDrawerOpen<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDrawerOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseAlarm<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseAlarm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenDrawerAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenDrawerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainDeviceAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainDeviceAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetStatisticsAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateStatisticsAsync<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateStatisticsAsync(&*(&statistics as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDeviceRequested<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseDeviceRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReleaseDeviceRequested<Impl: IClaimedCashDrawerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReleaseDeviceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedCashDrawer, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            IsDrawerOpen: IsDrawerOpen::<Impl, IMPL_OFFSET>,
            CloseAlarm: CloseAlarm::<Impl, IMPL_OFFSET>,
            OpenDrawerAsync: OpenDrawerAsync::<Impl, IMPL_OFFSET>,
            EnableAsync: EnableAsync::<Impl, IMPL_OFFSET>,
            DisableAsync: DisableAsync::<Impl, IMPL_OFFSET>,
            RetainDeviceAsync: RetainDeviceAsync::<Impl, IMPL_OFFSET>,
            ResetStatisticsAsync: ResetStatisticsAsync::<Impl, IMPL_OFFSET>,
            UpdateStatisticsAsync: UpdateStatisticsAsync::<Impl, IMPL_OFFSET>,
            ReleaseDeviceRequested: ReleaseDeviceRequested::<Impl, IMPL_OFFSET>,
            RemoveReleaseDeviceRequested: RemoveReleaseDeviceRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedCashDrawer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedCashDrawer2Impl: Sized {
    fn Closed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedCashDrawer2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedCashDrawer2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedCashDrawer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedCashDrawer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedCashDrawer2Vtbl {
        unsafe extern "system" fn Closed<Impl: IClaimedCashDrawer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IClaimedCashDrawer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedCashDrawer2, BASE_OFFSET>(),
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedCashDrawer2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedCashDrawerClosedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedCashDrawerClosedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedCashDrawerClosedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedCashDrawerClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedJournalPrinterImpl: Sized {
    fn CreateJob(&mut self) -> ::windows::core::Result<JournalPrintJob>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClaimedJournalPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedJournalPrinter";
}
#[cfg(feature = "implement_exclusive")]
impl IClaimedJournalPrinterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedJournalPrinterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedJournalPrinterVtbl {
        unsafe extern "system" fn CreateJob<Impl: IClaimedJournalPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateJob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedJournalPrinter, BASE_OFFSET>(), CreateJob: CreateJob::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedJournalPrinter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedLineDisplayImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&mut self) -> ::windows::core::Result<LineDisplayCapabilities>;
    fn PhysicalDeviceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PhysicalDeviceDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlVersion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceServiceVersion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultWindow(&mut self) -> ::windows::core::Result<LineDisplayWindow>;
    fn RetainDevice(&mut self) -> ::windows::core::Result<()>;
    fn ReleaseDeviceRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedLineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedLineDisplay";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedLineDisplayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedLineDisplayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedLineDisplayVtbl {
        unsafe extern "system" fn DeviceId<Impl: IClaimedLineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: IClaimedLineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalDeviceName<Impl: IClaimedLineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalDeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalDeviceDescription<Impl: IClaimedLineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalDeviceDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceControlDescription<Impl: IClaimedLineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceControlDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceControlVersion<Impl: IClaimedLineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceControlVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceServiceVersion<Impl: IClaimedLineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceServiceVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultWindow<Impl: IClaimedLineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainDevice<Impl: IClaimedLineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetainDevice().into()
        }
        unsafe extern "system" fn ReleaseDeviceRequested<Impl: IClaimedLineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseDeviceRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReleaseDeviceRequested<Impl: IClaimedLineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReleaseDeviceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedLineDisplay, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
            PhysicalDeviceName: PhysicalDeviceName::<Impl, IMPL_OFFSET>,
            PhysicalDeviceDescription: PhysicalDeviceDescription::<Impl, IMPL_OFFSET>,
            DeviceControlDescription: DeviceControlDescription::<Impl, IMPL_OFFSET>,
            DeviceControlVersion: DeviceControlVersion::<Impl, IMPL_OFFSET>,
            DeviceServiceVersion: DeviceServiceVersion::<Impl, IMPL_OFFSET>,
            DefaultWindow: DefaultWindow::<Impl, IMPL_OFFSET>,
            RetainDevice: RetainDevice::<Impl, IMPL_OFFSET>,
            ReleaseDeviceRequested: ReleaseDeviceRequested::<Impl, IMPL_OFFSET>,
            RemoveReleaseDeviceRequested: RemoveReleaseDeviceRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedLineDisplay as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IClaimedLineDisplay2Impl: Sized {
    fn GetStatisticsAsync(&mut self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn CheckHealthAsync(&mut self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn CheckPowerStatusAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>;
    fn StatusUpdated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SupportedScreenSizesInCharacters(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Size>>;
    fn MaxBitmapSizeInPixels(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SupportedCharacterSets(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>>;
    fn CustomGlyphs(&mut self) -> ::windows::core::Result<LineDisplayCustomGlyphs>;
    fn GetAttributes(&mut self) -> ::windows::core::Result<LineDisplayAttributes>;
    fn TryUpdateAttributesAsync(&mut self, attributes: &::core::option::Option<LineDisplayAttributes>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetDescriptorAsync(&mut self, descriptor: u32, descriptorstate: LineDisplayDescriptorState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryClearDescriptorsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryCreateWindowAsync(&mut self, viewport: &super::super::Foundation::Rect, windowsize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayWindow>>;
    fn TryStoreStorageFileBitmapAsync(&mut self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>;
    fn TryStoreStorageFileBitmapWithAlignmentAsync(&mut self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>;
    fn TryStoreStorageFileBitmapWithAlignmentAndWidthAsync(&mut self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedLineDisplay2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedLineDisplay2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl IClaimedLineDisplay2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedLineDisplay2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedLineDisplay2Vtbl {
        unsafe extern "system" fn GetStatisticsAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHealthAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckHealthAsync(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckPowerStatusAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckPowerStatusAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUpdated<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUpdated<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportedScreenSizesInCharacters<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedScreenSizesInCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBitmapSizeInPixels<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBitmapSizeInPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCharacterSets<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedCharacterSets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomGlyphs<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomGlyphs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateAttributesAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdateAttributesAsync(&*(&attributes as *const <LineDisplayAttributes as ::windows::core::Abi>::Abi as *const <LineDisplayAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetDescriptorAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: u32, descriptorstate: LineDisplayDescriptorState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetDescriptorAsync(descriptor, descriptorstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryClearDescriptorsAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryClearDescriptorsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateWindowAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: super::super::Foundation::Rect, windowsize: super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateWindowAsync(&*(&viewport as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&windowsize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryStoreStorageFileBitmapAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryStoreStorageFileBitmapAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryStoreStorageFileBitmapWithAlignmentAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryStoreStorageFileBitmapWithAlignmentAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), horizontalalignment, verticalalignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryStoreStorageFileBitmapWithAlignmentAndWidthAsync<Impl: IClaimedLineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryStoreStorageFileBitmapWithAlignmentAndWidthAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), horizontalalignment, verticalalignment, widthinpixels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedLineDisplay2, BASE_OFFSET>(),
            GetStatisticsAsync: GetStatisticsAsync::<Impl, IMPL_OFFSET>,
            CheckHealthAsync: CheckHealthAsync::<Impl, IMPL_OFFSET>,
            CheckPowerStatusAsync: CheckPowerStatusAsync::<Impl, IMPL_OFFSET>,
            StatusUpdated: StatusUpdated::<Impl, IMPL_OFFSET>,
            RemoveStatusUpdated: RemoveStatusUpdated::<Impl, IMPL_OFFSET>,
            SupportedScreenSizesInCharacters: SupportedScreenSizesInCharacters::<Impl, IMPL_OFFSET>,
            MaxBitmapSizeInPixels: MaxBitmapSizeInPixels::<Impl, IMPL_OFFSET>,
            SupportedCharacterSets: SupportedCharacterSets::<Impl, IMPL_OFFSET>,
            CustomGlyphs: CustomGlyphs::<Impl, IMPL_OFFSET>,
            GetAttributes: GetAttributes::<Impl, IMPL_OFFSET>,
            TryUpdateAttributesAsync: TryUpdateAttributesAsync::<Impl, IMPL_OFFSET>,
            TrySetDescriptorAsync: TrySetDescriptorAsync::<Impl, IMPL_OFFSET>,
            TryClearDescriptorsAsync: TryClearDescriptorsAsync::<Impl, IMPL_OFFSET>,
            TryCreateWindowAsync: TryCreateWindowAsync::<Impl, IMPL_OFFSET>,
            TryStoreStorageFileBitmapAsync: TryStoreStorageFileBitmapAsync::<Impl, IMPL_OFFSET>,
            TryStoreStorageFileBitmapWithAlignmentAsync: TryStoreStorageFileBitmapWithAlignmentAsync::<Impl, IMPL_OFFSET>,
            TryStoreStorageFileBitmapWithAlignmentAndWidthAsync: TryStoreStorageFileBitmapWithAlignmentAndWidthAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedLineDisplay2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedLineDisplay3Impl: Sized {
    fn Closed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedLineDisplay3 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedLineDisplay3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedLineDisplay3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedLineDisplay3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedLineDisplay3Vtbl {
        unsafe extern "system" fn Closed<Impl: IClaimedLineDisplay3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IClaimedLineDisplay3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedLineDisplay3, BASE_OFFSET>(),
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedLineDisplay3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedLineDisplayClosedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedLineDisplayClosedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedLineDisplayClosedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedLineDisplayClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedLineDisplayStaticsImpl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorWithConnectionTypes(&mut self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedLineDisplayStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedLineDisplayStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedLineDisplayStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedLineDisplayStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedLineDisplayStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IClaimedLineDisplayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IClaimedLineDisplayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: IClaimedLineDisplayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedLineDisplayStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorWithConnectionTypes: GetDeviceSelectorWithConnectionTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedLineDisplayStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IClaimedMagneticStripeReaderImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsDisabledOnDataReceived(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsDisabledOnDataReceived(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsDecodeDataEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsDecodeDataEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsDeviceAuthenticated(&mut self) -> ::windows::core::Result<bool>;
    fn SetDataEncryptionAlgorithm(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn DataEncryptionAlgorithm(&mut self) -> ::windows::core::Result<u32>;
    fn SetTracksToRead(&mut self, value: MagneticStripeReaderTrackIds) -> ::windows::core::Result<()>;
    fn TracksToRead(&mut self) -> ::windows::core::Result<MagneticStripeReaderTrackIds>;
    fn SetIsTransmitSentinelsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsTransmitSentinelsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn EnableAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RetainDevice(&mut self) -> ::windows::core::Result<()>;
    fn SetErrorReportingType(&mut self, value: MagneticStripeReaderErrorReportingType) -> ::windows::core::Result<()>;
    fn RetrieveDeviceAuthenticationDataAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn AuthenticateDeviceAsync(&mut self, responsetoken: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeAuthenticateDeviceAsync(&mut self, responsetoken: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateKeyAsync(&mut self, key: &::windows::core::HSTRING, keyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResetStatisticsAsync(&mut self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateStatisticsAsync(&mut self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BankCardDataReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBankCardDataReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AamvaCardDataReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAamvaCardDataReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VendorSpecificDataReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVendorSpecificDataReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReleaseDeviceRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedMagneticStripeReader>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedMagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedMagneticStripeReader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IClaimedMagneticStripeReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedMagneticStripeReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedMagneticStripeReaderVtbl {
        unsafe extern "system" fn DeviceId<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDisabledOnDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDisabledOnDataReceived(value).into()
        }
        unsafe extern "system" fn IsDisabledOnDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisabledOnDataReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDecodeDataEnabled<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDecodeDataEnabled(value).into()
        }
        unsafe extern "system" fn IsDecodeDataEnabled<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDecodeDataEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDeviceAuthenticated<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDeviceAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataEncryptionAlgorithm<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataEncryptionAlgorithm(value).into()
        }
        unsafe extern "system" fn DataEncryptionAlgorithm<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataEncryptionAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTracksToRead<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MagneticStripeReaderTrackIds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTracksToRead(value).into()
        }
        unsafe extern "system" fn TracksToRead<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackIds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TracksToRead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTransmitSentinelsEnabled<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTransmitSentinelsEnabled(value).into()
        }
        unsafe extern "system" fn IsTransmitSentinelsEnabled<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransmitSentinelsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainDevice<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetainDevice().into()
        }
        unsafe extern "system" fn SetErrorReportingType<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MagneticStripeReaderErrorReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorReportingType(value).into()
        }
        unsafe extern "system" fn RetrieveDeviceAuthenticationDataAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveDeviceAuthenticationDataAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticateDeviceAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateDeviceAsync(::core::slice::from_raw_parts(::core::mem::transmute_copy(&responsetoken), responseToken_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeAuthenticateDeviceAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeAuthenticateDeviceAsync(::core::slice::from_raw_parts(::core::mem::transmute_copy(&responsetoken), responseToken_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateKeyAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateKeyAsync(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&keyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetStatisticsAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateStatisticsAsync<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateStatisticsAsync(&*(&statistics as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BankCardDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BankCardDataReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBankCardDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBankCardDataReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AamvaCardDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AamvaCardDataReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAamvaCardDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAamvaCardDataReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VendorSpecificDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VendorSpecificDataReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVendorSpecificDataReceived<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVendorSpecificDataReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReleaseDeviceRequested<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseDeviceRequested(&*(&handler as *const <super::super::Foundation::EventHandler<ClaimedMagneticStripeReader> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ClaimedMagneticStripeReader> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReleaseDeviceRequested<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReleaseDeviceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorOccurred<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorOccurred(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveErrorOccurred<Impl: IClaimedMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveErrorOccurred(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedMagneticStripeReader, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsDisabledOnDataReceived: SetIsDisabledOnDataReceived::<Impl, IMPL_OFFSET>,
            IsDisabledOnDataReceived: IsDisabledOnDataReceived::<Impl, IMPL_OFFSET>,
            SetIsDecodeDataEnabled: SetIsDecodeDataEnabled::<Impl, IMPL_OFFSET>,
            IsDecodeDataEnabled: IsDecodeDataEnabled::<Impl, IMPL_OFFSET>,
            IsDeviceAuthenticated: IsDeviceAuthenticated::<Impl, IMPL_OFFSET>,
            SetDataEncryptionAlgorithm: SetDataEncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            DataEncryptionAlgorithm: DataEncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            SetTracksToRead: SetTracksToRead::<Impl, IMPL_OFFSET>,
            TracksToRead: TracksToRead::<Impl, IMPL_OFFSET>,
            SetIsTransmitSentinelsEnabled: SetIsTransmitSentinelsEnabled::<Impl, IMPL_OFFSET>,
            IsTransmitSentinelsEnabled: IsTransmitSentinelsEnabled::<Impl, IMPL_OFFSET>,
            EnableAsync: EnableAsync::<Impl, IMPL_OFFSET>,
            DisableAsync: DisableAsync::<Impl, IMPL_OFFSET>,
            RetainDevice: RetainDevice::<Impl, IMPL_OFFSET>,
            SetErrorReportingType: SetErrorReportingType::<Impl, IMPL_OFFSET>,
            RetrieveDeviceAuthenticationDataAsync: RetrieveDeviceAuthenticationDataAsync::<Impl, IMPL_OFFSET>,
            AuthenticateDeviceAsync: AuthenticateDeviceAsync::<Impl, IMPL_OFFSET>,
            DeAuthenticateDeviceAsync: DeAuthenticateDeviceAsync::<Impl, IMPL_OFFSET>,
            UpdateKeyAsync: UpdateKeyAsync::<Impl, IMPL_OFFSET>,
            ResetStatisticsAsync: ResetStatisticsAsync::<Impl, IMPL_OFFSET>,
            UpdateStatisticsAsync: UpdateStatisticsAsync::<Impl, IMPL_OFFSET>,
            BankCardDataReceived: BankCardDataReceived::<Impl, IMPL_OFFSET>,
            RemoveBankCardDataReceived: RemoveBankCardDataReceived::<Impl, IMPL_OFFSET>,
            AamvaCardDataReceived: AamvaCardDataReceived::<Impl, IMPL_OFFSET>,
            RemoveAamvaCardDataReceived: RemoveAamvaCardDataReceived::<Impl, IMPL_OFFSET>,
            VendorSpecificDataReceived: VendorSpecificDataReceived::<Impl, IMPL_OFFSET>,
            RemoveVendorSpecificDataReceived: RemoveVendorSpecificDataReceived::<Impl, IMPL_OFFSET>,
            ReleaseDeviceRequested: ReleaseDeviceRequested::<Impl, IMPL_OFFSET>,
            RemoveReleaseDeviceRequested: RemoveReleaseDeviceRequested::<Impl, IMPL_OFFSET>,
            ErrorOccurred: ErrorOccurred::<Impl, IMPL_OFFSET>,
            RemoveErrorOccurred: RemoveErrorOccurred::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedMagneticStripeReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedMagneticStripeReader2Impl: Sized {
    fn Closed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedMagneticStripeReader2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedMagneticStripeReader2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedMagneticStripeReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedMagneticStripeReader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedMagneticStripeReader2Vtbl {
        unsafe extern "system" fn Closed<Impl: IClaimedMagneticStripeReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IClaimedMagneticStripeReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedMagneticStripeReader2, BASE_OFFSET>(),
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedMagneticStripeReader2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedMagneticStripeReaderClosedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedMagneticStripeReaderClosedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedMagneticStripeReaderClosedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedMagneticStripeReaderClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IClaimedPosPrinterImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetCharacterSet(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CharacterSet(&mut self) -> ::windows::core::Result<u32>;
    fn IsCoverOpen(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCharacterSetMappingEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsCharacterSetMappingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetMapMode(&mut self, value: PosPrinterMapMode) -> ::windows::core::Result<()>;
    fn MapMode(&mut self) -> ::windows::core::Result<PosPrinterMapMode>;
    fn Receipt(&mut self) -> ::windows::core::Result<ClaimedReceiptPrinter>;
    fn Slip(&mut self) -> ::windows::core::Result<ClaimedSlipPrinter>;
    fn Journal(&mut self) -> ::windows::core::Result<ClaimedJournalPrinter>;
    fn EnableAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DisableAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RetainDeviceAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ResetStatisticsAsync(&mut self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UpdateStatisticsAsync(&mut self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ReleaseDeviceRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedPosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedPosPrinter";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IClaimedPosPrinterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedPosPrinterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedPosPrinterVtbl {
        unsafe extern "system" fn DeviceId<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterSet<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharacterSet(value).into()
        }
        unsafe extern "system" fn CharacterSet<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCoverOpen<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCoverOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCharacterSetMappingEnabled<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCharacterSetMappingEnabled(value).into()
        }
        unsafe extern "system" fn IsCharacterSetMappingEnabled<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCharacterSetMappingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMapMode<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PosPrinterMapMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMapMode(value).into()
        }
        unsafe extern "system" fn MapMode<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterMapMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receipt<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receipt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Slip<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Slip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAsync<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableAsync<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainDeviceAsync<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainDeviceAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetStatisticsAsync<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateStatisticsAsync<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateStatisticsAsync(&*(&statistics as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDeviceRequested<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseDeviceRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReleaseDeviceRequested<Impl: IClaimedPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReleaseDeviceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedPosPrinter, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetCharacterSet: SetCharacterSet::<Impl, IMPL_OFFSET>,
            CharacterSet: CharacterSet::<Impl, IMPL_OFFSET>,
            IsCoverOpen: IsCoverOpen::<Impl, IMPL_OFFSET>,
            SetIsCharacterSetMappingEnabled: SetIsCharacterSetMappingEnabled::<Impl, IMPL_OFFSET>,
            IsCharacterSetMappingEnabled: IsCharacterSetMappingEnabled::<Impl, IMPL_OFFSET>,
            SetMapMode: SetMapMode::<Impl, IMPL_OFFSET>,
            MapMode: MapMode::<Impl, IMPL_OFFSET>,
            Receipt: Receipt::<Impl, IMPL_OFFSET>,
            Slip: Slip::<Impl, IMPL_OFFSET>,
            Journal: Journal::<Impl, IMPL_OFFSET>,
            EnableAsync: EnableAsync::<Impl, IMPL_OFFSET>,
            DisableAsync: DisableAsync::<Impl, IMPL_OFFSET>,
            RetainDeviceAsync: RetainDeviceAsync::<Impl, IMPL_OFFSET>,
            ResetStatisticsAsync: ResetStatisticsAsync::<Impl, IMPL_OFFSET>,
            UpdateStatisticsAsync: UpdateStatisticsAsync::<Impl, IMPL_OFFSET>,
            ReleaseDeviceRequested: ReleaseDeviceRequested::<Impl, IMPL_OFFSET>,
            RemoveReleaseDeviceRequested: RemoveReleaseDeviceRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedPosPrinter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedPosPrinter2Impl: Sized {
    fn Closed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedPosPrinter2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedPosPrinter2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedPosPrinter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedPosPrinter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedPosPrinter2Vtbl {
        unsafe extern "system" fn Closed<Impl: IClaimedPosPrinter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IClaimedPosPrinter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedPosPrinter2, BASE_OFFSET>(),
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedPosPrinter2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedPosPrinterClosedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedPosPrinterClosedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedPosPrinterClosedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedPosPrinterClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedReceiptPrinterImpl: Sized {
    fn SidewaysMaxLines(&mut self) -> ::windows::core::Result<u32>;
    fn SidewaysMaxChars(&mut self) -> ::windows::core::Result<u32>;
    fn LinesToPaperCut(&mut self) -> ::windows::core::Result<u32>;
    fn PageSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn PrintArea(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn CreateJob(&mut self) -> ::windows::core::Result<ReceiptPrintJob>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedReceiptPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedReceiptPrinter";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedReceiptPrinterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedReceiptPrinterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedReceiptPrinterVtbl {
        unsafe extern "system" fn SidewaysMaxLines<Impl: IClaimedReceiptPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SidewaysMaxLines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SidewaysMaxChars<Impl: IClaimedReceiptPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SidewaysMaxChars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinesToPaperCut<Impl: IClaimedReceiptPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinesToPaperCut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageSize<Impl: IClaimedReceiptPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrintArea<Impl: IClaimedReceiptPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintArea() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJob<Impl: IClaimedReceiptPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateJob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedReceiptPrinter, BASE_OFFSET>(),
            SidewaysMaxLines: SidewaysMaxLines::<Impl, IMPL_OFFSET>,
            SidewaysMaxChars: SidewaysMaxChars::<Impl, IMPL_OFFSET>,
            LinesToPaperCut: LinesToPaperCut::<Impl, IMPL_OFFSET>,
            PageSize: PageSize::<Impl, IMPL_OFFSET>,
            PrintArea: PrintArea::<Impl, IMPL_OFFSET>,
            CreateJob: CreateJob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedReceiptPrinter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClaimedSlipPrinterImpl: Sized {
    fn SidewaysMaxLines(&mut self) -> ::windows::core::Result<u32>;
    fn SidewaysMaxChars(&mut self) -> ::windows::core::Result<u32>;
    fn MaxLines(&mut self) -> ::windows::core::Result<u32>;
    fn LinesNearEndToEnd(&mut self) -> ::windows::core::Result<u32>;
    fn PrintSide(&mut self) -> ::windows::core::Result<PosPrinterPrintSide>;
    fn PageSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn PrintArea(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn OpenJaws(&mut self) -> ::windows::core::Result<()>;
    fn CloseJaws(&mut self) -> ::windows::core::Result<()>;
    fn InsertSlipAsync(&mut self, timeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RemoveSlipAsync(&mut self, timeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ChangePrintSide(&mut self, printside: PosPrinterPrintSide) -> ::windows::core::Result<()>;
    fn CreateJob(&mut self) -> ::windows::core::Result<SlipPrintJob>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClaimedSlipPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.IClaimedSlipPrinter";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClaimedSlipPrinterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClaimedSlipPrinterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClaimedSlipPrinterVtbl {
        unsafe extern "system" fn SidewaysMaxLines<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SidewaysMaxLines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SidewaysMaxChars<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SidewaysMaxChars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxLines<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxLines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinesNearEndToEnd<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinesNearEndToEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintSide<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterPrintSide) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintSide() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageSize<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrintArea<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintArea() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenJaws<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenJaws().into()
        }
        unsafe extern "system" fn CloseJaws<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseJaws().into()
        }
        unsafe extern "system" fn InsertSlipAsync<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertSlipAsync(&*(&timeout as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSlipAsync<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveSlipAsync(&*(&timeout as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangePrintSide<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printside: PosPrinterPrintSide) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangePrintSide(printside).into()
        }
        unsafe extern "system" fn CreateJob<Impl: IClaimedSlipPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateJob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClaimedSlipPrinter, BASE_OFFSET>(),
            SidewaysMaxLines: SidewaysMaxLines::<Impl, IMPL_OFFSET>,
            SidewaysMaxChars: SidewaysMaxChars::<Impl, IMPL_OFFSET>,
            MaxLines: MaxLines::<Impl, IMPL_OFFSET>,
            LinesNearEndToEnd: LinesNearEndToEnd::<Impl, IMPL_OFFSET>,
            PrintSide: PrintSide::<Impl, IMPL_OFFSET>,
            PageSize: PageSize::<Impl, IMPL_OFFSET>,
            PrintArea: PrintArea::<Impl, IMPL_OFFSET>,
            OpenJaws: OpenJaws::<Impl, IMPL_OFFSET>,
            CloseJaws: CloseJaws::<Impl, IMPL_OFFSET>,
            InsertSlipAsync: InsertSlipAsync::<Impl, IMPL_OFFSET>,
            RemoveSlipAsync: RemoveSlipAsync::<Impl, IMPL_OFFSET>,
            ChangePrintSide: ChangePrintSide::<Impl, IMPL_OFFSET>,
            CreateJob: CreateJob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClaimedSlipPrinter as ::windows::core::Interface>::IID
    }
}
pub trait ICommonClaimedPosPrinterStationImpl: Sized {
    fn SetCharactersPerLine(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CharactersPerLine(&mut self) -> ::windows::core::Result<u32>;
    fn SetLineHeight(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn LineHeight(&mut self) -> ::windows::core::Result<u32>;
    fn SetLineSpacing(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn LineSpacing(&mut self) -> ::windows::core::Result<u32>;
    fn LineWidth(&mut self) -> ::windows::core::Result<u32>;
    fn SetIsLetterQuality(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsLetterQuality(&mut self) -> ::windows::core::Result<bool>;
    fn IsPaperNearEnd(&mut self) -> ::windows::core::Result<bool>;
    fn SetColorCartridge(&mut self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()>;
    fn ColorCartridge(&mut self) -> ::windows::core::Result<PosPrinterColorCartridge>;
    fn IsCoverOpen(&mut self) -> ::windows::core::Result<bool>;
    fn IsCartridgeRemoved(&mut self) -> ::windows::core::Result<bool>;
    fn IsCartridgeEmpty(&mut self) -> ::windows::core::Result<bool>;
    fn IsHeadCleaning(&mut self) -> ::windows::core::Result<bool>;
    fn IsPaperEmpty(&mut self) -> ::windows::core::Result<bool>;
    fn IsReadyToPrint(&mut self) -> ::windows::core::Result<bool>;
    fn ValidateData(&mut self, data: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICommonClaimedPosPrinterStation {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonClaimedPosPrinterStation";
}
impl ICommonClaimedPosPrinterStationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonClaimedPosPrinterStationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonClaimedPosPrinterStationVtbl {
        unsafe extern "system" fn SetCharactersPerLine<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharactersPerLine(value).into()
        }
        unsafe extern "system" fn CharactersPerLine<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharactersPerLine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineHeight(value).into()
        }
        unsafe extern "system" fn LineHeight<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineSpacing<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineSpacing(value).into()
        }
        unsafe extern "system" fn LineSpacing<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineWidth<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLetterQuality<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLetterQuality(value).into()
        }
        unsafe extern "system" fn IsLetterQuality<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLetterQuality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEnd<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaperNearEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorCartridge<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PosPrinterColorCartridge) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorCartridge(value).into()
        }
        unsafe extern "system" fn ColorCartridge<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCartridge) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorCartridge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCoverOpen<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCoverOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeRemoved<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCartridgeRemoved() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeEmpty<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCartridgeEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHeadCleaning<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHeadCleaning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmpty<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaperEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadyToPrint<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadyToPrint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateData<Impl: ICommonClaimedPosPrinterStationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateData(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommonClaimedPosPrinterStation, BASE_OFFSET>(),
            SetCharactersPerLine: SetCharactersPerLine::<Impl, IMPL_OFFSET>,
            CharactersPerLine: CharactersPerLine::<Impl, IMPL_OFFSET>,
            SetLineHeight: SetLineHeight::<Impl, IMPL_OFFSET>,
            LineHeight: LineHeight::<Impl, IMPL_OFFSET>,
            SetLineSpacing: SetLineSpacing::<Impl, IMPL_OFFSET>,
            LineSpacing: LineSpacing::<Impl, IMPL_OFFSET>,
            LineWidth: LineWidth::<Impl, IMPL_OFFSET>,
            SetIsLetterQuality: SetIsLetterQuality::<Impl, IMPL_OFFSET>,
            IsLetterQuality: IsLetterQuality::<Impl, IMPL_OFFSET>,
            IsPaperNearEnd: IsPaperNearEnd::<Impl, IMPL_OFFSET>,
            SetColorCartridge: SetColorCartridge::<Impl, IMPL_OFFSET>,
            ColorCartridge: ColorCartridge::<Impl, IMPL_OFFSET>,
            IsCoverOpen: IsCoverOpen::<Impl, IMPL_OFFSET>,
            IsCartridgeRemoved: IsCartridgeRemoved::<Impl, IMPL_OFFSET>,
            IsCartridgeEmpty: IsCartridgeEmpty::<Impl, IMPL_OFFSET>,
            IsHeadCleaning: IsHeadCleaning::<Impl, IMPL_OFFSET>,
            IsPaperEmpty: IsPaperEmpty::<Impl, IMPL_OFFSET>,
            IsReadyToPrint: IsReadyToPrint::<Impl, IMPL_OFFSET>,
            ValidateData: ValidateData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonClaimedPosPrinterStation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonPosPrintStationCapabilitiesImpl: Sized {
    fn IsPrinterPresent(&mut self) -> ::windows::core::Result<bool>;
    fn IsDualColorSupported(&mut self) -> ::windows::core::Result<bool>;
    fn ColorCartridgeCapabilities(&mut self) -> ::windows::core::Result<PosPrinterColorCapabilities>;
    fn CartridgeSensors(&mut self) -> ::windows::core::Result<PosPrinterCartridgeSensors>;
    fn IsBoldSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsItalicSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsUnderlineSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsDoubleHighPrintSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsDoubleWidePrintSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsDoubleHighDoubleWidePrintSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPaperEmptySensorSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPaperNearEndSensorSupported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedCharactersPerLine(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ICommonPosPrintStationCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonPosPrintStationCapabilities";
}
#[cfg(feature = "Foundation_Collections")]
impl ICommonPosPrintStationCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonPosPrintStationCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonPosPrintStationCapabilitiesVtbl {
        unsafe extern "system" fn IsPrinterPresent<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPrinterPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDualColorSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDualColorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorCartridgeCapabilities<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorCartridgeCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CartridgeSensors<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterCartridgeSensors) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CartridgeSensors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBoldSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBoldSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsItalicSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsItalicSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUnderlineSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUnderlineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighPrintSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDoubleHighPrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleWidePrintSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDoubleWidePrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighDoubleWidePrintSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDoubleHighDoubleWidePrintSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmptySensorSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaperEmptySensorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEndSensorSupported<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaperNearEndSensorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCharactersPerLine<Impl: ICommonPosPrintStationCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedCharactersPerLine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommonPosPrintStationCapabilities, BASE_OFFSET>(),
            IsPrinterPresent: IsPrinterPresent::<Impl, IMPL_OFFSET>,
            IsDualColorSupported: IsDualColorSupported::<Impl, IMPL_OFFSET>,
            ColorCartridgeCapabilities: ColorCartridgeCapabilities::<Impl, IMPL_OFFSET>,
            CartridgeSensors: CartridgeSensors::<Impl, IMPL_OFFSET>,
            IsBoldSupported: IsBoldSupported::<Impl, IMPL_OFFSET>,
            IsItalicSupported: IsItalicSupported::<Impl, IMPL_OFFSET>,
            IsUnderlineSupported: IsUnderlineSupported::<Impl, IMPL_OFFSET>,
            IsDoubleHighPrintSupported: IsDoubleHighPrintSupported::<Impl, IMPL_OFFSET>,
            IsDoubleWidePrintSupported: IsDoubleWidePrintSupported::<Impl, IMPL_OFFSET>,
            IsDoubleHighDoubleWidePrintSupported: IsDoubleHighDoubleWidePrintSupported::<Impl, IMPL_OFFSET>,
            IsPaperEmptySensorSupported: IsPaperEmptySensorSupported::<Impl, IMPL_OFFSET>,
            IsPaperNearEndSensorSupported: IsPaperNearEndSensorSupported::<Impl, IMPL_OFFSET>,
            SupportedCharactersPerLine: SupportedCharactersPerLine::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonPosPrintStationCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonReceiptSlipCapabilitiesImpl: Sized + ICommonPosPrintStationCapabilitiesImpl {
    fn IsBarcodeSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsBitmapSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsLeft90RotationSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsRight90RotationSupported(&mut self) -> ::windows::core::Result<bool>;
    fn Is180RotationSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPrintAreaSupported(&mut self) -> ::windows::core::Result<bool>;
    fn RuledLineCapabilities(&mut self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities>;
    fn SupportedBarcodeRotations(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
    fn SupportedBitmapRotations(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ICommonReceiptSlipCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonReceiptSlipCapabilities";
}
#[cfg(feature = "Foundation_Collections")]
impl ICommonReceiptSlipCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonReceiptSlipCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonReceiptSlipCapabilitiesVtbl {
        unsafe extern "system" fn IsBarcodeSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBarcodeSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBitmapSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBitmapSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLeft90RotationSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLeft90RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRight90RotationSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRight90RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is180RotationSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Is180RotationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrintAreaSupported<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPrintAreaSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuledLineCapabilities<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterRuledLineCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RuledLineCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBarcodeRotations<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedBarcodeRotations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBitmapRotations<Impl: ICommonReceiptSlipCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedBitmapRotations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommonReceiptSlipCapabilities, BASE_OFFSET>(),
            IsBarcodeSupported: IsBarcodeSupported::<Impl, IMPL_OFFSET>,
            IsBitmapSupported: IsBitmapSupported::<Impl, IMPL_OFFSET>,
            IsLeft90RotationSupported: IsLeft90RotationSupported::<Impl, IMPL_OFFSET>,
            IsRight90RotationSupported: IsRight90RotationSupported::<Impl, IMPL_OFFSET>,
            Is180RotationSupported: Is180RotationSupported::<Impl, IMPL_OFFSET>,
            IsPrintAreaSupported: IsPrintAreaSupported::<Impl, IMPL_OFFSET>,
            RuledLineCapabilities: RuledLineCapabilities::<Impl, IMPL_OFFSET>,
            SupportedBarcodeRotations: SupportedBarcodeRotations::<Impl, IMPL_OFFSET>,
            SupportedBitmapRotations: SupportedBitmapRotations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonReceiptSlipCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJournalPrintJobImpl: Sized {
    fn Print(&mut self, data: &::windows::core::HSTRING, printoptions: &::core::option::Option<PosPrinterPrintOptions>) -> ::windows::core::Result<()>;
    fn FeedPaperByLine(&mut self, linecount: i32) -> ::windows::core::Result<()>;
    fn FeedPaperByMapModeUnit(&mut self, distance: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJournalPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IJournalPrintJob";
}
#[cfg(feature = "implement_exclusive")]
impl IJournalPrintJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJournalPrintJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJournalPrintJobVtbl {
        unsafe extern "system" fn Print<Impl: IJournalPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Print(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&printoptions as *const <PosPrinterPrintOptions as ::windows::core::Abi>::Abi as *const <PosPrinterPrintOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FeedPaperByLine<Impl: IJournalPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedPaperByLine(linecount).into()
        }
        unsafe extern "system" fn FeedPaperByMapModeUnit<Impl: IJournalPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedPaperByMapModeUnit(distance).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJournalPrintJob, BASE_OFFSET>(),
            Print: Print::<Impl, IMPL_OFFSET>,
            FeedPaperByLine: FeedPaperByLine::<Impl, IMPL_OFFSET>,
            FeedPaperByMapModeUnit: FeedPaperByMapModeUnit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJournalPrintJob as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJournalPrinterCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJournalPrinterCapabilitiesVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IJournalPrinterCapabilities, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJournalPrinterCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJournalPrinterCapabilities2Impl: Sized {
    fn IsReverseVideoSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsStrikethroughSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsSuperscriptSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsSubscriptSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByLineSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByMapModeUnitSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJournalPrinterCapabilities2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IJournalPrinterCapabilities2";
}
#[cfg(feature = "implement_exclusive")]
impl IJournalPrinterCapabilities2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJournalPrinterCapabilities2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJournalPrinterCapabilities2Vtbl {
        unsafe extern "system" fn IsReverseVideoSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReverseVideoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStrikethroughSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStrikethroughSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSuperscriptSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSuperscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscriptSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSubscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByLineSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByLineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByMapModeUnitSupported<Impl: IJournalPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByMapModeUnitSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJournalPrinterCapabilities2, BASE_OFFSET>(),
            IsReverseVideoSupported: IsReverseVideoSupported::<Impl, IMPL_OFFSET>,
            IsStrikethroughSupported: IsStrikethroughSupported::<Impl, IMPL_OFFSET>,
            IsSuperscriptSupported: IsSuperscriptSupported::<Impl, IMPL_OFFSET>,
            IsSubscriptSupported: IsSubscriptSupported::<Impl, IMPL_OFFSET>,
            IsReversePaperFeedByLineSupported: IsReversePaperFeedByLineSupported::<Impl, IMPL_OFFSET>,
            IsReversePaperFeedByMapModeUnitSupported: IsReversePaperFeedByMapModeUnitSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJournalPrinterCapabilities2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILineDisplayImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&mut self) -> ::windows::core::Result<LineDisplayCapabilities>;
    fn PhysicalDeviceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PhysicalDeviceDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlVersion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceServiceVersion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ClaimAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplay";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILineDisplayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayVtbl {
        unsafe extern "system" fn DeviceId<Impl: ILineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: ILineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalDeviceName<Impl: ILineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalDeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalDeviceDescription<Impl: ILineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalDeviceDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceControlDescription<Impl: ILineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceControlDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceControlVersion<Impl: ILineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceControlVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceServiceVersion<Impl: ILineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceServiceVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClaimAsync<Impl: ILineDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClaimAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplay, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
            PhysicalDeviceName: PhysicalDeviceName::<Impl, IMPL_OFFSET>,
            PhysicalDeviceDescription: PhysicalDeviceDescription::<Impl, IMPL_OFFSET>,
            DeviceControlDescription: DeviceControlDescription::<Impl, IMPL_OFFSET>,
            DeviceControlVersion: DeviceControlVersion::<Impl, IMPL_OFFSET>,
            DeviceServiceVersion: DeviceServiceVersion::<Impl, IMPL_OFFSET>,
            ClaimAsync: ClaimAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplay as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILineDisplay2Impl: Sized {
    fn CheckPowerStatusAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineDisplay2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplay2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILineDisplay2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplay2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplay2Vtbl {
        unsafe extern "system" fn CheckPowerStatusAsync<Impl: ILineDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckPowerStatusAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplay2, BASE_OFFSET>(),
            CheckPowerStatusAsync: CheckPowerStatusAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplay2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILineDisplayAttributesImpl: Sized {
    fn IsPowerNotifyEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPowerNotifyEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Brightness(&mut self) -> ::windows::core::Result<i32>;
    fn SetBrightness(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn BlinkRate(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBlinkRate(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ScreenSizeInCharacters(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetScreenSizeInCharacters(&mut self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn CharacterSet(&mut self) -> ::windows::core::Result<i32>;
    fn SetCharacterSet(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn IsCharacterSetMappingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCharacterSetMappingEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentWindow(&mut self) -> ::windows::core::Result<LineDisplayWindow>;
    fn SetCurrentWindow(&mut self, value: &::core::option::Option<LineDisplayWindow>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineDisplayAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayAttributes";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILineDisplayAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayAttributesVtbl {
        unsafe extern "system" fn IsPowerNotifyEnabled<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPowerNotifyEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPowerNotifyEnabled<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPowerNotifyEnabled(value).into()
        }
        unsafe extern "system" fn Brightness<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBrightness<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBrightness(value).into()
        }
        unsafe extern "system" fn BlinkRate<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlinkRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlinkRate<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlinkRate(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScreenSizeInCharacters<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenSizeInCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScreenSizeInCharacters<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScreenSizeInCharacters(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CharacterSet<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterSet<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharacterSet(value).into()
        }
        unsafe extern "system" fn IsCharacterSetMappingEnabled<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCharacterSetMappingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCharacterSetMappingEnabled<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCharacterSetMappingEnabled(value).into()
        }
        unsafe extern "system" fn CurrentWindow<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentWindow<Impl: ILineDisplayAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrentWindow(&*(&value as *const <LineDisplayWindow as ::windows::core::Abi>::Abi as *const <LineDisplayWindow as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayAttributes, BASE_OFFSET>(),
            IsPowerNotifyEnabled: IsPowerNotifyEnabled::<Impl, IMPL_OFFSET>,
            SetIsPowerNotifyEnabled: SetIsPowerNotifyEnabled::<Impl, IMPL_OFFSET>,
            Brightness: Brightness::<Impl, IMPL_OFFSET>,
            SetBrightness: SetBrightness::<Impl, IMPL_OFFSET>,
            BlinkRate: BlinkRate::<Impl, IMPL_OFFSET>,
            SetBlinkRate: SetBlinkRate::<Impl, IMPL_OFFSET>,
            ScreenSizeInCharacters: ScreenSizeInCharacters::<Impl, IMPL_OFFSET>,
            SetScreenSizeInCharacters: SetScreenSizeInCharacters::<Impl, IMPL_OFFSET>,
            CharacterSet: CharacterSet::<Impl, IMPL_OFFSET>,
            SetCharacterSet: SetCharacterSet::<Impl, IMPL_OFFSET>,
            IsCharacterSetMappingEnabled: IsCharacterSetMappingEnabled::<Impl, IMPL_OFFSET>,
            SetIsCharacterSetMappingEnabled: SetIsCharacterSetMappingEnabled::<Impl, IMPL_OFFSET>,
            CurrentWindow: CurrentWindow::<Impl, IMPL_OFFSET>,
            SetCurrentWindow: SetCurrentWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayCapabilitiesImpl: Sized {
    fn IsStatisticsReportingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn PowerReportingType(&mut self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn CanChangeScreenSize(&mut self) -> ::windows::core::Result<bool>;
    fn CanDisplayBitmaps(&mut self) -> ::windows::core::Result<bool>;
    fn CanReadCharacterAtCursor(&mut self) -> ::windows::core::Result<bool>;
    fn CanMapCharacterSets(&mut self) -> ::windows::core::Result<bool>;
    fn CanDisplayCustomGlyphs(&mut self) -> ::windows::core::Result<bool>;
    fn CanReverse(&mut self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity>;
    fn CanBlink(&mut self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity>;
    fn CanChangeBlinkRate(&mut self) -> ::windows::core::Result<bool>;
    fn IsBrightnessSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsCursorSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsHorizontalMarqueeSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsVerticalMarqueeSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsInterCharacterWaitSupported(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedDescriptors(&mut self) -> ::windows::core::Result<u32>;
    fn SupportedWindows(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayCapabilitiesVtbl {
        unsafe extern "system" fn IsStatisticsReportingSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatisticsReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsUpdatingSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatisticsUpdatingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerReportingType<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanChangeScreenSize<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanChangeScreenSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDisplayBitmaps<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDisplayBitmaps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanReadCharacterAtCursor<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanReadCharacterAtCursor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMapCharacterSets<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMapCharacterSets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDisplayCustomGlyphs<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDisplayCustomGlyphs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanReverse<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanReverse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanBlink<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanBlink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanChangeBlinkRate<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanChangeBlinkRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBrightnessSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBrightnessSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCursorSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCursorSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHorizontalMarqueeSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHorizontalMarqueeSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVerticalMarqueeSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVerticalMarqueeSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInterCharacterWaitSupported<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInterCharacterWaitSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedDescriptors<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWindows<Impl: ILineDisplayCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWindows() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayCapabilities, BASE_OFFSET>(),
            IsStatisticsReportingSupported: IsStatisticsReportingSupported::<Impl, IMPL_OFFSET>,
            IsStatisticsUpdatingSupported: IsStatisticsUpdatingSupported::<Impl, IMPL_OFFSET>,
            PowerReportingType: PowerReportingType::<Impl, IMPL_OFFSET>,
            CanChangeScreenSize: CanChangeScreenSize::<Impl, IMPL_OFFSET>,
            CanDisplayBitmaps: CanDisplayBitmaps::<Impl, IMPL_OFFSET>,
            CanReadCharacterAtCursor: CanReadCharacterAtCursor::<Impl, IMPL_OFFSET>,
            CanMapCharacterSets: CanMapCharacterSets::<Impl, IMPL_OFFSET>,
            CanDisplayCustomGlyphs: CanDisplayCustomGlyphs::<Impl, IMPL_OFFSET>,
            CanReverse: CanReverse::<Impl, IMPL_OFFSET>,
            CanBlink: CanBlink::<Impl, IMPL_OFFSET>,
            CanChangeBlinkRate: CanChangeBlinkRate::<Impl, IMPL_OFFSET>,
            IsBrightnessSupported: IsBrightnessSupported::<Impl, IMPL_OFFSET>,
            IsCursorSupported: IsCursorSupported::<Impl, IMPL_OFFSET>,
            IsHorizontalMarqueeSupported: IsHorizontalMarqueeSupported::<Impl, IMPL_OFFSET>,
            IsVerticalMarqueeSupported: IsVerticalMarqueeSupported::<Impl, IMPL_OFFSET>,
            IsInterCharacterWaitSupported: IsInterCharacterWaitSupported::<Impl, IMPL_OFFSET>,
            SupportedDescriptors: SupportedDescriptors::<Impl, IMPL_OFFSET>,
            SupportedWindows: SupportedWindows::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILineDisplayCursorImpl: Sized {
    fn CanCustomize(&mut self) -> ::windows::core::Result<bool>;
    fn IsBlinkSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsBlockSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsHalfBlockSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsUnderlineSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsReverseSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsOtherSupported(&mut self) -> ::windows::core::Result<bool>;
    fn GetAttributes(&mut self) -> ::windows::core::Result<LineDisplayCursorAttributes>;
    fn TryUpdateAttributesAsync(&mut self, attributes: &::core::option::Option<LineDisplayCursorAttributes>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineDisplayCursor {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayCursor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILineDisplayCursorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayCursorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayCursorVtbl {
        unsafe extern "system" fn CanCustomize<Impl: ILineDisplayCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCustomize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBlinkSupported<Impl: ILineDisplayCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBlinkSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBlockSupported<Impl: ILineDisplayCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBlockSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHalfBlockSupported<Impl: ILineDisplayCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHalfBlockSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUnderlineSupported<Impl: ILineDisplayCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUnderlineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReverseSupported<Impl: ILineDisplayCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReverseSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOtherSupported<Impl: ILineDisplayCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOtherSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: ILineDisplayCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateAttributesAsync<Impl: ILineDisplayCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdateAttributesAsync(&*(&attributes as *const <LineDisplayCursorAttributes as ::windows::core::Abi>::Abi as *const <LineDisplayCursorAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayCursor, BASE_OFFSET>(),
            CanCustomize: CanCustomize::<Impl, IMPL_OFFSET>,
            IsBlinkSupported: IsBlinkSupported::<Impl, IMPL_OFFSET>,
            IsBlockSupported: IsBlockSupported::<Impl, IMPL_OFFSET>,
            IsHalfBlockSupported: IsHalfBlockSupported::<Impl, IMPL_OFFSET>,
            IsUnderlineSupported: IsUnderlineSupported::<Impl, IMPL_OFFSET>,
            IsReverseSupported: IsReverseSupported::<Impl, IMPL_OFFSET>,
            IsOtherSupported: IsOtherSupported::<Impl, IMPL_OFFSET>,
            GetAttributes: GetAttributes::<Impl, IMPL_OFFSET>,
            TryUpdateAttributesAsync: TryUpdateAttributesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayCursor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILineDisplayCursorAttributesImpl: Sized {
    fn IsBlinkEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsBlinkEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CursorType(&mut self) -> ::windows::core::Result<LineDisplayCursorType>;
    fn SetCursorType(&mut self, value: LineDisplayCursorType) -> ::windows::core::Result<()>;
    fn IsAutoAdvanceEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsAutoAdvanceEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetPosition(&mut self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineDisplayCursorAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayCursorAttributes";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILineDisplayCursorAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayCursorAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayCursorAttributesVtbl {
        unsafe extern "system" fn IsBlinkEnabled<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBlinkEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsBlinkEnabled<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsBlinkEnabled(value).into()
        }
        unsafe extern "system" fn CursorType<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayCursorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CursorType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCursorType<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LineDisplayCursorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCursorType(value).into()
        }
        unsafe extern "system" fn IsAutoAdvanceEnabled<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAutoAdvanceEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAutoAdvanceEnabled<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAutoAdvanceEnabled(value).into()
        }
        unsafe extern "system" fn Position<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: ILineDisplayCursorAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayCursorAttributes, BASE_OFFSET>(),
            IsBlinkEnabled: IsBlinkEnabled::<Impl, IMPL_OFFSET>,
            SetIsBlinkEnabled: SetIsBlinkEnabled::<Impl, IMPL_OFFSET>,
            CursorType: CursorType::<Impl, IMPL_OFFSET>,
            SetCursorType: SetCursorType::<Impl, IMPL_OFFSET>,
            IsAutoAdvanceEnabled: IsAutoAdvanceEnabled::<Impl, IMPL_OFFSET>,
            SetIsAutoAdvanceEnabled: SetIsAutoAdvanceEnabled::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayCursorAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ILineDisplayCustomGlyphsImpl: Sized {
    fn SizeInPixels(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SupportedGlyphCodes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn TryRedefineAsync(&mut self, glyphcode: u32, glyphdata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineDisplayCustomGlyphs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayCustomGlyphs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ILineDisplayCustomGlyphsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayCustomGlyphsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayCustomGlyphsVtbl {
        unsafe extern "system" fn SizeInPixels<Impl: ILineDisplayCustomGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeInPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedGlyphCodes<Impl: ILineDisplayCustomGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedGlyphCodes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRedefineAsync<Impl: ILineDisplayCustomGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphcode: u32, glyphdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRedefineAsync(glyphcode, &*(&glyphdata as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayCustomGlyphs, BASE_OFFSET>(),
            SizeInPixels: SizeInPixels::<Impl, IMPL_OFFSET>,
            SupportedGlyphCodes: SupportedGlyphCodes::<Impl, IMPL_OFFSET>,
            TryRedefineAsync: TryRedefineAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayCustomGlyphs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILineDisplayMarqueeImpl: Sized {
    fn Format(&mut self) -> ::windows::core::Result<LineDisplayMarqueeFormat>;
    fn SetFormat(&mut self, value: LineDisplayMarqueeFormat) -> ::windows::core::Result<()>;
    fn RepeatWaitInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetRepeatWaitInterval(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ScrollWaitInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetScrollWaitInterval(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TryStartScrollingAsync(&mut self, direction: LineDisplayScrollDirection) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryStopScrollingAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineDisplayMarquee {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayMarquee";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILineDisplayMarqueeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayMarqueeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayMarqueeVtbl {
        unsafe extern "system" fn Format<Impl: ILineDisplayMarqueeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayMarqueeFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFormat<Impl: ILineDisplayMarqueeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LineDisplayMarqueeFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(value).into()
        }
        unsafe extern "system" fn RepeatWaitInterval<Impl: ILineDisplayMarqueeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RepeatWaitInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepeatWaitInterval<Impl: ILineDisplayMarqueeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRepeatWaitInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScrollWaitInterval<Impl: ILineDisplayMarqueeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollWaitInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollWaitInterval<Impl: ILineDisplayMarqueeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScrollWaitInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryStartScrollingAsync<Impl: ILineDisplayMarqueeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: LineDisplayScrollDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryStartScrollingAsync(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryStopScrollingAsync<Impl: ILineDisplayMarqueeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryStopScrollingAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayMarquee, BASE_OFFSET>(),
            Format: Format::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            RepeatWaitInterval: RepeatWaitInterval::<Impl, IMPL_OFFSET>,
            SetRepeatWaitInterval: SetRepeatWaitInterval::<Impl, IMPL_OFFSET>,
            ScrollWaitInterval: ScrollWaitInterval::<Impl, IMPL_OFFSET>,
            SetScrollWaitInterval: SetScrollWaitInterval::<Impl, IMPL_OFFSET>,
            TryStartScrollingAsync: TryStartScrollingAsync::<Impl, IMPL_OFFSET>,
            TryStopScrollingAsync: TryStopScrollingAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayMarquee as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILineDisplayStaticsImpl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>>;
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorWithConnectionTypes(&mut self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineDisplayStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILineDisplayStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: ILineDisplayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefaultAsync<Impl: ILineDisplayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: ILineDisplayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: ILineDisplayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorWithConnectionTypes: GetDeviceSelectorWithConnectionTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStatics2Impl: Sized {
    fn StatisticsCategorySelector(&mut self) -> ::windows::core::Result<LineDisplayStatisticsCategorySelector>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayStatics2Vtbl {
        unsafe extern "system" fn StatisticsCategorySelector<Impl: ILineDisplayStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatisticsCategorySelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayStatics2, BASE_OFFSET>(),
            StatisticsCategorySelector: StatisticsCategorySelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStatisticsCategorySelectorImpl: Sized {
    fn AllStatistics(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UnifiedPosStatistics(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ManufacturerStatistics(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayStatisticsCategorySelector {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayStatisticsCategorySelector";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayStatisticsCategorySelectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayStatisticsCategorySelectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayStatisticsCategorySelectorVtbl {
        unsafe extern "system" fn AllStatistics<Impl: ILineDisplayStatisticsCategorySelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnifiedPosStatistics<Impl: ILineDisplayStatisticsCategorySelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnifiedPosStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerStatistics<Impl: ILineDisplayStatisticsCategorySelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManufacturerStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayStatisticsCategorySelector, BASE_OFFSET>(),
            AllStatistics: AllStatistics::<Impl, IMPL_OFFSET>,
            UnifiedPosStatistics: UnifiedPosStatistics::<Impl, IMPL_OFFSET>,
            ManufacturerStatistics: ManufacturerStatistics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayStatisticsCategorySelector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStatusUpdatedEventArgsImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<LineDisplayPowerStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILineDisplayStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayStatusUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILineDisplayStatusUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayStatusUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayStatusUpdatedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: ILineDisplayStatusUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayPowerStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayStatusUpdatedEventArgs, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayStatusUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILineDisplayStoredBitmapImpl: Sized {
    fn EscapeSequence(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TryDeleteAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineDisplayStoredBitmap {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayStoredBitmap";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILineDisplayStoredBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayStoredBitmapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayStoredBitmapVtbl {
        unsafe extern "system" fn EscapeSequence<Impl: ILineDisplayStoredBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EscapeSequence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDeleteAsync<Impl: ILineDisplayStoredBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDeleteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayStoredBitmap, BASE_OFFSET>(),
            EscapeSequence: EscapeSequence::<Impl, IMPL_OFFSET>,
            TryDeleteAsync: TryDeleteAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayStoredBitmap as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILineDisplayWindowImpl: Sized {
    fn SizeInCharacters(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn InterCharacterWaitInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetInterCharacterWaitInterval(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TryRefreshAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayTextAsync(&mut self, text: &::windows::core::HSTRING, displayattribute: LineDisplayTextAttribute) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayTextAtPositionAsync(&mut self, text: &::windows::core::HSTRING, displayattribute: LineDisplayTextAttribute, startposition: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayTextNormalAsync(&mut self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryScrollTextAsync(&mut self, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryClearTextAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineDisplayWindow {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayWindow";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILineDisplayWindowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayWindowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayWindowVtbl {
        unsafe extern "system" fn SizeInCharacters<Impl: ILineDisplayWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeInCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterCharacterWaitInterval<Impl: ILineDisplayWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterCharacterWaitInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterCharacterWaitInterval<Impl: ILineDisplayWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterCharacterWaitInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryRefreshAsync<Impl: ILineDisplayWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRefreshAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayTextAsync<Impl: ILineDisplayWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayattribute: LineDisplayTextAttribute, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDisplayTextAsync(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), displayattribute) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayTextAtPositionAsync<Impl: ILineDisplayWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayattribute: LineDisplayTextAttribute, startposition: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDisplayTextAtPositionAsync(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), displayattribute, &*(&startposition as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayTextNormalAsync<Impl: ILineDisplayWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDisplayTextNormalAsync(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryScrollTextAsync<Impl: ILineDisplayWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryScrollTextAsync(direction, numberofcolumnsorrows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryClearTextAsync<Impl: ILineDisplayWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryClearTextAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayWindow, BASE_OFFSET>(),
            SizeInCharacters: SizeInCharacters::<Impl, IMPL_OFFSET>,
            InterCharacterWaitInterval: InterCharacterWaitInterval::<Impl, IMPL_OFFSET>,
            SetInterCharacterWaitInterval: SetInterCharacterWaitInterval::<Impl, IMPL_OFFSET>,
            TryRefreshAsync: TryRefreshAsync::<Impl, IMPL_OFFSET>,
            TryDisplayTextAsync: TryDisplayTextAsync::<Impl, IMPL_OFFSET>,
            TryDisplayTextAtPositionAsync: TryDisplayTextAtPositionAsync::<Impl, IMPL_OFFSET>,
            TryDisplayTextNormalAsync: TryDisplayTextNormalAsync::<Impl, IMPL_OFFSET>,
            TryScrollTextAsync: TryScrollTextAsync::<Impl, IMPL_OFFSET>,
            TryClearTextAsync: TryClearTextAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait ILineDisplayWindow2Impl: Sized {
    fn Cursor(&mut self) -> ::windows::core::Result<LineDisplayCursor>;
    fn Marquee(&mut self) -> ::windows::core::Result<LineDisplayMarquee>;
    fn ReadCharacterAtCursorAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn TryDisplayStoredBitmapAtCursorAsync(&mut self, bitmap: &::core::option::Option<LineDisplayStoredBitmap>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtCursorAsync(&mut self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync(&mut self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync(&mut self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtPointAsync(&mut self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, offsetinpixels: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtPointWithWidthAsync(&mut self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, offsetinpixels: &super::super::Foundation::Point, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILineDisplayWindow2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.ILineDisplayWindow2";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ILineDisplayWindow2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILineDisplayWindow2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILineDisplayWindow2Vtbl {
        unsafe extern "system" fn Cursor<Impl: ILineDisplayWindow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cursor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Marquee<Impl: ILineDisplayWindow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Marquee() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadCharacterAtCursorAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadCharacterAtCursorAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStoredBitmapAtCursorAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDisplayStoredBitmapAtCursorAsync(&*(&bitmap as *const <LineDisplayStoredBitmap as ::windows::core::Abi>::Abi as *const <LineDisplayStoredBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStorageFileBitmapAtCursorAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDisplayStorageFileBitmapAtCursorAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), horizontalalignment, verticalalignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), horizontalalignment, verticalalignment, widthinpixels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStorageFileBitmapAtPointAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, offsetinpixels: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDisplayStorageFileBitmapAtPointAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), &*(&offsetinpixels as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDisplayStorageFileBitmapAtPointWithWidthAsync<Impl: ILineDisplayWindow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, offsetinpixels: super::super::Foundation::Point, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDisplayStorageFileBitmapAtPointWithWidthAsync(&*(&bitmap as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), &*(&offsetinpixels as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), widthinpixels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILineDisplayWindow2, BASE_OFFSET>(),
            Cursor: Cursor::<Impl, IMPL_OFFSET>,
            Marquee: Marquee::<Impl, IMPL_OFFSET>,
            ReadCharacterAtCursorAsync: ReadCharacterAtCursorAsync::<Impl, IMPL_OFFSET>,
            TryDisplayStoredBitmapAtCursorAsync: TryDisplayStoredBitmapAtCursorAsync::<Impl, IMPL_OFFSET>,
            TryDisplayStorageFileBitmapAtCursorAsync: TryDisplayStorageFileBitmapAtCursorAsync::<Impl, IMPL_OFFSET>,
            TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync: TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync::<Impl, IMPL_OFFSET>,
            TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync: TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync::<Impl, IMPL_OFFSET>,
            TryDisplayStorageFileBitmapAtPointAsync: TryDisplayStorageFileBitmapAtPointAsync::<Impl, IMPL_OFFSET>,
            TryDisplayStorageFileBitmapAtPointWithWidthAsync: TryDisplayStorageFileBitmapAtPointWithWidthAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILineDisplayWindow2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMagneticStripeReaderImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&mut self) -> ::windows::core::Result<MagneticStripeReaderCapabilities>;
    fn SupportedCardTypes(&mut self) -> ::windows::core::Result<::windows::core::Array<u32>>;
    fn DeviceAuthenticationProtocol(&mut self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationProtocol>;
    fn CheckHealthAsync(&mut self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ClaimReaderAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedMagneticStripeReader>>;
    fn RetrieveStatisticsAsync(&mut self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn GetErrorReportingType(&mut self) -> ::windows::core::Result<MagneticStripeReaderErrorReportingType>;
    fn StatusUpdated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMagneticStripeReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderVtbl {
        unsafe extern "system" fn DeviceId<Impl: IMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: IMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCardTypes<Impl: IMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn DeviceAuthenticationProtocol<Impl: IMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderAuthenticationProtocol) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAuthenticationProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHealthAsync<Impl: IMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckHealthAsync(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClaimReaderAsync<Impl: IMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClaimReaderAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveStatisticsAsync<Impl: IMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorReportingType<Impl: IMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderErrorReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUpdated<Impl: IMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUpdated<Impl: IMagneticStripeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReader, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
            SupportedCardTypes: SupportedCardTypes::<Impl, IMPL_OFFSET>,
            DeviceAuthenticationProtocol: DeviceAuthenticationProtocol::<Impl, IMPL_OFFSET>,
            CheckHealthAsync: CheckHealthAsync::<Impl, IMPL_OFFSET>,
            ClaimReaderAsync: ClaimReaderAsync::<Impl, IMPL_OFFSET>,
            RetrieveStatisticsAsync: RetrieveStatisticsAsync::<Impl, IMPL_OFFSET>,
            GetErrorReportingType: GetErrorReportingType::<Impl, IMPL_OFFSET>,
            StatusUpdated: StatusUpdated::<Impl, IMPL_OFFSET>,
            RemoveStatusUpdated: RemoveStatusUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl: Sized {
    fn Report(&mut self) -> ::windows::core::Result<MagneticStripeReaderReport>;
    fn LicenseNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExpirationDate(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Restrictions(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Class(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Endorsements(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BirthDate(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Surname(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Suffix(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Gender(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HairColor(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EyeColor(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Height(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Weight(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Address(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn City(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PostalCode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderAamvaCardDataReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderAamvaCardDataReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderAamvaCardDataReceivedEventArgsVtbl {
        unsafe extern "system" fn Report<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Report() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LicenseNumber<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restrictions<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Restrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Class<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Endorsements<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Endorsements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BirthDate<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BirthDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstName<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Surname<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Surname() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suffix<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suffix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gender<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gender() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HairColor<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HairColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EyeColor<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EyeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Weight<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn City<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).City() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostalCode<Impl: IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderAamvaCardDataReceivedEventArgs, BASE_OFFSET>(),
            Report: Report::<Impl, IMPL_OFFSET>,
            LicenseNumber: LicenseNumber::<Impl, IMPL_OFFSET>,
            ExpirationDate: ExpirationDate::<Impl, IMPL_OFFSET>,
            Restrictions: Restrictions::<Impl, IMPL_OFFSET>,
            Class: Class::<Impl, IMPL_OFFSET>,
            Endorsements: Endorsements::<Impl, IMPL_OFFSET>,
            BirthDate: BirthDate::<Impl, IMPL_OFFSET>,
            FirstName: FirstName::<Impl, IMPL_OFFSET>,
            Surname: Surname::<Impl, IMPL_OFFSET>,
            Suffix: Suffix::<Impl, IMPL_OFFSET>,
            Gender: Gender::<Impl, IMPL_OFFSET>,
            HairColor: HairColor::<Impl, IMPL_OFFSET>,
            EyeColor: EyeColor::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            Weight: Weight::<Impl, IMPL_OFFSET>,
            Address: Address::<Impl, IMPL_OFFSET>,
            City: City::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            PostalCode: PostalCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderAamvaCardDataReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderBankCardDataReceivedEventArgsImpl: Sized {
    fn Report(&mut self) -> ::windows::core::Result<MagneticStripeReaderReport>;
    fn AccountNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExpirationDate(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceCode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MiddleInitial(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Surname(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Suffix(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderBankCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderBankCardDataReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderBankCardDataReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderBankCardDataReceivedEventArgsVtbl {
        unsafe extern "system" fn Report<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Report() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccountNumber<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceCode<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstName<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MiddleInitial<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MiddleInitial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Surname<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Surname() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suffix<Impl: IMagneticStripeReaderBankCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suffix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderBankCardDataReceivedEventArgs, BASE_OFFSET>(),
            Report: Report::<Impl, IMPL_OFFSET>,
            AccountNumber: AccountNumber::<Impl, IMPL_OFFSET>,
            ExpirationDate: ExpirationDate::<Impl, IMPL_OFFSET>,
            ServiceCode: ServiceCode::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            FirstName: FirstName::<Impl, IMPL_OFFSET>,
            MiddleInitial: MiddleInitial::<Impl, IMPL_OFFSET>,
            Surname: Surname::<Impl, IMPL_OFFSET>,
            Suffix: Suffix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderBankCardDataReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderCapabilitiesImpl: Sized {
    fn CardAuthentication(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedEncryptionAlgorithms(&mut self) -> ::windows::core::Result<u32>;
    fn AuthenticationLevel(&mut self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationLevel>;
    fn IsIsoSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsJisOneSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsJisTwoSupported(&mut self) -> ::windows::core::Result<bool>;
    fn PowerReportingType(&mut self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsTrackDataMaskingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsTransmitSentinelsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderCapabilitiesVtbl {
        unsafe extern "system" fn CardAuthentication<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardAuthentication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedEncryptionAlgorithms<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedEncryptionAlgorithms() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationLevel<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderAuthenticationLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIsoSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIsoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsJisOneSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsJisOneSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsJisTwoSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsJisTwoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerReportingType<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsReportingSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatisticsReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsUpdatingSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatisticsUpdatingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTrackDataMaskingSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTrackDataMaskingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransmitSentinelsSupported<Impl: IMagneticStripeReaderCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransmitSentinelsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderCapabilities, BASE_OFFSET>(),
            CardAuthentication: CardAuthentication::<Impl, IMPL_OFFSET>,
            SupportedEncryptionAlgorithms: SupportedEncryptionAlgorithms::<Impl, IMPL_OFFSET>,
            AuthenticationLevel: AuthenticationLevel::<Impl, IMPL_OFFSET>,
            IsIsoSupported: IsIsoSupported::<Impl, IMPL_OFFSET>,
            IsJisOneSupported: IsJisOneSupported::<Impl, IMPL_OFFSET>,
            IsJisTwoSupported: IsJisTwoSupported::<Impl, IMPL_OFFSET>,
            PowerReportingType: PowerReportingType::<Impl, IMPL_OFFSET>,
            IsStatisticsReportingSupported: IsStatisticsReportingSupported::<Impl, IMPL_OFFSET>,
            IsStatisticsUpdatingSupported: IsStatisticsUpdatingSupported::<Impl, IMPL_OFFSET>,
            IsTrackDataMaskingSupported: IsTrackDataMaskingSupported::<Impl, IMPL_OFFSET>,
            IsTransmitSentinelsSupported: IsTransmitSentinelsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderCardTypesStaticsImpl: Sized {
    fn Unknown(&mut self) -> ::windows::core::Result<u32>;
    fn Bank(&mut self) -> ::windows::core::Result<u32>;
    fn Aamva(&mut self) -> ::windows::core::Result<u32>;
    fn ExtendedBase(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderCardTypesStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderCardTypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderCardTypesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderCardTypesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderCardTypesStaticsVtbl {
        unsafe extern "system" fn Unknown<Impl: IMagneticStripeReaderCardTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unknown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bank<Impl: IMagneticStripeReaderCardTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bank() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Aamva<Impl: IMagneticStripeReaderCardTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Aamva() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedBase<Impl: IMagneticStripeReaderCardTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedBase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderCardTypesStatics, BASE_OFFSET>(),
            Unknown: Unknown::<Impl, IMPL_OFFSET>,
            Bank: Bank::<Impl, IMPL_OFFSET>,
            Aamva: Aamva::<Impl, IMPL_OFFSET>,
            ExtendedBase: ExtendedBase::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderCardTypesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderEncryptionAlgorithmsStaticsImpl: Sized {
    fn None(&mut self) -> ::windows::core::Result<u32>;
    fn TripleDesDukpt(&mut self) -> ::windows::core::Result<u32>;
    fn ExtendedBase(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderEncryptionAlgorithmsStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderEncryptionAlgorithmsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderEncryptionAlgorithmsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderEncryptionAlgorithmsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderEncryptionAlgorithmsStaticsVtbl {
        unsafe extern "system" fn None<Impl: IMagneticStripeReaderEncryptionAlgorithmsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).None() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TripleDesDukpt<Impl: IMagneticStripeReaderEncryptionAlgorithmsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TripleDesDukpt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedBase<Impl: IMagneticStripeReaderEncryptionAlgorithmsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedBase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderEncryptionAlgorithmsStatics, BASE_OFFSET>(),
            None: None::<Impl, IMPL_OFFSET>,
            TripleDesDukpt: TripleDesDukpt::<Impl, IMPL_OFFSET>,
            ExtendedBase: ExtendedBase::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderEncryptionAlgorithmsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderErrorOccurredEventArgsImpl: Sized {
    fn Track1Status(&mut self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn Track2Status(&mut self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn Track3Status(&mut self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn Track4Status(&mut self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn ErrorData(&mut self) -> ::windows::core::Result<UnifiedPosErrorData>;
    fn PartialInputData(&mut self) -> ::windows::core::Result<MagneticStripeReaderReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderErrorOccurredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderErrorOccurredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderErrorOccurredEventArgsVtbl {
        unsafe extern "system" fn Track1Status<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track1Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track2Status<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track2Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track3Status<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track3Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track4Status<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track4Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorData<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartialInputData<Impl: IMagneticStripeReaderErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PartialInputData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderErrorOccurredEventArgs, BASE_OFFSET>(),
            Track1Status: Track1Status::<Impl, IMPL_OFFSET>,
            Track2Status: Track2Status::<Impl, IMPL_OFFSET>,
            Track3Status: Track3Status::<Impl, IMPL_OFFSET>,
            Track4Status: Track4Status::<Impl, IMPL_OFFSET>,
            ErrorData: ErrorData::<Impl, IMPL_OFFSET>,
            PartialInputData: PartialInputData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderErrorOccurredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMagneticStripeReaderReportImpl: Sized {
    fn CardType(&mut self) -> ::windows::core::Result<u32>;
    fn Track1(&mut self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Track2(&mut self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Track3(&mut self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Track4(&mut self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn CardAuthenticationData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CardAuthenticationDataLength(&mut self) -> ::windows::core::Result<u32>;
    fn AdditionalSecurityInformation(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagneticStripeReaderReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderReport";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMagneticStripeReaderReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderReportVtbl {
        unsafe extern "system" fn CardType<Impl: IMagneticStripeReaderReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track1<Impl: IMagneticStripeReaderReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track2<Impl: IMagneticStripeReaderReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track3<Impl: IMagneticStripeReaderReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track4<Impl: IMagneticStripeReaderReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track4() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMagneticStripeReaderReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardAuthenticationData<Impl: IMagneticStripeReaderReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardAuthenticationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardAuthenticationDataLength<Impl: IMagneticStripeReaderReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardAuthenticationDataLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdditionalSecurityInformation<Impl: IMagneticStripeReaderReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdditionalSecurityInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderReport, BASE_OFFSET>(),
            CardType: CardType::<Impl, IMPL_OFFSET>,
            Track1: Track1::<Impl, IMPL_OFFSET>,
            Track2: Track2::<Impl, IMPL_OFFSET>,
            Track3: Track3::<Impl, IMPL_OFFSET>,
            Track4: Track4::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            CardAuthenticationData: CardAuthenticationData::<Impl, IMPL_OFFSET>,
            CardAuthenticationDataLength: CardAuthenticationDataLength::<Impl, IMPL_OFFSET>,
            AdditionalSecurityInformation: AdditionalSecurityInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMagneticStripeReaderStaticsImpl: Sized {
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagneticStripeReaderStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMagneticStripeReaderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderStaticsVtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: IMagneticStripeReaderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IMagneticStripeReaderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IMagneticStripeReaderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderStatics, BASE_OFFSET>(),
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&mut self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: IMagneticStripeReaderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderStatics2, BASE_OFFSET>(),
            GetDeviceSelectorWithConnectionTypes: GetDeviceSelectorWithConnectionTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderStatusUpdatedEventArgsImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<MagneticStripeReaderStatus>;
    fn ExtendedStatus(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderStatusUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderStatusUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderStatusUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderStatusUpdatedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IMagneticStripeReaderStatusUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IMagneticStripeReaderStatusUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderStatusUpdatedEventArgs, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedStatus: ExtendedStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderStatusUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMagneticStripeReaderTrackDataImpl: Sized {
    fn Data(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn DiscretionaryData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn EncryptedData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMagneticStripeReaderTrackData {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderTrackData";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMagneticStripeReaderTrackDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderTrackDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderTrackDataVtbl {
        unsafe extern "system" fn Data<Impl: IMagneticStripeReaderTrackDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscretionaryData<Impl: IMagneticStripeReaderTrackDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscretionaryData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptedData<Impl: IMagneticStripeReaderTrackDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptedData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderTrackData, BASE_OFFSET>(),
            Data: Data::<Impl, IMPL_OFFSET>,
            DiscretionaryData: DiscretionaryData::<Impl, IMPL_OFFSET>,
            EncryptedData: EncryptedData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderTrackData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgsImpl: Sized {
    fn Report(&mut self) -> ::windows::core::Result<MagneticStripeReaderReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgsVtbl {
        unsafe extern "system" fn Report<Impl: IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Report() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs, BASE_OFFSET>(),
            Report: Report::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPosPrinterImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&mut self) -> ::windows::core::Result<PosPrinterCapabilities>;
    fn SupportedCharacterSets(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn SupportedTypeFaces(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Status(&mut self) -> ::windows::core::Result<PosPrinterStatus>;
    fn ClaimPrinterAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedPosPrinter>>;
    fn CheckHealthAsync(&mut self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetStatisticsAsync(&mut self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn StatusUpdated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinter";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPosPrinterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterVtbl {
        unsafe extern "system" fn DeviceId<Impl: IPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Capabilities<Impl: IPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCharacterSets<Impl: IPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedCharacterSets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTypeFaces<Impl: IPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedTypeFaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClaimPrinterAsync<Impl: IPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClaimPrinterAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHealthAsync<Impl: IPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckHealthAsync(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatisticsAsync<Impl: IPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatisticsAsync(&*(&statisticscategories as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUpdated<Impl: IPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUpdated<Impl: IPosPrinterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinter, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
            SupportedCharacterSets: SupportedCharacterSets::<Impl, IMPL_OFFSET>,
            SupportedTypeFaces: SupportedTypeFaces::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            ClaimPrinterAsync: ClaimPrinterAsync::<Impl, IMPL_OFFSET>,
            CheckHealthAsync: CheckHealthAsync::<Impl, IMPL_OFFSET>,
            GetStatisticsAsync: GetStatisticsAsync::<Impl, IMPL_OFFSET>,
            StatusUpdated: StatusUpdated::<Impl, IMPL_OFFSET>,
            RemoveStatusUpdated: RemoveStatusUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPosPrinter2Impl: Sized {
    fn SupportedBarcodeSymbologies(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn GetFontProperty(&mut self, typeface: &::windows::core::HSTRING) -> ::windows::core::Result<PosPrinterFontProperty>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPosPrinter2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinter2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPosPrinter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinter2Vtbl {
        unsafe extern "system" fn SupportedBarcodeSymbologies<Impl: IPosPrinter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedBarcodeSymbologies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontProperty<Impl: IPosPrinter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeface: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontProperty(&*(&typeface as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinter2, BASE_OFFSET>(),
            SupportedBarcodeSymbologies: SupportedBarcodeSymbologies::<Impl, IMPL_OFFSET>,
            GetFontProperty: GetFontProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterCapabilitiesImpl: Sized {
    fn PowerReportingType(&mut self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn DefaultCharacterSet(&mut self) -> ::windows::core::Result<u32>;
    fn HasCoverSensor(&mut self) -> ::windows::core::Result<bool>;
    fn CanMapCharacterSet(&mut self) -> ::windows::core::Result<bool>;
    fn IsTransactionSupported(&mut self) -> ::windows::core::Result<bool>;
    fn Receipt(&mut self) -> ::windows::core::Result<ReceiptPrinterCapabilities>;
    fn Slip(&mut self) -> ::windows::core::Result<SlipPrinterCapabilities>;
    fn Journal(&mut self) -> ::windows::core::Result<JournalPrinterCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterCapabilitiesVtbl {
        unsafe extern "system" fn PowerReportingType<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerReportingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsReportingSupported<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatisticsReportingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStatisticsUpdatingSupported<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStatisticsUpdatingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultCharacterSet<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultCharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCoverSensor<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCoverSensor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMapCharacterSet<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMapCharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransactionSupported<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactionSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receipt<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receipt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Slip<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Slip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IPosPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterCapabilities, BASE_OFFSET>(),
            PowerReportingType: PowerReportingType::<Impl, IMPL_OFFSET>,
            IsStatisticsReportingSupported: IsStatisticsReportingSupported::<Impl, IMPL_OFFSET>,
            IsStatisticsUpdatingSupported: IsStatisticsUpdatingSupported::<Impl, IMPL_OFFSET>,
            DefaultCharacterSet: DefaultCharacterSet::<Impl, IMPL_OFFSET>,
            HasCoverSensor: HasCoverSensor::<Impl, IMPL_OFFSET>,
            CanMapCharacterSet: CanMapCharacterSet::<Impl, IMPL_OFFSET>,
            IsTransactionSupported: IsTransactionSupported::<Impl, IMPL_OFFSET>,
            Receipt: Receipt::<Impl, IMPL_OFFSET>,
            Slip: Slip::<Impl, IMPL_OFFSET>,
            Journal: Journal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterCharacterSetIdsStaticsImpl: Sized {
    fn Utf16LE(&mut self) -> ::windows::core::Result<u32>;
    fn Ascii(&mut self) -> ::windows::core::Result<u32>;
    fn Ansi(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterCharacterSetIdsStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterCharacterSetIdsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterCharacterSetIdsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterCharacterSetIdsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterCharacterSetIdsStaticsVtbl {
        unsafe extern "system" fn Utf16LE<Impl: IPosPrinterCharacterSetIdsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Utf16LE() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ascii<Impl: IPosPrinterCharacterSetIdsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ascii() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ansi<Impl: IPosPrinterCharacterSetIdsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ansi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterCharacterSetIdsStatics, BASE_OFFSET>(),
            Utf16LE: Utf16LE::<Impl, IMPL_OFFSET>,
            Ascii: Ascii::<Impl, IMPL_OFFSET>,
            Ansi: Ansi::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterCharacterSetIdsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPosPrinterFontPropertyImpl: Sized {
    fn TypeFace(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsScalableToAnySize(&mut self) -> ::windows::core::Result<bool>;
    fn CharacterSizes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SizeUInt32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPosPrinterFontProperty {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterFontProperty";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPosPrinterFontPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterFontPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterFontPropertyVtbl {
        unsafe extern "system" fn TypeFace<Impl: IPosPrinterFontPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TypeFace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScalableToAnySize<Impl: IPosPrinterFontPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScalableToAnySize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacterSizes<Impl: IPosPrinterFontPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterSizes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterFontProperty, BASE_OFFSET>(),
            TypeFace: TypeFace::<Impl, IMPL_OFFSET>,
            IsScalableToAnySize: IsScalableToAnySize::<Impl, IMPL_OFFSET>,
            CharacterSizes: CharacterSizes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterFontProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPosPrinterJobImpl: Sized {
    fn Print(&mut self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrintLine(&mut self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrintNewline(&mut self) -> ::windows::core::Result<()>;
    fn ExecuteAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPosPrinterJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterJob";
}
#[cfg(feature = "Foundation")]
impl IPosPrinterJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterJobVtbl {
        unsafe extern "system" fn Print<Impl: IPosPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Print(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrintLine<Impl: IPosPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintLine(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrintNewline<Impl: IPosPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintNewline().into()
        }
        unsafe extern "system" fn ExecuteAsync<Impl: IPosPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecuteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterJob, BASE_OFFSET>(),
            Print: Print::<Impl, IMPL_OFFSET>,
            PrintLine: PrintLine::<Impl, IMPL_OFFSET>,
            PrintNewline: PrintNewline::<Impl, IMPL_OFFSET>,
            ExecuteAsync: ExecuteAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterJob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterPrintOptionsImpl: Sized {
    fn TypeFace(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTypeFace(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CharacterHeight(&mut self) -> ::windows::core::Result<u32>;
    fn SetCharacterHeight(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Bold(&mut self) -> ::windows::core::Result<bool>;
    fn SetBold(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Italic(&mut self) -> ::windows::core::Result<bool>;
    fn SetItalic(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Underline(&mut self) -> ::windows::core::Result<bool>;
    fn SetUnderline(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ReverseVideo(&mut self) -> ::windows::core::Result<bool>;
    fn SetReverseVideo(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Strikethrough(&mut self) -> ::windows::core::Result<bool>;
    fn SetStrikethrough(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Superscript(&mut self) -> ::windows::core::Result<bool>;
    fn SetSuperscript(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Subscript(&mut self) -> ::windows::core::Result<bool>;
    fn SetSubscript(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DoubleWide(&mut self) -> ::windows::core::Result<bool>;
    fn SetDoubleWide(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DoubleHigh(&mut self) -> ::windows::core::Result<bool>;
    fn SetDoubleHigh(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Alignment(&mut self) -> ::windows::core::Result<PosPrinterAlignment>;
    fn SetAlignment(&mut self, value: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn CharacterSet(&mut self) -> ::windows::core::Result<u32>;
    fn SetCharacterSet(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterPrintOptions {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterPrintOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterPrintOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterPrintOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterPrintOptionsVtbl {
        unsafe extern "system" fn TypeFace<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TypeFace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeFace<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTypeFace(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CharacterHeight<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterHeight<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharacterHeight(value).into()
        }
        unsafe extern "system" fn Bold<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBold(value).into()
        }
        unsafe extern "system" fn Italic<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Italic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItalic(value).into()
        }
        unsafe extern "system" fn Underline<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Underline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnderline(value).into()
        }
        unsafe extern "system" fn ReverseVideo<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReverseVideo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReverseVideo<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReverseVideo(value).into()
        }
        unsafe extern "system" fn Strikethrough<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strikethrough() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrikethrough(value).into()
        }
        unsafe extern "system" fn Superscript<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Superscript() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuperscript<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuperscript(value).into()
        }
        unsafe extern "system" fn Subscript<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subscript() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscript<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubscript(value).into()
        }
        unsafe extern "system" fn DoubleWide<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoubleWide() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoubleWide<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDoubleWide(value).into()
        }
        unsafe extern "system" fn DoubleHigh<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoubleHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoubleHigh<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDoubleHigh(value).into()
        }
        unsafe extern "system" fn Alignment<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Alignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlignment(value).into()
        }
        unsafe extern "system" fn CharacterSet<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterSet<Impl: IPosPrinterPrintOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharacterSet(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterPrintOptions, BASE_OFFSET>(),
            TypeFace: TypeFace::<Impl, IMPL_OFFSET>,
            SetTypeFace: SetTypeFace::<Impl, IMPL_OFFSET>,
            CharacterHeight: CharacterHeight::<Impl, IMPL_OFFSET>,
            SetCharacterHeight: SetCharacterHeight::<Impl, IMPL_OFFSET>,
            Bold: Bold::<Impl, IMPL_OFFSET>,
            SetBold: SetBold::<Impl, IMPL_OFFSET>,
            Italic: Italic::<Impl, IMPL_OFFSET>,
            SetItalic: SetItalic::<Impl, IMPL_OFFSET>,
            Underline: Underline::<Impl, IMPL_OFFSET>,
            SetUnderline: SetUnderline::<Impl, IMPL_OFFSET>,
            ReverseVideo: ReverseVideo::<Impl, IMPL_OFFSET>,
            SetReverseVideo: SetReverseVideo::<Impl, IMPL_OFFSET>,
            Strikethrough: Strikethrough::<Impl, IMPL_OFFSET>,
            SetStrikethrough: SetStrikethrough::<Impl, IMPL_OFFSET>,
            Superscript: Superscript::<Impl, IMPL_OFFSET>,
            SetSuperscript: SetSuperscript::<Impl, IMPL_OFFSET>,
            Subscript: Subscript::<Impl, IMPL_OFFSET>,
            SetSubscript: SetSubscript::<Impl, IMPL_OFFSET>,
            DoubleWide: DoubleWide::<Impl, IMPL_OFFSET>,
            SetDoubleWide: SetDoubleWide::<Impl, IMPL_OFFSET>,
            DoubleHigh: DoubleHigh::<Impl, IMPL_OFFSET>,
            SetDoubleHigh: SetDoubleHigh::<Impl, IMPL_OFFSET>,
            Alignment: Alignment::<Impl, IMPL_OFFSET>,
            SetAlignment: SetAlignment::<Impl, IMPL_OFFSET>,
            CharacterSet: CharacterSet::<Impl, IMPL_OFFSET>,
            SetCharacterSet: SetCharacterSet::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterPrintOptions as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterReleaseDeviceRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterReleaseDeviceRequestedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterReleaseDeviceRequestedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterReleaseDeviceRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPosPrinterStaticsImpl: Sized {
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPosPrinterStatics {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPosPrinterStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterStaticsVtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: IPosPrinterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IPosPrinterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IPosPrinterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterStatics, BASE_OFFSET>(),
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&mut self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterStatics2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelectorWithConnectionTypes<Impl: IPosPrinterStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithConnectionTypes(connectiontypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterStatics2, BASE_OFFSET>(),
            GetDeviceSelectorWithConnectionTypes: GetDeviceSelectorWithConnectionTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterStatusImpl: Sized {
    fn StatusKind(&mut self) -> ::windows::core::Result<PosPrinterStatusKind>;
    fn ExtendedStatus(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterStatus";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterStatusVtbl {
        unsafe extern "system" fn StatusKind<Impl: IPosPrinterStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterStatusKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IPosPrinterStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterStatus, BASE_OFFSET>(),
            StatusKind: StatusKind::<Impl, IMPL_OFFSET>,
            ExtendedStatus: ExtendedStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterStatusUpdatedEventArgsImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<PosPrinterStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPosPrinterStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterStatusUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPosPrinterStatusUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPosPrinterStatusUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPosPrinterStatusUpdatedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IPosPrinterStatusUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPosPrinterStatusUpdatedEventArgs, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPosPrinterStatusUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
pub trait IReceiptOrSlipJobImpl: Sized + IPosPrinterJobImpl {
    fn SetBarcodeRotation(&mut self, value: PosPrinterRotation) -> ::windows::core::Result<()>;
    fn SetPrintRotation(&mut self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()>;
    fn SetPrintArea(&mut self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetBitmap(&mut self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn SetBitmapCustomWidthStandardAlign(&mut self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()>;
    fn SetCustomAlignedBitmap(&mut self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn SetBitmapCustomWidthCustomAlign(&mut self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()>;
    fn PrintSavedBitmap(&mut self, bitmapnumber: u32) -> ::windows::core::Result<()>;
    fn DrawRuledLine(&mut self, positionlist: &::windows::core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()>;
    fn PrintBarcode(&mut self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn PrintBarcodeCustomAlign(&mut self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn PrintBitmap(&mut self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn PrintBitmapCustomWidthStandardAlign(&mut self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()>;
    fn PrintCustomAlignedBitmap(&mut self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn PrintBitmapCustomWidthCustomAlign(&mut self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
impl ::windows::core::RuntimeName for IReceiptOrSlipJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptOrSlipJob";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
impl IReceiptOrSlipJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReceiptOrSlipJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReceiptOrSlipJobVtbl {
        unsafe extern "system" fn SetBarcodeRotation<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PosPrinterRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBarcodeRotation(value).into()
        }
        unsafe extern "system" fn SetPrintRotation<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintRotation(value, includebitmaps).into()
        }
        unsafe extern "system" fn SetPrintArea<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintArea(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetBitmap<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmap(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthStandardAlign<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmapCustomWidthStandardAlign(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment, width).into()
        }
        unsafe extern "system" fn SetCustomAlignedBitmap<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomAlignedBitmap(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthCustomAlign<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmapCustomWidthCustomAlign(bitmapnumber, &*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance, width).into()
        }
        unsafe extern "system" fn PrintSavedBitmap<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintSavedBitmap(bitmapnumber).into()
        }
        unsafe extern "system" fn DrawRuledLine<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, positionlist: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawRuledLine(&*(&positionlist as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), linedirection, linewidth, linestyle, linecolor).into()
        }
        unsafe extern "system" fn PrintBarcode<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintBarcode(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), symbology, height, width, textposition, alignment).into()
        }
        unsafe extern "system" fn PrintBarcodeCustomAlign<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintBarcodeCustomAlign(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), symbology, height, width, textposition, alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmap<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintBitmap(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthStandardAlign<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintBitmapCustomWidthStandardAlign(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignment, width).into()
        }
        unsafe extern "system" fn PrintCustomAlignedBitmap<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignmentdistance: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintCustomAlignedBitmap(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthCustomAlign<Impl: IReceiptOrSlipJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrintBitmapCustomWidthCustomAlign(&*(&bitmap as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapFrame as ::windows::core::DefaultType>::DefaultType), alignmentdistance, width).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReceiptOrSlipJob, BASE_OFFSET>(),
            SetBarcodeRotation: SetBarcodeRotation::<Impl, IMPL_OFFSET>,
            SetPrintRotation: SetPrintRotation::<Impl, IMPL_OFFSET>,
            SetPrintArea: SetPrintArea::<Impl, IMPL_OFFSET>,
            SetBitmap: SetBitmap::<Impl, IMPL_OFFSET>,
            SetBitmapCustomWidthStandardAlign: SetBitmapCustomWidthStandardAlign::<Impl, IMPL_OFFSET>,
            SetCustomAlignedBitmap: SetCustomAlignedBitmap::<Impl, IMPL_OFFSET>,
            SetBitmapCustomWidthCustomAlign: SetBitmapCustomWidthCustomAlign::<Impl, IMPL_OFFSET>,
            PrintSavedBitmap: PrintSavedBitmap::<Impl, IMPL_OFFSET>,
            DrawRuledLine: DrawRuledLine::<Impl, IMPL_OFFSET>,
            PrintBarcode: PrintBarcode::<Impl, IMPL_OFFSET>,
            PrintBarcodeCustomAlign: PrintBarcodeCustomAlign::<Impl, IMPL_OFFSET>,
            PrintBitmap: PrintBitmap::<Impl, IMPL_OFFSET>,
            PrintBitmapCustomWidthStandardAlign: PrintBitmapCustomWidthStandardAlign::<Impl, IMPL_OFFSET>,
            PrintCustomAlignedBitmap: PrintCustomAlignedBitmap::<Impl, IMPL_OFFSET>,
            PrintBitmapCustomWidthCustomAlign: PrintBitmapCustomWidthCustomAlign::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReceiptOrSlipJob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrintJobImpl: Sized {
    fn MarkFeed(&mut self, kind: PosPrinterMarkFeedKind) -> ::windows::core::Result<()>;
    fn CutPaper(&mut self, percentage: f64) -> ::windows::core::Result<()>;
    fn CutPaperDefault(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReceiptPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptPrintJob";
}
#[cfg(feature = "implement_exclusive")]
impl IReceiptPrintJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReceiptPrintJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReceiptPrintJobVtbl {
        unsafe extern "system" fn MarkFeed<Impl: IReceiptPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: PosPrinterMarkFeedKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MarkFeed(kind).into()
        }
        unsafe extern "system" fn CutPaper<Impl: IReceiptPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, percentage: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CutPaper(percentage).into()
        }
        unsafe extern "system" fn CutPaperDefault<Impl: IReceiptPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CutPaperDefault().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReceiptPrintJob, BASE_OFFSET>(),
            MarkFeed: MarkFeed::<Impl, IMPL_OFFSET>,
            CutPaper: CutPaper::<Impl, IMPL_OFFSET>,
            CutPaperDefault: CutPaperDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReceiptPrintJob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrintJob2Impl: Sized {
    fn StampPaper(&mut self) -> ::windows::core::Result<()>;
    fn Print(&mut self, data: &::windows::core::HSTRING, printoptions: &::core::option::Option<PosPrinterPrintOptions>) -> ::windows::core::Result<()>;
    fn FeedPaperByLine(&mut self, linecount: i32) -> ::windows::core::Result<()>;
    fn FeedPaperByMapModeUnit(&mut self, distance: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReceiptPrintJob2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptPrintJob2";
}
#[cfg(feature = "implement_exclusive")]
impl IReceiptPrintJob2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReceiptPrintJob2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReceiptPrintJob2Vtbl {
        unsafe extern "system" fn StampPaper<Impl: IReceiptPrintJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StampPaper().into()
        }
        unsafe extern "system" fn Print<Impl: IReceiptPrintJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Print(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&printoptions as *const <PosPrinterPrintOptions as ::windows::core::Abi>::Abi as *const <PosPrinterPrintOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FeedPaperByLine<Impl: IReceiptPrintJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedPaperByLine(linecount).into()
        }
        unsafe extern "system" fn FeedPaperByMapModeUnit<Impl: IReceiptPrintJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedPaperByMapModeUnit(distance).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReceiptPrintJob2, BASE_OFFSET>(),
            StampPaper: StampPaper::<Impl, IMPL_OFFSET>,
            Print: Print::<Impl, IMPL_OFFSET>,
            FeedPaperByLine: FeedPaperByLine::<Impl, IMPL_OFFSET>,
            FeedPaperByMapModeUnit: FeedPaperByMapModeUnit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReceiptPrintJob2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrinterCapabilitiesImpl: Sized {
    fn CanCutPaper(&mut self) -> ::windows::core::Result<bool>;
    fn IsStampSupported(&mut self) -> ::windows::core::Result<bool>;
    fn MarkFeedCapabilities(&mut self) -> ::windows::core::Result<PosPrinterMarkFeedCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReceiptPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptPrinterCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IReceiptPrinterCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReceiptPrinterCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReceiptPrinterCapabilitiesVtbl {
        unsafe extern "system" fn CanCutPaper<Impl: IReceiptPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCutPaper() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStampSupported<Impl: IReceiptPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStampSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkFeedCapabilities<Impl: IReceiptPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterMarkFeedCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkFeedCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReceiptPrinterCapabilities, BASE_OFFSET>(),
            CanCutPaper: CanCutPaper::<Impl, IMPL_OFFSET>,
            IsStampSupported: IsStampSupported::<Impl, IMPL_OFFSET>,
            MarkFeedCapabilities: MarkFeedCapabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReceiptPrinterCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrinterCapabilities2Impl: Sized {
    fn IsReverseVideoSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsStrikethroughSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsSuperscriptSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsSubscriptSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByLineSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByMapModeUnitSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReceiptPrinterCapabilities2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptPrinterCapabilities2";
}
#[cfg(feature = "implement_exclusive")]
impl IReceiptPrinterCapabilities2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReceiptPrinterCapabilities2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReceiptPrinterCapabilities2Vtbl {
        unsafe extern "system" fn IsReverseVideoSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReverseVideoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStrikethroughSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStrikethroughSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSuperscriptSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSuperscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscriptSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSubscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByLineSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByLineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByMapModeUnitSupported<Impl: IReceiptPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByMapModeUnitSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReceiptPrinterCapabilities2, BASE_OFFSET>(),
            IsReverseVideoSupported: IsReverseVideoSupported::<Impl, IMPL_OFFSET>,
            IsStrikethroughSupported: IsStrikethroughSupported::<Impl, IMPL_OFFSET>,
            IsSuperscriptSupported: IsSuperscriptSupported::<Impl, IMPL_OFFSET>,
            IsSubscriptSupported: IsSubscriptSupported::<Impl, IMPL_OFFSET>,
            IsReversePaperFeedByLineSupported: IsReversePaperFeedByLineSupported::<Impl, IMPL_OFFSET>,
            IsReversePaperFeedByMapModeUnitSupported: IsReversePaperFeedByMapModeUnitSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReceiptPrinterCapabilities2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlipPrintJobImpl: Sized {
    fn Print(&mut self, data: &::windows::core::HSTRING, printoptions: &::core::option::Option<PosPrinterPrintOptions>) -> ::windows::core::Result<()>;
    fn FeedPaperByLine(&mut self, linecount: i32) -> ::windows::core::Result<()>;
    fn FeedPaperByMapModeUnit(&mut self, distance: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlipPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.ISlipPrintJob";
}
#[cfg(feature = "implement_exclusive")]
impl ISlipPrintJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISlipPrintJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISlipPrintJobVtbl {
        unsafe extern "system" fn Print<Impl: ISlipPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Print(&*(&data as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&printoptions as *const <PosPrinterPrintOptions as ::windows::core::Abi>::Abi as *const <PosPrinterPrintOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FeedPaperByLine<Impl: ISlipPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedPaperByLine(linecount).into()
        }
        unsafe extern "system" fn FeedPaperByMapModeUnit<Impl: ISlipPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedPaperByMapModeUnit(distance).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISlipPrintJob, BASE_OFFSET>(),
            Print: Print::<Impl, IMPL_OFFSET>,
            FeedPaperByLine: FeedPaperByLine::<Impl, IMPL_OFFSET>,
            FeedPaperByMapModeUnit: FeedPaperByMapModeUnit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISlipPrintJob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlipPrinterCapabilitiesImpl: Sized {
    fn IsFullLengthSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsBothSidesPrintingSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlipPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ISlipPrinterCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl ISlipPrinterCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISlipPrinterCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISlipPrinterCapabilitiesVtbl {
        unsafe extern "system" fn IsFullLengthSupported<Impl: ISlipPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFullLengthSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBothSidesPrintingSupported<Impl: ISlipPrinterCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBothSidesPrintingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISlipPrinterCapabilities, BASE_OFFSET>(),
            IsFullLengthSupported: IsFullLengthSupported::<Impl, IMPL_OFFSET>,
            IsBothSidesPrintingSupported: IsBothSidesPrintingSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISlipPrinterCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlipPrinterCapabilities2Impl: Sized {
    fn IsReverseVideoSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsStrikethroughSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsSuperscriptSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsSubscriptSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByLineSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByMapModeUnitSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlipPrinterCapabilities2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.ISlipPrinterCapabilities2";
}
#[cfg(feature = "implement_exclusive")]
impl ISlipPrinterCapabilities2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISlipPrinterCapabilities2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISlipPrinterCapabilities2Vtbl {
        unsafe extern "system" fn IsReverseVideoSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReverseVideoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStrikethroughSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStrikethroughSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSuperscriptSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSuperscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscriptSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSubscriptSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByLineSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByLineSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReversePaperFeedByMapModeUnitSupported<Impl: ISlipPrinterCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReversePaperFeedByMapModeUnitSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISlipPrinterCapabilities2, BASE_OFFSET>(),
            IsReverseVideoSupported: IsReverseVideoSupported::<Impl, IMPL_OFFSET>,
            IsStrikethroughSupported: IsStrikethroughSupported::<Impl, IMPL_OFFSET>,
            IsSuperscriptSupported: IsSuperscriptSupported::<Impl, IMPL_OFFSET>,
            IsSubscriptSupported: IsSubscriptSupported::<Impl, IMPL_OFFSET>,
            IsReversePaperFeedByLineSupported: IsReversePaperFeedByLineSupported::<Impl, IMPL_OFFSET>,
            IsReversePaperFeedByMapModeUnitSupported: IsReversePaperFeedByMapModeUnitSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISlipPrinterCapabilities2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnifiedPosErrorDataImpl: Sized {
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Severity(&mut self) -> ::windows::core::Result<UnifiedPosErrorSeverity>;
    fn Reason(&mut self) -> ::windows::core::Result<UnifiedPosErrorReason>;
    fn ExtendedReason(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnifiedPosErrorData {
    const NAME: &'static str = "Windows.Devices.PointOfService.IUnifiedPosErrorData";
}
#[cfg(feature = "implement_exclusive")]
impl IUnifiedPosErrorDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnifiedPosErrorDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnifiedPosErrorDataVtbl {
        unsafe extern "system" fn Message<Impl: IUnifiedPosErrorDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Severity<Impl: IUnifiedPosErrorDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosErrorSeverity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Severity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Impl: IUnifiedPosErrorDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosErrorReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedReason<Impl: IUnifiedPosErrorDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUnifiedPosErrorData, BASE_OFFSET>(),
            Message: Message::<Impl, IMPL_OFFSET>,
            Severity: Severity::<Impl, IMPL_OFFSET>,
            Reason: Reason::<Impl, IMPL_OFFSET>,
            ExtendedReason: ExtendedReason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnifiedPosErrorData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnifiedPosErrorDataFactoryImpl: Sized {
    fn CreateInstance(&mut self, message: &::windows::core::HSTRING, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32) -> ::windows::core::Result<UnifiedPosErrorData>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnifiedPosErrorDataFactory {
    const NAME: &'static str = "Windows.Devices.PointOfService.IUnifiedPosErrorDataFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUnifiedPosErrorDataFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnifiedPosErrorDataFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnifiedPosErrorDataFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IUnifiedPosErrorDataFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), severity, reason, extendedreason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUnifiedPosErrorDataFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnifiedPosErrorDataFactory as ::windows::core::Interface>::IID
    }
}
