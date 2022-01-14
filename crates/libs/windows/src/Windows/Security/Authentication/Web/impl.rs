#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebAuthenticationBrokerStatics_Impl: Sized {
    fn AuthenticateWithCallbackUriAsync(&mut self, options: WebAuthenticationOptions, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, callbackuri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
    fn AuthenticateWithoutCallbackUriAsync(&mut self, options: WebAuthenticationOptions, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
    fn GetCurrentApplicationCallbackUri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAuthenticationBrokerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAuthenticationBrokerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationBrokerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAuthenticationBrokerStatics_Vtbl {
        unsafe extern "system" fn AuthenticateWithCallbackUriAsync<Impl: IWebAuthenticationBrokerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: WebAuthenticationOptions, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateWithCallbackUriAsync(options, &*(&requesturi as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&callbackuri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticateWithoutCallbackUriAsync<Impl: IWebAuthenticationBrokerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: WebAuthenticationOptions, requesturi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateWithoutCallbackUriAsync(options, &*(&requesturi as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentApplicationCallbackUri<Impl: IWebAuthenticationBrokerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentApplicationCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAuthenticationBrokerStatics, BASE_OFFSET>(),
            AuthenticateWithCallbackUriAsync: AuthenticateWithCallbackUriAsync::<Impl, IMPL_OFFSET>,
            AuthenticateWithoutCallbackUriAsync: AuthenticateWithoutCallbackUriAsync::<Impl, IMPL_OFFSET>,
            GetCurrentApplicationCallbackUri: GetCurrentApplicationCallbackUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationBrokerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWebAuthenticationBrokerStatics2_Impl: Sized {
    fn AuthenticateAndContinue(&mut self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AuthenticateWithCallbackUriAndContinue(&mut self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, callbackuri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue(&mut self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, callbackuri: &::core::option::Option<super::super::super::Foundation::Uri>, continuationdata: &::core::option::Option<super::super::super::Foundation::Collections::ValueSet>, options: WebAuthenticationOptions) -> ::windows::core::Result<()>;
    fn AuthenticateSilentlyAsync(&mut self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
    fn AuthenticateSilentlyWithOptionsAsync(&mut self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, options: WebAuthenticationOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAuthenticationBrokerStatics2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWebAuthenticationBrokerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationBrokerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAuthenticationBrokerStatics2_Vtbl {
        unsafe extern "system" fn AuthenticateAndContinue<Impl: IWebAuthenticationBrokerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AuthenticateAndContinue(&*(&requesturi as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticateWithCallbackUriAndContinue<Impl: IWebAuthenticationBrokerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AuthenticateWithCallbackUriAndContinue(&*(&requesturi as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&callbackuri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue<Impl: IWebAuthenticationBrokerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr, continuationdata: ::windows::core::RawPtr, options: WebAuthenticationOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue(
                    &*(&requesturi as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                    &*(&callbackuri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                    &*(&continuationdata as *const <super::super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType),
                    options,
                )
                .into()
        }
        unsafe extern "system" fn AuthenticateSilentlyAsync<Impl: IWebAuthenticationBrokerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateSilentlyAsync(&*(&requesturi as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticateSilentlyWithOptionsAsync<Impl: IWebAuthenticationBrokerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, options: WebAuthenticationOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateSilentlyWithOptionsAsync(&*(&requesturi as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAuthenticationBrokerStatics2, BASE_OFFSET>(),
            AuthenticateAndContinue: AuthenticateAndContinue::<Impl, IMPL_OFFSET>,
            AuthenticateWithCallbackUriAndContinue: AuthenticateWithCallbackUriAndContinue::<Impl, IMPL_OFFSET>,
            AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue: AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue::<Impl, IMPL_OFFSET>,
            AuthenticateSilentlyAsync: AuthenticateSilentlyAsync::<Impl, IMPL_OFFSET>,
            AuthenticateSilentlyWithOptionsAsync: AuthenticateSilentlyWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationBrokerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationResult_Impl: Sized {
    fn ResponseData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ResponseStatus(&mut self) -> ::windows::core::Result<WebAuthenticationStatus>;
    fn ResponseErrorDetail(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.IWebAuthenticationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAuthenticationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAuthenticationResult_Vtbl {
        unsafe extern "system" fn ResponseData<Impl: IWebAuthenticationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseStatus<Impl: IWebAuthenticationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAuthenticationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseErrorDetail<Impl: IWebAuthenticationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseErrorDetail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAuthenticationResult, BASE_OFFSET>(),
            ResponseData: ResponseData::<Impl, IMPL_OFFSET>,
            ResponseStatus: ResponseStatus::<Impl, IMPL_OFFSET>,
            ResponseErrorDetail: ResponseErrorDetail::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationResult as ::windows::core::Interface>::IID
    }
}
