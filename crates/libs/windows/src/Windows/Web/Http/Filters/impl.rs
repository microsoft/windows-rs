#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IHttpBaseProtocolFilter_Impl: Sized {
    fn AllowAutoRedirect(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowAutoRedirect(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUI(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowUI(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AutomaticDecompression(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutomaticDecompression(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CacheControl(&mut self) -> ::windows::core::Result<HttpCacheControl>;
    fn CookieManager(&mut self) -> ::windows::core::Result<super::HttpCookieManager>;
    fn ClientCertificate(&mut self) -> ::windows::core::Result<super::super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&mut self, value: &::core::option::Option<super::super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
    fn IgnorableServerCertificateErrors(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn MaxConnectionsPerServer(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxConnectionsPerServer(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ProxyCredential(&mut self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&mut self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn ServerCredential(&mut self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&mut self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn UseProxy(&mut self) -> ::windows::core::Result<bool>;
    fn SetUseProxy(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilter {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IHttpBaseProtocolFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpBaseProtocolFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpBaseProtocolFilter_Vtbl {
        unsafe extern "system" fn AllowAutoRedirect<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowAutoRedirect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowAutoRedirect<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowAutoRedirect(value).into()
        }
        unsafe extern "system" fn AllowUI<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowUI<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowUI(value).into()
        }
        unsafe extern "system" fn AutomaticDecompression<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomaticDecompression() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomaticDecompression<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutomaticDecompression(value).into()
        }
        unsafe extern "system" fn CacheControl<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CookieManager<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CookieManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientCertificate<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientCertificate<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificate(&*(&value as *const <super::super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IgnorableServerCertificateErrors<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IgnorableServerCertificateErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxConnectionsPerServer<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxConnectionsPerServer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxConnectionsPerServer<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxConnectionsPerServer(value).into()
        }
        unsafe extern "system" fn ProxyCredential<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyCredential<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyCredential(&*(&value as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServerCredential<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCredential<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerCredential(&*(&value as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UseProxy<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseProxy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseProxy<Impl: IHttpBaseProtocolFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseProxy(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpBaseProtocolFilter, BASE_OFFSET>(),
            AllowAutoRedirect: AllowAutoRedirect::<Impl, IMPL_OFFSET>,
            SetAllowAutoRedirect: SetAllowAutoRedirect::<Impl, IMPL_OFFSET>,
            AllowUI: AllowUI::<Impl, IMPL_OFFSET>,
            SetAllowUI: SetAllowUI::<Impl, IMPL_OFFSET>,
            AutomaticDecompression: AutomaticDecompression::<Impl, IMPL_OFFSET>,
            SetAutomaticDecompression: SetAutomaticDecompression::<Impl, IMPL_OFFSET>,
            CacheControl: CacheControl::<Impl, IMPL_OFFSET>,
            CookieManager: CookieManager::<Impl, IMPL_OFFSET>,
            ClientCertificate: ClientCertificate::<Impl, IMPL_OFFSET>,
            SetClientCertificate: SetClientCertificate::<Impl, IMPL_OFFSET>,
            IgnorableServerCertificateErrors: IgnorableServerCertificateErrors::<Impl, IMPL_OFFSET>,
            MaxConnectionsPerServer: MaxConnectionsPerServer::<Impl, IMPL_OFFSET>,
            SetMaxConnectionsPerServer: SetMaxConnectionsPerServer::<Impl, IMPL_OFFSET>,
            ProxyCredential: ProxyCredential::<Impl, IMPL_OFFSET>,
            SetProxyCredential: SetProxyCredential::<Impl, IMPL_OFFSET>,
            ServerCredential: ServerCredential::<Impl, IMPL_OFFSET>,
            SetServerCredential: SetServerCredential::<Impl, IMPL_OFFSET>,
            UseProxy: UseProxy::<Impl, IMPL_OFFSET>,
            SetUseProxy: SetUseProxy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpBaseProtocolFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilter2_Impl: Sized {
    fn MaxVersion(&mut self) -> ::windows::core::Result<super::HttpVersion>;
    fn SetMaxVersion(&mut self, value: super::HttpVersion) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilter2 {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilter2";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpBaseProtocolFilter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpBaseProtocolFilter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpBaseProtocolFilter2_Vtbl {
        unsafe extern "system" fn MaxVersion<Impl: IHttpBaseProtocolFilter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::HttpVersion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxVersion<Impl: IHttpBaseProtocolFilter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::HttpVersion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxVersion(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpBaseProtocolFilter2, BASE_OFFSET>(),
            MaxVersion: MaxVersion::<Impl, IMPL_OFFSET>,
            SetMaxVersion: SetMaxVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpBaseProtocolFilter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilter3_Impl: Sized {
    fn CookieUsageBehavior(&mut self) -> ::windows::core::Result<HttpCookieUsageBehavior>;
    fn SetCookieUsageBehavior(&mut self, value: HttpCookieUsageBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilter3 {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilter3";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpBaseProtocolFilter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpBaseProtocolFilter3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpBaseProtocolFilter3_Vtbl {
        unsafe extern "system" fn CookieUsageBehavior<Impl: IHttpBaseProtocolFilter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HttpCookieUsageBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CookieUsageBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCookieUsageBehavior<Impl: IHttpBaseProtocolFilter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: HttpCookieUsageBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCookieUsageBehavior(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpBaseProtocolFilter3, BASE_OFFSET>(),
            CookieUsageBehavior: CookieUsageBehavior::<Impl, IMPL_OFFSET>,
            SetCookieUsageBehavior: SetCookieUsageBehavior::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpBaseProtocolFilter3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHttpBaseProtocolFilter4_Impl: Sized {
    fn ServerCustomValidationRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerCustomValidationRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ClearAuthenticationCache(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilter4 {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilter4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHttpBaseProtocolFilter4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpBaseProtocolFilter4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpBaseProtocolFilter4_Vtbl {
        unsafe extern "system" fn ServerCustomValidationRequested<Impl: IHttpBaseProtocolFilter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCustomValidationRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveServerCustomValidationRequested<Impl: IHttpBaseProtocolFilter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServerCustomValidationRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearAuthenticationCache<Impl: IHttpBaseProtocolFilter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAuthenticationCache().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpBaseProtocolFilter4, BASE_OFFSET>(),
            ServerCustomValidationRequested: ServerCustomValidationRequested::<Impl, IMPL_OFFSET>,
            RemoveServerCustomValidationRequested: RemoveServerCustomValidationRequested::<Impl, IMPL_OFFSET>,
            ClearAuthenticationCache: ClearAuthenticationCache::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpBaseProtocolFilter4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IHttpBaseProtocolFilter5_Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::super::super::System::User>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilter5 {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilter5";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IHttpBaseProtocolFilter5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpBaseProtocolFilter5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpBaseProtocolFilter5_Vtbl {
        unsafe extern "system" fn User<Impl: IHttpBaseProtocolFilter5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpBaseProtocolFilter5, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpBaseProtocolFilter5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IHttpBaseProtocolFilterStatics_Impl: Sized {
    fn CreateForUser(&mut self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<HttpBaseProtocolFilter>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilterStatics {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilterStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IHttpBaseProtocolFilterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpBaseProtocolFilterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpBaseProtocolFilterStatics_Vtbl {
        unsafe extern "system" fn CreateForUser<Impl: IHttpBaseProtocolFilterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpBaseProtocolFilterStatics, BASE_OFFSET>(),
            CreateForUser: CreateForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpBaseProtocolFilterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCacheControl_Impl: Sized {
    fn ReadBehavior(&mut self) -> ::windows::core::Result<HttpCacheReadBehavior>;
    fn SetReadBehavior(&mut self, value: HttpCacheReadBehavior) -> ::windows::core::Result<()>;
    fn WriteBehavior(&mut self) -> ::windows::core::Result<HttpCacheWriteBehavior>;
    fn SetWriteBehavior(&mut self, value: HttpCacheWriteBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCacheControl {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpCacheControl";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCacheControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCacheControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpCacheControl_Vtbl {
        unsafe extern "system" fn ReadBehavior<Impl: IHttpCacheControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HttpCacheReadBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadBehavior<Impl: IHttpCacheControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: HttpCacheReadBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadBehavior(value).into()
        }
        unsafe extern "system" fn WriteBehavior<Impl: IHttpCacheControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HttpCacheWriteBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteBehavior<Impl: IHttpCacheControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: HttpCacheWriteBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteBehavior(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpCacheControl, BASE_OFFSET>(),
            ReadBehavior: ReadBehavior::<Impl, IMPL_OFFSET>,
            SetReadBehavior: SetReadBehavior::<Impl, IMPL_OFFSET>,
            WriteBehavior: WriteBehavior::<Impl, IMPL_OFFSET>,
            SetWriteBehavior: SetWriteBehavior::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpCacheControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IHttpFilter_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn SendRequestAsync(&mut self, request: &::core::option::Option<super::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IHttpFilter {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpFilter";
}
#[cfg(feature = "Foundation")]
impl IHttpFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpFilter_Vtbl {
        unsafe extern "system" fn SendRequestAsync<Impl: IHttpFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendRequestAsync(&*(&request as *const <super::HttpRequestMessage as ::windows::core::Abi>::Abi as *const <super::HttpRequestMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpFilter, BASE_OFFSET>(), SendRequestAsync: SendRequestAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IHttpServerCustomValidationRequestedEventArgs_Impl: Sized {
    fn RequestMessage(&mut self) -> ::windows::core::Result<super::HttpRequestMessage>;
    fn ServerCertificate(&mut self) -> ::windows::core::Result<super::super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(&mut self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>>;
    fn Reject(&mut self) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpServerCustomValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpServerCustomValidationRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IHttpServerCustomValidationRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpServerCustomValidationRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpServerCustomValidationRequestedEventArgs_Vtbl {
        unsafe extern "system" fn RequestMessage<Impl: IHttpServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificate<Impl: IHttpServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrorSeverity<Impl: IHttpServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificateErrorSeverity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrors<Impl: IHttpServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificateErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerIntermediateCertificates<Impl: IHttpServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerIntermediateCertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Impl: IHttpServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IHttpServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpServerCustomValidationRequestedEventArgs, BASE_OFFSET>(),
            RequestMessage: RequestMessage::<Impl, IMPL_OFFSET>,
            ServerCertificate: ServerCertificate::<Impl, IMPL_OFFSET>,
            ServerCertificateErrorSeverity: ServerCertificateErrorSeverity::<Impl, IMPL_OFFSET>,
            ServerCertificateErrors: ServerCertificateErrors::<Impl, IMPL_OFFSET>,
            ServerIntermediateCertificates: ServerIntermediateCertificates::<Impl, IMPL_OFFSET>,
            Reject: Reject::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpServerCustomValidationRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
