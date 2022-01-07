#[cfg(feature = "implement_exclusive")]
pub trait ISysStorageProviderEventReceivedEventArgsImpl: Sized {
    fn Json(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISysStorageProviderEventReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISysStorageProviderEventReceivedEventArgsVtbl {
    pub const fn new<Impl: ISysStorageProviderEventReceivedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISysStorageProviderEventReceivedEventArgsVtbl {
        unsafe extern "system" fn Json<Impl: ISysStorageProviderEventReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Json() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISysStorageProviderEventReceivedEventArgs>, base.5, Json::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISysStorageProviderEventReceivedEventArgsFactoryImpl: Sized {
    fn CreateInstance(&self, json: &::windows::core::HSTRING) -> ::windows::core::Result<SysStorageProviderEventReceivedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISysStorageProviderEventReceivedEventArgsFactory {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderEventReceivedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISysStorageProviderEventReceivedEventArgsFactoryVtbl {
    pub const fn new<Impl: ISysStorageProviderEventReceivedEventArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISysStorageProviderEventReceivedEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISysStorageProviderEventReceivedEventArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, json: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&json as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISysStorageProviderEventReceivedEventArgsFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
pub trait ISysStorageProviderEventSourceImpl: Sized {
    fn EventReceived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEventReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISysStorageProviderEventSource {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderEventSource";
}
impl ISysStorageProviderEventSourceVtbl {
    pub const fn new<Impl: ISysStorageProviderEventSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISysStorageProviderEventSourceVtbl {
        unsafe extern "system" fn EventReceived<Impl: ISysStorageProviderEventSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EventReceived(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventReceived<Impl: ISysStorageProviderEventSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveEventReceived(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISysStorageProviderEventSource>, base.5, EventReceived::<Impl, OFFSET>, RemoveEventReceived::<Impl, OFFSET>)
    }
}
pub trait ISysStorageProviderHandlerFactoryImpl: Sized {
    fn GetHttpRequestProvider(&self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<ISysStorageProviderHttpRequestProvider>;
    fn GetEventSource(&self, syncrootid: &::windows::core::HSTRING, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<ISysStorageProviderEventSource>;
}
impl ::windows::core::RuntimeName for ISysStorageProviderHandlerFactory {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHandlerFactory";
}
impl ISysStorageProviderHandlerFactoryVtbl {
    pub const fn new<Impl: ISysStorageProviderHandlerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISysStorageProviderHandlerFactoryVtbl {
        unsafe extern "system" fn GetHttpRequestProvider<Impl: ISysStorageProviderHandlerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHttpRequestProvider(&*(&syncrootid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventSource<Impl: ISysStorageProviderHandlerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEventSource(&*(&syncrootid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISysStorageProviderHandlerFactory>, base.5, GetHttpRequestProvider::<Impl, OFFSET>, GetEventSource::<Impl, OFFSET>)
    }
}
pub trait ISysStorageProviderHttpRequestProviderImpl: Sized {
    fn SendRequestAsync(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>;
}
impl ::windows::core::RuntimeName for ISysStorageProviderHttpRequestProvider {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHttpRequestProvider";
}
impl ISysStorageProviderHttpRequestProviderVtbl {
    pub const fn new<Impl: ISysStorageProviderHttpRequestProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISysStorageProviderHttpRequestProviderVtbl {
        unsafe extern "system" fn SendRequestAsync<Impl: ISysStorageProviderHttpRequestProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendRequestAsync(&*(&request as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::Abi>::Abi as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISysStorageProviderHttpRequestProvider>, base.5, SendRequestAsync::<Impl, OFFSET>)
    }
}
