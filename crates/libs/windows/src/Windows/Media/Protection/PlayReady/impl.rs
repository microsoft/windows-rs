#[cfg(feature = "deprecated")]
pub trait INDClosedCaptionDataReceivedEventArgs_Impl: Sized {
    fn ClosedCaptionDataFormat(&self) -> ::windows::core::Result<NDClosedCaptionFormat>;
    fn PresentationTimestamp(&self) -> ::windows::core::Result<i64>;
    fn ClosedCaptionData(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDClosedCaptionDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDClosedCaptionDataReceivedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDClosedCaptionDataReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>() -> INDClosedCaptionDataReceivedEventArgs_Vtbl {
        unsafe extern "system" fn ClosedCaptionDataFormat<Identity: ::windows::core::IUnknownImpl, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDClosedCaptionFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClosedCaptionDataFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationTimestamp<Identity: ::windows::core::IUnknownImpl, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PresentationTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedCaptionData<Identity: ::windows::core::IUnknownImpl, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDClosedCaptionDataReceivedEventArgs, OFFSET>(),
            ClosedCaptionDataFormat: ClosedCaptionDataFormat::<Identity, Impl, OFFSET>,
            PresentationTimestamp: PresentationTimestamp::<Identity, Impl, OFFSET>,
            ClosedCaptionData: ClosedCaptionData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDClosedCaptionDataReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDCustomData_Impl: Sized {
    fn CustomDataTypeID(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn CustomData(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDCustomData";
}
#[cfg(feature = "deprecated")]
impl INDCustomData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDCustomData_Impl, const OFFSET: isize>() -> INDCustomData_Vtbl {
        unsafe extern "system" fn CustomDataTypeID<Identity: ::windows::core::IUnknownImpl, Impl: INDCustomData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn CustomData<Identity: ::windows::core::IUnknownImpl, Impl: INDCustomData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDCustomData, OFFSET>(),
            CustomDataTypeID: CustomDataTypeID::<Identity, Impl, OFFSET>,
            CustomData: CustomData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDCustomData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait INDDownloadEngine_Impl: Sized {
    fn Open(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, sessionidbytes: &[u8]) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn Seek(&self, startposition: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CanSeek(&self) -> ::windows::core::Result<bool>;
    fn BufferFullMinThresholdInSamples(&self) -> ::windows::core::Result<u32>;
    fn BufferFullMaxThresholdInSamples(&self) -> ::windows::core::Result<u32>;
    fn Notifier(&self) -> ::windows::core::Result<NDDownloadEngineNotifier>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDDownloadEngine {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDDownloadEngine";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl INDDownloadEngine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngine_Impl, const OFFSET: isize>() -> INDDownloadEngine_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, sessionIDBytes_array_size: u32, sessionidbytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute(&uri), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as _)).into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startposition: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Seek(::core::mem::transmute(&startposition)).into()
        }
        unsafe extern "system" fn CanSeek<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanSeek() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferFullMinThresholdInSamples<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BufferFullMinThresholdInSamples() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferFullMaxThresholdInSamples<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BufferFullMaxThresholdInSamples() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notifier<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDDownloadEngine, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            CanSeek: CanSeek::<Identity, Impl, OFFSET>,
            BufferFullMinThresholdInSamples: BufferFullMinThresholdInSamples::<Identity, Impl, OFFSET>,
            BufferFullMaxThresholdInSamples: BufferFullMaxThresholdInSamples::<Identity, Impl, OFFSET>,
            Notifier: Notifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDDownloadEngine as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDDownloadEngineNotifier_Impl: Sized {
    fn OnStreamOpened(&self) -> ::windows::core::Result<()>;
    fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> ::windows::core::Result<()>;
    fn OnContentIDReceived(&self, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<()>;
    fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> ::windows::core::Result<()>;
    fn OnEndOfStream(&self) -> ::windows::core::Result<()>;
    fn OnNetworkError(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDDownloadEngineNotifier";
}
#[cfg(feature = "deprecated")]
impl INDDownloadEngineNotifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>() -> INDDownloadEngineNotifier_Vtbl {
        unsafe extern "system" fn OnStreamOpened<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStreamOpened().into()
        }
        unsafe extern "system" fn OnPlayReadyObjectReceived<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPlayReadyObjectReceived(::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as _)).into()
        }
        unsafe extern "system" fn OnContentIDReceived<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, licensefetchdescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnContentIDReceived(::core::mem::transmute(&licensefetchdescriptor)).into()
        }
        unsafe extern "system" fn OnDataReceived<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDataReceived(::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as _), bytesreceived).into()
        }
        unsafe extern "system" fn OnEndOfStream<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEndOfStream().into()
        }
        unsafe extern "system" fn OnNetworkError<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnNetworkError().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDDownloadEngineNotifier, OFFSET>(),
            OnStreamOpened: OnStreamOpened::<Identity, Impl, OFFSET>,
            OnPlayReadyObjectReceived: OnPlayReadyObjectReceived::<Identity, Impl, OFFSET>,
            OnContentIDReceived: OnContentIDReceived::<Identity, Impl, OFFSET>,
            OnDataReceived: OnDataReceived::<Identity, Impl, OFFSET>,
            OnEndOfStream: OnEndOfStream::<Identity, Impl, OFFSET>,
            OnNetworkError: OnNetworkError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDDownloadEngineNotifier as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchCompletedEventArgs_Impl: Sized {
    fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDLicenseFetchCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchCompletedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDLicenseFetchCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchCompletedEventArgs_Impl, const OFFSET: isize>() -> INDLicenseFetchCompletedEventArgs_Vtbl {
        unsafe extern "system" fn ResponseCustomData<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDLicenseFetchCompletedEventArgs, OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDLicenseFetchCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchDescriptor_Impl: Sized {
    fn ContentIDType(&self) -> ::windows::core::Result<NDContentIDType>;
    fn ContentID(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn LicenseFetchChallengeCustomData(&self) -> ::windows::core::Result<INDCustomData>;
    fn SetLicenseFetchChallengeCustomData(&self, licensefetchchallengecustomdata: &::core::option::Option<INDCustomData>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDLicenseFetchDescriptor {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptor";
}
#[cfg(feature = "deprecated")]
impl INDLicenseFetchDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>() -> INDLicenseFetchDescriptor_Vtbl {
        unsafe extern "system" fn ContentIDType<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDContentIDType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContentIDType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentID<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn LicenseFetchChallengeCustomData<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LicenseFetchChallengeCustomData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLicenseFetchChallengeCustomData<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, licensefetchchallengecustomdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLicenseFetchChallengeCustomData(::core::mem::transmute(&licensefetchchallengecustomdata)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDLicenseFetchDescriptor, OFFSET>(),
            ContentIDType: ContentIDType::<Identity, Impl, OFFSET>,
            ContentID: ContentID::<Identity, Impl, OFFSET>,
            LicenseFetchChallengeCustomData: LicenseFetchChallengeCustomData::<Identity, Impl, OFFSET>,
            SetLicenseFetchChallengeCustomData: SetLicenseFetchChallengeCustomData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDLicenseFetchDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchResult_Impl: Sized {
    fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDLicenseFetchResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchResult";
}
#[cfg(feature = "deprecated")]
impl INDLicenseFetchResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchResult_Impl, const OFFSET: isize>() -> INDLicenseFetchResult_Vtbl {
        unsafe extern "system" fn ResponseCustomData<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDLicenseFetchResult, OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDLicenseFetchResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait INDMessenger_Impl: Sized {
    fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDMessenger";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl INDMessenger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDMessenger_Impl, const OFFSET: isize>() -> INDMessenger_Vtbl {
        unsafe extern "system" fn SendRegistrationRequestAsync<Identity: ::windows::core::IUnknownImpl, Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendRegistrationRequestAsync(::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&challengedatabytes), challengeDataBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendProximityDetectionStartAsync<Identity: ::windows::core::IUnknownImpl, Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendProximityDetectionStartAsync(pdtype, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&transmitterchannelbytes), transmitterChannelBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&challengedatabytes), challengeDataBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendProximityDetectionResponseAsync<Identity: ::windows::core::IUnknownImpl, Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, responseDataBytes_array_size: u32, responsedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendProximityDetectionResponseAsync(pdtype, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&transmitterchannelbytes), transmitterChannelBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&responsedatabytes), responseDataBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendLicenseFetchRequestAsync<Identity: ::windows::core::IUnknownImpl, Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDMessenger, OFFSET>(),
            SendRegistrationRequestAsync: SendRegistrationRequestAsync::<Identity, Impl, OFFSET>,
            SendProximityDetectionStartAsync: SendProximityDetectionStartAsync::<Identity, Impl, OFFSET>,
            SendProximityDetectionResponseAsync: SendProximityDetectionResponseAsync::<Identity, Impl, OFFSET>,
            SendLicenseFetchRequestAsync: SendLicenseFetchRequestAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDMessenger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDProximityDetectionCompletedEventArgs_Impl: Sized {
    fn ProximityDetectionRetryCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDProximityDetectionCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDProximityDetectionCompletedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDProximityDetectionCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDProximityDetectionCompletedEventArgs_Impl, const OFFSET: isize>() -> INDProximityDetectionCompletedEventArgs_Vtbl {
        unsafe extern "system" fn ProximityDetectionRetryCount<Identity: ::windows::core::IUnknownImpl, Impl: INDProximityDetectionCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDProximityDetectionCompletedEventArgs, OFFSET>(),
            ProximityDetectionRetryCount: ProximityDetectionRetryCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDProximityDetectionCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDRegistrationCompletedEventArgs_Impl: Sized {
    fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData>;
    fn TransmitterProperties(&self) -> ::windows::core::Result<INDTransmitterProperties>;
    fn TransmitterCertificateAccepted(&self) -> ::windows::core::Result<bool>;
    fn SetTransmitterCertificateAccepted(&self, accept: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDRegistrationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDRegistrationCompletedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDRegistrationCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>() -> INDRegistrationCompletedEventArgs_Vtbl {
        unsafe extern "system" fn ResponseCustomData<Identity: ::windows::core::IUnknownImpl, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseCustomData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmitterProperties<Identity: ::windows::core::IUnknownImpl, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransmitterProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmitterCertificateAccepted<Identity: ::windows::core::IUnknownImpl, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransmitterCertificateAccepted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransmitterCertificateAccepted<Identity: ::windows::core::IUnknownImpl, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accept: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransmitterCertificateAccepted(accept).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDRegistrationCompletedEventArgs, OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
            TransmitterProperties: TransmitterProperties::<Identity, Impl, OFFSET>,
            TransmitterCertificateAccepted: TransmitterCertificateAccepted::<Identity, Impl, OFFSET>,
            SetTransmitterCertificateAccepted: SetTransmitterCertificateAccepted::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDRegistrationCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDSendResult_Impl: Sized {
    fn Response(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDSendResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDSendResult";
}
#[cfg(feature = "deprecated")]
impl INDSendResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDSendResult_Impl, const OFFSET: isize>() -> INDSendResult_Vtbl {
        unsafe extern "system" fn Response<Identity: ::windows::core::IUnknownImpl, Impl: INDSendResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INDSendResult, OFFSET>(), Response: Response::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDSendResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
pub trait INDStartResult_Impl: Sized {
    fn MediaStreamSource(&self) -> ::windows::core::Result<super::super::Core::MediaStreamSource>;
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDStartResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStartResult";
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl INDStartResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStartResult_Impl, const OFFSET: isize>() -> INDStartResult_Vtbl {
        unsafe extern "system" fn MediaStreamSource<Identity: ::windows::core::IUnknownImpl, Impl: INDStartResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDStartResult, OFFSET>(),
            MediaStreamSource: MediaStreamSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDStartResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait INDStorageFileHelper_Impl: Sized {
    fn GetFileURLs(&self, file: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStorageFileHelper";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl INDStorageFileHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStorageFileHelper_Impl, const OFFSET: isize>() -> INDStorageFileHelper_Vtbl {
        unsafe extern "system" fn GetFileURLs<Identity: ::windows::core::IUnknownImpl, Impl: INDStorageFileHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFileURLs(::core::mem::transmute(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INDStorageFileHelper, OFFSET>(), GetFileURLs: GetFileURLs::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDStorageFileHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
pub trait INDStreamParser_Impl: Sized {
    fn ParseData(&self, databytes: &[u8]) -> ::windows::core::Result<()>;
    fn GetStreamInformation(&self, descriptor: &::core::option::Option<super::super::Core::IMediaStreamDescriptor>, streamtype: &mut NDMediaStreamType) -> ::windows::core::Result<u32>;
    fn BeginOfStream(&self) -> ::windows::core::Result<()>;
    fn EndOfStream(&self) -> ::windows::core::Result<()>;
    fn Notifier(&self) -> ::windows::core::Result<NDStreamParserNotifier>;
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDStreamParser {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStreamParser";
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl INDStreamParser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParser_Impl, const OFFSET: isize>() -> INDStreamParser_Vtbl {
        unsafe extern "system" fn ParseData<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ParseData(::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as _)).into()
        }
        unsafe extern "system" fn GetStreamInformation<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStreamInformation(::core::mem::transmute(&descriptor), ::core::mem::transmute_copy(&streamtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginOfStream<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginOfStream().into()
        }
        unsafe extern "system" fn EndOfStream<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndOfStream().into()
        }
        unsafe extern "system" fn Notifier<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDStreamParser, OFFSET>(),
            ParseData: ParseData::<Identity, Impl, OFFSET>,
            GetStreamInformation: GetStreamInformation::<Identity, Impl, OFFSET>,
            BeginOfStream: BeginOfStream::<Identity, Impl, OFFSET>,
            EndOfStream: EndOfStream::<Identity, Impl, OFFSET>,
            Notifier: Notifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDStreamParser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
pub trait INDStreamParserNotifier_Impl: Sized {
    fn OnContentIDReceived(&self, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<()>;
    fn OnMediaStreamDescriptorCreated(&self, audiostreamdescriptors: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, videostreamdescriptors: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn OnSampleParsed(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: &::core::option::Option<super::super::Core::MediaStreamSample>, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> ::windows::core::Result<()>;
    fn OnBeginSetupDecryptor(&self, descriptor: &::core::option::Option<super::super::Core::IMediaStreamDescriptor>, keyid: &::windows::core::GUID, probytes: &[u8]) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStreamParserNotifier";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
impl INDStreamParserNotifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>() -> INDStreamParserNotifier_Vtbl {
        unsafe extern "system" fn OnContentIDReceived<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, licensefetchdescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnContentIDReceived(::core::mem::transmute(&licensefetchdescriptor)).into()
        }
        unsafe extern "system" fn OnMediaStreamDescriptorCreated<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostreamdescriptors: ::windows::core::RawPtr, videostreamdescriptors: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnMediaStreamDescriptorCreated(::core::mem::transmute(&audiostreamdescriptors), ::core::mem::transmute(&videostreamdescriptors)).into()
        }
        unsafe extern "system" fn OnSampleParsed<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamid: u32, streamtype: NDMediaStreamType, streamsample: ::windows::core::RawPtr, pts: i64, ccformat: NDClosedCaptionFormat, ccDataBytes_array_size: u32, ccdatabytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSampleParsed(streamid, streamtype, ::core::mem::transmute(&streamsample), pts, ccformat, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&ccdatabytes), ccDataBytes_array_size as _)).into()
        }
        unsafe extern "system" fn OnBeginSetupDecryptor<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, keyid: ::windows::core::GUID, proBytes_array_size: u32, probytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnBeginSetupDecryptor(::core::mem::transmute(&descriptor), ::core::mem::transmute(&keyid), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&probytes), proBytes_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDStreamParserNotifier, OFFSET>(),
            OnContentIDReceived: OnContentIDReceived::<Identity, Impl, OFFSET>,
            OnMediaStreamDescriptorCreated: OnMediaStreamDescriptorCreated::<Identity, Impl, OFFSET>,
            OnSampleParsed: OnSampleParsed::<Identity, Impl, OFFSET>,
            OnBeginSetupDecryptor: OnBeginSetupDecryptor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDStreamParserNotifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait INDTransmitterProperties_Impl: Sized {
    fn CertificateType(&self) -> ::windows::core::Result<NDCertificateType>;
    fn PlatformIdentifier(&self) -> ::windows::core::Result<NDCertificatePlatformID>;
    fn SupportedFeatures(&self) -> ::windows::core::Result<::windows::core::Array<NDCertificateFeature>>;
    fn SecurityLevel(&self) -> ::windows::core::Result<u32>;
    fn SecurityVersion(&self) -> ::windows::core::Result<u32>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ClientID(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn ModelDigest(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn ModelManufacturerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for INDTransmitterProperties {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDTransmitterProperties";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl INDTransmitterProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>() -> INDTransmitterProperties_Vtbl {
        unsafe extern "system" fn CertificateType<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDCertificateType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CertificateType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlatformIdentifier<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDCertificatePlatformID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlatformIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFeatures<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn SecurityLevel<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SecurityLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityVersion<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SecurityVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientID<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn ModelDigest<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn ModelManufacturerName<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModelManufacturerName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelName<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModelName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumber<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INDTransmitterProperties, OFFSET>(),
            CertificateType: CertificateType::<Identity, Impl, OFFSET>,
            PlatformIdentifier: PlatformIdentifier::<Identity, Impl, OFFSET>,
            SupportedFeatures: SupportedFeatures::<Identity, Impl, OFFSET>,
            SecurityLevel: SecurityLevel::<Identity, Impl, OFFSET>,
            SecurityVersion: SecurityVersion::<Identity, Impl, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, Impl, OFFSET>,
            ClientID: ClientID::<Identity, Impl, OFFSET>,
            ModelDigest: ModelDigest::<Identity, Impl, OFFSET>,
            ModelManufacturerName: ModelManufacturerName::<Identity, Impl, OFFSET>,
            ModelName: ModelName::<Identity, Impl, OFFSET>,
            ModelNumber: ModelNumber::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDTransmitterProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPlayReadyDomain_Impl: Sized {
    fn AccountId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Revision(&self) -> ::windows::core::Result<u32>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DomainJoinUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyDomain";
}
#[cfg(feature = "Foundation")]
impl IPlayReadyDomain_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>() -> IPlayReadyDomain_Vtbl {
        unsafe extern "system" fn AccountId<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceId<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revision<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Revision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainJoinUrl<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyDomain, OFFSET>(),
            AccountId: AccountId::<Identity, Impl, OFFSET>,
            ServiceId: ServiceId::<Identity, Impl, OFFSET>,
            Revision: Revision::<Identity, Impl, OFFSET>,
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            DomainJoinUrl: DomainJoinUrl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyDomain as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPlayReadyLicense_Impl: Sized {
    fn FullyEvaluated(&self) -> ::windows::core::Result<bool>;
    fn UsableForPlay(&self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ExpireAfterFirstPlay(&self) -> ::windows::core::Result<u32>;
    fn DomainAccountID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ChainDepth(&self) -> ::windows::core::Result<u32>;
    fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicense";
}
#[cfg(feature = "Foundation")]
impl IPlayReadyLicense_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>() -> IPlayReadyLicense_Vtbl {
        unsafe extern "system" fn FullyEvaluated<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FullyEvaluated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsableForPlay<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UsableForPlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpireAfterFirstPlay<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExpireAfterFirstPlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainAccountID<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DomainAccountID() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChainDepth<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ChainDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKIDAtChainDepth<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chaindepth: u32, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyLicense, OFFSET>(),
            FullyEvaluated: FullyEvaluated::<Identity, Impl, OFFSET>,
            UsableForPlay: UsableForPlay::<Identity, Impl, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, Impl, OFFSET>,
            ExpireAfterFirstPlay: ExpireAfterFirstPlay::<Identity, Impl, OFFSET>,
            DomainAccountID: DomainAccountID::<Identity, Impl, OFFSET>,
            ChainDepth: ChainDepth::<Identity, Impl, OFFSET>,
            GetKIDAtChainDepth: GetKIDAtChainDepth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyLicense as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPlayReadyLicenseAcquisitionServiceRequest_Impl: Sized + super::IMediaProtectionServiceRequest_Impl + IPlayReadyServiceRequest_Impl {
    fn ContentHeader(&self) -> ::windows::core::Result<PlayReadyContentHeader>;
    fn SetContentHeader(&self, value: &::core::option::Option<PlayReadyContentHeader>) -> ::windows::core::Result<()>;
    fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainServiceId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest";
}
#[cfg(feature = "Foundation")]
impl IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>() -> IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
        unsafe extern "system" fn ContentHeader<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContentHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentHeader<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContentHeader(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn DomainServiceId<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DomainServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainServiceId<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDomainServiceId(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyLicenseAcquisitionServiceRequest, OFFSET>(),
            ContentHeader: ContentHeader::<Identity, Impl, OFFSET>,
            SetContentHeader: SetContentHeader::<Identity, Impl, OFFSET>,
            DomainServiceId: DomainServiceId::<Identity, Impl, OFFSET>,
            SetDomainServiceId: SetDomainServiceId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyLicenseAcquisitionServiceRequest as ::windows::core::Interface>::IID
    }
}
pub trait IPlayReadyLicenseSession_Impl: Sized {
    fn CreateLAServiceRequest(&self) -> ::windows::core::Result<IPlayReadyLicenseAcquisitionServiceRequest>;
    fn ConfigureMediaProtectionManager(&self, mpm: &::core::option::Option<super::MediaProtectionManager>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession";
}
impl IPlayReadyLicenseSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseSession_Impl, const OFFSET: isize>() -> IPlayReadyLicenseSession_Vtbl {
        unsafe extern "system" fn CreateLAServiceRequest<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLAServiceRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureMediaProtectionManager<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mpm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConfigureMediaProtectionManager(::core::mem::transmute(&mpm)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyLicenseSession, OFFSET>(),
            CreateLAServiceRequest: CreateLAServiceRequest::<Identity, Impl, OFFSET>,
            ConfigureMediaProtectionManager: ConfigureMediaProtectionManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyLicenseSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPlayReadyLicenseSession2_Impl: Sized + IPlayReadyLicenseSession_Impl {
    fn CreateLicenseIterable(&self, contentheader: &::core::option::Option<PlayReadyContentHeader>, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IPlayReadyLicenseSession2 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession2";
}
#[cfg(feature = "Foundation_Collections")]
impl IPlayReadyLicenseSession2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseSession2_Impl, const OFFSET: isize>() -> IPlayReadyLicenseSession2_Vtbl {
        unsafe extern "system" fn CreateLicenseIterable<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentheader: ::windows::core::RawPtr, fullyevaluated: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLicenseIterable(::core::mem::transmute(&contentheader), fullyevaluated) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyLicenseSession2, OFFSET>(),
            CreateLicenseIterable: CreateLicenseIterable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyLicenseSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPlayReadySecureStopServiceRequest_Impl: Sized + super::IMediaProtectionServiceRequest_Impl + IPlayReadyServiceRequest_Impl {
    fn SessionID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn UpdateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Stopped(&self) -> ::windows::core::Result<bool>;
    fn PublisherCertificate(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest";
}
#[cfg(feature = "Foundation")]
impl IPlayReadySecureStopServiceRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>() -> IPlayReadySecureStopServiceRequest_Vtbl {
        unsafe extern "system" fn SessionID<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionID() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateTime<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UpdateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stopped<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Stopped() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublisherCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadySecureStopServiceRequest, OFFSET>(),
            SessionID: SessionID::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            UpdateTime: UpdateTime::<Identity, Impl, OFFSET>,
            Stopped: Stopped::<Identity, Impl, OFFSET>,
            PublisherCertificate: PublisherCertificate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadySecureStopServiceRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPlayReadyServiceRequest_Impl: Sized + super::IMediaProtectionServiceRequest_Impl {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest>;
    fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage>;
    fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPlayReadyServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest";
}
#[cfg(feature = "Foundation")]
impl IPlayReadyServiceRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>() -> IPlayReadyServiceRequest_Vtbl {
        unsafe extern "system" fn Uri<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUri(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ResponseCustomData<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseCustomData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChallengeCustomData<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ChallengeCustomData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChallengeCustomData<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetChallengeCustomData(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BeginServiceRequest<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginServiceRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextServiceRequest<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NextServiceRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateManualEnablingChallenge<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerateManualEnablingChallenge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessManualEnablingResponse<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, responseBytes_array_size: u32, responsebytes: *const u8, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayReadyServiceRequest, OFFSET>(),
            Uri: Uri::<Identity, Impl, OFFSET>,
            SetUri: SetUri::<Identity, Impl, OFFSET>,
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
            ChallengeCustomData: ChallengeCustomData::<Identity, Impl, OFFSET>,
            SetChallengeCustomData: SetChallengeCustomData::<Identity, Impl, OFFSET>,
            BeginServiceRequest: BeginServiceRequest::<Identity, Impl, OFFSET>,
            NextServiceRequest: NextServiceRequest::<Identity, Impl, OFFSET>,
            GenerateManualEnablingChallenge: GenerateManualEnablingChallenge::<Identity, Impl, OFFSET>,
            ProcessManualEnablingResponse: ProcessManualEnablingResponse::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayReadyServiceRequest as ::windows::core::Interface>::IID
    }
}
