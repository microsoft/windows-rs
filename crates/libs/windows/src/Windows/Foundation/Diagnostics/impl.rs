#[cfg(feature = "implement_exclusive")]
pub trait IAsyncCausalityTracerStaticsImpl: Sized {
    fn TraceOperationCreation(&self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, operationname: &::windows::core::HSTRING, relatedcontext: u64) -> ::windows::core::Result<()>;
    fn TraceOperationCompletion(&self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows::core::Result<()>;
    fn TraceOperationRelation(&self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows::core::Result<()>;
    fn TraceSynchronousWorkStart(&self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows::core::Result<()>;
    fn TraceSynchronousWorkCompletion(&self, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::core::Result<()>;
    fn TracingStatusChanged(&self, handler: &::core::option::Option<super::EventHandler<TracingStatusChangedEventArgs>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveTracingStatusChanged(&self, cookie: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAsyncCausalityTracerStatics {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IAsyncCausalityTracerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAsyncCausalityTracerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncCausalityTracerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncCausalityTracerStaticsVtbl {
        unsafe extern "system" fn TraceOperationCreation<Impl: IAsyncCausalityTracerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, operationname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, relatedcontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TraceOperationCreation(tracelevel, source, &*(&platformid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), operationid, &*(&operationname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), relatedcontext).into()
        }
        unsafe extern "system" fn TraceOperationCompletion<Impl: IAsyncCausalityTracerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TraceOperationCompletion(tracelevel, source, &*(&platformid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), operationid, status).into()
        }
        unsafe extern "system" fn TraceOperationRelation<Impl: IAsyncCausalityTracerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TraceOperationRelation(tracelevel, source, &*(&platformid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), operationid, relation).into()
        }
        unsafe extern "system" fn TraceSynchronousWorkStart<Impl: IAsyncCausalityTracerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TraceSynchronousWorkStart(tracelevel, source, &*(&platformid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), operationid, work).into()
        }
        unsafe extern "system" fn TraceSynchronousWorkCompletion<Impl: IAsyncCausalityTracerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TraceSynchronousWorkCompletion(tracelevel, source, work).into()
        }
        unsafe extern "system" fn TracingStatusChanged<Impl: IAsyncCausalityTracerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TracingStatusChanged(&*(&handler as *const <super::EventHandler<TracingStatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::EventHandler<TracingStatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTracingStatusChanged<Impl: IAsyncCausalityTracerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTracingStatusChanged(&*(&cookie as *const <super::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAsyncCausalityTracerStatics>,
            ::windows::core::GetTrustLevel,
            TraceOperationCreation::<Impl, IMPL_OFFSET>,
            TraceOperationCompletion::<Impl, IMPL_OFFSET>,
            TraceOperationRelation::<Impl, IMPL_OFFSET>,
            TraceSynchronousWorkStart::<Impl, IMPL_OFFSET>,
            TraceSynchronousWorkCompletion::<Impl, IMPL_OFFSET>,
            TracingStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveTracingStatusChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncCausalityTracerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IErrorDetailsImpl: Sized {
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LongDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HelpUri(&self) -> ::windows::core::Result<super::Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IErrorDetails {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IErrorDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IErrorDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IErrorDetailsVtbl {
        unsafe extern "system" fn Description<Impl: IErrorDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDescription<Impl: IErrorDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HelpUri<Impl: IErrorDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HelpUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IErrorDetails>, ::windows::core::GetTrustLevel, Description::<Impl, IMPL_OFFSET>, LongDescription::<Impl, IMPL_OFFSET>, HelpUri::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IErrorDetailsStaticsImpl: Sized {
    fn CreateFromHResultAsync(&self, errorcode: i32) -> ::windows::core::Result<super::IAsyncOperation<ErrorDetails>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IErrorDetailsStatics {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IErrorDetailsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IErrorDetailsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorDetailsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IErrorDetailsStaticsVtbl {
        unsafe extern "system" fn CreateFromHResultAsync<Impl: IErrorDetailsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromHResultAsync(errorcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IErrorDetailsStatics>, ::windows::core::GetTrustLevel, CreateFromHResultAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorDetailsStatics as ::windows::core::Interface>::IID
    }
}
pub trait IErrorReportingSettingsImpl: Sized {
    fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::core::Result<()>;
    fn GetErrorOptions(&self) -> ::windows::core::Result<ErrorOptions>;
}
impl ::windows::core::RuntimeName for IErrorReportingSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IErrorReportingSettings";
}
impl IErrorReportingSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorReportingSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IErrorReportingSettingsVtbl {
        unsafe extern "system" fn SetErrorOptions<Impl: IErrorReportingSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ErrorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorOptions(value).into()
        }
        unsafe extern "system" fn GetErrorOptions<Impl: IErrorReportingSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ErrorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IErrorReportingSettings>, ::windows::core::GetTrustLevel, SetErrorOptions::<Impl, IMPL_OFFSET>, GetErrorOptions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorReportingSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Storage")]
pub trait IFileLoggingSessionImpl: Sized + IClosableImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn AddLoggingChannelWithLevel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows::core::Result<()>;
    fn RemoveLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn CloseAndSaveToFileAsync(&self) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn LogFileGenerated(&self, handler: &::core::option::Option<super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveLogFileGenerated(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows::core::RuntimeName for IFileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IFileLoggingSession";
}
#[cfg(feature = "Storage")]
impl IFileLoggingSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileLoggingSessionVtbl {
        unsafe extern "system" fn Name<Impl: IFileLoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Impl: IFileLoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Impl: IFileLoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr, maxlevel: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLoggingChannelWithLevel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Impl: IFileLoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CloseAndSaveToFileAsync<Impl: IFileLoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseAndSaveToFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFileGenerated<Impl: IFileLoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogFileGenerated(&*(&handler as *const <super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs> as ::windows::core::Abi>::Abi as *const <super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLogFileGenerated<Impl: IFileLoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLogFileGenerated(&*(&token as *const <super::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFileLoggingSession>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, IMPL_OFFSET>,
            AddLoggingChannel::<Impl, IMPL_OFFSET>,
            AddLoggingChannelWithLevel::<Impl, IMPL_OFFSET>,
            RemoveLoggingChannel::<Impl, IMPL_OFFSET>,
            CloseAndSaveToFileAsync::<Impl, IMPL_OFFSET>,
            LogFileGenerated::<Impl, IMPL_OFFSET>,
            RemoveLogFileGenerated::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileLoggingSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileLoggingSessionFactoryImpl: Sized {
    fn Create(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<FileLoggingSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileLoggingSessionFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IFileLoggingSessionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFileLoggingSessionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSessionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileLoggingSessionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IFileLoggingSessionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileLoggingSessionFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileLoggingSessionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
pub trait ILogFileGeneratedEventArgsImpl: Sized {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILogFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILogFileGeneratedEventArgs";
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ILogFileGeneratedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILogFileGeneratedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILogFileGeneratedEventArgsVtbl {
        unsafe extern "system" fn File<Impl: ILogFileGeneratedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILogFileGeneratedEventArgs>, ::windows::core::GetTrustLevel, File::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILogFileGeneratedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingActivityImpl: Sized + IClosableImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingActivity {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingActivity";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingActivityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingActivityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingActivityVtbl {
        unsafe extern "system" fn Name<Impl: ILoggingActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ILoggingActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoggingActivity>, ::windows::core::GetTrustLevel, Name::<Impl, IMPL_OFFSET>, Id::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingActivity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingActivity2Impl: Sized + IClosableImpl + ILoggingActivityImpl + ILoggingTargetImpl {
    fn Channel(&self) -> ::windows::core::Result<LoggingChannel>;
    fn StopActivity(&self, stopeventname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StopActivityWithFields(&self, stopeventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<()>;
    fn StopActivityWithFieldsAndOptions(&self, stopeventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingActivity2 {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingActivity2";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingActivity2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingActivity2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingActivity2Vtbl {
        unsafe extern "system" fn Channel<Impl: ILoggingActivity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopActivity<Impl: ILoggingActivity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopActivity(&*(&stopeventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopActivityWithFields<Impl: ILoggingActivity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopActivityWithFields(&*(&stopeventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopActivityWithFieldsAndOptions<Impl: ILoggingActivity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .StopActivityWithFieldsAndOptions(&*(&stopeventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LoggingOptions as ::windows::core::Abi>::Abi as *const <LoggingOptions as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoggingActivity2>, ::windows::core::GetTrustLevel, Channel::<Impl, IMPL_OFFSET>, StopActivity::<Impl, IMPL_OFFSET>, StopActivityWithFields::<Impl, IMPL_OFFSET>, StopActivityWithFieldsAndOptions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingActivity2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingActivityFactoryImpl: Sized {
    fn CreateLoggingActivity(&self, activityname: &::windows::core::HSTRING, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<LoggingActivity>;
    fn CreateLoggingActivityWithLevel(&self, activityname: &::windows::core::HSTRING, loggingchannel: &::core::option::Option<ILoggingChannel>, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingActivityFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingActivityFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingActivityFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingActivityFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingActivityFactoryVtbl {
        unsafe extern "system" fn CreateLoggingActivity<Impl: ILoggingActivityFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, loggingchannel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLoggingActivity(&*(&activityname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLoggingActivityWithLevel<Impl: ILoggingActivityFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, loggingchannel: ::windows::core::RawPtr, level: LoggingLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLoggingActivityWithLevel(&*(&activityname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType), level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoggingActivityFactory>, ::windows::core::GetTrustLevel, CreateLoggingActivity::<Impl, IMPL_OFFSET>, CreateLoggingActivityWithLevel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingActivityFactory as ::windows::core::Interface>::IID
    }
}
pub trait ILoggingChannelImpl: Sized + IClosableImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn Level(&self) -> ::windows::core::Result<LoggingLevel>;
    fn LogMessage(&self, eventstring: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogMessageWithLevel(&self, eventstring: &::windows::core::HSTRING, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LogValuePair(&self, value1: &::windows::core::HSTRING, value2: i32) -> ::windows::core::Result<()>;
    fn LogValuePairWithLevel(&self, value1: &::windows::core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LoggingEnabled(&self, handler: &::core::option::Option<super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveLoggingEnabled(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannel";
}
impl ILoggingChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannelVtbl {
        unsafe extern "system" fn Name<Impl: ILoggingChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: ILoggingChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Level<Impl: ILoggingChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Level() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogMessage<Impl: ILoggingChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogMessage(&*(&eventstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogMessageWithLevel<Impl: ILoggingChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogMessageWithLevel(&*(&eventstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), level).into()
        }
        unsafe extern "system" fn LogValuePair<Impl: ILoggingChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogValuePair(&*(&value1 as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value2).into()
        }
        unsafe extern "system" fn LogValuePairWithLevel<Impl: ILoggingChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogValuePairWithLevel(&*(&value1 as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value2, level).into()
        }
        unsafe extern "system" fn LoggingEnabled<Impl: ILoggingChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoggingEnabled(&*(&handler as *const <super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoggingEnabled<Impl: ILoggingChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLoggingEnabled(&*(&token as *const <super::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILoggingChannel>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            Level::<Impl, IMPL_OFFSET>,
            LogMessage::<Impl, IMPL_OFFSET>,
            LogMessageWithLevel::<Impl, IMPL_OFFSET>,
            LogValuePair::<Impl, IMPL_OFFSET>,
            LogValuePairWithLevel::<Impl, IMPL_OFFSET>,
            LoggingEnabled::<Impl, IMPL_OFFSET>,
            RemoveLoggingEnabled::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannel2Impl: Sized + IClosableImpl + ILoggingChannelImpl + ILoggingTargetImpl {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingChannel2 {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannel2";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingChannel2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannel2Vtbl {
        unsafe extern "system" fn Id<Impl: ILoggingChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoggingChannel2>, ::windows::core::GetTrustLevel, Id::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannel2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelFactoryImpl: Sized {
    fn Create(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingChannel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingChannelFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannelFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingChannelFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannelFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannelFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ILoggingChannelFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoggingChannelFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannelFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelFactory2Impl: Sized {
    fn CreateWithOptions(&self, name: &::windows::core::HSTRING, options: &::core::option::Option<LoggingChannelOptions>) -> ::windows::core::Result<LoggingChannel>;
    fn CreateWithOptionsAndId(&self, name: &::windows::core::HSTRING, options: &::core::option::Option<LoggingChannelOptions>, id: &::windows::core::GUID) -> ::windows::core::Result<LoggingChannel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingChannelFactory2 {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannelFactory2";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingChannelFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannelFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannelFactory2Vtbl {
        unsafe extern "system" fn CreateWithOptions<Impl: ILoggingChannelFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithOptions(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LoggingChannelOptions as ::windows::core::Abi>::Abi as *const <LoggingChannelOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithOptionsAndId<Impl: ILoggingChannelFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, id: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithOptionsAndId(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LoggingChannelOptions as ::windows::core::Abi>::Abi as *const <LoggingChannelOptions as ::windows::core::DefaultType>::DefaultType), &*(&id as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoggingChannelFactory2>, ::windows::core::GetTrustLevel, CreateWithOptions::<Impl, IMPL_OFFSET>, CreateWithOptionsAndId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannelFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelOptionsImpl: Sized {
    fn Group(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetGroup(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingChannelOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannelOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingChannelOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannelOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannelOptionsVtbl {
        unsafe extern "system" fn Group<Impl: ILoggingChannelOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Impl: ILoggingChannelOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroup(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoggingChannelOptions>, ::windows::core::GetTrustLevel, Group::<Impl, IMPL_OFFSET>, SetGroup::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannelOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelOptionsFactoryImpl: Sized {
    fn Create(&self, group: &::windows::core::GUID) -> ::windows::core::Result<LoggingChannelOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingChannelOptionsFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannelOptionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingChannelOptionsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannelOptionsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannelOptionsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ILoggingChannelOptionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&group as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoggingChannelOptionsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannelOptionsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingFieldsImpl: Sized {
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn BeginStruct(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BeginStructWithTags(&self, name: &::windows::core::HSTRING, tags: i32) -> ::windows::core::Result<()>;
    fn EndStruct(&self) -> ::windows::core::Result<()>;
    fn AddEmpty(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddEmptyWithFormat(&self, name: &::windows::core::HSTRING, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddEmptyWithFormatAndTags(&self, name: &::windows::core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt8(&self, name: &::windows::core::HSTRING, value: u8) -> ::windows::core::Result<()>;
    fn AddUInt8WithFormat(&self, name: &::windows::core::HSTRING, value: u8, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt8WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt8Array(&self, name: &::windows::core::HSTRING, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt8ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<u8 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt8ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<u8 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt16(&self, name: &::windows::core::HSTRING, value: i16) -> ::windows::core::Result<()>;
    fn AddInt16WithFormat(&self, name: &::windows::core::HSTRING, value: i16, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt16WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt16Array(&self, name: &::windows::core::HSTRING, value: &[<i16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddInt16ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<i16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt16ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<i16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt16(&self, name: &::windows::core::HSTRING, value: u16) -> ::windows::core::Result<()>;
    fn AddUInt16WithFormat(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt16WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt16Array(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt16ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt16ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt32(&self, name: &::windows::core::HSTRING, value: i32) -> ::windows::core::Result<()>;
    fn AddInt32WithFormat(&self, name: &::windows::core::HSTRING, value: i32, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt32WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt32Array(&self, name: &::windows::core::HSTRING, value: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddInt32ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<i32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt32ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<i32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt32(&self, name: &::windows::core::HSTRING, value: u32) -> ::windows::core::Result<()>;
    fn AddUInt32WithFormat(&self, name: &::windows::core::HSTRING, value: u32, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt32WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt32Array(&self, name: &::windows::core::HSTRING, value: &[<u32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt32ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<u32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt32ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<u32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt64(&self, name: &::windows::core::HSTRING, value: i64) -> ::windows::core::Result<()>;
    fn AddInt64WithFormat(&self, name: &::windows::core::HSTRING, value: i64, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt64WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt64Array(&self, name: &::windows::core::HSTRING, value: &[<i64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddInt64ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<i64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt64ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<i64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt64(&self, name: &::windows::core::HSTRING, value: u64) -> ::windows::core::Result<()>;
    fn AddUInt64WithFormat(&self, name: &::windows::core::HSTRING, value: u64, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt64WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt64Array(&self, name: &::windows::core::HSTRING, value: &[<u64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt64ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<u64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt64ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<u64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSingle(&self, name: &::windows::core::HSTRING, value: f32) -> ::windows::core::Result<()>;
    fn AddSingleWithFormat(&self, name: &::windows::core::HSTRING, value: f32, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSingleWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSingleArray(&self, name: &::windows::core::HSTRING, value: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddSingleArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<f32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSingleArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<f32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDouble(&self, name: &::windows::core::HSTRING, value: f64) -> ::windows::core::Result<()>;
    fn AddDoubleWithFormat(&self, name: &::windows::core::HSTRING, value: f64, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDoubleWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDoubleArray(&self, name: &::windows::core::HSTRING, value: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddDoubleArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<f64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDoubleArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<f64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddChar16(&self, name: &::windows::core::HSTRING, value: u16) -> ::windows::core::Result<()>;
    fn AddChar16WithFormat(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddChar16WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddChar16Array(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddChar16ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddChar16ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddBoolean(&self, name: &::windows::core::HSTRING, value: bool) -> ::windows::core::Result<()>;
    fn AddBooleanWithFormat(&self, name: &::windows::core::HSTRING, value: bool, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddBooleanWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddBooleanArray(&self, name: &::windows::core::HSTRING, value: &[<bool as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddBooleanArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<bool as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddBooleanArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<bool as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddString(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddStringWithFormat(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddStringWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddStringArray(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddStringArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddStringArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddGuid(&self, name: &::windows::core::HSTRING, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AddGuidWithFormat(&self, name: &::windows::core::HSTRING, value: &::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddGuidWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddGuidArray(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddGuidArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddGuidArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDateTime(&self, name: &::windows::core::HSTRING, value: &super::DateTime) -> ::windows::core::Result<()>;
    fn AddDateTimeWithFormat(&self, name: &::windows::core::HSTRING, value: &super::DateTime, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDateTimeWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDateTimeArray(&self, name: &::windows::core::HSTRING, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddDateTimeArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDateTimeArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddTimeSpan(&self, name: &::windows::core::HSTRING, value: &super::TimeSpan) -> ::windows::core::Result<()>;
    fn AddTimeSpanWithFormat(&self, name: &::windows::core::HSTRING, value: &super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddTimeSpanWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddTimeSpanArray(&self, name: &::windows::core::HSTRING, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddTimeSpanArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddTimeSpanArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddPoint(&self, name: &::windows::core::HSTRING, value: &super::Point) -> ::windows::core::Result<()>;
    fn AddPointWithFormat(&self, name: &::windows::core::HSTRING, value: &super::Point, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddPointWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddPointArray(&self, name: &::windows::core::HSTRING, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddPointArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddPointArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSize(&self, name: &::windows::core::HSTRING, value: &super::Size) -> ::windows::core::Result<()>;
    fn AddSizeWithFormat(&self, name: &::windows::core::HSTRING, value: &super::Size, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSizeWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSizeArray(&self, name: &::windows::core::HSTRING, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddSizeArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSizeArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddRect(&self, name: &::windows::core::HSTRING, value: &super::Rect) -> ::windows::core::Result<()>;
    fn AddRectWithFormat(&self, name: &::windows::core::HSTRING, value: &super::Rect, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddRectWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddRectArray(&self, name: &::windows::core::HSTRING, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddRectArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddRectArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingFields {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingFields";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingFieldsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingFieldsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingFieldsVtbl {
        unsafe extern "system" fn Clear<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn BeginStruct<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginStruct(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BeginStructWithTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginStructWithTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), tags).into()
        }
        unsafe extern "system" fn EndStruct<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndStruct().into()
        }
        unsafe extern "system" fn AddEmpty<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEmpty(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddEmptyWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEmptyWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddEmptyWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEmptyWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddUInt8<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddUInt8WithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddUInt8WithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddUInt8Array<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddUInt8ArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddUInt8ArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddInt16<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddInt16WithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddInt16WithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddInt16Array<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddInt16ArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddInt16ArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddUInt16<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddUInt16WithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddUInt16WithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddUInt16Array<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddUInt16ArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddUInt16ArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddInt32<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddInt32WithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddInt32WithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddInt32Array<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddInt32ArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddInt32ArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddUInt32<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddUInt32WithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddUInt32WithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddUInt32Array<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddUInt32ArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddUInt32ArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddInt64<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddInt64WithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddInt64WithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddInt64Array<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddInt64ArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddInt64ArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddUInt64<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddUInt64WithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddUInt64WithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddUInt64Array<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddUInt64ArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddUInt64ArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddSingle<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingle(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddSingleWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingleWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddSingleWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingleWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddSingleArray<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingleArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddSingleArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingleArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddSingleArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingleArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddDouble<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDouble(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddDoubleWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDoubleWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddDoubleWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDoubleWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddDoubleArray<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDoubleArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddDoubleArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDoubleArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddDoubleArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDoubleArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddChar16<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddChar16WithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddChar16WithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddChar16Array<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddChar16ArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddChar16ArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddBoolean<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBoolean(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddBooleanWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBooleanWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddBooleanWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBooleanWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddBooleanArray<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBooleanArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddBooleanArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBooleanArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddBooleanArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBooleanArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddString<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddString(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddStringWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddStringWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddStringArray<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddStringArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddStringArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddGuid<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuid(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddGuidWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuidWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddGuidWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuidWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddGuidArray<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuidArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddGuidArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuidArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddGuidArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuidArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddDateTime<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTime(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::DateTime as ::windows::core::Abi>::Abi as *const <super::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddDateTimeWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTimeWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::DateTime as ::windows::core::Abi>::Abi as *const <super::DateTime as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddDateTimeWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTimeWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::DateTime as ::windows::core::Abi>::Abi as *const <super::DateTime as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddDateTimeArray<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTimeArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddDateTimeArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTimeArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddDateTimeArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTimeArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddTimeSpan<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpan(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::TimeSpan as ::windows::core::Abi>::Abi as *const <super::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddTimeSpanWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpanWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::TimeSpan as ::windows::core::Abi>::Abi as *const <super::TimeSpan as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddTimeSpanWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpanWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::TimeSpan as ::windows::core::Abi>::Abi as *const <super::TimeSpan as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddTimeSpanArray<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpanArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddTimeSpanArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpanArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddTimeSpanArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpanArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddPoint<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPoint(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Point as ::windows::core::Abi>::Abi as *const <super::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddPointWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPointWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Point as ::windows::core::Abi>::Abi as *const <super::Point as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddPointWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPointWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Point as ::windows::core::Abi>::Abi as *const <super::Point as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddPointArray<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPointArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddPointArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPointArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddPointArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPointArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddSize<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSize(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Size as ::windows::core::Abi>::Abi as *const <super::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddSizeWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSizeWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Size as ::windows::core::Abi>::Abi as *const <super::Size as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddSizeWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSizeWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Size as ::windows::core::Abi>::Abi as *const <super::Size as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddSizeArray<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSizeArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddSizeArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSizeArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddSizeArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSizeArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddRect<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRect(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Rect as ::windows::core::Abi>::Abi as *const <super::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddRectWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRectWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Rect as ::windows::core::Abi>::Abi as *const <super::Rect as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddRectWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRectWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Rect as ::windows::core::Abi>::Abi as *const <super::Rect as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddRectArray<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRectArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddRectArrayWithFormat<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRectArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddRectArrayWithFormatAndTags<Impl: ILoggingFieldsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRectArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILoggingFields>,
            ::windows::core::GetTrustLevel,
            Clear::<Impl, IMPL_OFFSET>,
            BeginStruct::<Impl, IMPL_OFFSET>,
            BeginStructWithTags::<Impl, IMPL_OFFSET>,
            EndStruct::<Impl, IMPL_OFFSET>,
            AddEmpty::<Impl, IMPL_OFFSET>,
            AddEmptyWithFormat::<Impl, IMPL_OFFSET>,
            AddEmptyWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt8::<Impl, IMPL_OFFSET>,
            AddUInt8WithFormat::<Impl, IMPL_OFFSET>,
            AddUInt8WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt8Array::<Impl, IMPL_OFFSET>,
            AddUInt8ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddUInt8ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt16::<Impl, IMPL_OFFSET>,
            AddInt16WithFormat::<Impl, IMPL_OFFSET>,
            AddInt16WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt16Array::<Impl, IMPL_OFFSET>,
            AddInt16ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddInt16ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt16::<Impl, IMPL_OFFSET>,
            AddUInt16WithFormat::<Impl, IMPL_OFFSET>,
            AddUInt16WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt16Array::<Impl, IMPL_OFFSET>,
            AddUInt16ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddUInt16ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt32::<Impl, IMPL_OFFSET>,
            AddInt32WithFormat::<Impl, IMPL_OFFSET>,
            AddInt32WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt32Array::<Impl, IMPL_OFFSET>,
            AddInt32ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddInt32ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt32::<Impl, IMPL_OFFSET>,
            AddUInt32WithFormat::<Impl, IMPL_OFFSET>,
            AddUInt32WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt32Array::<Impl, IMPL_OFFSET>,
            AddUInt32ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddUInt32ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt64::<Impl, IMPL_OFFSET>,
            AddInt64WithFormat::<Impl, IMPL_OFFSET>,
            AddInt64WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt64Array::<Impl, IMPL_OFFSET>,
            AddInt64ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddInt64ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt64::<Impl, IMPL_OFFSET>,
            AddUInt64WithFormat::<Impl, IMPL_OFFSET>,
            AddUInt64WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt64Array::<Impl, IMPL_OFFSET>,
            AddUInt64ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddUInt64ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddSingle::<Impl, IMPL_OFFSET>,
            AddSingleWithFormat::<Impl, IMPL_OFFSET>,
            AddSingleWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddSingleArray::<Impl, IMPL_OFFSET>,
            AddSingleArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddSingleArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddDouble::<Impl, IMPL_OFFSET>,
            AddDoubleWithFormat::<Impl, IMPL_OFFSET>,
            AddDoubleWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddDoubleArray::<Impl, IMPL_OFFSET>,
            AddDoubleArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddDoubleArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddChar16::<Impl, IMPL_OFFSET>,
            AddChar16WithFormat::<Impl, IMPL_OFFSET>,
            AddChar16WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddChar16Array::<Impl, IMPL_OFFSET>,
            AddChar16ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddChar16ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddBoolean::<Impl, IMPL_OFFSET>,
            AddBooleanWithFormat::<Impl, IMPL_OFFSET>,
            AddBooleanWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddBooleanArray::<Impl, IMPL_OFFSET>,
            AddBooleanArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddBooleanArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddString::<Impl, IMPL_OFFSET>,
            AddStringWithFormat::<Impl, IMPL_OFFSET>,
            AddStringWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddStringArray::<Impl, IMPL_OFFSET>,
            AddStringArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddStringArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddGuid::<Impl, IMPL_OFFSET>,
            AddGuidWithFormat::<Impl, IMPL_OFFSET>,
            AddGuidWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddGuidArray::<Impl, IMPL_OFFSET>,
            AddGuidArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddGuidArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddDateTime::<Impl, IMPL_OFFSET>,
            AddDateTimeWithFormat::<Impl, IMPL_OFFSET>,
            AddDateTimeWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddDateTimeArray::<Impl, IMPL_OFFSET>,
            AddDateTimeArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddDateTimeArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddTimeSpan::<Impl, IMPL_OFFSET>,
            AddTimeSpanWithFormat::<Impl, IMPL_OFFSET>,
            AddTimeSpanWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddTimeSpanArray::<Impl, IMPL_OFFSET>,
            AddTimeSpanArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddTimeSpanArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddPoint::<Impl, IMPL_OFFSET>,
            AddPointWithFormat::<Impl, IMPL_OFFSET>,
            AddPointWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddPointArray::<Impl, IMPL_OFFSET>,
            AddPointArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddPointArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddSize::<Impl, IMPL_OFFSET>,
            AddSizeWithFormat::<Impl, IMPL_OFFSET>,
            AddSizeWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddSizeArray::<Impl, IMPL_OFFSET>,
            AddSizeArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddSizeArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddRect::<Impl, IMPL_OFFSET>,
            AddRectWithFormat::<Impl, IMPL_OFFSET>,
            AddRectWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddRectArray::<Impl, IMPL_OFFSET>,
            AddRectArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddRectArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingFields as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingOptionsImpl: Sized {
    fn Keywords(&self) -> ::windows::core::Result<i64>;
    fn SetKeywords(&self, value: i64) -> ::windows::core::Result<()>;
    fn Tags(&self) -> ::windows::core::Result<i32>;
    fn SetTags(&self, value: i32) -> ::windows::core::Result<()>;
    fn Task(&self) -> ::windows::core::Result<i16>;
    fn SetTask(&self, value: i16) -> ::windows::core::Result<()>;
    fn Opcode(&self) -> ::windows::core::Result<LoggingOpcode>;
    fn SetOpcode(&self, value: LoggingOpcode) -> ::windows::core::Result<()>;
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetActivityId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RelatedActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetRelatedActivityId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingOptionsVtbl {
        unsafe extern "system" fn Keywords<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Keywords() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeywords<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeywords(value).into()
        }
        unsafe extern "system" fn Tags<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tags() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTags<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTags(value).into()
        }
        unsafe extern "system" fn Task<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Task() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTask<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTask(value).into()
        }
        unsafe extern "system" fn Opcode<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LoggingOpcode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opcode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpcode<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LoggingOpcode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpcode(value).into()
        }
        unsafe extern "system" fn ActivityId<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityId<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivityId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelatedActivityId<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelatedActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelatedActivityId<Impl: ILoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelatedActivityId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILoggingOptions>,
            ::windows::core::GetTrustLevel,
            Keywords::<Impl, IMPL_OFFSET>,
            SetKeywords::<Impl, IMPL_OFFSET>,
            Tags::<Impl, IMPL_OFFSET>,
            SetTags::<Impl, IMPL_OFFSET>,
            Task::<Impl, IMPL_OFFSET>,
            SetTask::<Impl, IMPL_OFFSET>,
            Opcode::<Impl, IMPL_OFFSET>,
            SetOpcode::<Impl, IMPL_OFFSET>,
            ActivityId::<Impl, IMPL_OFFSET>,
            SetActivityId::<Impl, IMPL_OFFSET>,
            RelatedActivityId::<Impl, IMPL_OFFSET>,
            SetRelatedActivityId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingOptionsFactoryImpl: Sized {
    fn CreateWithKeywords(&self, keywords: i64) -> ::windows::core::Result<LoggingOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingOptionsFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingOptionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingOptionsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingOptionsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingOptionsFactoryVtbl {
        unsafe extern "system" fn CreateWithKeywords<Impl: ILoggingOptionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: i64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithKeywords(keywords) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoggingOptionsFactory>, ::windows::core::GetTrustLevel, CreateWithKeywords::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingOptionsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Storage")]
pub trait ILoggingSessionImpl: Sized + IClosableImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SaveToFileAsync(&self, folder: &::core::option::Option<super::super::Storage::IStorageFolder>, filename: &::windows::core::HSTRING) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn AddLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn AddLoggingChannelWithLevel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows::core::Result<()>;
    fn RemoveLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows::core::RuntimeName for ILoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingSession";
}
#[cfg(feature = "Storage")]
impl ILoggingSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingSessionVtbl {
        unsafe extern "system" fn Name<Impl: ILoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToFileAsync<Impl: ILoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveToFileAsync(&*(&folder as *const <super::super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType), &*(&filename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Impl: ILoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Impl: ILoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr, maxlevel: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLoggingChannelWithLevel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Impl: ILoggingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoggingSession>, ::windows::core::GetTrustLevel, Name::<Impl, IMPL_OFFSET>, SaveToFileAsync::<Impl, IMPL_OFFSET>, AddLoggingChannel::<Impl, IMPL_OFFSET>, AddLoggingChannelWithLevel::<Impl, IMPL_OFFSET>, RemoveLoggingChannel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingSessionFactoryImpl: Sized {
    fn Create(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingSessionFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingSessionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingSessionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingSessionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingSessionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ILoggingSessionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoggingSessionFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingSessionFactory as ::windows::core::Interface>::IID
    }
}
pub trait ILoggingTargetImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool>;
    fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool>;
    fn LogEvent(&self, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogEventWithFields(&self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<()>;
    fn LogEventWithFieldsAndLevel(&self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LogEventWithFieldsAndOptions(&self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<()>;
    fn StartActivity(&self, starteventname: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFields(&self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndLevel(&self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndOptions(&self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<LoggingActivity>;
}
impl ::windows::core::RuntimeName for ILoggingTarget {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingTarget";
}
impl ILoggingTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingTargetVtbl {
        unsafe extern "system" fn IsEnabled<Impl: ILoggingTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabledWithLevel<Impl: ILoggingTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabledWithLevel(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledWithLevelAndKeywords<Impl: ILoggingTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabledWithLevelAndKeywords(level, keywords) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogEvent<Impl: ILoggingTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogEvent(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogEventWithFields<Impl: ILoggingTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogEventWithFields(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndLevel<Impl: ILoggingTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogEventWithFieldsAndLevel(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), level).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndOptions<Impl: ILoggingTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .LogEventWithFieldsAndOptions(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), level, &*(&options as *const <LoggingOptions as ::windows::core::Abi>::Abi as *const <LoggingOptions as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn StartActivity<Impl: ILoggingTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartActivity(&*(&starteventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFields<Impl: ILoggingTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartActivityWithFields(&*(&starteventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndLevel<Impl: ILoggingTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartActivityWithFieldsAndLevel(&*(&starteventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndOptions<Impl: ILoggingTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartActivityWithFieldsAndOptions(&*(&starteventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), level, &*(&options as *const <LoggingOptions as ::windows::core::Abi>::Abi as *const <LoggingOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILoggingTarget>,
            ::windows::core::GetTrustLevel,
            IsEnabled::<Impl, IMPL_OFFSET>,
            IsEnabledWithLevel::<Impl, IMPL_OFFSET>,
            IsEnabledWithLevelAndKeywords::<Impl, IMPL_OFFSET>,
            LogEvent::<Impl, IMPL_OFFSET>,
            LogEventWithFields::<Impl, IMPL_OFFSET>,
            LogEventWithFieldsAndLevel::<Impl, IMPL_OFFSET>,
            LogEventWithFieldsAndOptions::<Impl, IMPL_OFFSET>,
            StartActivity::<Impl, IMPL_OFFSET>,
            StartActivityWithFields::<Impl, IMPL_OFFSET>,
            StartActivityWithFieldsAndLevel::<Impl, IMPL_OFFSET>,
            StartActivityWithFieldsAndOptions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITracingStatusChangedEventArgsImpl: Sized {
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn TraceLevel(&self) -> ::windows::core::Result<CausalityTraceLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITracingStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ITracingStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITracingStatusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITracingStatusChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITracingStatusChangedEventArgsVtbl {
        unsafe extern "system" fn Enabled<Impl: ITracingStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TraceLevel<Impl: ITracingStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CausalityTraceLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TraceLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITracingStatusChangedEventArgs>, ::windows::core::GetTrustLevel, Enabled::<Impl, IMPL_OFFSET>, TraceLevel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITracingStatusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
