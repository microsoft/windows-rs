#[doc = "*Required features: `\"System_Implementation_FileExplorer\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait ISysStorageProviderEventSource_Impl: Sized {
    fn EventReceived(&self, handler: ::core::option::Option<&super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEventReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ISysStorageProviderEventSource {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderEventSource";
}
#[cfg(feature = "Foundation")]
impl ISysStorageProviderEventSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: isize>() -> ISysStorageProviderEventSource_Vtbl {
        unsafe extern "system" fn EventReceived<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventReceived(::windows::core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventReceived<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveEventReceived(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ISysStorageProviderEventSource, OFFSET>(),
            EventReceived: EventReceived::<Identity, Impl, OFFSET>,
            RemoveEventReceived: RemoveEventReceived::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISysStorageProviderEventSource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"System_Implementation_FileExplorer\"`, `\"implement\"`*"]
pub trait ISysStorageProviderHandlerFactory_Impl: Sized {
    fn GetHttpRequestProvider(&self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<ISysStorageProviderHttpRequestProvider>;
    fn GetEventSource(&self, syncrootid: &::windows::core::HSTRING, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<ISysStorageProviderEventSource>;
}
impl ::windows::core::RuntimeName for ISysStorageProviderHandlerFactory {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHandlerFactory";
}
impl ISysStorageProviderHandlerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>() -> ISysStorageProviderHandlerFactory_Vtbl {
        unsafe extern "system" fn GetHttpRequestProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncrootid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHttpRequestProvider(::core::mem::transmute(&syncrootid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncrootid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, eventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventSource(::core::mem::transmute(&syncrootid), ::core::mem::transmute(&eventname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ISysStorageProviderHandlerFactory, OFFSET>(),
            GetHttpRequestProvider: GetHttpRequestProvider::<Identity, Impl, OFFSET>,
            GetEventSource: GetEventSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISysStorageProviderHandlerFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"System_Implementation_FileExplorer\"`, `\"Foundation\"`, `\"Web_Http\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
pub trait ISysStorageProviderHttpRequestProvider_Impl: Sized {
    fn SendRequestAsync(&self, request: ::core::option::Option<&super::super::super::Web::Http::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>;
}
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
impl ::windows::core::RuntimeName for ISysStorageProviderHttpRequestProvider {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHttpRequestProvider";
}
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
impl ISysStorageProviderHttpRequestProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderHttpRequestProvider_Impl, const OFFSET: isize>() -> ISysStorageProviderHttpRequestProvider_Vtbl {
        unsafe extern "system" fn SendRequestAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderHttpRequestProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SendRequestAsync(::windows::core::from_raw_borrowed(&request)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ISysStorageProviderHttpRequestProvider, OFFSET>(),
            SendRequestAsync: SendRequestAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISysStorageProviderHttpRequestProvider as ::windows::core::ComInterface>::IID
    }
}
