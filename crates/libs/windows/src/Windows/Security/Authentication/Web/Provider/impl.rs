#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderBaseReportOperation_Impl: Sized {
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
    fn ReportError(&self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderBaseReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderBaseReportOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: isize>() -> IWebAccountProviderBaseReportOperation_Vtbl {
        unsafe extern "system" fn ReportCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportCompleted().into()
        }
        unsafe extern "system" fn ReportError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportError(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderBaseReportOperation, OFFSET>(),
            ReportCompleted: ReportCompleted::<Identity, Impl, OFFSET>,
            ReportError: ReportError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderBaseReportOperation as ::windows::core::Interface>::IID
    }
}
pub trait IWebAccountProviderOperation_Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation";
}
impl IWebAccountProviderOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderOperation_Impl, const OFFSET: isize>() -> IWebAccountProviderOperation_Vtbl {
        unsafe extern "system" fn Kind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountProviderOperationKind) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Kind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderOperation, OFFSET>(), Kind: Kind::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderSilentReportOperation_Impl: Sized + IWebAccountProviderBaseReportOperation_Impl {
    fn ReportUserInteractionRequired(&self) -> ::windows::core::Result<()>;
    fn ReportUserInteractionRequiredWithError(&self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderSilentReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderSilentReportOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: isize>() -> IWebAccountProviderSilentReportOperation_Vtbl {
        unsafe extern "system" fn ReportUserInteractionRequired<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportUserInteractionRequired().into()
        }
        unsafe extern "system" fn ReportUserInteractionRequiredWithError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportUserInteractionRequiredWithError(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderSilentReportOperation, OFFSET>(),
            ReportUserInteractionRequired: ReportUserInteractionRequired::<Identity, Impl, OFFSET>,
            ReportUserInteractionRequiredWithError: ReportUserInteractionRequiredWithError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderSilentReportOperation as ::windows::core::Interface>::IID
    }
}
pub trait IWebAccountProviderTokenObjects_Impl: Sized {
    fn Operation(&self) -> ::windows::core::Result<IWebAccountProviderOperation>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderTokenObjects {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects";
}
impl IWebAccountProviderTokenObjects_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenObjects_Impl, const OFFSET: isize>() -> IWebAccountProviderTokenObjects_Vtbl {
        unsafe extern "system" fn Operation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenObjects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderTokenObjects, OFFSET>(),
            Operation: Operation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenObjects as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "System")]
pub trait IWebAccountProviderTokenObjects2_Impl: Sized + IWebAccountProviderTokenObjects_Impl {
    fn User(&self) -> ::windows::core::Result<super::super::super::super::System::User>;
}
#[cfg(feature = "System")]
impl ::windows::core::RuntimeName for IWebAccountProviderTokenObjects2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2";
}
#[cfg(feature = "System")]
impl IWebAccountProviderTokenObjects2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenObjects2_Impl, const OFFSET: isize>() -> IWebAccountProviderTokenObjects2_Vtbl {
        unsafe extern "system" fn User<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenObjects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.User() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderTokenObjects2, OFFSET>(), User: User::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenObjects2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IWebAccountProviderTokenOperation_Impl: Sized + IWebAccountProviderOperation_Impl {
    fn ProviderRequest(&self) -> ::windows::core::Result<WebProviderTokenRequest>;
    fn ProviderResponses(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>>;
    fn SetCacheExpirationTime(&self, value: &super::super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn CacheExpirationTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IWebAccountProviderTokenOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation";
}
#[cfg(feature = "Foundation_Collections")]
impl IWebAccountProviderTokenOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>() -> IWebAccountProviderTokenOperation_Vtbl {
        unsafe extern "system" fn ProviderRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProviderRequest() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderResponses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProviderResponses() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCacheExpirationTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCacheExpirationTime(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CacheExpirationTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CacheExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderTokenOperation, OFFSET>(),
            ProviderRequest: ProviderRequest::<Identity, Impl, OFFSET>,
            ProviderResponses: ProviderResponses::<Identity, Impl, OFFSET>,
            SetCacheExpirationTime: SetCacheExpirationTime::<Identity, Impl, OFFSET>,
            CacheExpirationTime: CacheExpirationTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderUIReportOperation_Impl: Sized + IWebAccountProviderBaseReportOperation_Impl {
    fn ReportUserCanceled(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderUIReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderUIReportOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderUIReportOperation_Impl, const OFFSET: isize>() -> IWebAccountProviderUIReportOperation_Vtbl {
        unsafe extern "system" fn ReportUserCanceled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderUIReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportUserCanceled().into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderUIReportOperation, OFFSET>(),
            ReportUserCanceled: ReportUserCanceled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderUIReportOperation as ::windows::core::Interface>::IID
    }
}
