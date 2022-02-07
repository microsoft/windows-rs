#[cfg(feature = "Foundation")]
pub trait ISysStorageProviderEventSource_Impl: Sized {
    fn EventReceived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEventReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ISysStorageProviderEventSource {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderEventSource";
}
#[cfg(feature = "Foundation")]
impl ISysStorageProviderEventSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: isize>() -> ISysStorageProviderEventSource_Vtbl {
        unsafe extern "system" fn EventReceived<Identity: ::windows::core::IUnknownImpl, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventReceived(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventReceived<Identity: ::windows::core::IUnknownImpl, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveEventReceived(::core::mem::transmute(&token)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISysStorageProviderEventSource, OFFSET>(),
            EventReceived: EventReceived::<Identity, Impl, OFFSET>,
            RemoveEventReceived: RemoveEventReceived::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISysStorageProviderEventSource as ::windows::core::Interface>::IID
    }
}
pub trait ISysStorageProviderHandlerFactory_Impl: Sized {
    fn GetHttpRequestProvider(&self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<ISysStorageProviderHttpRequestProvider>;
    fn GetEventSource(&self, syncrootid: &::windows::core::HSTRING, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<ISysStorageProviderEventSource>;
}
impl ::windows::core::RuntimeName for ISysStorageProviderHandlerFactory {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHandlerFactory";
}
impl ISysStorageProviderHandlerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>() -> ISysStorageProviderHandlerFactory_Vtbl {
        unsafe extern "system" fn GetHttpRequestProvider<Identity: ::windows::core::IUnknownImpl, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHttpRequestProvider(::core::mem::transmute(&syncrootid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventSource<Identity: ::windows::core::IUnknownImpl, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEventSource(::core::mem::transmute(&syncrootid), ::core::mem::transmute(&eventname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISysStorageProviderHandlerFactory, OFFSET>(),
            GetHttpRequestProvider: GetHttpRequestProvider::<Identity, Impl, OFFSET>,
            GetEventSource: GetEventSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISysStorageProviderHandlerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
pub trait ISysStorageProviderHttpRequestProvider_Impl: Sized {
    fn SendRequestAsync(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>;
}
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
impl ::windows::core::RuntimeName for ISysStorageProviderHttpRequestProvider {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHttpRequestProvider";
}
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
impl ISysStorageProviderHttpRequestProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISysStorageProviderHttpRequestProvider_Impl, const OFFSET: isize>() -> ISysStorageProviderHttpRequestProvider_Vtbl {
        unsafe extern "system" fn SendRequestAsync<Identity: ::windows::core::IUnknownImpl, Impl: ISysStorageProviderHttpRequestProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendRequestAsync(::core::mem::transmute(&request)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISysStorageProviderHttpRequestProvider, OFFSET>(),
            SendRequestAsync: SendRequestAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISysStorageProviderHttpRequestProvider as ::windows::core::Interface>::IID
    }
}
