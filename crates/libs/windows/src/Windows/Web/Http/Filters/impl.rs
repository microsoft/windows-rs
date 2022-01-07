#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilterImpl: Sized {
    fn AllowAutoRedirect(&self) -> ::windows::core::Result<bool>;
    fn SetAllowAutoRedirect(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUI(&self) -> ::windows::core::Result<bool>;
    fn SetAllowUI(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutomaticDecompression(&self) -> ::windows::core::Result<bool>;
    fn SetAutomaticDecompression(&self, value: bool) -> ::windows::core::Result<()>;
    fn CacheControl(&self) -> ::windows::core::Result<HttpCacheControl>;
    fn CookieManager(&self) -> ::windows::core::Result<super::HttpCookieManager>;
    fn ClientCertificate(&self) -> ::windows::core::Result<super::super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&self, value: &::core::option::Option<super::super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
    fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn MaxConnectionsPerServer(&self) -> ::windows::core::Result<u32>;
    fn SetMaxConnectionsPerServer(&self, value: u32) -> ::windows::core::Result<()>;
    fn ProxyCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn ServerCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn UseProxy(&self) -> ::windows::core::Result<bool>;
    fn SetUseProxy(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilter {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpBaseProtocolFilterVtbl {
    pub const fn new<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHttpBaseProtocolFilterVtbl {
        unsafe extern "system" fn AllowAutoRedirect<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowAutoRedirect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowAutoRedirect<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowAutoRedirect(value).into()
        }
        unsafe extern "system" fn AllowUI<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowUI<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowUI(value).into()
        }
        unsafe extern "system" fn AutomaticDecompression<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutomaticDecompression() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomaticDecompression<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutomaticDecompression(value).into()
        }
        unsafe extern "system" fn CacheControl<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CacheControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CookieManager<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CookieManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientCertificate<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientCertificate<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetClientCertificate(&*(&value as *const <super::super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IgnorableServerCertificateErrors<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IgnorableServerCertificateErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxConnectionsPerServer<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxConnectionsPerServer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxConnectionsPerServer<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxConnectionsPerServer(value).into()
        }
        unsafe extern "system" fn ProxyCredential<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProxyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyCredential<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProxyCredential(&*(&value as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServerCredential<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCredential<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetServerCredential(&*(&value as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UseProxy<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UseProxy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseProxy<Impl: IHttpBaseProtocolFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUseProxy(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IHttpBaseProtocolFilter>,
            base.5,
            AllowAutoRedirect::<Impl, OFFSET>,
            SetAllowAutoRedirect::<Impl, OFFSET>,
            AllowUI::<Impl, OFFSET>,
            SetAllowUI::<Impl, OFFSET>,
            AutomaticDecompression::<Impl, OFFSET>,
            SetAutomaticDecompression::<Impl, OFFSET>,
            CacheControl::<Impl, OFFSET>,
            CookieManager::<Impl, OFFSET>,
            ClientCertificate::<Impl, OFFSET>,
            SetClientCertificate::<Impl, OFFSET>,
            IgnorableServerCertificateErrors::<Impl, OFFSET>,
            MaxConnectionsPerServer::<Impl, OFFSET>,
            SetMaxConnectionsPerServer::<Impl, OFFSET>,
            ProxyCredential::<Impl, OFFSET>,
            SetProxyCredential::<Impl, OFFSET>,
            ServerCredential::<Impl, OFFSET>,
            SetServerCredential::<Impl, OFFSET>,
            UseProxy::<Impl, OFFSET>,
            SetUseProxy::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilter2Impl: Sized {
    fn MaxVersion(&self) -> ::windows::core::Result<super::HttpVersion>;
    fn SetMaxVersion(&self, value: super::HttpVersion) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilter2 {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilter2";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpBaseProtocolFilter2Vtbl {
    pub const fn new<Impl: IHttpBaseProtocolFilter2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHttpBaseProtocolFilter2Vtbl {
        unsafe extern "system" fn MaxVersion<Impl: IHttpBaseProtocolFilter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::HttpVersion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxVersion<Impl: IHttpBaseProtocolFilter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::HttpVersion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxVersion(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHttpBaseProtocolFilter2>, base.5, MaxVersion::<Impl, OFFSET>, SetMaxVersion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilter3Impl: Sized {
    fn CookieUsageBehavior(&self) -> ::windows::core::Result<HttpCookieUsageBehavior>;
    fn SetCookieUsageBehavior(&self, value: HttpCookieUsageBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilter3 {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilter3";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpBaseProtocolFilter3Vtbl {
    pub const fn new<Impl: IHttpBaseProtocolFilter3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHttpBaseProtocolFilter3Vtbl {
        unsafe extern "system" fn CookieUsageBehavior<Impl: IHttpBaseProtocolFilter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HttpCookieUsageBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CookieUsageBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCookieUsageBehavior<Impl: IHttpBaseProtocolFilter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: HttpCookieUsageBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCookieUsageBehavior(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHttpBaseProtocolFilter3>, base.5, CookieUsageBehavior::<Impl, OFFSET>, SetCookieUsageBehavior::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilter4Impl: Sized {
    fn ServerCustomValidationRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerCustomValidationRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ClearAuthenticationCache(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilter4 {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilter4";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpBaseProtocolFilter4Vtbl {
    pub const fn new<Impl: IHttpBaseProtocolFilter4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHttpBaseProtocolFilter4Vtbl {
        unsafe extern "system" fn ServerCustomValidationRequested<Impl: IHttpBaseProtocolFilter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerCustomValidationRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveServerCustomValidationRequested<Impl: IHttpBaseProtocolFilter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveServerCustomValidationRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearAuthenticationCache<Impl: IHttpBaseProtocolFilter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ClearAuthenticationCache().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHttpBaseProtocolFilter4>, base.5, ServerCustomValidationRequested::<Impl, OFFSET>, RemoveServerCustomValidationRequested::<Impl, OFFSET>, ClearAuthenticationCache::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilter5Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilter5 {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilter5";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpBaseProtocolFilter5Vtbl {
    pub const fn new<Impl: IHttpBaseProtocolFilter5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHttpBaseProtocolFilter5Vtbl {
        unsafe extern "system" fn User<Impl: IHttpBaseProtocolFilter5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHttpBaseProtocolFilter5>, base.5, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilterStaticsImpl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<HttpBaseProtocolFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpBaseProtocolFilterStatics {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpBaseProtocolFilterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpBaseProtocolFilterStaticsVtbl {
    pub const fn new<Impl: IHttpBaseProtocolFilterStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHttpBaseProtocolFilterStaticsVtbl {
        unsafe extern "system" fn CreateForUser<Impl: IHttpBaseProtocolFilterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHttpBaseProtocolFilterStatics>, base.5, CreateForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCacheControlImpl: Sized {
    fn ReadBehavior(&self) -> ::windows::core::Result<HttpCacheReadBehavior>;
    fn SetReadBehavior(&self, value: HttpCacheReadBehavior) -> ::windows::core::Result<()>;
    fn WriteBehavior(&self) -> ::windows::core::Result<HttpCacheWriteBehavior>;
    fn SetWriteBehavior(&self, value: HttpCacheWriteBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCacheControl {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpCacheControl";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCacheControlVtbl {
    pub const fn new<Impl: IHttpCacheControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHttpCacheControlVtbl {
        unsafe extern "system" fn ReadBehavior<Impl: IHttpCacheControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HttpCacheReadBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadBehavior<Impl: IHttpCacheControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: HttpCacheReadBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReadBehavior(value).into()
        }
        unsafe extern "system" fn WriteBehavior<Impl: IHttpCacheControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HttpCacheWriteBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteBehavior<Impl: IHttpCacheControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: HttpCacheWriteBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWriteBehavior(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHttpCacheControl>, base.5, ReadBehavior::<Impl, OFFSET>, SetReadBehavior::<Impl, OFFSET>, WriteBehavior::<Impl, OFFSET>, SetWriteBehavior::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IHttpFilterImpl: Sized + IClosableImpl {
    fn SendRequestAsync(&self, request: &::core::option::Option<super::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IHttpFilter {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpFilter";
}
#[cfg(feature = "Foundation")]
impl IHttpFilterVtbl {
    pub const fn new<Impl: IHttpFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHttpFilterVtbl {
        unsafe extern "system" fn SendRequestAsync<Impl: IHttpFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendRequestAsync(&*(&request as *const <super::HttpRequestMessage as ::windows::core::Abi>::Abi as *const <super::HttpRequestMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHttpFilter>, base.5, SendRequestAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpServerCustomValidationRequestedEventArgsImpl: Sized {
    fn RequestMessage(&self) -> ::windows::core::Result<super::HttpRequestMessage>;
    fn ServerCertificate(&self) -> ::windows::core::Result<super::super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>>;
    fn Reject(&self) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpServerCustomValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpServerCustomValidationRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpServerCustomValidationRequestedEventArgsVtbl {
    pub const fn new<Impl: IHttpServerCustomValidationRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHttpServerCustomValidationRequestedEventArgsVtbl {
        unsafe extern "system" fn RequestMessage<Impl: IHttpServerCustomValidationRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificate<Impl: IHttpServerCustomValidationRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrorSeverity<Impl: IHttpServerCustomValidationRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerCertificateErrorSeverity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrors<Impl: IHttpServerCustomValidationRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerCertificateErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerIntermediateCertificates<Impl: IHttpServerCustomValidationRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerIntermediateCertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Impl: IHttpServerCustomValidationRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IHttpServerCustomValidationRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHttpServerCustomValidationRequestedEventArgs>, base.5, RequestMessage::<Impl, OFFSET>, ServerCertificate::<Impl, OFFSET>, ServerCertificateErrorSeverity::<Impl, OFFSET>, ServerCertificateErrors::<Impl, OFFSET>, ServerIntermediateCertificates::<Impl, OFFSET>, Reject::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
