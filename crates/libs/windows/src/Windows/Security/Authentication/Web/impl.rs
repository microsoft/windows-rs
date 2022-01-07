#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationBrokerStaticsImpl: Sized {
    fn AuthenticateWithCallbackUriAsync(&self, options: WebAuthenticationOptions, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, callbackuri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
    fn AuthenticateWithoutCallbackUriAsync(&self, options: WebAuthenticationOptions, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
    fn GetCurrentApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAuthenticationBrokerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAuthenticationBrokerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationBrokerStaticsImpl, const OFFSET: isize>() -> IWebAuthenticationBrokerStaticsVtbl {
        unsafe extern "system" fn AuthenticateWithCallbackUriAsync<Impl: IWebAuthenticationBrokerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: WebAuthenticationOptions, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AuthenticateWithoutCallbackUriAsync<Impl: IWebAuthenticationBrokerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: WebAuthenticationOptions, requesturi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentApplicationCallbackUri<Impl: IWebAuthenticationBrokerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebAuthenticationBrokerStatics>, ::windows::core::GetTrustLevel, AuthenticateWithCallbackUriAsync::<Impl, OFFSET>, AuthenticateWithoutCallbackUriAsync::<Impl, OFFSET>, GetCurrentApplicationCallbackUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationBrokerStatics2Impl: Sized {
    fn AuthenticateAndContinue(&self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AuthenticateWithCallbackUriAndContinue(&self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, callbackuri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue(&self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, callbackuri: &::core::option::Option<super::super::super::Foundation::Uri>, continuationdata: &::core::option::Option<super::super::super::Foundation::Collections::ValueSet>, options: WebAuthenticationOptions) -> ::windows::core::Result<()>;
    fn AuthenticateSilentlyAsync(&self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
    fn AuthenticateSilentlyWithOptionsAsync(&self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, options: WebAuthenticationOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAuthenticationBrokerStatics2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAuthenticationBrokerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationBrokerStatics2Impl, const OFFSET: isize>() -> IWebAuthenticationBrokerStatics2Vtbl {
        unsafe extern "system" fn AuthenticateAndContinue<Impl: IWebAuthenticationBrokerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AuthenticateAndContinue(&*(&requesturi as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticateWithCallbackUriAndContinue<Impl: IWebAuthenticationBrokerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AuthenticateWithCallbackUriAndContinue(&*(&requesturi as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&callbackuri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue<Impl: IWebAuthenticationBrokerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr, continuationdata: ::windows::core::RawPtr, options: WebAuthenticationOptions) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AuthenticateSilentlyAsync<Impl: IWebAuthenticationBrokerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AuthenticateSilentlyWithOptionsAsync<Impl: IWebAuthenticationBrokerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, options: WebAuthenticationOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWebAuthenticationBrokerStatics2>,
            ::windows::core::GetTrustLevel,
            AuthenticateAndContinue::<Impl, OFFSET>,
            AuthenticateWithCallbackUriAndContinue::<Impl, OFFSET>,
            AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue::<Impl, OFFSET>,
            AuthenticateSilentlyAsync::<Impl, OFFSET>,
            AuthenticateSilentlyWithOptionsAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationResultImpl: Sized {
    fn ResponseData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ResponseStatus(&self) -> ::windows::core::Result<WebAuthenticationStatus>;
    fn ResponseErrorDetail(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.IWebAuthenticationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAuthenticationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationResultImpl, const OFFSET: isize>() -> IWebAuthenticationResultVtbl {
        unsafe extern "system" fn ResponseData<Impl: IWebAuthenticationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResponseStatus<Impl: IWebAuthenticationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAuthenticationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResponseErrorDetail<Impl: IWebAuthenticationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebAuthenticationResult>, ::windows::core::GetTrustLevel, ResponseData::<Impl, OFFSET>, ResponseStatus::<Impl, OFFSET>, ResponseErrorDetail::<Impl, OFFSET>)
    }
}
