#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHttpDiagnosticProviderImpl: Sized {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn RequestSent(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRequestSent(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResponseReceived(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveResponseReceived(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestResponseCompleted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRequestResponseCompleted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpDiagnosticProvider {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.IHttpDiagnosticProvider";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHttpDiagnosticProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpDiagnosticProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpDiagnosticProviderVtbl {
        unsafe extern "system" fn Start<Impl: IHttpDiagnosticProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IHttpDiagnosticProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn RequestSent<Impl: IHttpDiagnosticProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSent(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRequestSent<Impl: IHttpDiagnosticProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRequestSent(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResponseReceived<Impl: IHttpDiagnosticProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseReceived(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResponseReceived<Impl: IHttpDiagnosticProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResponseReceived(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestResponseCompleted<Impl: IHttpDiagnosticProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestResponseCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRequestResponseCompleted<Impl: IHttpDiagnosticProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRequestResponseCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpDiagnosticProvider, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            RequestSent: RequestSent::<Impl, IMPL_OFFSET>,
            RemoveRequestSent: RemoveRequestSent::<Impl, IMPL_OFFSET>,
            ResponseReceived: ResponseReceived::<Impl, IMPL_OFFSET>,
            RemoveResponseReceived: RemoveResponseReceived::<Impl, IMPL_OFFSET>,
            RequestResponseCompleted: RequestResponseCompleted::<Impl, IMPL_OFFSET>,
            RemoveRequestResponseCompleted: RemoveRequestResponseCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpDiagnosticProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHttpDiagnosticProviderRequestResponseCompletedEventArgsImpl: Sized {
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Timestamps(&mut self) -> ::windows::core::Result<HttpDiagnosticProviderRequestResponseTimestamps>;
    fn RequestedUri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ProcessId(&mut self) -> ::windows::core::Result<u32>;
    fn ThreadId(&mut self) -> ::windows::core::Result<u32>;
    fn Initiator(&mut self) -> ::windows::core::Result<HttpDiagnosticRequestInitiator>;
    fn SourceLocations(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseCompletedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHttpDiagnosticProviderRequestResponseCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpDiagnosticProviderRequestResponseCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpDiagnosticProviderRequestResponseCompletedEventArgsVtbl {
        unsafe extern "system" fn ActivityId<Impl: IHttpDiagnosticProviderRequestResponseCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Timestamps<Impl: IHttpDiagnosticProviderRequestResponseCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedUri<Impl: IHttpDiagnosticProviderRequestResponseCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessId<Impl: IHttpDiagnosticProviderRequestResponseCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThreadId<Impl: IHttpDiagnosticProviderRequestResponseCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThreadId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initiator<Impl: IHttpDiagnosticProviderRequestResponseCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initiator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceLocations<Impl: IHttpDiagnosticProviderRequestResponseCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpDiagnosticProviderRequestResponseCompletedEventArgs, BASE_OFFSET>(),
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            Timestamps: Timestamps::<Impl, IMPL_OFFSET>,
            RequestedUri: RequestedUri::<Impl, IMPL_OFFSET>,
            ProcessId: ProcessId::<Impl, IMPL_OFFSET>,
            ThreadId: ThreadId::<Impl, IMPL_OFFSET>,
            Initiator: Initiator::<Impl, IMPL_OFFSET>,
            SourceLocations: SourceLocations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpDiagnosticProviderRequestResponseCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHttpDiagnosticProviderRequestResponseTimestampsImpl: Sized {
    fn CacheCheckedTimestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ConnectionInitiatedTimestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn NameResolvedTimestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SslNegotiatedTimestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ConnectionCompletedTimestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn RequestSentTimestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn RequestCompletedTimestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ResponseReceivedTimestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ResponseCompletedTimestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpDiagnosticProviderRequestResponseTimestamps {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestResponseTimestamps";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHttpDiagnosticProviderRequestResponseTimestampsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpDiagnosticProviderRequestResponseTimestampsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpDiagnosticProviderRequestResponseTimestampsVtbl {
        unsafe extern "system" fn CacheCheckedTimestamp<Impl: IHttpDiagnosticProviderRequestResponseTimestampsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheCheckedTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionInitiatedTimestamp<Impl: IHttpDiagnosticProviderRequestResponseTimestampsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionInitiatedTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameResolvedTimestamp<Impl: IHttpDiagnosticProviderRequestResponseTimestampsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameResolvedTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SslNegotiatedTimestamp<Impl: IHttpDiagnosticProviderRequestResponseTimestampsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SslNegotiatedTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionCompletedTimestamp<Impl: IHttpDiagnosticProviderRequestResponseTimestampsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionCompletedTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestSentTimestamp<Impl: IHttpDiagnosticProviderRequestResponseTimestampsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSentTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCompletedTimestamp<Impl: IHttpDiagnosticProviderRequestResponseTimestampsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCompletedTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseReceivedTimestamp<Impl: IHttpDiagnosticProviderRequestResponseTimestampsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseReceivedTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseCompletedTimestamp<Impl: IHttpDiagnosticProviderRequestResponseTimestampsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseCompletedTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpDiagnosticProviderRequestResponseTimestamps, BASE_OFFSET>(),
            CacheCheckedTimestamp: CacheCheckedTimestamp::<Impl, IMPL_OFFSET>,
            ConnectionInitiatedTimestamp: ConnectionInitiatedTimestamp::<Impl, IMPL_OFFSET>,
            NameResolvedTimestamp: NameResolvedTimestamp::<Impl, IMPL_OFFSET>,
            SslNegotiatedTimestamp: SslNegotiatedTimestamp::<Impl, IMPL_OFFSET>,
            ConnectionCompletedTimestamp: ConnectionCompletedTimestamp::<Impl, IMPL_OFFSET>,
            RequestSentTimestamp: RequestSentTimestamp::<Impl, IMPL_OFFSET>,
            RequestCompletedTimestamp: RequestCompletedTimestamp::<Impl, IMPL_OFFSET>,
            ResponseReceivedTimestamp: ResponseReceivedTimestamp::<Impl, IMPL_OFFSET>,
            ResponseCompletedTimestamp: ResponseCompletedTimestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpDiagnosticProviderRequestResponseTimestamps as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHttpDiagnosticProviderRequestSentEventArgsImpl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Message(&mut self) -> ::windows::core::Result<super::HttpRequestMessage>;
    fn ProcessId(&mut self) -> ::windows::core::Result<u32>;
    fn ThreadId(&mut self) -> ::windows::core::Result<u32>;
    fn Initiator(&mut self) -> ::windows::core::Result<HttpDiagnosticRequestInitiator>;
    fn SourceLocations(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpDiagnosticProviderRequestSentEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderRequestSentEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHttpDiagnosticProviderRequestSentEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpDiagnosticProviderRequestSentEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpDiagnosticProviderRequestSentEventArgsVtbl {
        unsafe extern "system" fn Timestamp<Impl: IHttpDiagnosticProviderRequestSentEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Impl: IHttpDiagnosticProviderRequestSentEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Message<Impl: IHttpDiagnosticProviderRequestSentEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProcessId<Impl: IHttpDiagnosticProviderRequestSentEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThreadId<Impl: IHttpDiagnosticProviderRequestSentEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThreadId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initiator<Impl: IHttpDiagnosticProviderRequestSentEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initiator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceLocations<Impl: IHttpDiagnosticProviderRequestSentEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpDiagnosticProviderRequestSentEventArgs, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
            ProcessId: ProcessId::<Impl, IMPL_OFFSET>,
            ThreadId: ThreadId::<Impl, IMPL_OFFSET>,
            Initiator: Initiator::<Impl, IMPL_OFFSET>,
            SourceLocations: SourceLocations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpDiagnosticProviderRequestSentEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHttpDiagnosticProviderResponseReceivedEventArgsImpl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Message(&mut self) -> ::windows::core::Result<super::HttpResponseMessage>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpDiagnosticProviderResponseReceivedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderResponseReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHttpDiagnosticProviderResponseReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpDiagnosticProviderResponseReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpDiagnosticProviderResponseReceivedEventArgsVtbl {
        unsafe extern "system" fn Timestamp<Impl: IHttpDiagnosticProviderResponseReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Impl: IHttpDiagnosticProviderResponseReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Message<Impl: IHttpDiagnosticProviderResponseReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpDiagnosticProviderResponseReceivedEventArgs, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpDiagnosticProviderResponseReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System_Diagnostics", feature = "implement_exclusive"))]
pub trait IHttpDiagnosticProviderStaticsImpl: Sized {
    fn CreateFromProcessDiagnosticInfo(&mut self, processdiagnosticinfo: &::core::option::Option<super::super::super::System::Diagnostics::ProcessDiagnosticInfo>) -> ::windows::core::Result<HttpDiagnosticProvider>;
}
#[cfg(all(feature = "System_Diagnostics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpDiagnosticProviderStatics {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.IHttpDiagnosticProviderStatics";
}
#[cfg(all(feature = "System_Diagnostics", feature = "implement_exclusive"))]
impl IHttpDiagnosticProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpDiagnosticProviderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpDiagnosticProviderStaticsVtbl {
        unsafe extern "system" fn CreateFromProcessDiagnosticInfo<Impl: IHttpDiagnosticProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processdiagnosticinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromProcessDiagnosticInfo(&*(&processdiagnosticinfo as *const <super::super::super::System::Diagnostics::ProcessDiagnosticInfo as ::windows::core::Abi>::Abi as *const <super::super::super::System::Diagnostics::ProcessDiagnosticInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpDiagnosticProviderStatics, BASE_OFFSET>(),
            CreateFromProcessDiagnosticInfo: CreateFromProcessDiagnosticInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpDiagnosticProviderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHttpDiagnosticSourceLocationImpl: Sized {
    fn SourceUri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn LineNumber(&mut self) -> ::windows::core::Result<u64>;
    fn ColumnNumber(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpDiagnosticSourceLocation {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.IHttpDiagnosticSourceLocation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHttpDiagnosticSourceLocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpDiagnosticSourceLocationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpDiagnosticSourceLocationVtbl {
        unsafe extern "system" fn SourceUri<Impl: IHttpDiagnosticSourceLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineNumber<Impl: IHttpDiagnosticSourceLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnNumber<Impl: IHttpDiagnosticSourceLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpDiagnosticSourceLocation, BASE_OFFSET>(),
            SourceUri: SourceUri::<Impl, IMPL_OFFSET>,
            LineNumber: LineNumber::<Impl, IMPL_OFFSET>,
            ColumnNumber: ColumnNumber::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpDiagnosticSourceLocation as ::windows::core::Interface>::IID
    }
}
