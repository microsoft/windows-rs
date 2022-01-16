#[cfg(feature = "deprecated")]
pub trait INDClosedCaptionDataReceivedEventArgs_Impl: Sized {
    fn ClosedCaptionDataFormat(&mut self) -> ::windows::core::Result<NDClosedCaptionFormat>;
    fn PresentationTimestamp(&mut self) -> ::windows::core::Result<i64>;
    fn ClosedCaptionData(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDClosedCaptionDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDClosedCaptionDataReceivedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDClosedCaptionDataReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDClosedCaptionDataReceivedEventArgs_Vtbl {
        unsafe extern "system" fn ClosedCaptionDataFormat<Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDClosedCaptionFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedCaptionDataFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationTimestamp<Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedCaptionData<Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedCaptionData() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDClosedCaptionDataReceivedEventArgs, BASE_OFFSET>(),
            ClosedCaptionDataFormat: ClosedCaptionDataFormat::<Impl, IMPL_OFFSET>,
            PresentationTimestamp: PresentationTimestamp::<Impl, IMPL_OFFSET>,
            ClosedCaptionData: ClosedCaptionData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDClosedCaptionDataReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDCustomData_Impl: Sized {
    fn CustomDataTypeID(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn CustomData(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDCustomData";
}
#[cfg(feature = "deprecated")]
impl INDCustomData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDCustomData_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDCustomData_Vtbl {
        unsafe extern "system" fn CustomDataTypeID<Impl: INDCustomData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomDataTypeID() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomData<Impl: INDCustomData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomData() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDCustomData, BASE_OFFSET>(),
            CustomDataTypeID: CustomDataTypeID::<Impl, IMPL_OFFSET>,
            CustomData: CustomData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDCustomData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait INDDownloadEngine_Impl: Sized {
    fn Open(&mut self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Seek(&mut self, startposition: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CanSeek(&mut self) -> ::windows::core::Result<bool>;
    fn BufferFullMinThresholdInSamples(&mut self) -> ::windows::core::Result<u32>;
    fn BufferFullMaxThresholdInSamples(&mut self) -> ::windows::core::Result<u32>;
    fn Notifier(&mut self) -> ::windows::core::Result<NDDownloadEngineNotifier>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDDownloadEngine {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDDownloadEngine";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl INDDownloadEngine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngine_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDDownloadEngine_Vtbl {
        unsafe extern "system" fn Open<Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, sessionIDBytes_array_size: u32, sessionidbytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(&*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as _)).into()
        }
        unsafe extern "system" fn Pause<Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Close<Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Seek<Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startposition: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Seek(&*(&startposition as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanSeek<Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSeek() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferFullMinThresholdInSamples<Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferFullMinThresholdInSamples() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferFullMaxThresholdInSamples<Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferFullMaxThresholdInSamples() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notifier<Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDDownloadEngine, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
            CanSeek: CanSeek::<Impl, IMPL_OFFSET>,
            BufferFullMinThresholdInSamples: BufferFullMinThresholdInSamples::<Impl, IMPL_OFFSET>,
            BufferFullMaxThresholdInSamples: BufferFullMaxThresholdInSamples::<Impl, IMPL_OFFSET>,
            Notifier: Notifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDDownloadEngine as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDDownloadEngineNotifier_Impl: Sized {
    fn OnStreamOpened(&mut self) -> ::windows::core::Result<()>;
    fn OnPlayReadyObjectReceived(&mut self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn OnContentIDReceived(&mut self, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<()>;
    fn OnDataReceived(&mut self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], bytesreceived: u32) -> ::windows::core::Result<()>;
    fn OnEndOfStream(&mut self) -> ::windows::core::Result<()>;
    fn OnNetworkError(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDDownloadEngineNotifier";
}
#[cfg(feature = "deprecated")]
impl INDDownloadEngineNotifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngineNotifier_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDDownloadEngineNotifier_Vtbl {
        unsafe extern "system" fn OnStreamOpened<Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStreamOpened().into()
        }
        unsafe extern "system" fn OnPlayReadyObjectReceived<Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPlayReadyObjectReceived(::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as _)).into()
        }
        unsafe extern "system" fn OnContentIDReceived<Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, licensefetchdescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnContentIDReceived(&*(&licensefetchdescriptor as *const <INDLicenseFetchDescriptor as ::windows::core::Abi>::Abi as *const <INDLicenseFetchDescriptor as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnDataReceived<Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDataReceived(::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as _), bytesreceived).into()
        }
        unsafe extern "system" fn OnEndOfStream<Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEndOfStream().into()
        }
        unsafe extern "system" fn OnNetworkError<Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNetworkError().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDDownloadEngineNotifier, BASE_OFFSET>(),
            OnStreamOpened: OnStreamOpened::<Impl, IMPL_OFFSET>,
            OnPlayReadyObjectReceived: OnPlayReadyObjectReceived::<Impl, IMPL_OFFSET>,
            OnContentIDReceived: OnContentIDReceived::<Impl, IMPL_OFFSET>,
            OnDataReceived: OnDataReceived::<Impl, IMPL_OFFSET>,
            OnEndOfStream: OnEndOfStream::<Impl, IMPL_OFFSET>,
            OnNetworkError: OnNetworkError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDDownloadEngineNotifier as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchCompletedEventArgs_Impl: Sized {
    fn ResponseCustomData(&mut self) -> ::windows::core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDLicenseFetchCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchCompletedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDLicenseFetchCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDLicenseFetchCompletedEventArgs_Vtbl {
        unsafe extern "system" fn ResponseCustomData<Impl: INDLicenseFetchCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseCustomData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDLicenseFetchCompletedEventArgs, BASE_OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDLicenseFetchCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchDescriptor_Impl: Sized {
    fn ContentIDType(&mut self) -> ::windows::core::Result<NDContentIDType>;
    fn ContentID(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn LicenseFetchChallengeCustomData(&mut self) -> ::windows::core::Result<INDCustomData>;
    fn SetLicenseFetchChallengeCustomData(&mut self, licensefetchchallengecustomdata: &::core::option::Option<INDCustomData>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDLicenseFetchDescriptor {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptor";
}
#[cfg(feature = "deprecated")]
impl INDLicenseFetchDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchDescriptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDLicenseFetchDescriptor_Vtbl {
        unsafe extern "system" fn ContentIDType<Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDContentIDType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentIDType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentID<Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentID() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LicenseFetchChallengeCustomData<Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseFetchChallengeCustomData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLicenseFetchChallengeCustomData<Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, licensefetchchallengecustomdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLicenseFetchChallengeCustomData(&*(&licensefetchchallengecustomdata as *const <INDCustomData as ::windows::core::Abi>::Abi as *const <INDCustomData as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDLicenseFetchDescriptor, BASE_OFFSET>(),
            ContentIDType: ContentIDType::<Impl, IMPL_OFFSET>,
            ContentID: ContentID::<Impl, IMPL_OFFSET>,
            LicenseFetchChallengeCustomData: LicenseFetchChallengeCustomData::<Impl, IMPL_OFFSET>,
            SetLicenseFetchChallengeCustomData: SetLicenseFetchChallengeCustomData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDLicenseFetchDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchResult_Impl: Sized {
    fn ResponseCustomData(&mut self) -> ::windows::core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDLicenseFetchResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchResult";
}
#[cfg(feature = "deprecated")]
impl INDLicenseFetchResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDLicenseFetchResult_Vtbl {
        unsafe extern "system" fn ResponseCustomData<Impl: INDLicenseFetchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseCustomData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDLicenseFetchResult, BASE_OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDLicenseFetchResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait INDMessenger_Impl: Sized {
    fn SendRegistrationRequestAsync(&mut self, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionStartAsync(&mut self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionResponseAsync(&mut self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], responsedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendLicenseFetchRequestAsync(&mut self, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDMessenger";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl INDMessenger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDMessenger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDMessenger_Vtbl {
        unsafe extern "system" fn SendRegistrationRequestAsync<Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendRegistrationRequestAsync(::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&challengedatabytes), challengeDataBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendProximityDetectionStartAsync<Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendProximityDetectionStartAsync(pdtype, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&transmitterchannelbytes), transmitterChannelBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&challengedatabytes), challengeDataBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendProximityDetectionResponseAsync<Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, responseDataBytes_array_size: u32, responsedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendProximityDetectionResponseAsync(pdtype, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&transmitterchannelbytes), transmitterChannelBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&responsedatabytes), responseDataBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendLicenseFetchRequestAsync<Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendLicenseFetchRequestAsync(::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&challengedatabytes), challengeDataBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDMessenger, BASE_OFFSET>(),
            SendRegistrationRequestAsync: SendRegistrationRequestAsync::<Impl, IMPL_OFFSET>,
            SendProximityDetectionStartAsync: SendProximityDetectionStartAsync::<Impl, IMPL_OFFSET>,
            SendProximityDetectionResponseAsync: SendProximityDetectionResponseAsync::<Impl, IMPL_OFFSET>,
            SendLicenseFetchRequestAsync: SendLicenseFetchRequestAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDMessenger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDProximityDetectionCompletedEventArgs_Impl: Sized {
    fn ProximityDetectionRetryCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDProximityDetectionCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDProximityDetectionCompletedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDProximityDetectionCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDProximityDetectionCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDProximityDetectionCompletedEventArgs_Vtbl {
        unsafe extern "system" fn ProximityDetectionRetryCount<Impl: INDProximityDetectionCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProximityDetectionRetryCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDProximityDetectionCompletedEventArgs, BASE_OFFSET>(),
            ProximityDetectionRetryCount: ProximityDetectionRetryCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDProximityDetectionCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDRegistrationCompletedEventArgs_Impl: Sized {
    fn ResponseCustomData(&mut self) -> ::windows::core::Result<INDCustomData>;
    fn TransmitterProperties(&mut self) -> ::windows::core::Result<INDTransmitterProperties>;
    fn TransmitterCertificateAccepted(&mut self) -> ::windows::core::Result<bool>;
    fn SetTransmitterCertificateAccepted(&mut self, accept: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDRegistrationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDRegistrationCompletedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDRegistrationCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDRegistrationCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDRegistrationCompletedEventArgs_Vtbl {
        unsafe extern "system" fn ResponseCustomData<Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseCustomData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmitterProperties<Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmitterProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmitterCertificateAccepted<Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmitterCertificateAccepted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransmitterCertificateAccepted<Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accept: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransmitterCertificateAccepted(accept).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDRegistrationCompletedEventArgs, BASE_OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Impl, IMPL_OFFSET>,
            TransmitterProperties: TransmitterProperties::<Impl, IMPL_OFFSET>,
            TransmitterCertificateAccepted: TransmitterCertificateAccepted::<Impl, IMPL_OFFSET>,
            SetTransmitterCertificateAccepted: SetTransmitterCertificateAccepted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDRegistrationCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDSendResult_Impl: Sized {
    fn Response(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDSendResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDSendResult";
}
#[cfg(feature = "deprecated")]
impl INDSendResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDSendResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDSendResult_Vtbl {
        unsafe extern "system" fn Response<Impl: INDSendResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Response() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INDSendResult, BASE_OFFSET>(), Response: Response::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDSendResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
pub trait INDStartResult_Impl: Sized {
    fn MediaStreamSource(&mut self) -> ::windows::core::Result<super::super::Core::MediaStreamSource>;
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDStartResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStartResult";
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl INDStartResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStartResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDStartResult_Vtbl {
        unsafe extern "system" fn MediaStreamSource<Impl: INDStartResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaStreamSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDStartResult, BASE_OFFSET>(),
            MediaStreamSource: MediaStreamSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDStartResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait INDStorageFileHelper_Impl: Sized {
    fn GetFileURLs(&mut self, file: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStorageFileHelper";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl INDStorageFileHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStorageFileHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDStorageFileHelper_Vtbl {
        unsafe extern "system" fn GetFileURLs<Impl: INDStorageFileHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileURLs(&*(&file as *const <super::super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INDStorageFileHelper, BASE_OFFSET>(), GetFileURLs: GetFileURLs::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDStorageFileHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
pub trait INDStreamParser_Impl: Sized {
    fn ParseData(&mut self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetStreamInformation(&mut self, descriptor: &::core::option::Option<super::super::Core::IMediaStreamDescriptor>, streamtype: &mut NDMediaStreamType) -> ::windows::core::Result<u32>;
    fn BeginOfStream(&mut self) -> ::windows::core::Result<()>;
    fn EndOfStream(&mut self) -> ::windows::core::Result<()>;
    fn Notifier(&mut self) -> ::windows::core::Result<NDStreamParserNotifier>;
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDStreamParser {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStreamParser";
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl INDStreamParser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParser_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDStreamParser_Vtbl {
        unsafe extern "system" fn ParseData<Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseData(::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as _)).into()
        }
        unsafe extern "system" fn GetStreamInformation<Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamInformation(&*(&descriptor as *const <super::super::Core::IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <super::super::Core::IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&streamtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginOfStream<Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginOfStream().into()
        }
        unsafe extern "system" fn EndOfStream<Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndOfStream().into()
        }
        unsafe extern "system" fn Notifier<Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDStreamParser, BASE_OFFSET>(),
            ParseData: ParseData::<Impl, IMPL_OFFSET>,
            GetStreamInformation: GetStreamInformation::<Impl, IMPL_OFFSET>,
            BeginOfStream: BeginOfStream::<Impl, IMPL_OFFSET>,
            EndOfStream: EndOfStream::<Impl, IMPL_OFFSET>,
            Notifier: Notifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDStreamParser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
pub trait INDStreamParserNotifier_Impl: Sized {
    fn OnContentIDReceived(&mut self, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<()>;
    fn OnMediaStreamDescriptorCreated(&mut self, audiostreamdescriptors: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, videostreamdescriptors: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn OnSampleParsed(&mut self, streamid: u32, streamtype: NDMediaStreamType, streamsample: &::core::option::Option<super::super::Core::MediaStreamSample>, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn OnBeginSetupDecryptor(&mut self, descriptor: &::core::option::Option<super::super::Core::IMediaStreamDescriptor>, keyid: &::windows::core::GUID, probytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStreamParserNotifier";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
impl INDStreamParserNotifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParserNotifier_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDStreamParserNotifier_Vtbl {
        unsafe extern "system" fn OnContentIDReceived<Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, licensefetchdescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnContentIDReceived(&*(&licensefetchdescriptor as *const <INDLicenseFetchDescriptor as ::windows::core::Abi>::Abi as *const <INDLicenseFetchDescriptor as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnMediaStreamDescriptorCreated<Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostreamdescriptors: ::windows::core::RawPtr, videostreamdescriptors: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .OnMediaStreamDescriptorCreated(
                    &*(&audiostreamdescriptors as *const <super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor> as ::windows::core::DefaultType>::DefaultType),
                    &*(&videostreamdescriptors as *const <super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor> as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn OnSampleParsed<Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamid: u32, streamtype: NDMediaStreamType, streamsample: ::windows::core::RawPtr, pts: i64, ccformat: NDClosedCaptionFormat, ccDataBytes_array_size: u32, ccdatabytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSampleParsed(streamid, streamtype, &*(&streamsample as *const <super::super::Core::MediaStreamSample as ::windows::core::Abi>::Abi as *const <super::super::Core::MediaStreamSample as ::windows::core::DefaultType>::DefaultType), pts, ccformat, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&ccdatabytes), ccDataBytes_array_size as _)).into()
        }
        unsafe extern "system" fn OnBeginSetupDecryptor<Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, keyid: ::windows::core::GUID, proBytes_array_size: u32, probytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .OnBeginSetupDecryptor(&*(&descriptor as *const <super::super::Core::IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <super::super::Core::IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType), &*(&keyid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&probytes), proBytes_array_size as _))
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDStreamParserNotifier, BASE_OFFSET>(),
            OnContentIDReceived: OnContentIDReceived::<Impl, IMPL_OFFSET>,
            OnMediaStreamDescriptorCreated: OnMediaStreamDescriptorCreated::<Impl, IMPL_OFFSET>,
            OnSampleParsed: OnSampleParsed::<Impl, IMPL_OFFSET>,
            OnBeginSetupDecryptor: OnBeginSetupDecryptor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDStreamParserNotifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait INDTransmitterProperties_Impl: Sized {
    fn CertificateType(&mut self) -> ::windows::core::Result<NDCertificateType>;
    fn PlatformIdentifier(&mut self) -> ::windows::core::Result<NDCertificatePlatformID>;
    fn SupportedFeatures(&mut self) -> ::windows::core::Result<::windows::core::Array<NDCertificateFeature>>;
    fn SecurityLevel(&mut self) -> ::windows::core::Result<u32>;
    fn SecurityVersion(&mut self) -> ::windows::core::Result<u32>;
    fn ExpirationDate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ClientID(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn ModelDigest(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn ModelManufacturerName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDTransmitterProperties {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDTransmitterProperties";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl INDTransmitterProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDTransmitterProperties_Vtbl {
        unsafe extern "system" fn CertificateType<Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDCertificateType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CertificateType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlatformIdentifier<Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDCertificatePlatformID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlatformIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFeatures<Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedFeatures() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityLevel<Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecurityLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityVersion<Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecurityVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClientID<Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientID() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelDigest<Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelDigest() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelManufacturerName<Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelManufacturerName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelName<Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumber<Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDTransmitterProperties, BASE_OFFSET>(),
            CertificateType: CertificateType::<Impl, IMPL_OFFSET>,
            PlatformIdentifier: PlatformIdentifier::<Impl, IMPL_OFFSET>,
            SupportedFeatures: SupportedFeatures::<Impl, IMPL_OFFSET>,
            SecurityLevel: SecurityLevel::<Impl, IMPL_OFFSET>,
            SecurityVersion: SecurityVersion::<Impl, IMPL_OFFSET>,
            ExpirationDate: ExpirationDate::<Impl, IMPL_OFFSET>,
            ClientID: ClientID::<Impl, IMPL_OFFSET>,
            ModelDigest: ModelDigest::<Impl, IMPL_OFFSET>,
            ModelManufacturerName: ModelManufacturerName::<Impl, IMPL_OFFSET>,
            ModelName: ModelName::<Impl, IMPL_OFFSET>,
            ModelNumber: ModelNumber::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDTransmitterProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPlayReadyDomain_Impl: Sized {
    fn AccountId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ServiceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Revision(&mut self) -> ::windows::core::Result<u32>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DomainJoinUrl(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyDomain";
}
#[cfg(feature = "Foundation")]
impl IPlayReadyDomain_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyDomain_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayReadyDomain_Vtbl {
        unsafe extern "system" fn AccountId<Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceId<Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revision<Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainJoinUrl<Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainJoinUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyDomain, BASE_OFFSET>(),
            AccountId: AccountId::<Impl, IMPL_OFFSET>,
            ServiceId: ServiceId::<Impl, IMPL_OFFSET>,
            Revision: Revision::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            DomainJoinUrl: DomainJoinUrl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyDomain as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPlayReadyLicense_Impl: Sized {
    fn FullyEvaluated(&mut self) -> ::windows::core::Result<bool>;
    fn UsableForPlay(&mut self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ExpireAfterFirstPlay(&mut self) -> ::windows::core::Result<u32>;
    fn DomainAccountID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ChainDepth(&mut self) -> ::windows::core::Result<u32>;
    fn GetKIDAtChainDepth(&mut self, chaindepth: u32) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicense";
}
#[cfg(feature = "Foundation")]
impl IPlayReadyLicense_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicense_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayReadyLicense_Vtbl {
        unsafe extern "system" fn FullyEvaluated<Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullyEvaluated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsableForPlay<Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsableForPlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpireAfterFirstPlay<Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpireAfterFirstPlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainAccountID<Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainAccountID() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChainDepth<Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChainDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKIDAtChainDepth<Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chaindepth: u32, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKIDAtChainDepth(chaindepth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyLicense, BASE_OFFSET>(),
            FullyEvaluated: FullyEvaluated::<Impl, IMPL_OFFSET>,
            UsableForPlay: UsableForPlay::<Impl, IMPL_OFFSET>,
            ExpirationDate: ExpirationDate::<Impl, IMPL_OFFSET>,
            ExpireAfterFirstPlay: ExpireAfterFirstPlay::<Impl, IMPL_OFFSET>,
            DomainAccountID: DomainAccountID::<Impl, IMPL_OFFSET>,
            ChainDepth: ChainDepth::<Impl, IMPL_OFFSET>,
            GetKIDAtChainDepth: GetKIDAtChainDepth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyLicense as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPlayReadyLicenseAcquisitionServiceRequest_Impl: Sized + super::IMediaProtectionServiceRequest_Impl + IPlayReadyServiceRequest_Impl {
    fn ContentHeader(&mut self) -> ::windows::core::Result<PlayReadyContentHeader>;
    fn SetContentHeader(&mut self, value: &::core::option::Option<PlayReadyContentHeader>) -> ::windows::core::Result<()>;
    fn DomainServiceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainServiceId(&mut self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest";
}
#[cfg(feature = "Foundation")]
impl IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
        unsafe extern "system" fn ContentHeader<Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentHeader<Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentHeader(&*(&value as *const <PlayReadyContentHeader as ::windows::core::Abi>::Abi as *const <PlayReadyContentHeader as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DomainServiceId<Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainServiceId<Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainServiceId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyLicenseAcquisitionServiceRequest, BASE_OFFSET>(),
            ContentHeader: ContentHeader::<Impl, IMPL_OFFSET>,
            SetContentHeader: SetContentHeader::<Impl, IMPL_OFFSET>,
            DomainServiceId: DomainServiceId::<Impl, IMPL_OFFSET>,
            SetDomainServiceId: SetDomainServiceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyLicenseAcquisitionServiceRequest as ::windows::core::Interface>::IID
    }
}
pub trait IPlayReadyLicenseSession_Impl: Sized {
    fn CreateLAServiceRequest(&mut self) -> ::windows::core::Result<IPlayReadyLicenseAcquisitionServiceRequest>;
    fn ConfigureMediaProtectionManager(&mut self, mpm: &::core::option::Option<super::MediaProtectionManager>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession";
}
impl IPlayReadyLicenseSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayReadyLicenseSession_Vtbl {
        unsafe extern "system" fn CreateLAServiceRequest<Impl: IPlayReadyLicenseSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLAServiceRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureMediaProtectionManager<Impl: IPlayReadyLicenseSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mpm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureMediaProtectionManager(&*(&mpm as *const <super::MediaProtectionManager as ::windows::core::Abi>::Abi as *const <super::MediaProtectionManager as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyLicenseSession, BASE_OFFSET>(),
            CreateLAServiceRequest: CreateLAServiceRequest::<Impl, IMPL_OFFSET>,
            ConfigureMediaProtectionManager: ConfigureMediaProtectionManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyLicenseSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPlayReadyLicenseSession2_Impl: Sized + IPlayReadyLicenseSession_Impl {
    fn CreateLicenseIterable(&mut self, contentheader: &::core::option::Option<PlayReadyContentHeader>, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IPlayReadyLicenseSession2 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession2";
}
#[cfg(feature = "Foundation_Collections")]
impl IPlayReadyLicenseSession2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseSession2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayReadyLicenseSession2_Vtbl {
        unsafe extern "system" fn CreateLicenseIterable<Impl: IPlayReadyLicenseSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentheader: ::windows::core::RawPtr, fullyevaluated: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLicenseIterable(&*(&contentheader as *const <PlayReadyContentHeader as ::windows::core::Abi>::Abi as *const <PlayReadyContentHeader as ::windows::core::DefaultType>::DefaultType), fullyevaluated) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyLicenseSession2, BASE_OFFSET>(),
            CreateLicenseIterable: CreateLicenseIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyLicenseSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPlayReadySecureStopServiceRequest_Impl: Sized + super::IMediaProtectionServiceRequest_Impl + IPlayReadyServiceRequest_Impl {
    fn SessionID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn UpdateTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Stopped(&mut self) -> ::windows::core::Result<bool>;
    fn PublisherCertificate(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest";
}
#[cfg(feature = "Foundation")]
impl IPlayReadySecureStopServiceRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadySecureStopServiceRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayReadySecureStopServiceRequest_Vtbl {
        unsafe extern "system" fn SessionID<Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionID() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateTime<Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stopped<Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublisherCertificate<Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublisherCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadySecureStopServiceRequest, BASE_OFFSET>(),
            SessionID: SessionID::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            UpdateTime: UpdateTime::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            PublisherCertificate: PublisherCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadySecureStopServiceRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPlayReadyServiceRequest_Impl: Sized + super::IMediaProtectionServiceRequest_Impl {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&mut self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ResponseCustomData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ChallengeCustomData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetChallengeCustomData(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BeginServiceRequest(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn NextServiceRequest(&mut self) -> ::windows::core::Result<IPlayReadyServiceRequest>;
    fn GenerateManualEnablingChallenge(&mut self) -> ::windows::core::Result<PlayReadySoapMessage>;
    fn ProcessManualEnablingResponse(&mut self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPlayReadyServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest";
}
#[cfg(feature = "Foundation")]
impl IPlayReadyServiceRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayReadyServiceRequest_Vtbl {
        unsafe extern "system" fn Uri<Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResponseCustomData<Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseCustomData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChallengeCustomData<Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChallengeCustomData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChallengeCustomData<Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChallengeCustomData(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BeginServiceRequest<Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginServiceRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextServiceRequest<Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextServiceRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateManualEnablingChallenge<Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateManualEnablingChallenge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessManualEnablingResponse<Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, responseBytes_array_size: u32, responsebytes: *const u8, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessManualEnablingResponse(::core::slice::from_raw_parts(::core::mem::transmute_copy(&responsebytes), responseBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyServiceRequest, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            SetUri: SetUri::<Impl, IMPL_OFFSET>,
            ResponseCustomData: ResponseCustomData::<Impl, IMPL_OFFSET>,
            ChallengeCustomData: ChallengeCustomData::<Impl, IMPL_OFFSET>,
            SetChallengeCustomData: SetChallengeCustomData::<Impl, IMPL_OFFSET>,
            BeginServiceRequest: BeginServiceRequest::<Impl, IMPL_OFFSET>,
            NextServiceRequest: NextServiceRequest::<Impl, IMPL_OFFSET>,
            GenerateManualEnablingChallenge: GenerateManualEnablingChallenge::<Impl, IMPL_OFFSET>,
            ProcessManualEnablingResponse: ProcessManualEnablingResponse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyServiceRequest as ::windows::core::Interface>::IID
    }
}
