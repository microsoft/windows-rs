#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderBaseReportOperation_Impl: Sized {
    fn ReportCompleted(&mut self) -> ::windows::core::Result<()>;
    fn ReportError(&mut self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderBaseReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderBaseReportOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderBaseReportOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderBaseReportOperation_Vtbl {
        unsafe extern "system" fn ReportCompleted<Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        unsafe extern "system" fn ReportError<Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportError(&*(&value as *const <super::Core::WebProviderError as ::windows::core::Abi>::Abi as *const <super::Core::WebProviderError as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderBaseReportOperation, BASE_OFFSET>(),
            ReportCompleted: ReportCompleted::<Impl, IMPL_OFFSET>,
            ReportError: ReportError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderBaseReportOperation as ::windows::core::Interface>::IID
    }
}
pub trait IWebAccountProviderOperation_Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<WebAccountProviderOperationKind>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation";
}
impl IWebAccountProviderOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderOperation_Vtbl {
        unsafe extern "system" fn Kind<Impl: IWebAccountProviderOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountProviderOperationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderOperation, BASE_OFFSET>(), Kind: Kind::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderSilentReportOperation_Impl: Sized + IWebAccountProviderBaseReportOperation_Impl {
    fn ReportUserInteractionRequired(&mut self) -> ::windows::core::Result<()>;
    fn ReportUserInteractionRequiredWithError(&mut self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderSilentReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderSilentReportOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderSilentReportOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderSilentReportOperation_Vtbl {
        unsafe extern "system" fn ReportUserInteractionRequired<Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportUserInteractionRequired().into()
        }
        unsafe extern "system" fn ReportUserInteractionRequiredWithError<Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportUserInteractionRequiredWithError(&*(&value as *const <super::Core::WebProviderError as ::windows::core::Abi>::Abi as *const <super::Core::WebProviderError as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderSilentReportOperation, BASE_OFFSET>(),
            ReportUserInteractionRequired: ReportUserInteractionRequired::<Impl, IMPL_OFFSET>,
            ReportUserInteractionRequiredWithError: ReportUserInteractionRequiredWithError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderSilentReportOperation as ::windows::core::Interface>::IID
    }
}
pub trait IWebAccountProviderTokenObjects_Impl: Sized {
    fn Operation(&mut self) -> ::windows::core::Result<IWebAccountProviderOperation>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderTokenObjects {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects";
}
impl IWebAccountProviderTokenObjects_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderTokenObjects_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderTokenObjects_Vtbl {
        unsafe extern "system" fn Operation<Impl: IWebAccountProviderTokenObjects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderTokenObjects, BASE_OFFSET>(),
            Operation: Operation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenObjects as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "System")]
pub trait IWebAccountProviderTokenObjects2_Impl: Sized + IWebAccountProviderTokenObjects_Impl {
    fn User(&mut self) -> ::windows::core::Result<super::super::super::super::System::User>;
}
#[cfg(feature = "System")]
impl ::windows::core::RuntimeName for IWebAccountProviderTokenObjects2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2";
}
#[cfg(feature = "System")]
impl IWebAccountProviderTokenObjects2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderTokenObjects2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderTokenObjects2_Vtbl {
        unsafe extern "system" fn User<Impl: IWebAccountProviderTokenObjects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderTokenObjects2, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenObjects2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IWebAccountProviderTokenOperation_Impl: Sized + IWebAccountProviderOperation_Impl {
    fn ProviderRequest(&mut self) -> ::windows::core::Result<WebProviderTokenRequest>;
    fn ProviderResponses(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>>;
    fn SetCacheExpirationTime(&mut self, value: &super::super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn CacheExpirationTime(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IWebAccountProviderTokenOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IWebAccountProviderTokenOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderTokenOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderTokenOperation_Vtbl {
        unsafe extern "system" fn ProviderRequest<Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderResponses<Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderResponses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCacheExpirationTime<Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCacheExpirationTime(&*(&value as *const <super::super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CacheExpirationTime<Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderTokenOperation, BASE_OFFSET>(),
            ProviderRequest: ProviderRequest::<Impl, IMPL_OFFSET>,
            ProviderResponses: ProviderResponses::<Impl, IMPL_OFFSET>,
            SetCacheExpirationTime: SetCacheExpirationTime::<Impl, IMPL_OFFSET>,
            CacheExpirationTime: CacheExpirationTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderUIReportOperation_Impl: Sized + IWebAccountProviderBaseReportOperation_Impl {
    fn ReportUserCanceled(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderUIReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderUIReportOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderUIReportOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderUIReportOperation_Vtbl {
        unsafe extern "system" fn ReportUserCanceled<Impl: IWebAccountProviderUIReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportUserCanceled().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderUIReportOperation, BASE_OFFSET>(),
            ReportUserCanceled: ReportUserCanceled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderUIReportOperation as ::windows::core::Interface>::IID
    }
}
