#[cfg(feature = "implement_exclusive")]
pub trait IAsyncCausalityTracerStatics_Impl: Sized {
    fn TraceOperationCreation(&mut self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, operationname: &::windows::core::HSTRING, relatedcontext: u64) -> ::windows::core::Result<()>;
    fn TraceOperationCompletion(&mut self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows::core::Result<()>;
    fn TraceOperationRelation(&mut self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows::core::Result<()>;
    fn TraceSynchronousWorkStart(&mut self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows::core::Result<()>;
    fn TraceSynchronousWorkCompletion(&mut self, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::core::Result<()>;
    fn TracingStatusChanged(&mut self, handler: &::core::option::Option<super::EventHandler<TracingStatusChangedEventArgs>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveTracingStatusChanged(&mut self, cookie: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAsyncCausalityTracerStatics {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IAsyncCausalityTracerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAsyncCausalityTracerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncCausalityTracerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncCausalityTracerStatics_Vtbl {
        unsafe extern "system" fn TraceOperationCreation<Impl: IAsyncCausalityTracerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, operationname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, relatedcontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TraceOperationCreation(tracelevel, source, &*(&platformid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), operationid, &*(&operationname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), relatedcontext).into()
        }
        unsafe extern "system" fn TraceOperationCompletion<Impl: IAsyncCausalityTracerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TraceOperationCompletion(tracelevel, source, &*(&platformid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), operationid, status).into()
        }
        unsafe extern "system" fn TraceOperationRelation<Impl: IAsyncCausalityTracerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TraceOperationRelation(tracelevel, source, &*(&platformid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), operationid, relation).into()
        }
        unsafe extern "system" fn TraceSynchronousWorkStart<Impl: IAsyncCausalityTracerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TraceSynchronousWorkStart(tracelevel, source, &*(&platformid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), operationid, work).into()
        }
        unsafe extern "system" fn TraceSynchronousWorkCompletion<Impl: IAsyncCausalityTracerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TraceSynchronousWorkCompletion(tracelevel, source, work).into()
        }
        unsafe extern "system" fn TracingStatusChanged<Impl: IAsyncCausalityTracerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTracingStatusChanged<Impl: IAsyncCausalityTracerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTracingStatusChanged(&*(&cookie as *const <super::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsyncCausalityTracerStatics, BASE_OFFSET>(),
            TraceOperationCreation: TraceOperationCreation::<Impl, IMPL_OFFSET>,
            TraceOperationCompletion: TraceOperationCompletion::<Impl, IMPL_OFFSET>,
            TraceOperationRelation: TraceOperationRelation::<Impl, IMPL_OFFSET>,
            TraceSynchronousWorkStart: TraceSynchronousWorkStart::<Impl, IMPL_OFFSET>,
            TraceSynchronousWorkCompletion: TraceSynchronousWorkCompletion::<Impl, IMPL_OFFSET>,
            TracingStatusChanged: TracingStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveTracingStatusChanged: RemoveTracingStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncCausalityTracerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IErrorDetails_Impl: Sized {
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LongDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HelpUri(&mut self) -> ::windows::core::Result<super::Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IErrorDetails {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IErrorDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IErrorDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IErrorDetails_Vtbl {
        unsafe extern "system" fn Description<Impl: IErrorDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LongDescription<Impl: IErrorDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HelpUri<Impl: IErrorDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IErrorDetails, BASE_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            LongDescription: LongDescription::<Impl, IMPL_OFFSET>,
            HelpUri: HelpUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IErrorDetailsStatics_Impl: Sized {
    fn CreateFromHResultAsync(&mut self, errorcode: i32) -> ::windows::core::Result<super::IAsyncOperation<ErrorDetails>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IErrorDetailsStatics {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IErrorDetailsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IErrorDetailsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorDetailsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IErrorDetailsStatics_Vtbl {
        unsafe extern "system" fn CreateFromHResultAsync<Impl: IErrorDetailsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IErrorDetailsStatics, BASE_OFFSET>(),
            CreateFromHResultAsync: CreateFromHResultAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorDetailsStatics as ::windows::core::Interface>::IID
    }
}
pub trait IErrorReportingSettings_Impl: Sized {
    fn SetErrorOptions(&mut self, value: ErrorOptions) -> ::windows::core::Result<()>;
    fn GetErrorOptions(&mut self) -> ::windows::core::Result<ErrorOptions>;
}
impl ::windows::core::RuntimeName for IErrorReportingSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IErrorReportingSettings";
}
impl IErrorReportingSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorReportingSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IErrorReportingSettings_Vtbl {
        unsafe extern "system" fn SetErrorOptions<Impl: IErrorReportingSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ErrorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorOptions(value).into()
        }
        unsafe extern "system" fn GetErrorOptions<Impl: IErrorReportingSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ErrorOptions) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IErrorReportingSettings, BASE_OFFSET>(),
            SetErrorOptions: SetErrorOptions::<Impl, IMPL_OFFSET>,
            GetErrorOptions: GetErrorOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorReportingSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Storage")]
pub trait IFileLoggingSession_Impl: Sized + super::IClosable_Impl {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddLoggingChannel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn AddLoggingChannelWithLevel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows::core::Result<()>;
    fn RemoveLoggingChannel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn CloseAndSaveToFileAsync(&mut self) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn LogFileGenerated(&mut self, handler: &::core::option::Option<super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveLogFileGenerated(&mut self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows::core::RuntimeName for IFileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IFileLoggingSession";
}
#[cfg(feature = "Storage")]
impl IFileLoggingSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileLoggingSession_Vtbl {
        unsafe extern "system" fn Name<Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddLoggingChannel<Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr, maxlevel: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLoggingChannelWithLevel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CloseAndSaveToFileAsync<Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LogFileGenerated<Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveLogFileGenerated<Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLogFileGenerated(&*(&token as *const <super::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileLoggingSession, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Impl, IMPL_OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Impl, IMPL_OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Impl, IMPL_OFFSET>,
            CloseAndSaveToFileAsync: CloseAndSaveToFileAsync::<Impl, IMPL_OFFSET>,
            LogFileGenerated: LogFileGenerated::<Impl, IMPL_OFFSET>,
            RemoveLogFileGenerated: RemoveLogFileGenerated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileLoggingSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileLoggingSessionFactory_Impl: Sized {
    fn Create(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<FileLoggingSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileLoggingSessionFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IFileLoggingSessionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFileLoggingSessionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSessionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileLoggingSessionFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IFileLoggingSessionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFileLoggingSessionFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileLoggingSessionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
pub trait ILogFileGeneratedEventArgs_Impl: Sized {
    fn File(&mut self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILogFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILogFileGeneratedEventArgs";
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ILogFileGeneratedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILogFileGeneratedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILogFileGeneratedEventArgs_Vtbl {
        unsafe extern "system" fn File<Impl: ILogFileGeneratedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILogFileGeneratedEventArgs, BASE_OFFSET>(), File: File::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILogFileGeneratedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingActivity_Impl: Sized + super::IClosable_Impl {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingActivity {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingActivity";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingActivity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingActivity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingActivity_Vtbl {
        unsafe extern "system" fn Name<Impl: ILoggingActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: ILoggingActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingActivity, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingActivity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingActivity2_Impl: Sized + super::IClosable_Impl + ILoggingActivity_Impl + ILoggingTarget_Impl {
    fn Channel(&mut self) -> ::windows::core::Result<LoggingChannel>;
    fn StopActivity(&mut self, stopeventname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StopActivityWithFields(&mut self, stopeventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<()>;
    fn StopActivityWithFieldsAndOptions(&mut self, stopeventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingActivity2 {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingActivity2";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingActivity2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingActivity2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingActivity2_Vtbl {
        unsafe extern "system" fn Channel<Impl: ILoggingActivity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StopActivity<Impl: ILoggingActivity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopActivity(&*(&stopeventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopActivityWithFields<Impl: ILoggingActivity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopActivityWithFields(&*(&stopeventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopActivityWithFieldsAndOptions<Impl: ILoggingActivity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .StopActivityWithFieldsAndOptions(&*(&stopeventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LoggingOptions as ::windows::core::Abi>::Abi as *const <LoggingOptions as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingActivity2, BASE_OFFSET>(),
            Channel: Channel::<Impl, IMPL_OFFSET>,
            StopActivity: StopActivity::<Impl, IMPL_OFFSET>,
            StopActivityWithFields: StopActivityWithFields::<Impl, IMPL_OFFSET>,
            StopActivityWithFieldsAndOptions: StopActivityWithFieldsAndOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingActivity2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingActivityFactory_Impl: Sized {
    fn CreateLoggingActivity(&mut self, activityname: &::windows::core::HSTRING, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<LoggingActivity>;
    fn CreateLoggingActivityWithLevel(&mut self, activityname: &::windows::core::HSTRING, loggingchannel: &::core::option::Option<ILoggingChannel>, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingActivityFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingActivityFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingActivityFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingActivityFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingActivityFactory_Vtbl {
        unsafe extern "system" fn CreateLoggingActivity<Impl: ILoggingActivityFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, loggingchannel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateLoggingActivityWithLevel<Impl: ILoggingActivityFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, loggingchannel: ::windows::core::RawPtr, level: LoggingLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingActivityFactory, BASE_OFFSET>(),
            CreateLoggingActivity: CreateLoggingActivity::<Impl, IMPL_OFFSET>,
            CreateLoggingActivityWithLevel: CreateLoggingActivityWithLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingActivityFactory as ::windows::core::Interface>::IID
    }
}
pub trait ILoggingChannel_Impl: Sized + super::IClosable_Impl {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn Level(&mut self) -> ::windows::core::Result<LoggingLevel>;
    fn LogMessage(&mut self, eventstring: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogMessageWithLevel(&mut self, eventstring: &::windows::core::HSTRING, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LogValuePair(&mut self, value1: &::windows::core::HSTRING, value2: i32) -> ::windows::core::Result<()>;
    fn LogValuePairWithLevel(&mut self, value1: &::windows::core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LoggingEnabled(&mut self, handler: &::core::option::Option<super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveLoggingEnabled(&mut self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannel";
}
impl ILoggingChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannel_Vtbl {
        unsafe extern "system" fn Name<Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Enabled<Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Level<Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LoggingLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LogMessage<Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogMessage(&*(&eventstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogMessageWithLevel<Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogMessageWithLevel(&*(&eventstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), level).into()
        }
        unsafe extern "system" fn LogValuePair<Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogValuePair(&*(&value1 as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value2).into()
        }
        unsafe extern "system" fn LogValuePairWithLevel<Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogValuePairWithLevel(&*(&value1 as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value2, level).into()
        }
        unsafe extern "system" fn LoggingEnabled<Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveLoggingEnabled<Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLoggingEnabled(&*(&token as *const <super::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingChannel, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            Level: Level::<Impl, IMPL_OFFSET>,
            LogMessage: LogMessage::<Impl, IMPL_OFFSET>,
            LogMessageWithLevel: LogMessageWithLevel::<Impl, IMPL_OFFSET>,
            LogValuePair: LogValuePair::<Impl, IMPL_OFFSET>,
            LogValuePairWithLevel: LogValuePairWithLevel::<Impl, IMPL_OFFSET>,
            LoggingEnabled: LoggingEnabled::<Impl, IMPL_OFFSET>,
            RemoveLoggingEnabled: RemoveLoggingEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannel2_Impl: Sized + super::IClosable_Impl + ILoggingChannel_Impl + ILoggingTarget_Impl {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingChannel2 {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannel2";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingChannel2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannel2_Vtbl {
        unsafe extern "system" fn Id<Impl: ILoggingChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingChannel2, BASE_OFFSET>(), Id: Id::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannel2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelFactory_Impl: Sized {
    fn Create(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingChannel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingChannelFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannelFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingChannelFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannelFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannelFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ILoggingChannelFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingChannelFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannelFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelFactory2_Impl: Sized {
    fn CreateWithOptions(&mut self, name: &::windows::core::HSTRING, options: &::core::option::Option<LoggingChannelOptions>) -> ::windows::core::Result<LoggingChannel>;
    fn CreateWithOptionsAndId(&mut self, name: &::windows::core::HSTRING, options: &::core::option::Option<LoggingChannelOptions>, id: &::windows::core::GUID) -> ::windows::core::Result<LoggingChannel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingChannelFactory2 {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannelFactory2";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingChannelFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannelFactory2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannelFactory2_Vtbl {
        unsafe extern "system" fn CreateWithOptions<Impl: ILoggingChannelFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithOptionsAndId<Impl: ILoggingChannelFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, id: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingChannelFactory2, BASE_OFFSET>(),
            CreateWithOptions: CreateWithOptions::<Impl, IMPL_OFFSET>,
            CreateWithOptionsAndId: CreateWithOptionsAndId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannelFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelOptions_Impl: Sized {
    fn Group(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetGroup(&mut self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingChannelOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannelOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingChannelOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannelOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannelOptions_Vtbl {
        unsafe extern "system" fn Group<Impl: ILoggingChannelOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetGroup<Impl: ILoggingChannelOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroup(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingChannelOptions, BASE_OFFSET>(),
            Group: Group::<Impl, IMPL_OFFSET>,
            SetGroup: SetGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannelOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelOptionsFactory_Impl: Sized {
    fn Create(&mut self, group: &::windows::core::GUID) -> ::windows::core::Result<LoggingChannelOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingChannelOptionsFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannelOptionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingChannelOptionsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannelOptionsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingChannelOptionsFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ILoggingChannelOptionsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingChannelOptionsFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannelOptionsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingFields_Impl: Sized {
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn BeginStruct(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BeginStructWithTags(&mut self, name: &::windows::core::HSTRING, tags: i32) -> ::windows::core::Result<()>;
    fn EndStruct(&mut self) -> ::windows::core::Result<()>;
    fn AddEmpty(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddEmptyWithFormat(&mut self, name: &::windows::core::HSTRING, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddEmptyWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt8(&mut self, name: &::windows::core::HSTRING, value: u8) -> ::windows::core::Result<()>;
    fn AddUInt8WithFormat(&mut self, name: &::windows::core::HSTRING, value: u8, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt8WithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt8Array(&mut self, name: &::windows::core::HSTRING, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt8ArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<u8 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt8ArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<u8 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt16(&mut self, name: &::windows::core::HSTRING, value: i16) -> ::windows::core::Result<()>;
    fn AddInt16WithFormat(&mut self, name: &::windows::core::HSTRING, value: i16, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt16WithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt16Array(&mut self, name: &::windows::core::HSTRING, value: &[<i16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddInt16ArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<i16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt16ArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<i16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt16(&mut self, name: &::windows::core::HSTRING, value: u16) -> ::windows::core::Result<()>;
    fn AddUInt16WithFormat(&mut self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt16WithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt16Array(&mut self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt16ArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt16ArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt32(&mut self, name: &::windows::core::HSTRING, value: i32) -> ::windows::core::Result<()>;
    fn AddInt32WithFormat(&mut self, name: &::windows::core::HSTRING, value: i32, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt32WithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt32Array(&mut self, name: &::windows::core::HSTRING, value: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddInt32ArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<i32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt32ArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<i32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt32(&mut self, name: &::windows::core::HSTRING, value: u32) -> ::windows::core::Result<()>;
    fn AddUInt32WithFormat(&mut self, name: &::windows::core::HSTRING, value: u32, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt32WithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt32Array(&mut self, name: &::windows::core::HSTRING, value: &[<u32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt32ArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<u32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt32ArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<u32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt64(&mut self, name: &::windows::core::HSTRING, value: i64) -> ::windows::core::Result<()>;
    fn AddInt64WithFormat(&mut self, name: &::windows::core::HSTRING, value: i64, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt64WithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt64Array(&mut self, name: &::windows::core::HSTRING, value: &[<i64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddInt64ArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<i64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt64ArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<i64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt64(&mut self, name: &::windows::core::HSTRING, value: u64) -> ::windows::core::Result<()>;
    fn AddUInt64WithFormat(&mut self, name: &::windows::core::HSTRING, value: u64, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt64WithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt64Array(&mut self, name: &::windows::core::HSTRING, value: &[<u64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt64ArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<u64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt64ArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<u64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSingle(&mut self, name: &::windows::core::HSTRING, value: f32) -> ::windows::core::Result<()>;
    fn AddSingleWithFormat(&mut self, name: &::windows::core::HSTRING, value: f32, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSingleWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSingleArray(&mut self, name: &::windows::core::HSTRING, value: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddSingleArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<f32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSingleArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<f32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDouble(&mut self, name: &::windows::core::HSTRING, value: f64) -> ::windows::core::Result<()>;
    fn AddDoubleWithFormat(&mut self, name: &::windows::core::HSTRING, value: f64, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDoubleWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDoubleArray(&mut self, name: &::windows::core::HSTRING, value: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddDoubleArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<f64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDoubleArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<f64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddChar16(&mut self, name: &::windows::core::HSTRING, value: u16) -> ::windows::core::Result<()>;
    fn AddChar16WithFormat(&mut self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddChar16WithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddChar16Array(&mut self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddChar16ArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddChar16ArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddBoolean(&mut self, name: &::windows::core::HSTRING, value: bool) -> ::windows::core::Result<()>;
    fn AddBooleanWithFormat(&mut self, name: &::windows::core::HSTRING, value: bool, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddBooleanWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddBooleanArray(&mut self, name: &::windows::core::HSTRING, value: &[<bool as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddBooleanArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<bool as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddBooleanArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<bool as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddString(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddStringWithFormat(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddStringWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddStringArray(&mut self, name: &::windows::core::HSTRING, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddStringArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddStringArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddGuid(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AddGuidWithFormat(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddGuidWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddGuidArray(&mut self, name: &::windows::core::HSTRING, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddGuidArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddGuidArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDateTime(&mut self, name: &::windows::core::HSTRING, value: &super::DateTime) -> ::windows::core::Result<()>;
    fn AddDateTimeWithFormat(&mut self, name: &::windows::core::HSTRING, value: &super::DateTime, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDateTimeWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDateTimeArray(&mut self, name: &::windows::core::HSTRING, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddDateTimeArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDateTimeArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddTimeSpan(&mut self, name: &::windows::core::HSTRING, value: &super::TimeSpan) -> ::windows::core::Result<()>;
    fn AddTimeSpanWithFormat(&mut self, name: &::windows::core::HSTRING, value: &super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddTimeSpanWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddTimeSpanArray(&mut self, name: &::windows::core::HSTRING, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddTimeSpanArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddTimeSpanArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddPoint(&mut self, name: &::windows::core::HSTRING, value: &super::Point) -> ::windows::core::Result<()>;
    fn AddPointWithFormat(&mut self, name: &::windows::core::HSTRING, value: &super::Point, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddPointWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddPointArray(&mut self, name: &::windows::core::HSTRING, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddPointArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddPointArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSize(&mut self, name: &::windows::core::HSTRING, value: &super::Size) -> ::windows::core::Result<()>;
    fn AddSizeWithFormat(&mut self, name: &::windows::core::HSTRING, value: &super::Size, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSizeWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSizeArray(&mut self, name: &::windows::core::HSTRING, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddSizeArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSizeArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddRect(&mut self, name: &::windows::core::HSTRING, value: &super::Rect) -> ::windows::core::Result<()>;
    fn AddRectWithFormat(&mut self, name: &::windows::core::HSTRING, value: &super::Rect, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddRectWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddRectArray(&mut self, name: &::windows::core::HSTRING, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddRectArrayWithFormat(&mut self, name: &::windows::core::HSTRING, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddRectArrayWithFormatAndTags(&mut self, name: &::windows::core::HSTRING, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingFields {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingFields";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingFields_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingFields_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingFields_Vtbl {
        unsafe extern "system" fn Clear<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn BeginStruct<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginStruct(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BeginStructWithTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginStructWithTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), tags).into()
        }
        unsafe extern "system" fn EndStruct<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndStruct().into()
        }
        unsafe extern "system" fn AddEmpty<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEmpty(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddEmptyWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEmptyWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddEmptyWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEmptyWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddUInt8<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddUInt8WithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddUInt8WithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddUInt8Array<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddUInt8ArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddUInt8ArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt8ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddInt16<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddInt16WithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddInt16WithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddInt16Array<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddInt16ArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddInt16ArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt16ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddUInt16<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddUInt16WithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddUInt16WithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddUInt16Array<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddUInt16ArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddUInt16ArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt16ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddInt32<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddInt32WithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddInt32WithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddInt32Array<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddInt32ArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddInt32ArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddUInt32<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddUInt32WithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddUInt32WithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddUInt32Array<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddUInt32ArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddUInt32ArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt32ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddInt64<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddInt64WithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddInt64WithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddInt64Array<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddInt64ArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddInt64ArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt64ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddUInt64<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddUInt64WithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddUInt64WithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddUInt64Array<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddUInt64ArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddUInt64ArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddUInt64ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddSingle<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingle(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddSingleWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingleWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddSingleWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingleWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddSingleArray<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingleArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddSingleArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingleArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddSingleArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSingleArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddDouble<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDouble(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddDoubleWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDoubleWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddDoubleWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDoubleWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddDoubleArray<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDoubleArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddDoubleArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDoubleArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddDoubleArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDoubleArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddChar16<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddChar16WithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16WithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddChar16WithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16WithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddChar16Array<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16Array(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddChar16ArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16ArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddChar16ArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChar16ArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddBoolean<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBoolean(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn AddBooleanWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBooleanWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format).into()
        }
        unsafe extern "system" fn AddBooleanWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBooleanWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, format, tags).into()
        }
        unsafe extern "system" fn AddBooleanArray<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBooleanArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddBooleanArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBooleanArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddBooleanArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBooleanArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddString<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddString(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddStringWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddStringWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddStringArray<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddStringArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddStringArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddGuid<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuid(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddGuidWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuidWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddGuidWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuidWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddGuidArray<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuidArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddGuidArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuidArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddGuidArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddGuidArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddDateTime<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTime(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::DateTime as ::windows::core::Abi>::Abi as *const <super::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddDateTimeWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTimeWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::DateTime as ::windows::core::Abi>::Abi as *const <super::DateTime as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddDateTimeWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTimeWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::DateTime as ::windows::core::Abi>::Abi as *const <super::DateTime as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddDateTimeArray<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTimeArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddDateTimeArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTimeArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddDateTimeArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDateTimeArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddTimeSpan<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpan(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::TimeSpan as ::windows::core::Abi>::Abi as *const <super::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddTimeSpanWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpanWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::TimeSpan as ::windows::core::Abi>::Abi as *const <super::TimeSpan as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddTimeSpanWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpanWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::TimeSpan as ::windows::core::Abi>::Abi as *const <super::TimeSpan as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddTimeSpanArray<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpanArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddTimeSpanArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpanArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddTimeSpanArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTimeSpanArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddPoint<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPoint(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Point as ::windows::core::Abi>::Abi as *const <super::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddPointWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPointWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Point as ::windows::core::Abi>::Abi as *const <super::Point as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddPointWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPointWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Point as ::windows::core::Abi>::Abi as *const <super::Point as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddPointArray<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPointArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddPointArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPointArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddPointArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPointArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddSize<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSize(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Size as ::windows::core::Abi>::Abi as *const <super::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddSizeWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSizeWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Size as ::windows::core::Abi>::Abi as *const <super::Size as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddSizeWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSizeWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Size as ::windows::core::Abi>::Abi as *const <super::Size as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddSizeArray<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSizeArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddSizeArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSizeArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddSizeArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSizeArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        unsafe extern "system" fn AddRect<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRect(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Rect as ::windows::core::Abi>::Abi as *const <super::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddRectWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRectWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Rect as ::windows::core::Abi>::Abi as *const <super::Rect as ::windows::core::DefaultType>::DefaultType), format).into()
        }
        unsafe extern "system" fn AddRectWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRectWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Rect as ::windows::core::Abi>::Abi as *const <super::Rect as ::windows::core::DefaultType>::DefaultType), format, tags).into()
        }
        unsafe extern "system" fn AddRectArray<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRectArray(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn AddRectArrayWithFormat<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRectArrayWithFormat(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format).into()
        }
        unsafe extern "system" fn AddRectArrayWithFormatAndTags<Impl: ILoggingFields_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRectArrayWithFormatAndTags(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _), format, tags).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingFields, BASE_OFFSET>(),
            Clear: Clear::<Impl, IMPL_OFFSET>,
            BeginStruct: BeginStruct::<Impl, IMPL_OFFSET>,
            BeginStructWithTags: BeginStructWithTags::<Impl, IMPL_OFFSET>,
            EndStruct: EndStruct::<Impl, IMPL_OFFSET>,
            AddEmpty: AddEmpty::<Impl, IMPL_OFFSET>,
            AddEmptyWithFormat: AddEmptyWithFormat::<Impl, IMPL_OFFSET>,
            AddEmptyWithFormatAndTags: AddEmptyWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt8: AddUInt8::<Impl, IMPL_OFFSET>,
            AddUInt8WithFormat: AddUInt8WithFormat::<Impl, IMPL_OFFSET>,
            AddUInt8WithFormatAndTags: AddUInt8WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt8Array: AddUInt8Array::<Impl, IMPL_OFFSET>,
            AddUInt8ArrayWithFormat: AddUInt8ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddUInt8ArrayWithFormatAndTags: AddUInt8ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt16: AddInt16::<Impl, IMPL_OFFSET>,
            AddInt16WithFormat: AddInt16WithFormat::<Impl, IMPL_OFFSET>,
            AddInt16WithFormatAndTags: AddInt16WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt16Array: AddInt16Array::<Impl, IMPL_OFFSET>,
            AddInt16ArrayWithFormat: AddInt16ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddInt16ArrayWithFormatAndTags: AddInt16ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt16: AddUInt16::<Impl, IMPL_OFFSET>,
            AddUInt16WithFormat: AddUInt16WithFormat::<Impl, IMPL_OFFSET>,
            AddUInt16WithFormatAndTags: AddUInt16WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt16Array: AddUInt16Array::<Impl, IMPL_OFFSET>,
            AddUInt16ArrayWithFormat: AddUInt16ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddUInt16ArrayWithFormatAndTags: AddUInt16ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt32: AddInt32::<Impl, IMPL_OFFSET>,
            AddInt32WithFormat: AddInt32WithFormat::<Impl, IMPL_OFFSET>,
            AddInt32WithFormatAndTags: AddInt32WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt32Array: AddInt32Array::<Impl, IMPL_OFFSET>,
            AddInt32ArrayWithFormat: AddInt32ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddInt32ArrayWithFormatAndTags: AddInt32ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt32: AddUInt32::<Impl, IMPL_OFFSET>,
            AddUInt32WithFormat: AddUInt32WithFormat::<Impl, IMPL_OFFSET>,
            AddUInt32WithFormatAndTags: AddUInt32WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt32Array: AddUInt32Array::<Impl, IMPL_OFFSET>,
            AddUInt32ArrayWithFormat: AddUInt32ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddUInt32ArrayWithFormatAndTags: AddUInt32ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt64: AddInt64::<Impl, IMPL_OFFSET>,
            AddInt64WithFormat: AddInt64WithFormat::<Impl, IMPL_OFFSET>,
            AddInt64WithFormatAndTags: AddInt64WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddInt64Array: AddInt64Array::<Impl, IMPL_OFFSET>,
            AddInt64ArrayWithFormat: AddInt64ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddInt64ArrayWithFormatAndTags: AddInt64ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt64: AddUInt64::<Impl, IMPL_OFFSET>,
            AddUInt64WithFormat: AddUInt64WithFormat::<Impl, IMPL_OFFSET>,
            AddUInt64WithFormatAndTags: AddUInt64WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddUInt64Array: AddUInt64Array::<Impl, IMPL_OFFSET>,
            AddUInt64ArrayWithFormat: AddUInt64ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddUInt64ArrayWithFormatAndTags: AddUInt64ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddSingle: AddSingle::<Impl, IMPL_OFFSET>,
            AddSingleWithFormat: AddSingleWithFormat::<Impl, IMPL_OFFSET>,
            AddSingleWithFormatAndTags: AddSingleWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddSingleArray: AddSingleArray::<Impl, IMPL_OFFSET>,
            AddSingleArrayWithFormat: AddSingleArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddSingleArrayWithFormatAndTags: AddSingleArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddDouble: AddDouble::<Impl, IMPL_OFFSET>,
            AddDoubleWithFormat: AddDoubleWithFormat::<Impl, IMPL_OFFSET>,
            AddDoubleWithFormatAndTags: AddDoubleWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddDoubleArray: AddDoubleArray::<Impl, IMPL_OFFSET>,
            AddDoubleArrayWithFormat: AddDoubleArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddDoubleArrayWithFormatAndTags: AddDoubleArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddChar16: AddChar16::<Impl, IMPL_OFFSET>,
            AddChar16WithFormat: AddChar16WithFormat::<Impl, IMPL_OFFSET>,
            AddChar16WithFormatAndTags: AddChar16WithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddChar16Array: AddChar16Array::<Impl, IMPL_OFFSET>,
            AddChar16ArrayWithFormat: AddChar16ArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddChar16ArrayWithFormatAndTags: AddChar16ArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddBoolean: AddBoolean::<Impl, IMPL_OFFSET>,
            AddBooleanWithFormat: AddBooleanWithFormat::<Impl, IMPL_OFFSET>,
            AddBooleanWithFormatAndTags: AddBooleanWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddBooleanArray: AddBooleanArray::<Impl, IMPL_OFFSET>,
            AddBooleanArrayWithFormat: AddBooleanArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddBooleanArrayWithFormatAndTags: AddBooleanArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddString: AddString::<Impl, IMPL_OFFSET>,
            AddStringWithFormat: AddStringWithFormat::<Impl, IMPL_OFFSET>,
            AddStringWithFormatAndTags: AddStringWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddStringArray: AddStringArray::<Impl, IMPL_OFFSET>,
            AddStringArrayWithFormat: AddStringArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddStringArrayWithFormatAndTags: AddStringArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddGuid: AddGuid::<Impl, IMPL_OFFSET>,
            AddGuidWithFormat: AddGuidWithFormat::<Impl, IMPL_OFFSET>,
            AddGuidWithFormatAndTags: AddGuidWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddGuidArray: AddGuidArray::<Impl, IMPL_OFFSET>,
            AddGuidArrayWithFormat: AddGuidArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddGuidArrayWithFormatAndTags: AddGuidArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddDateTime: AddDateTime::<Impl, IMPL_OFFSET>,
            AddDateTimeWithFormat: AddDateTimeWithFormat::<Impl, IMPL_OFFSET>,
            AddDateTimeWithFormatAndTags: AddDateTimeWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddDateTimeArray: AddDateTimeArray::<Impl, IMPL_OFFSET>,
            AddDateTimeArrayWithFormat: AddDateTimeArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddDateTimeArrayWithFormatAndTags: AddDateTimeArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddTimeSpan: AddTimeSpan::<Impl, IMPL_OFFSET>,
            AddTimeSpanWithFormat: AddTimeSpanWithFormat::<Impl, IMPL_OFFSET>,
            AddTimeSpanWithFormatAndTags: AddTimeSpanWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddTimeSpanArray: AddTimeSpanArray::<Impl, IMPL_OFFSET>,
            AddTimeSpanArrayWithFormat: AddTimeSpanArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddTimeSpanArrayWithFormatAndTags: AddTimeSpanArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddPoint: AddPoint::<Impl, IMPL_OFFSET>,
            AddPointWithFormat: AddPointWithFormat::<Impl, IMPL_OFFSET>,
            AddPointWithFormatAndTags: AddPointWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddPointArray: AddPointArray::<Impl, IMPL_OFFSET>,
            AddPointArrayWithFormat: AddPointArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddPointArrayWithFormatAndTags: AddPointArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddSize: AddSize::<Impl, IMPL_OFFSET>,
            AddSizeWithFormat: AddSizeWithFormat::<Impl, IMPL_OFFSET>,
            AddSizeWithFormatAndTags: AddSizeWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddSizeArray: AddSizeArray::<Impl, IMPL_OFFSET>,
            AddSizeArrayWithFormat: AddSizeArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddSizeArrayWithFormatAndTags: AddSizeArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddRect: AddRect::<Impl, IMPL_OFFSET>,
            AddRectWithFormat: AddRectWithFormat::<Impl, IMPL_OFFSET>,
            AddRectWithFormatAndTags: AddRectWithFormatAndTags::<Impl, IMPL_OFFSET>,
            AddRectArray: AddRectArray::<Impl, IMPL_OFFSET>,
            AddRectArrayWithFormat: AddRectArrayWithFormat::<Impl, IMPL_OFFSET>,
            AddRectArrayWithFormatAndTags: AddRectArrayWithFormatAndTags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingFields as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingOptions_Impl: Sized {
    fn Keywords(&mut self) -> ::windows::core::Result<i64>;
    fn SetKeywords(&mut self, value: i64) -> ::windows::core::Result<()>;
    fn Tags(&mut self) -> ::windows::core::Result<i32>;
    fn SetTags(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Task(&mut self) -> ::windows::core::Result<i16>;
    fn SetTask(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn Opcode(&mut self) -> ::windows::core::Result<LoggingOpcode>;
    fn SetOpcode(&mut self, value: LoggingOpcode) -> ::windows::core::Result<()>;
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetActivityId(&mut self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RelatedActivityId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetRelatedActivityId(&mut self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingOptions_Vtbl {
        unsafe extern "system" fn Keywords<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeywords<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeywords(value).into()
        }
        unsafe extern "system" fn Tags<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTags<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTags(value).into()
        }
        unsafe extern "system" fn Task<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTask<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTask(value).into()
        }
        unsafe extern "system" fn Opcode<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LoggingOpcode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOpcode<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LoggingOpcode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpcode(value).into()
        }
        unsafe extern "system" fn ActivityId<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetActivityId<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivityId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelatedActivityId<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRelatedActivityId<Impl: ILoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelatedActivityId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingOptions, BASE_OFFSET>(),
            Keywords: Keywords::<Impl, IMPL_OFFSET>,
            SetKeywords: SetKeywords::<Impl, IMPL_OFFSET>,
            Tags: Tags::<Impl, IMPL_OFFSET>,
            SetTags: SetTags::<Impl, IMPL_OFFSET>,
            Task: Task::<Impl, IMPL_OFFSET>,
            SetTask: SetTask::<Impl, IMPL_OFFSET>,
            Opcode: Opcode::<Impl, IMPL_OFFSET>,
            SetOpcode: SetOpcode::<Impl, IMPL_OFFSET>,
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            SetActivityId: SetActivityId::<Impl, IMPL_OFFSET>,
            RelatedActivityId: RelatedActivityId::<Impl, IMPL_OFFSET>,
            SetRelatedActivityId: SetRelatedActivityId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingOptionsFactory_Impl: Sized {
    fn CreateWithKeywords(&mut self, keywords: i64) -> ::windows::core::Result<LoggingOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingOptionsFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingOptionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingOptionsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingOptionsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingOptionsFactory_Vtbl {
        unsafe extern "system" fn CreateWithKeywords<Impl: ILoggingOptionsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: i64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingOptionsFactory, BASE_OFFSET>(),
            CreateWithKeywords: CreateWithKeywords::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingOptionsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Storage")]
pub trait ILoggingSession_Impl: Sized + super::IClosable_Impl {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SaveToFileAsync(&mut self, folder: &::core::option::Option<super::super::Storage::IStorageFolder>, filename: &::windows::core::HSTRING) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn AddLoggingChannel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn AddLoggingChannelWithLevel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows::core::Result<()>;
    fn RemoveLoggingChannel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows::core::RuntimeName for ILoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingSession";
}
#[cfg(feature = "Storage")]
impl ILoggingSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingSession_Vtbl {
        unsafe extern "system" fn Name<Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SaveToFileAsync<Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddLoggingChannel<Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr, maxlevel: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLoggingChannelWithLevel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingSession, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SaveToFileAsync: SaveToFileAsync::<Impl, IMPL_OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Impl, IMPL_OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Impl, IMPL_OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingSessionFactory_Impl: Sized {
    fn Create(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoggingSessionFactory {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingSessionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILoggingSessionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingSessionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingSessionFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ILoggingSessionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingSessionFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingSessionFactory as ::windows::core::Interface>::IID
    }
}
pub trait ILoggingTarget_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsEnabledWithLevel(&mut self, level: LoggingLevel) -> ::windows::core::Result<bool>;
    fn IsEnabledWithLevelAndKeywords(&mut self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool>;
    fn LogEvent(&mut self, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogEventWithFields(&mut self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<()>;
    fn LogEventWithFieldsAndLevel(&mut self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LogEventWithFieldsAndOptions(&mut self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<()>;
    fn StartActivity(&mut self, starteventname: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFields(&mut self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndLevel(&mut self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndOptions(&mut self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<LoggingActivity>;
}
impl ::windows::core::RuntimeName for ILoggingTarget {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingTarget";
}
impl ILoggingTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoggingTarget_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabledWithLevel<Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabledWithLevelAndKeywords<Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LogEvent<Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogEvent(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogEventWithFields<Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogEventWithFields(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndLevel<Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogEventWithFieldsAndLevel(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), level).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndOptions<Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .LogEventWithFieldsAndOptions(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), level, &*(&options as *const <LoggingOptions as ::windows::core::Abi>::Abi as *const <LoggingOptions as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn StartActivity<Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartActivityWithFields<Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartActivityWithFieldsAndLevel<Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartActivityWithFieldsAndOptions<Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingTarget, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            IsEnabledWithLevel: IsEnabledWithLevel::<Impl, IMPL_OFFSET>,
            IsEnabledWithLevelAndKeywords: IsEnabledWithLevelAndKeywords::<Impl, IMPL_OFFSET>,
            LogEvent: LogEvent::<Impl, IMPL_OFFSET>,
            LogEventWithFields: LogEventWithFields::<Impl, IMPL_OFFSET>,
            LogEventWithFieldsAndLevel: LogEventWithFieldsAndLevel::<Impl, IMPL_OFFSET>,
            LogEventWithFieldsAndOptions: LogEventWithFieldsAndOptions::<Impl, IMPL_OFFSET>,
            StartActivity: StartActivity::<Impl, IMPL_OFFSET>,
            StartActivityWithFields: StartActivityWithFields::<Impl, IMPL_OFFSET>,
            StartActivityWithFieldsAndLevel: StartActivityWithFieldsAndLevel::<Impl, IMPL_OFFSET>,
            StartActivityWithFieldsAndOptions: StartActivityWithFieldsAndOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITracingStatusChangedEventArgs_Impl: Sized {
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn TraceLevel(&mut self) -> ::windows::core::Result<CausalityTraceLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITracingStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ITracingStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITracingStatusChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITracingStatusChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITracingStatusChangedEventArgs_Vtbl {
        unsafe extern "system" fn Enabled<Impl: ITracingStatusChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TraceLevel<Impl: ITracingStatusChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CausalityTraceLevel) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITracingStatusChangedEventArgs, BASE_OFFSET>(),
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            TraceLevel: TraceLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITracingStatusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
