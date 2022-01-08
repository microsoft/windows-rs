#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait INDClientImpl: Sized {
    fn RegistrationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NDClient, INDRegistrationCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRegistrationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProximityDetectionCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NDClient, INDProximityDetectionCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveProximityDetectionCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LicenseFetchCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NDClient, INDLicenseFetchCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLicenseFetchCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReRegistrationNeeded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NDClient, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveReRegistrationNeeded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ClosedCaptionDataReceived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NDClient, INDClosedCaptionDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosedCaptionDataReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartAsync(&self, contenturl: &::core::option::Option<super::super::super::Foundation::Uri>, startasyncoptions: u32, registrationcustomdata: &::core::option::Option<INDCustomData>, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDStartResult>>;
    fn LicenseFetchAsync(&self, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDLicenseFetchResult>>;
    fn ReRegistrationAsync(&self, registrationcustomdata: &::core::option::Option<INDCustomData>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INDClient {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDClient";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl INDClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDClientImpl, const OFFSET: isize>() -> INDClientVtbl {
        unsafe extern "system" fn RegistrationCompleted<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistrationCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<NDClient, INDRegistrationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<NDClient, INDRegistrationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRegistrationCompleted<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRegistrationCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProximityDetectionCompleted<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProximityDetectionCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<NDClient, INDProximityDetectionCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<NDClient, INDProximityDetectionCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProximityDetectionCompleted<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProximityDetectionCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LicenseFetchCompleted<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseFetchCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<NDClient, INDLicenseFetchCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<NDClient, INDLicenseFetchCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLicenseFetchCompleted<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLicenseFetchCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReRegistrationNeeded<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReRegistrationNeeded(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<NDClient, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<NDClient, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReRegistrationNeeded<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReRegistrationNeeded(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedCaptionDataReceived<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedCaptionDataReceived(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<NDClient, INDClosedCaptionDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<NDClient, INDClosedCaptionDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosedCaptionDataReceived<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosedCaptionDataReceived(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartAsync<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenturl: ::windows::core::RawPtr, startasyncoptions: u32, registrationcustomdata: ::windows::core::RawPtr, licensefetchdescriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync(
                &*(&contenturl as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                startasyncoptions,
                &*(&registrationcustomdata as *const <INDCustomData as ::windows::core::Abi>::Abi as *const <INDCustomData as ::windows::core::DefaultType>::DefaultType),
                &*(&licensefetchdescriptor as *const <INDLicenseFetchDescriptor as ::windows::core::Abi>::Abi as *const <INDLicenseFetchDescriptor as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LicenseFetchAsync<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, licensefetchdescriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseFetchAsync(&*(&licensefetchdescriptor as *const <INDLicenseFetchDescriptor as ::windows::core::Abi>::Abi as *const <INDLicenseFetchDescriptor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReRegistrationAsync<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registrationcustomdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReRegistrationAsync(&*(&registrationcustomdata as *const <INDCustomData as ::windows::core::Abi>::Abi as *const <INDCustomData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: INDClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INDClient>,
            ::windows::core::GetTrustLevel,
            RegistrationCompleted::<Impl, OFFSET>,
            RemoveRegistrationCompleted::<Impl, OFFSET>,
            ProximityDetectionCompleted::<Impl, OFFSET>,
            RemoveProximityDetectionCompleted::<Impl, OFFSET>,
            LicenseFetchCompleted::<Impl, OFFSET>,
            RemoveLicenseFetchCompleted::<Impl, OFFSET>,
            ReRegistrationNeeded::<Impl, OFFSET>,
            RemoveReRegistrationNeeded::<Impl, OFFSET>,
            ClosedCaptionDataReceived::<Impl, OFFSET>,
            RemoveClosedCaptionDataReceived::<Impl, OFFSET>,
            StartAsync::<Impl, OFFSET>,
            LicenseFetchAsync::<Impl, OFFSET>,
            ReRegistrationAsync::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait INDClientFactoryImpl: Sized {
    fn CreateInstance(&self, downloadengine: &::core::option::Option<INDDownloadEngine>, streamparser: &::core::option::Option<INDStreamParser>, pmessenger: &::core::option::Option<INDMessenger>) -> ::windows::core::Result<NDClient>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INDClientFactory {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDClientFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl INDClientFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDClientFactoryImpl, const OFFSET: isize>() -> INDClientFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: INDClientFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadengine: ::windows::core::RawPtr, streamparser: ::windows::core::RawPtr, pmessenger: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&downloadengine as *const <INDDownloadEngine as ::windows::core::Abi>::Abi as *const <INDDownloadEngine as ::windows::core::DefaultType>::DefaultType), &*(&streamparser as *const <INDStreamParser as ::windows::core::Abi>::Abi as *const <INDStreamParser as ::windows::core::DefaultType>::DefaultType), &*(&pmessenger as *const <INDMessenger as ::windows::core::Abi>::Abi as *const <INDMessenger as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDClientFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDClosedCaptionDataReceivedEventArgsImpl: Sized {
    fn ClosedCaptionDataFormat(&self) -> ::windows::core::Result<NDClosedCaptionFormat>;
    fn PresentationTimestamp(&self) -> ::windows::core::Result<i64>;
    fn ClosedCaptionData(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDClosedCaptionDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDClosedCaptionDataReceivedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDClosedCaptionDataReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDClosedCaptionDataReceivedEventArgsImpl, const OFFSET: isize>() -> INDClosedCaptionDataReceivedEventArgsVtbl {
        unsafe extern "system" fn ClosedCaptionDataFormat<Impl: INDClosedCaptionDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDClosedCaptionFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PresentationTimestamp<Impl: INDClosedCaptionDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClosedCaptionData<Impl: INDClosedCaptionDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDClosedCaptionDataReceivedEventArgs>, ::windows::core::GetTrustLevel, ClosedCaptionDataFormat::<Impl, OFFSET>, PresentationTimestamp::<Impl, OFFSET>, ClosedCaptionData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDCustomDataImpl: Sized {
    fn CustomDataTypeID(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn CustomData(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDCustomData";
}
#[cfg(feature = "deprecated")]
impl INDCustomDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDCustomDataImpl, const OFFSET: isize>() -> INDCustomDataVtbl {
        unsafe extern "system" fn CustomDataTypeID<Impl: INDCustomDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CustomData<Impl: INDCustomDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDCustomData>, ::windows::core::GetTrustLevel, CustomDataTypeID::<Impl, OFFSET>, CustomData::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait INDCustomDataFactoryImpl: Sized {
    fn CreateInstance(&self, customdatatypeidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], customdatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<NDCustomData>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INDCustomDataFactory {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDCustomDataFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl INDCustomDataFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDCustomDataFactoryImpl, const OFFSET: isize>() -> INDCustomDataFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: INDCustomDataFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customDataTypeIDBytes_array_size: u32, customdatatypeidbytes: *const u8, customDataBytes_array_size: u32, customdatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::slice::from_raw_parts(::core::mem::transmute_copy(&customdatatypeidbytes), customDataTypeIDBytes_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&customdatabytes), customDataBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDCustomDataFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDDownloadEngineImpl: Sized {
    fn Open(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn Seek(&self, startposition: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CanSeek(&self) -> ::windows::core::Result<bool>;
    fn BufferFullMinThresholdInSamples(&self) -> ::windows::core::Result<u32>;
    fn BufferFullMaxThresholdInSamples(&self) -> ::windows::core::Result<u32>;
    fn Notifier(&self) -> ::windows::core::Result<NDDownloadEngineNotifier>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDDownloadEngine {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDDownloadEngine";
}
#[cfg(feature = "deprecated")]
impl INDDownloadEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngineImpl, const OFFSET: isize>() -> INDDownloadEngineVtbl {
        unsafe extern "system" fn Open<Impl: INDDownloadEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, sessionIDBytes_array_size: u32, sessionidbytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(&*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as _)).into()
        }
        unsafe extern "system" fn Pause<Impl: INDDownloadEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: INDDownloadEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Close<Impl: INDDownloadEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Seek<Impl: INDDownloadEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startposition: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Seek(&*(&startposition as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanSeek<Impl: INDDownloadEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BufferFullMinThresholdInSamples<Impl: INDDownloadEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BufferFullMaxThresholdInSamples<Impl: INDDownloadEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Notifier<Impl: INDDownloadEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INDDownloadEngine>,
            ::windows::core::GetTrustLevel,
            Open::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            Seek::<Impl, OFFSET>,
            CanSeek::<Impl, OFFSET>,
            BufferFullMinThresholdInSamples::<Impl, OFFSET>,
            BufferFullMaxThresholdInSamples::<Impl, OFFSET>,
            Notifier::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "deprecated")]
pub trait INDDownloadEngineNotifierImpl: Sized {
    fn OnStreamOpened(&self) -> ::windows::core::Result<()>;
    fn OnPlayReadyObjectReceived(&self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn OnContentIDReceived(&self, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<()>;
    fn OnDataReceived(&self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], bytesreceived: u32) -> ::windows::core::Result<()>;
    fn OnEndOfStream(&self) -> ::windows::core::Result<()>;
    fn OnNetworkError(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDDownloadEngineNotifier";
}
#[cfg(feature = "deprecated")]
impl INDDownloadEngineNotifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDDownloadEngineNotifierImpl, const OFFSET: isize>() -> INDDownloadEngineNotifierVtbl {
        unsafe extern "system" fn OnStreamOpened<Impl: INDDownloadEngineNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStreamOpened().into()
        }
        unsafe extern "system" fn OnPlayReadyObjectReceived<Impl: INDDownloadEngineNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPlayReadyObjectReceived(::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as _)).into()
        }
        unsafe extern "system" fn OnContentIDReceived<Impl: INDDownloadEngineNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, licensefetchdescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnContentIDReceived(&*(&licensefetchdescriptor as *const <INDLicenseFetchDescriptor as ::windows::core::Abi>::Abi as *const <INDLicenseFetchDescriptor as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnDataReceived<Impl: INDDownloadEngineNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDataReceived(::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as _), bytesreceived).into()
        }
        unsafe extern "system" fn OnEndOfStream<Impl: INDDownloadEngineNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEndOfStream().into()
        }
        unsafe extern "system" fn OnNetworkError<Impl: INDDownloadEngineNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNetworkError().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDDownloadEngineNotifier>, ::windows::core::GetTrustLevel, OnStreamOpened::<Impl, OFFSET>, OnPlayReadyObjectReceived::<Impl, OFFSET>, OnContentIDReceived::<Impl, OFFSET>, OnDataReceived::<Impl, OFFSET>, OnEndOfStream::<Impl, OFFSET>, OnNetworkError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchCompletedEventArgsImpl: Sized {
    fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDLicenseFetchCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchCompletedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDLicenseFetchCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchCompletedEventArgsImpl, const OFFSET: isize>() -> INDLicenseFetchCompletedEventArgsVtbl {
        unsafe extern "system" fn ResponseCustomData<Impl: INDLicenseFetchCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDLicenseFetchCompletedEventArgs>, ::windows::core::GetTrustLevel, ResponseCustomData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchDescriptorImpl: Sized {
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
impl INDLicenseFetchDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchDescriptorImpl, const OFFSET: isize>() -> INDLicenseFetchDescriptorVtbl {
        unsafe extern "system" fn ContentIDType<Impl: INDLicenseFetchDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDContentIDType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentID<Impl: INDLicenseFetchDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LicenseFetchChallengeCustomData<Impl: INDLicenseFetchDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLicenseFetchChallengeCustomData<Impl: INDLicenseFetchDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, licensefetchchallengecustomdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLicenseFetchChallengeCustomData(&*(&licensefetchchallengecustomdata as *const <INDCustomData as ::windows::core::Abi>::Abi as *const <INDCustomData as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDLicenseFetchDescriptor>, ::windows::core::GetTrustLevel, ContentIDType::<Impl, OFFSET>, ContentID::<Impl, OFFSET>, LicenseFetchChallengeCustomData::<Impl, OFFSET>, SetLicenseFetchChallengeCustomData::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait INDLicenseFetchDescriptorFactoryImpl: Sized {
    fn CreateInstance(&self, contentidtype: NDContentIDType, contentidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], licensefetchchallengecustomdata: &::core::option::Option<INDCustomData>) -> ::windows::core::Result<NDLicenseFetchDescriptor>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INDLicenseFetchDescriptorFactory {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptorFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl INDLicenseFetchDescriptorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchDescriptorFactoryImpl, const OFFSET: isize>() -> INDLicenseFetchDescriptorFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: INDLicenseFetchDescriptorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentidtype: NDContentIDType, contentIDBytes_array_size: u32, contentidbytes: *const u8, licensefetchchallengecustomdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(contentidtype, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&contentidbytes), contentIDBytes_array_size as _), &*(&licensefetchchallengecustomdata as *const <INDCustomData as ::windows::core::Abi>::Abi as *const <INDCustomData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDLicenseFetchDescriptorFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchResultImpl: Sized {
    fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDLicenseFetchResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchResult";
}
#[cfg(feature = "deprecated")]
impl INDLicenseFetchResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDLicenseFetchResultImpl, const OFFSET: isize>() -> INDLicenseFetchResultVtbl {
        unsafe extern "system" fn ResponseCustomData<Impl: INDLicenseFetchResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDLicenseFetchResult>, ::windows::core::GetTrustLevel, ResponseCustomData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDMessengerImpl: Sized {
    fn SendRegistrationRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], responsedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDMessenger";
}
#[cfg(feature = "deprecated")]
impl INDMessengerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDMessengerImpl, const OFFSET: isize>() -> INDMessengerVtbl {
        unsafe extern "system" fn SendRegistrationRequestAsync<Impl: INDMessengerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SendProximityDetectionStartAsync<Impl: INDMessengerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SendProximityDetectionResponseAsync<Impl: INDMessengerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, responseDataBytes_array_size: u32, responsedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SendLicenseFetchRequestAsync<Impl: INDMessengerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDMessenger>, ::windows::core::GetTrustLevel, SendRegistrationRequestAsync::<Impl, OFFSET>, SendProximityDetectionStartAsync::<Impl, OFFSET>, SendProximityDetectionResponseAsync::<Impl, OFFSET>, SendLicenseFetchRequestAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDProximityDetectionCompletedEventArgsImpl: Sized {
    fn ProximityDetectionRetryCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDProximityDetectionCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDProximityDetectionCompletedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDProximityDetectionCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDProximityDetectionCompletedEventArgsImpl, const OFFSET: isize>() -> INDProximityDetectionCompletedEventArgsVtbl {
        unsafe extern "system" fn ProximityDetectionRetryCount<Impl: INDProximityDetectionCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDProximityDetectionCompletedEventArgs>, ::windows::core::GetTrustLevel, ProximityDetectionRetryCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDRegistrationCompletedEventArgsImpl: Sized {
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
impl INDRegistrationCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDRegistrationCompletedEventArgsImpl, const OFFSET: isize>() -> INDRegistrationCompletedEventArgsVtbl {
        unsafe extern "system" fn ResponseCustomData<Impl: INDRegistrationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransmitterProperties<Impl: INDRegistrationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransmitterCertificateAccepted<Impl: INDRegistrationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransmitterCertificateAccepted<Impl: INDRegistrationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accept: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransmitterCertificateAccepted(accept).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDRegistrationCompletedEventArgs>, ::windows::core::GetTrustLevel, ResponseCustomData::<Impl, OFFSET>, TransmitterProperties::<Impl, OFFSET>, TransmitterCertificateAccepted::<Impl, OFFSET>, SetTransmitterCertificateAccepted::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDSendResultImpl: Sized {
    fn Response(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDSendResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDSendResult";
}
#[cfg(feature = "deprecated")]
impl INDSendResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDSendResultImpl, const OFFSET: isize>() -> INDSendResultVtbl {
        unsafe extern "system" fn Response<Impl: INDSendResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDSendResult>, ::windows::core::GetTrustLevel, Response::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDStartResultImpl: Sized {
    fn MediaStreamSource(&self) -> ::windows::core::Result<super::super::Core::MediaStreamSource>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDStartResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStartResult";
}
#[cfg(feature = "deprecated")]
impl INDStartResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStartResultImpl, const OFFSET: isize>() -> INDStartResultVtbl {
        unsafe extern "system" fn MediaStreamSource<Impl: INDStartResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDStartResult>, ::windows::core::GetTrustLevel, MediaStreamSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDStorageFileHelperImpl: Sized {
    fn GetFileURLs(&self, file: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStorageFileHelper";
}
#[cfg(feature = "deprecated")]
impl INDStorageFileHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStorageFileHelperImpl, const OFFSET: isize>() -> INDStorageFileHelperVtbl {
        unsafe extern "system" fn GetFileURLs<Impl: INDStorageFileHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDStorageFileHelper>, ::windows::core::GetTrustLevel, GetFileURLs::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDStreamParserImpl: Sized {
    fn ParseData(&self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetStreamInformation(&self, descriptor: &::core::option::Option<super::super::Core::IMediaStreamDescriptor>, streamtype: &mut NDMediaStreamType) -> ::windows::core::Result<u32>;
    fn BeginOfStream(&self) -> ::windows::core::Result<()>;
    fn EndOfStream(&self) -> ::windows::core::Result<()>;
    fn Notifier(&self) -> ::windows::core::Result<NDStreamParserNotifier>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDStreamParser {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStreamParser";
}
#[cfg(feature = "deprecated")]
impl INDStreamParserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParserImpl, const OFFSET: isize>() -> INDStreamParserVtbl {
        unsafe extern "system" fn ParseData<Impl: INDStreamParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseData(::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as _)).into()
        }
        unsafe extern "system" fn GetStreamInformation<Impl: INDStreamParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BeginOfStream<Impl: INDStreamParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginOfStream().into()
        }
        unsafe extern "system" fn EndOfStream<Impl: INDStreamParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndOfStream().into()
        }
        unsafe extern "system" fn Notifier<Impl: INDStreamParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDStreamParser>, ::windows::core::GetTrustLevel, ParseData::<Impl, OFFSET>, GetStreamInformation::<Impl, OFFSET>, BeginOfStream::<Impl, OFFSET>, EndOfStream::<Impl, OFFSET>, Notifier::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDStreamParserNotifierImpl: Sized {
    fn OnContentIDReceived(&self, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<()>;
    fn OnMediaStreamDescriptorCreated(&self, audiostreamdescriptors: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, videostreamdescriptors: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn OnSampleParsed(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: &::core::option::Option<super::super::Core::MediaStreamSample>, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn OnBeginSetupDecryptor(&self, descriptor: &::core::option::Option<super::super::Core::IMediaStreamDescriptor>, keyid: &::windows::core::GUID, probytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStreamParserNotifier";
}
#[cfg(feature = "deprecated")]
impl INDStreamParserNotifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDStreamParserNotifierImpl, const OFFSET: isize>() -> INDStreamParserNotifierVtbl {
        unsafe extern "system" fn OnContentIDReceived<Impl: INDStreamParserNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, licensefetchdescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnContentIDReceived(&*(&licensefetchdescriptor as *const <INDLicenseFetchDescriptor as ::windows::core::Abi>::Abi as *const <INDLicenseFetchDescriptor as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnMediaStreamDescriptorCreated<Impl: INDStreamParserNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostreamdescriptors: ::windows::core::RawPtr, videostreamdescriptors: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .OnMediaStreamDescriptorCreated(
                    &*(&audiostreamdescriptors as *const <super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor> as ::windows::core::DefaultType>::DefaultType),
                    &*(&videostreamdescriptors as *const <super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor> as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn OnSampleParsed<Impl: INDStreamParserNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamid: u32, streamtype: NDMediaStreamType, streamsample: ::windows::core::RawPtr, pts: i64, ccformat: NDClosedCaptionFormat, ccDataBytes_array_size: u32, ccdatabytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSampleParsed(streamid, streamtype, &*(&streamsample as *const <super::super::Core::MediaStreamSample as ::windows::core::Abi>::Abi as *const <super::super::Core::MediaStreamSample as ::windows::core::DefaultType>::DefaultType), pts, ccformat, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&ccdatabytes), ccDataBytes_array_size as _)).into()
        }
        unsafe extern "system" fn OnBeginSetupDecryptor<Impl: INDStreamParserNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, keyid: ::windows::core::GUID, proBytes_array_size: u32, probytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .OnBeginSetupDecryptor(&*(&descriptor as *const <super::super::Core::IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <super::super::Core::IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType), &*(&keyid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&probytes), proBytes_array_size as _))
                .into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDStreamParserNotifier>, ::windows::core::GetTrustLevel, OnContentIDReceived::<Impl, OFFSET>, OnMediaStreamDescriptorCreated::<Impl, OFFSET>, OnSampleParsed::<Impl, OFFSET>, OnBeginSetupDecryptor::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait INDTCPMessengerFactoryImpl: Sized {
    fn CreateInstance(&self, remotehostname: &::windows::core::HSTRING, remotehostport: u32) -> ::windows::core::Result<NDTCPMessenger>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INDTCPMessengerFactory {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDTCPMessengerFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl INDTCPMessengerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDTCPMessengerFactoryImpl, const OFFSET: isize>() -> INDTCPMessengerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: INDTCPMessengerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remotehostport: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&remotehostname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), remotehostport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INDTCPMessengerFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait INDTransmitterPropertiesImpl: Sized {
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
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for INDTransmitterProperties {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDTransmitterProperties";
}
#[cfg(feature = "deprecated")]
impl INDTransmitterPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>() -> INDTransmitterPropertiesVtbl {
        unsafe extern "system" fn CertificateType<Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDCertificateType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PlatformIdentifier<Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NDCertificatePlatformID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedFeatures<Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SecurityLevel<Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SecurityVersion<Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpirationDate<Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClientID<Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ModelDigest<Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ModelManufacturerName<Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ModelName<Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ModelNumber<Impl: INDTransmitterPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INDTransmitterProperties>,
            ::windows::core::GetTrustLevel,
            CertificateType::<Impl, OFFSET>,
            PlatformIdentifier::<Impl, OFFSET>,
            SupportedFeatures::<Impl, OFFSET>,
            SecurityLevel::<Impl, OFFSET>,
            SecurityVersion::<Impl, OFFSET>,
            ExpirationDate::<Impl, OFFSET>,
            ClientID::<Impl, OFFSET>,
            ModelDigest::<Impl, OFFSET>,
            ModelManufacturerName::<Impl, OFFSET>,
            ModelName::<Impl, OFFSET>,
            ModelNumber::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyContentHeaderImpl: Sized {
    fn KeyId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn KeyIdString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LicenseAcquisitionUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn LicenseAcquisitionUserInterfaceUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn EncryptionType(&self) -> ::windows::core::Result<PlayReadyEncryptionAlgorithm>;
    fn CustomAttributes(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DecryptorSetup(&self) -> ::windows::core::Result<PlayReadyDecryptorSetup>;
    fn GetSerializedHeader(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn HeaderWithEmbeddedUpdates(&self) -> ::windows::core::Result<PlayReadyContentHeader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyContentHeader {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyContentHeader";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyContentHeaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyContentHeaderImpl, const OFFSET: isize>() -> IPlayReadyContentHeaderVtbl {
        unsafe extern "system" fn KeyId<Impl: IPlayReadyContentHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyIdString<Impl: IPlayReadyContentHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyIdString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LicenseAcquisitionUrl<Impl: IPlayReadyContentHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseAcquisitionUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LicenseAcquisitionUserInterfaceUrl<Impl: IPlayReadyContentHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseAcquisitionUserInterfaceUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainServiceId<Impl: IPlayReadyContentHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EncryptionType<Impl: IPlayReadyContentHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PlayReadyEncryptionAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptionType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomAttributes<Impl: IPlayReadyContentHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecryptorSetup<Impl: IPlayReadyContentHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PlayReadyDecryptorSetup) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecryptorSetup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerializedHeader<Impl: IPlayReadyContentHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerializedHeader() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderWithEmbeddedUpdates<Impl: IPlayReadyContentHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderWithEmbeddedUpdates() {
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
            ::windows::core::GetRuntimeClassName::<IPlayReadyContentHeader>,
            ::windows::core::GetTrustLevel,
            KeyId::<Impl, OFFSET>,
            KeyIdString::<Impl, OFFSET>,
            LicenseAcquisitionUrl::<Impl, OFFSET>,
            LicenseAcquisitionUserInterfaceUrl::<Impl, OFFSET>,
            DomainServiceId::<Impl, OFFSET>,
            EncryptionType::<Impl, OFFSET>,
            CustomAttributes::<Impl, OFFSET>,
            DecryptorSetup::<Impl, OFFSET>,
            GetSerializedHeader::<Impl, OFFSET>,
            HeaderWithEmbeddedUpdates::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyContentHeader2Impl: Sized + IPlayReadyContentHeaderImpl {
    fn KeyIds(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::GUID>>;
    fn KeyIdStrings(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyContentHeader2 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyContentHeader2";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyContentHeader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyContentHeader2Impl, const OFFSET: isize>() -> IPlayReadyContentHeader2Vtbl {
        unsafe extern "system" fn KeyIds<Impl: IPlayReadyContentHeader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyIds() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyIdStrings<Impl: IPlayReadyContentHeader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyIdStrings() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyContentHeader2>, ::windows::core::GetTrustLevel, KeyIds::<Impl, OFFSET>, KeyIdStrings::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyContentHeaderFactoryImpl: Sized {
    fn CreateInstanceFromWindowsMediaDrmHeader(&self, headerbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], licenseacquisitionurl: &::core::option::Option<super::super::super::Foundation::Uri>, licenseacquisitionuserinterfaceurl: &::core::option::Option<super::super::super::Foundation::Uri>, customattributes: &::windows::core::HSTRING, domainserviceid: &::windows::core::GUID) -> ::windows::core::Result<PlayReadyContentHeader>;
    fn CreateInstanceFromComponents(&self, contentkeyid: &::windows::core::GUID, contentkeyidstring: &::windows::core::HSTRING, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: &::core::option::Option<super::super::super::Foundation::Uri>, licenseacquisitionuserinterfaceurl: &::core::option::Option<super::super::super::Foundation::Uri>, customattributes: &::windows::core::HSTRING, domainserviceid: &::windows::core::GUID) -> ::windows::core::Result<PlayReadyContentHeader>;
    fn CreateInstanceFromPlayReadyHeader(&self, headerbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadyContentHeader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyContentHeaderFactory {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyContentHeaderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyContentHeaderFactoryImpl, const OFFSET: isize>() -> IPlayReadyContentHeaderFactoryVtbl {
        unsafe extern "system" fn CreateInstanceFromWindowsMediaDrmHeader<Impl: IPlayReadyContentHeaderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headerBytes_array_size: u32, headerbytes: *const u8, licenseacquisitionurl: ::windows::core::RawPtr, licenseacquisitionuserinterfaceurl: ::windows::core::RawPtr, customattributes: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, domainserviceid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceFromWindowsMediaDrmHeader(
                ::core::slice::from_raw_parts(::core::mem::transmute_copy(&headerbytes), headerBytes_array_size as _),
                &*(&licenseacquisitionurl as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&licenseacquisitionuserinterfaceurl as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&customattributes as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&domainserviceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceFromComponents<Impl: IPlayReadyContentHeaderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentkeyid: ::windows::core::GUID, contentkeyidstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: ::windows::core::RawPtr, licenseacquisitionuserinterfaceurl: ::windows::core::RawPtr, customattributes: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, domainserviceid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceFromComponents(
                &*(&contentkeyid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&contentkeyidstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                contentencryptionalgorithm,
                &*(&licenseacquisitionurl as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&licenseacquisitionuserinterfaceurl as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&customattributes as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&domainserviceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceFromPlayReadyHeader<Impl: IPlayReadyContentHeaderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headerBytes_array_size: u32, headerbytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceFromPlayReadyHeader(::core::slice::from_raw_parts(::core::mem::transmute_copy(&headerbytes), headerBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyContentHeaderFactory>, ::windows::core::GetTrustLevel, CreateInstanceFromWindowsMediaDrmHeader::<Impl, OFFSET>, CreateInstanceFromComponents::<Impl, OFFSET>, CreateInstanceFromPlayReadyHeader::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyContentHeaderFactory2Impl: Sized {
    fn CreateInstanceFromComponents2(&self, dwflags: u32, contentkeyids: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType], contentkeyidstrings: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType], contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: &::core::option::Option<super::super::super::Foundation::Uri>, licenseacquisitionuserinterfaceurl: &::core::option::Option<super::super::super::Foundation::Uri>, customattributes: &::windows::core::HSTRING, domainserviceid: &::windows::core::GUID) -> ::windows::core::Result<PlayReadyContentHeader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyContentHeaderFactory2 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory2";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyContentHeaderFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyContentHeaderFactory2Impl, const OFFSET: isize>() -> IPlayReadyContentHeaderFactory2Vtbl {
        unsafe extern "system" fn CreateInstanceFromComponents2<Impl: IPlayReadyContentHeaderFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, contentKeyIds_array_size: u32, contentkeyids: &::windows::core::GUID, contentKeyIdStrings_array_size: u32, contentkeyidstrings: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: ::windows::core::RawPtr, licenseacquisitionuserinterfaceurl: ::windows::core::RawPtr, customattributes: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, domainserviceid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceFromComponents2(
                dwflags,
                ::core::slice::from_raw_parts(::core::mem::transmute_copy(&contentkeyids), contentKeyIds_array_size as _),
                ::core::slice::from_raw_parts(::core::mem::transmute_copy(&contentkeyidstrings), contentKeyIdStrings_array_size as _),
                contentencryptionalgorithm,
                &*(&licenseacquisitionurl as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&licenseacquisitionuserinterfaceurl as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&customattributes as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&domainserviceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyContentHeaderFactory2>, ::windows::core::GetTrustLevel, CreateInstanceFromComponents2::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyContentResolverImpl: Sized {
    fn ServiceRequest(&self, contentheader: &::core::option::Option<PlayReadyContentHeader>) -> ::windows::core::Result<IPlayReadyServiceRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyContentResolver {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyContentResolver";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyContentResolverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyContentResolverImpl, const OFFSET: isize>() -> IPlayReadyContentResolverVtbl {
        unsafe extern "system" fn ServiceRequest<Impl: IPlayReadyContentResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentheader: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceRequest(&*(&contentheader as *const <PlayReadyContentHeader as ::windows::core::Abi>::Abi as *const <PlayReadyContentHeader as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyContentResolver>, ::windows::core::GetTrustLevel, ServiceRequest::<Impl, OFFSET>)
    }
}
pub trait IPlayReadyDomainImpl: Sized {
    fn AccountId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Revision(&self) -> ::windows::core::Result<u32>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DomainJoinUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
impl ::windows::core::RuntimeName for IPlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyDomain";
}
impl IPlayReadyDomainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyDomainImpl, const OFFSET: isize>() -> IPlayReadyDomainVtbl {
        unsafe extern "system" fn AccountId<Impl: IPlayReadyDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServiceId<Impl: IPlayReadyDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Revision<Impl: IPlayReadyDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FriendlyName<Impl: IPlayReadyDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DomainJoinUrl<Impl: IPlayReadyDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyDomain>, ::windows::core::GetTrustLevel, AccountId::<Impl, OFFSET>, ServiceId::<Impl, OFFSET>, Revision::<Impl, OFFSET>, FriendlyName::<Impl, OFFSET>, DomainJoinUrl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyDomainIterableFactoryImpl: Sized {
    fn CreateInstance(&self, domainaccountid: &::windows::core::GUID) -> ::windows::core::Result<PlayReadyDomainIterable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyDomainIterableFactory {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyDomainIterableFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyDomainIterableFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyDomainIterableFactoryImpl, const OFFSET: isize>() -> IPlayReadyDomainIterableFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPlayReadyDomainIterableFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domainaccountid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&domainaccountid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyDomainIterableFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyDomainJoinServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn DomainAccountId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainAccountId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DomainFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDomainFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainServiceId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyDomainJoinServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyDomainJoinServiceRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyDomainJoinServiceRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyDomainJoinServiceRequestImpl, const OFFSET: isize>() -> IPlayReadyDomainJoinServiceRequestVtbl {
        unsafe extern "system" fn DomainAccountId<Impl: IPlayReadyDomainJoinServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainAccountId<Impl: IPlayReadyDomainJoinServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainAccountId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DomainFriendlyName<Impl: IPlayReadyDomainJoinServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainFriendlyName<Impl: IPlayReadyDomainJoinServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DomainServiceId<Impl: IPlayReadyDomainJoinServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDomainServiceId<Impl: IPlayReadyDomainJoinServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainServiceId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPlayReadyDomainJoinServiceRequest>,
            ::windows::core::GetTrustLevel,
            DomainAccountId::<Impl, OFFSET>,
            SetDomainAccountId::<Impl, OFFSET>,
            DomainFriendlyName::<Impl, OFFSET>,
            SetDomainFriendlyName::<Impl, OFFSET>,
            DomainServiceId::<Impl, OFFSET>,
            SetDomainServiceId::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyDomainLeaveServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn DomainAccountId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainAccountId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainServiceId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyDomainLeaveServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyDomainLeaveServiceRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyDomainLeaveServiceRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyDomainLeaveServiceRequestImpl, const OFFSET: isize>() -> IPlayReadyDomainLeaveServiceRequestVtbl {
        unsafe extern "system" fn DomainAccountId<Impl: IPlayReadyDomainLeaveServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainAccountId<Impl: IPlayReadyDomainLeaveServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainAccountId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DomainServiceId<Impl: IPlayReadyDomainLeaveServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDomainServiceId<Impl: IPlayReadyDomainLeaveServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainServiceId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyDomainLeaveServiceRequest>, ::windows::core::GetTrustLevel, DomainAccountId::<Impl, OFFSET>, SetDomainAccountId::<Impl, OFFSET>, DomainServiceId::<Impl, OFFSET>, SetDomainServiceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyITADataGeneratorImpl: Sized {
    fn GenerateData(&self, guidcpsystemid: &::windows::core::GUID, countofstreams: u32, configuration: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>, format: PlayReadyITADataFormat) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyITADataGenerator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyITADataGenerator";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyITADataGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyITADataGeneratorImpl, const OFFSET: isize>() -> IPlayReadyITADataGeneratorVtbl {
        unsafe extern "system" fn GenerateData<Impl: IPlayReadyITADataGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidcpsystemid: ::windows::core::GUID, countofstreams: u32, configuration: ::windows::core::RawPtr, format: PlayReadyITADataFormat, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateData(&*(&guidcpsystemid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), countofstreams, &*(&configuration as *const <super::super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType), format) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyITADataGenerator>, ::windows::core::GetTrustLevel, GenerateData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyIndividualizationServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyIndividualizationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyIndividualizationServiceRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyIndividualizationServiceRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyIndividualizationServiceRequestImpl, const OFFSET: isize>() -> IPlayReadyIndividualizationServiceRequestVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyIndividualizationServiceRequest>, ::windows::core::GetTrustLevel)
    }
}
pub trait IPlayReadyLicenseImpl: Sized {
    fn FullyEvaluated(&self) -> ::windows::core::Result<bool>;
    fn UsableForPlay(&self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ExpireAfterFirstPlay(&self) -> ::windows::core::Result<u32>;
    fn DomainAccountID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ChainDepth(&self) -> ::windows::core::Result<u32>;
    fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IPlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicense";
}
impl IPlayReadyLicenseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseImpl, const OFFSET: isize>() -> IPlayReadyLicenseVtbl {
        unsafe extern "system" fn FullyEvaluated<Impl: IPlayReadyLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsableForPlay<Impl: IPlayReadyLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpirationDate<Impl: IPlayReadyLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpireAfterFirstPlay<Impl: IPlayReadyLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DomainAccountID<Impl: IPlayReadyLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChainDepth<Impl: IPlayReadyLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetKIDAtChainDepth<Impl: IPlayReadyLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chaindepth: u32, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPlayReadyLicense>,
            ::windows::core::GetTrustLevel,
            FullyEvaluated::<Impl, OFFSET>,
            UsableForPlay::<Impl, OFFSET>,
            ExpirationDate::<Impl, OFFSET>,
            ExpireAfterFirstPlay::<Impl, OFFSET>,
            DomainAccountID::<Impl, OFFSET>,
            ChainDepth::<Impl, OFFSET>,
            GetKIDAtChainDepth::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicense2Impl: Sized + IPlayReadyLicenseImpl {
    fn SecureStopId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SecurityLevel(&self) -> ::windows::core::Result<u32>;
    fn InMemoryOnly(&self) -> ::windows::core::Result<bool>;
    fn ExpiresInRealTime(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyLicense2 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicense2";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyLicense2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicense2Impl, const OFFSET: isize>() -> IPlayReadyLicense2Vtbl {
        unsafe extern "system" fn SecureStopId<Impl: IPlayReadyLicense2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecureStopId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityLevel<Impl: IPlayReadyLicense2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InMemoryOnly<Impl: IPlayReadyLicense2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InMemoryOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpiresInRealTime<Impl: IPlayReadyLicense2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpiresInRealTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyLicense2>, ::windows::core::GetTrustLevel, SecureStopId::<Impl, OFFSET>, SecurityLevel::<Impl, OFFSET>, InMemoryOnly::<Impl, OFFSET>, ExpiresInRealTime::<Impl, OFFSET>)
    }
}
pub trait IPlayReadyLicenseAcquisitionServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn ContentHeader(&self) -> ::windows::core::Result<PlayReadyContentHeader>;
    fn SetContentHeader(&self, value: &::core::option::Option<PlayReadyContentHeader>) -> ::windows::core::Result<()>;
    fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainServiceId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest";
}
impl IPlayReadyLicenseAcquisitionServiceRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseAcquisitionServiceRequestImpl, const OFFSET: isize>() -> IPlayReadyLicenseAcquisitionServiceRequestVtbl {
        unsafe extern "system" fn ContentHeader<Impl: IPlayReadyLicenseAcquisitionServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentHeader<Impl: IPlayReadyLicenseAcquisitionServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentHeader(&*(&value as *const <PlayReadyContentHeader as ::windows::core::Abi>::Abi as *const <PlayReadyContentHeader as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DomainServiceId<Impl: IPlayReadyLicenseAcquisitionServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDomainServiceId<Impl: IPlayReadyLicenseAcquisitionServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainServiceId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyLicenseAcquisitionServiceRequest>, ::windows::core::GetTrustLevel, ContentHeader::<Impl, OFFSET>, SetContentHeader::<Impl, OFFSET>, DomainServiceId::<Impl, OFFSET>, SetDomainServiceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicenseAcquisitionServiceRequest2Impl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyLicenseAcquisitionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyLicenseAcquisitionServiceRequest2 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest2";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyLicenseAcquisitionServiceRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseAcquisitionServiceRequest2Impl, const OFFSET: isize>() -> IPlayReadyLicenseAcquisitionServiceRequest2Vtbl {
        unsafe extern "system" fn SessionId<Impl: IPlayReadyLicenseAcquisitionServiceRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyLicenseAcquisitionServiceRequest2>, ::windows::core::GetTrustLevel, SessionId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicenseAcquisitionServiceRequest3Impl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyLicenseAcquisitionServiceRequestImpl + IPlayReadyLicenseAcquisitionServiceRequest2Impl + IPlayReadyServiceRequestImpl {
    fn CreateLicenseIterable(&self, contentheader: &::core::option::Option<PlayReadyContentHeader>, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyLicenseAcquisitionServiceRequest3 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest3";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyLicenseAcquisitionServiceRequest3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseAcquisitionServiceRequest3Impl, const OFFSET: isize>() -> IPlayReadyLicenseAcquisitionServiceRequest3Vtbl {
        unsafe extern "system" fn CreateLicenseIterable<Impl: IPlayReadyLicenseAcquisitionServiceRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentheader: ::windows::core::RawPtr, fullyevaluated: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyLicenseAcquisitionServiceRequest3>, ::windows::core::GetTrustLevel, CreateLicenseIterable::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicenseIterableFactoryImpl: Sized {
    fn CreateInstance(&self, contentheader: &::core::option::Option<PlayReadyContentHeader>, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyLicenseIterableFactory {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseIterableFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyLicenseIterableFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseIterableFactoryImpl, const OFFSET: isize>() -> IPlayReadyLicenseIterableFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPlayReadyLicenseIterableFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentheader: ::windows::core::RawPtr, fullyevaluated: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&contentheader as *const <PlayReadyContentHeader as ::windows::core::Abi>::Abi as *const <PlayReadyContentHeader as ::windows::core::DefaultType>::DefaultType), fullyevaluated) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyLicenseIterableFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicenseManagementImpl: Sized {
    fn DeleteLicenses(&self, contentheader: &::core::option::Option<PlayReadyContentHeader>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyLicenseManagement {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseManagement";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyLicenseManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseManagementImpl, const OFFSET: isize>() -> IPlayReadyLicenseManagementVtbl {
        unsafe extern "system" fn DeleteLicenses<Impl: IPlayReadyLicenseManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentheader: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteLicenses(&*(&contentheader as *const <PlayReadyContentHeader as ::windows::core::Abi>::Abi as *const <PlayReadyContentHeader as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyLicenseManagement>, ::windows::core::GetTrustLevel, DeleteLicenses::<Impl, OFFSET>)
    }
}
pub trait IPlayReadyLicenseSessionImpl: Sized {
    fn CreateLAServiceRequest(&self) -> ::windows::core::Result<IPlayReadyLicenseAcquisitionServiceRequest>;
    fn ConfigureMediaProtectionManager(&self, mpm: &::core::option::Option<super::MediaProtectionManager>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession";
}
impl IPlayReadyLicenseSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseSessionImpl, const OFFSET: isize>() -> IPlayReadyLicenseSessionVtbl {
        unsafe extern "system" fn CreateLAServiceRequest<Impl: IPlayReadyLicenseSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConfigureMediaProtectionManager<Impl: IPlayReadyLicenseSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mpm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureMediaProtectionManager(&*(&mpm as *const <super::MediaProtectionManager as ::windows::core::Abi>::Abi as *const <super::MediaProtectionManager as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyLicenseSession>, ::windows::core::GetTrustLevel, CreateLAServiceRequest::<Impl, OFFSET>, ConfigureMediaProtectionManager::<Impl, OFFSET>)
    }
}
pub trait IPlayReadyLicenseSession2Impl: Sized + IPlayReadyLicenseSessionImpl {
    fn CreateLicenseIterable(&self, contentheader: &::core::option::Option<PlayReadyContentHeader>, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable>;
}
impl ::windows::core::RuntimeName for IPlayReadyLicenseSession2 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession2";
}
impl IPlayReadyLicenseSession2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseSession2Impl, const OFFSET: isize>() -> IPlayReadyLicenseSession2Vtbl {
        unsafe extern "system" fn CreateLicenseIterable<Impl: IPlayReadyLicenseSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentheader: ::windows::core::RawPtr, fullyevaluated: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyLicenseSession2>, ::windows::core::GetTrustLevel, CreateLicenseIterable::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicenseSessionFactoryImpl: Sized {
    fn CreateInstance(&self, configuration: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<PlayReadyLicenseSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyLicenseSessionFactory {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseSessionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyLicenseSessionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyLicenseSessionFactoryImpl, const OFFSET: isize>() -> IPlayReadyLicenseSessionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPlayReadyLicenseSessionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&configuration as *const <super::super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyLicenseSessionFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyMeteringReportServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn MeteringCertificate(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetMeteringCertificate(&self, meteringcertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyMeteringReportServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyMeteringReportServiceRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyMeteringReportServiceRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyMeteringReportServiceRequestImpl, const OFFSET: isize>() -> IPlayReadyMeteringReportServiceRequestVtbl {
        unsafe extern "system" fn MeteringCertificate<Impl: IPlayReadyMeteringReportServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeteringCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMeteringCertificate<Impl: IPlayReadyMeteringReportServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, meteringCertBytes_array_size: u32, meteringcertbytes: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMeteringCertificate(::core::slice::from_raw_parts(::core::mem::transmute_copy(&meteringcertbytes), meteringCertBytes_array_size as _)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyMeteringReportServiceRequest>, ::windows::core::GetTrustLevel, MeteringCertificate::<Impl, OFFSET>, SetMeteringCertificate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyRevocationServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyRevocationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyRevocationServiceRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyRevocationServiceRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyRevocationServiceRequestImpl, const OFFSET: isize>() -> IPlayReadyRevocationServiceRequestVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyRevocationServiceRequest>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadySecureStopIterableFactoryImpl: Sized {
    fn CreateInstance(&self, publishercertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadySecureStopIterable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadySecureStopIterableFactory {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadySecureStopIterableFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadySecureStopIterableFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadySecureStopIterableFactoryImpl, const OFFSET: isize>() -> IPlayReadySecureStopIterableFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPlayReadySecureStopIterableFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::slice::from_raw_parts(::core::mem::transmute_copy(&publishercertbytes), publisherCertBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadySecureStopIterableFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
pub trait IPlayReadySecureStopServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn SessionID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn UpdateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Stopped(&self) -> ::windows::core::Result<bool>;
    fn PublisherCertificate(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
impl ::windows::core::RuntimeName for IPlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest";
}
impl IPlayReadySecureStopServiceRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadySecureStopServiceRequestImpl, const OFFSET: isize>() -> IPlayReadySecureStopServiceRequestVtbl {
        unsafe extern "system" fn SessionID<Impl: IPlayReadySecureStopServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartTime<Impl: IPlayReadySecureStopServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateTime<Impl: IPlayReadySecureStopServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Stopped<Impl: IPlayReadySecureStopServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PublisherCertificate<Impl: IPlayReadySecureStopServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadySecureStopServiceRequest>, ::windows::core::GetTrustLevel, SessionID::<Impl, OFFSET>, StartTime::<Impl, OFFSET>, UpdateTime::<Impl, OFFSET>, Stopped::<Impl, OFFSET>, PublisherCertificate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadySecureStopServiceRequestFactoryImpl: Sized {
    fn CreateInstance(&self, publishercertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadySecureStopServiceRequest>;
    fn CreateInstanceFromSessionID(&self, sessionid: &::windows::core::GUID, publishercertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadySecureStopServiceRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadySecureStopServiceRequestFactory {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequestFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadySecureStopServiceRequestFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadySecureStopServiceRequestFactoryImpl, const OFFSET: isize>() -> IPlayReadySecureStopServiceRequestFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPlayReadySecureStopServiceRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::slice::from_raw_parts(::core::mem::transmute_copy(&publishercertbytes), publisherCertBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceFromSessionID<Impl: IPlayReadySecureStopServiceRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::GUID, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceFromSessionID(&*(&sessionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&publishercertbytes), publisherCertBytes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadySecureStopServiceRequestFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>, CreateInstanceFromSessionID::<Impl, OFFSET>)
    }
}
pub trait IPlayReadyServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest>;
    fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage>;
    fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT>;
}
impl ::windows::core::RuntimeName for IPlayReadyServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest";
}
impl IPlayReadyServiceRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyServiceRequestImpl, const OFFSET: isize>() -> IPlayReadyServiceRequestVtbl {
        unsafe extern "system" fn Uri<Impl: IPlayReadyServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUri<Impl: IPlayReadyServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResponseCustomData<Impl: IPlayReadyServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChallengeCustomData<Impl: IPlayReadyServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetChallengeCustomData<Impl: IPlayReadyServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChallengeCustomData(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BeginServiceRequest<Impl: IPlayReadyServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NextServiceRequest<Impl: IPlayReadyServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GenerateManualEnablingChallenge<Impl: IPlayReadyServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProcessManualEnablingResponse<Impl: IPlayReadyServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, responseBytes_array_size: u32, responsebytes: *const u8, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPlayReadyServiceRequest>,
            ::windows::core::GetTrustLevel,
            Uri::<Impl, OFFSET>,
            SetUri::<Impl, OFFSET>,
            ResponseCustomData::<Impl, OFFSET>,
            ChallengeCustomData::<Impl, OFFSET>,
            SetChallengeCustomData::<Impl, OFFSET>,
            BeginServiceRequest::<Impl, OFFSET>,
            NextServiceRequest::<Impl, OFFSET>,
            GenerateManualEnablingChallenge::<Impl, OFFSET>,
            ProcessManualEnablingResponse::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadySoapMessageImpl: Sized {
    fn GetMessageBody(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn MessageHeaders(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IPropertySet>;
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadySoapMessage {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadySoapMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadySoapMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadySoapMessageImpl, const OFFSET: isize>() -> IPlayReadySoapMessageVtbl {
        unsafe extern "system" fn GetMessageBody<Impl: IPlayReadySoapMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageBody() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageHeaders<Impl: IPlayReadySoapMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: IPlayReadySoapMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadySoapMessage>, ::windows::core::GetTrustLevel, GetMessageBody::<Impl, OFFSET>, MessageHeaders::<Impl, OFFSET>, Uri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyStaticsImpl: Sized {
    fn DomainJoinServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DomainLeaveServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IndividualizationServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LicenseAcquirerServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn MeteringReportServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RevocationServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn MediaProtectionSystemId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PlayReadySecurityVersion(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyStatics {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyStaticsImpl, const OFFSET: isize>() -> IPlayReadyStaticsVtbl {
        unsafe extern "system" fn DomainJoinServiceRequestType<Impl: IPlayReadyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainJoinServiceRequestType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainLeaveServiceRequestType<Impl: IPlayReadyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainLeaveServiceRequestType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndividualizationServiceRequestType<Impl: IPlayReadyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndividualizationServiceRequestType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LicenseAcquirerServiceRequestType<Impl: IPlayReadyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseAcquirerServiceRequestType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeteringReportServiceRequestType<Impl: IPlayReadyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeteringReportServiceRequestType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevocationServiceRequestType<Impl: IPlayReadyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevocationServiceRequestType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaProtectionSystemId<Impl: IPlayReadyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaProtectionSystemId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlayReadySecurityVersion<Impl: IPlayReadyStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayReadySecurityVersion() {
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
            ::windows::core::GetRuntimeClassName::<IPlayReadyStatics>,
            ::windows::core::GetTrustLevel,
            DomainJoinServiceRequestType::<Impl, OFFSET>,
            DomainLeaveServiceRequestType::<Impl, OFFSET>,
            IndividualizationServiceRequestType::<Impl, OFFSET>,
            LicenseAcquirerServiceRequestType::<Impl, OFFSET>,
            MeteringReportServiceRequestType::<Impl, OFFSET>,
            RevocationServiceRequestType::<Impl, OFFSET>,
            MediaProtectionSystemId::<Impl, OFFSET>,
            PlayReadySecurityVersion::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyStatics2Impl: Sized + IPlayReadyStaticsImpl {
    fn PlayReadyCertificateSecurityLevel(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyStatics2 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyStatics2Impl, const OFFSET: isize>() -> IPlayReadyStatics2Vtbl {
        unsafe extern "system" fn PlayReadyCertificateSecurityLevel<Impl: IPlayReadyStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayReadyCertificateSecurityLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyStatics2>, ::windows::core::GetTrustLevel, PlayReadyCertificateSecurityLevel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyStatics3Impl: Sized + IPlayReadyStaticsImpl + IPlayReadyStatics2Impl {
    fn SecureStopServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CheckSupportedHardware(&self, hwdrmfeature: PlayReadyHardwareDRMFeatures) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyStatics3 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyStatics3Impl, const OFFSET: isize>() -> IPlayReadyStatics3Vtbl {
        unsafe extern "system" fn SecureStopServiceRequestType<Impl: IPlayReadyStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecureStopServiceRequestType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckSupportedHardware<Impl: IPlayReadyStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwdrmfeature: PlayReadyHardwareDRMFeatures, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckSupportedHardware(hwdrmfeature) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyStatics3>, ::windows::core::GetTrustLevel, SecureStopServiceRequestType::<Impl, OFFSET>, CheckSupportedHardware::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyStatics4Impl: Sized + IPlayReadyStaticsImpl + IPlayReadyStatics2Impl + IPlayReadyStatics3Impl {
    fn InputTrustAuthorityToCreate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProtectionSystemId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyStatics4 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyStatics4Impl, const OFFSET: isize>() -> IPlayReadyStatics4Vtbl {
        unsafe extern "system" fn InputTrustAuthorityToCreate<Impl: IPlayReadyStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputTrustAuthorityToCreate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectionSystemId<Impl: IPlayReadyStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionSystemId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyStatics4>, ::windows::core::GetTrustLevel, InputTrustAuthorityToCreate::<Impl, OFFSET>, ProtectionSystemId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyStatics5Impl: Sized + IPlayReadyStaticsImpl + IPlayReadyStatics2Impl + IPlayReadyStatics3Impl + IPlayReadyStatics4Impl {
    fn HardwareDRMDisabledAtTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn HardwareDRMDisabledUntilTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ResetHardwareDRMDisabled(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlayReadyStatics5 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IPlayReadyStatics5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayReadyStatics5Impl, const OFFSET: isize>() -> IPlayReadyStatics5Vtbl {
        unsafe extern "system" fn HardwareDRMDisabledAtTime<Impl: IPlayReadyStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareDRMDisabledAtTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareDRMDisabledUntilTime<Impl: IPlayReadyStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareDRMDisabledUntilTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetHardwareDRMDisabled<Impl: IPlayReadyStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetHardwareDRMDisabled().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlayReadyStatics5>, ::windows::core::GetTrustLevel, HardwareDRMDisabledAtTime::<Impl, OFFSET>, HardwareDRMDisabledUntilTime::<Impl, OFFSET>, ResetHardwareDRMDisabled::<Impl, OFFSET>)
    }
}
