#[cfg(feature = "implement_exclusive")]
pub trait IFindAllAccountsResultImpl: Sized {
    fn Accounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>;
    fn Status(&self) -> ::windows::core::Result<FindAllWebAccountsStatus>;
    fn ProviderError(&self) -> ::windows::core::Result<WebProviderError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFindAllAccountsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IFindAllAccountsResult";
}
#[cfg(feature = "implement_exclusive")]
impl IFindAllAccountsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindAllAccountsResultImpl, const OFFSET: isize>() -> IFindAllAccountsResultVtbl {
        unsafe extern "system" fn Accounts<Impl: IFindAllAccountsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Accounts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFindAllAccountsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FindAllWebAccountsStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderError<Impl: IFindAllAccountsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFindAllAccountsResult>, ::windows::core::GetTrustLevel, Accounts::<Impl, OFFSET>, Status::<Impl, OFFSET>, ProviderError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountEventArgsImpl: Sized {
    fn Account(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountEventArgs {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAccountEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountEventArgsImpl, const OFFSET: isize>() -> IWebAccountEventArgsVtbl {
        unsafe extern "system" fn Account<Impl: IWebAccountEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Account() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebAccountEventArgs>, ::windows::core::GetTrustLevel, Account::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountMonitorImpl: Sized {
    fn Updated(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DefaultSignInAccountChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDefaultSignInAccountChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountMonitor {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAccountMonitor";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountMonitorImpl, const OFFSET: isize>() -> IWebAccountMonitorVtbl {
        unsafe extern "system" fn Updated<Impl: IWebAccountMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: IWebAccountMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IWebAccountMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IWebAccountMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultSignInAccountChanged<Impl: IWebAccountMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultSignInAccountChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDefaultSignInAccountChanged<Impl: IWebAccountMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDefaultSignInAccountChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebAccountMonitor>, ::windows::core::GetTrustLevel, Updated::<Impl, OFFSET>, RemoveUpdated::<Impl, OFFSET>, Removed::<Impl, OFFSET>, RemoveRemoved::<Impl, OFFSET>, DefaultSignInAccountChanged::<Impl, OFFSET>, RemoveDefaultSignInAccountChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountMonitor2Impl: Sized {
    fn AccountPictureUpdated(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountPictureUpdated(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountMonitor2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAccountMonitor2";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountMonitor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountMonitor2Impl, const OFFSET: isize>() -> IWebAccountMonitor2Vtbl {
        unsafe extern "system" fn AccountPictureUpdated<Impl: IWebAccountMonitor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountPictureUpdated(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccountPictureUpdated<Impl: IWebAccountMonitor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccountPictureUpdated(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebAccountMonitor2>, ::windows::core::GetTrustLevel, AccountPictureUpdated::<Impl, OFFSET>, RemoveAccountPictureUpdated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationCoreManagerStaticsImpl: Sized {
    fn GetTokenSilentlyAsync(&self, request: &::core::option::Option<WebTokenRequest>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn GetTokenSilentlyWithWebAccountAsync(&self, request: &::core::option::Option<WebTokenRequest>, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn RequestTokenAsync(&self, request: &::core::option::Option<WebTokenRequest>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn RequestTokenWithWebAccountAsync(&self, request: &::core::option::Option<WebTokenRequest>, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn FindAccountAsync(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn FindAccountProviderAsync(&self, webaccountproviderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
    fn FindAccountProviderWithAuthorityAsync(&self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAuthenticationCoreManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationCoreManagerStaticsImpl, const OFFSET: isize>() -> IWebAuthenticationCoreManagerStaticsVtbl {
        unsafe extern "system" fn GetTokenSilentlyAsync<Impl: IWebAuthenticationCoreManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTokenSilentlyAsync(&*(&request as *const <WebTokenRequest as ::windows::core::Abi>::Abi as *const <WebTokenRequest as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTokenSilentlyWithWebAccountAsync<Impl: IWebAuthenticationCoreManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTokenSilentlyWithWebAccountAsync(&*(&request as *const <WebTokenRequest as ::windows::core::Abi>::Abi as *const <WebTokenRequest as ::windows::core::DefaultType>::DefaultType), &*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestTokenAsync<Impl: IWebAuthenticationCoreManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestTokenAsync(&*(&request as *const <WebTokenRequest as ::windows::core::Abi>::Abi as *const <WebTokenRequest as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestTokenWithWebAccountAsync<Impl: IWebAuthenticationCoreManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestTokenWithWebAccountAsync(&*(&request as *const <WebTokenRequest as ::windows::core::Abi>::Abi as *const <WebTokenRequest as ::windows::core::DefaultType>::DefaultType), &*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAccountAsync<Impl: IWebAuthenticationCoreManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAccountAsync(&*(&provider as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::DefaultType>::DefaultType), &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAccountProviderAsync<Impl: IWebAuthenticationCoreManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAccountProviderAsync(&*(&webaccountproviderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAccountProviderWithAuthorityAsync<Impl: IWebAuthenticationCoreManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAccountProviderWithAuthorityAsync(&*(&webaccountproviderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&authority as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IWebAuthenticationCoreManagerStatics>,
            ::windows::core::GetTrustLevel,
            GetTokenSilentlyAsync::<Impl, OFFSET>,
            GetTokenSilentlyWithWebAccountAsync::<Impl, OFFSET>,
            RequestTokenAsync::<Impl, OFFSET>,
            RequestTokenWithWebAccountAsync::<Impl, OFFSET>,
            FindAccountAsync::<Impl, OFFSET>,
            FindAccountProviderAsync::<Impl, OFFSET>,
            FindAccountProviderWithAuthorityAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationCoreManagerStatics2Impl: Sized + IWebAuthenticationCoreManagerStaticsImpl {
    fn FindAccountProviderWithAuthorityForUserAsync(&self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING, user: &::core::option::Option<super::super::super::super::System::User>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerStatics2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAuthenticationCoreManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationCoreManagerStatics2Impl, const OFFSET: isize>() -> IWebAuthenticationCoreManagerStatics2Vtbl {
        unsafe extern "system" fn FindAccountProviderWithAuthorityForUserAsync<Impl: IWebAuthenticationCoreManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAccountProviderWithAuthorityForUserAsync(
                &*(&webaccountproviderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&authority as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebAuthenticationCoreManagerStatics2>, ::windows::core::GetTrustLevel, FindAccountProviderWithAuthorityForUserAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationCoreManagerStatics3Impl: Sized + IWebAuthenticationCoreManagerStaticsImpl {
    fn CreateWebAccountMonitor(&self, webaccounts: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::super::Credentials::WebAccount>>) -> ::windows::core::Result<WebAccountMonitor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerStatics3 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAuthenticationCoreManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationCoreManagerStatics3Impl, const OFFSET: isize>() -> IWebAuthenticationCoreManagerStatics3Vtbl {
        unsafe extern "system" fn CreateWebAccountMonitor<Impl: IWebAuthenticationCoreManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccounts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWebAccountMonitor(&*(&webaccounts as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::super::Credentials::WebAccount> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::super::Credentials::WebAccount> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebAuthenticationCoreManagerStatics3>, ::windows::core::GetTrustLevel, CreateWebAccountMonitor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationCoreManagerStatics4Impl: Sized + IWebAuthenticationCoreManagerStaticsImpl {
    fn FindAllAccountsAsync(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>;
    fn FindAllAccountsWithClientIdAsync(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>;
    fn FindSystemAccountProviderAsync(&self, webaccountproviderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
    fn FindSystemAccountProviderWithAuthorityAsync(&self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
    fn FindSystemAccountProviderWithAuthorityForUserAsync(&self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING, user: &::core::option::Option<super::super::super::super::System::User>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerStatics4 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAuthenticationCoreManagerStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationCoreManagerStatics4Impl, const OFFSET: isize>() -> IWebAuthenticationCoreManagerStatics4Vtbl {
        unsafe extern "system" fn FindAllAccountsAsync<Impl: IWebAuthenticationCoreManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAccountsAsync(&*(&provider as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAccountsWithClientIdAsync<Impl: IWebAuthenticationCoreManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAccountsWithClientIdAsync(&*(&provider as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::DefaultType>::DefaultType), &*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindSystemAccountProviderAsync<Impl: IWebAuthenticationCoreManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindSystemAccountProviderAsync(&*(&webaccountproviderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindSystemAccountProviderWithAuthorityAsync<Impl: IWebAuthenticationCoreManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindSystemAccountProviderWithAuthorityAsync(&*(&webaccountproviderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&authority as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindSystemAccountProviderWithAuthorityForUserAsync<Impl: IWebAuthenticationCoreManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindSystemAccountProviderWithAuthorityForUserAsync(
                &*(&webaccountproviderid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&authority as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
            ) {
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
            ::windows::core::GetRuntimeClassName::<IWebAuthenticationCoreManagerStatics4>,
            ::windows::core::GetTrustLevel,
            FindAllAccountsAsync::<Impl, OFFSET>,
            FindAllAccountsWithClientIdAsync::<Impl, OFFSET>,
            FindSystemAccountProviderAsync::<Impl, OFFSET>,
            FindSystemAccountProviderWithAuthorityAsync::<Impl, OFFSET>,
            FindSystemAccountProviderWithAuthorityForUserAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderErrorImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
    fn ErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebProviderError {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebProviderError";
}
#[cfg(feature = "implement_exclusive")]
impl IWebProviderErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderErrorImpl, const OFFSET: isize>() -> IWebProviderErrorVtbl {
        unsafe extern "system" fn ErrorCode<Impl: IWebProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorMessage<Impl: IWebProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IWebProviderErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebProviderError>, ::windows::core::GetTrustLevel, ErrorCode::<Impl, OFFSET>, ErrorMessage::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderErrorFactoryImpl: Sized {
    fn Create(&self, errorcode: u32, errormessage: &::windows::core::HSTRING) -> ::windows::core::Result<WebProviderError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebProviderErrorFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebProviderErrorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebProviderErrorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderErrorFactoryImpl, const OFFSET: isize>() -> IWebProviderErrorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IWebProviderErrorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: u32, errormessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(errorcode, &*(&errormessage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebProviderErrorFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenRequestImpl: Sized {
    fn WebAccountProvider(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccountProvider>;
    fn Scope(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ClientId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PromptType(&self) -> ::windows::core::Result<WebTokenRequestPromptType>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebTokenRequest {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IWebTokenRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenRequestImpl, const OFFSET: isize>() -> IWebTokenRequestVtbl {
        unsafe extern "system" fn WebAccountProvider<Impl: IWebTokenRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccountProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Impl: IWebTokenRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientId<Impl: IWebTokenRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptType<Impl: IWebTokenRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebTokenRequestPromptType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PromptType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IWebTokenRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebTokenRequest>, ::windows::core::GetTrustLevel, WebAccountProvider::<Impl, OFFSET>, Scope::<Impl, OFFSET>, ClientId::<Impl, OFFSET>, PromptType::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenRequest2Impl: Sized {
    fn AppProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebTokenRequest2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenRequest2";
}
#[cfg(feature = "implement_exclusive")]
impl IWebTokenRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenRequest2Impl, const OFFSET: isize>() -> IWebTokenRequest2Vtbl {
        unsafe extern "system" fn AppProperties<Impl: IWebTokenRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebTokenRequest2>, ::windows::core::GetTrustLevel, AppProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenRequest3Impl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCorrelationId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebTokenRequest3 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenRequest3";
}
#[cfg(feature = "implement_exclusive")]
impl IWebTokenRequest3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenRequest3Impl, const OFFSET: isize>() -> IWebTokenRequest3Vtbl {
        unsafe extern "system" fn CorrelationId<Impl: IWebTokenRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IWebTokenRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCorrelationId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebTokenRequest3>, ::windows::core::GetTrustLevel, CorrelationId::<Impl, OFFSET>, SetCorrelationId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenRequestFactoryImpl: Sized {
    fn Create(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, scope: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenRequest>;
    fn CreateWithPromptType(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, scope: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, prompttype: WebTokenRequestPromptType) -> ::windows::core::Result<WebTokenRequest>;
    fn CreateWithProvider(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>) -> ::windows::core::Result<WebTokenRequest>;
    fn CreateWithScope(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, scope: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebTokenRequestFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenRequestFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebTokenRequestFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenRequestFactoryImpl, const OFFSET: isize>() -> IWebTokenRequestFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IWebTokenRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&provider as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::DefaultType>::DefaultType),
                &*(&scope as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPromptType<Impl: IWebTokenRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, prompttype: WebTokenRequestPromptType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithPromptType(
                &*(&provider as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::DefaultType>::DefaultType),
                &*(&scope as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                prompttype,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithProvider<Impl: IWebTokenRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithProvider(&*(&provider as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithScope<Impl: IWebTokenRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithScope(&*(&provider as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccountProvider as ::windows::core::DefaultType>::DefaultType), &*(&scope as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebTokenRequestFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>, CreateWithPromptType::<Impl, OFFSET>, CreateWithProvider::<Impl, OFFSET>, CreateWithScope::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenRequestResultImpl: Sized {
    fn ResponseData(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<WebTokenResponse>>;
    fn ResponseStatus(&self) -> ::windows::core::Result<WebTokenRequestStatus>;
    fn ResponseError(&self) -> ::windows::core::Result<WebProviderError>;
    fn InvalidateCacheAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebTokenRequestResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenRequestResult";
}
#[cfg(feature = "implement_exclusive")]
impl IWebTokenRequestResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenRequestResultImpl, const OFFSET: isize>() -> IWebTokenRequestResultVtbl {
        unsafe extern "system" fn ResponseData<Impl: IWebTokenRequestResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResponseStatus<Impl: IWebTokenRequestResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebTokenRequestStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResponseError<Impl: IWebTokenRequestResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateCacheAsync<Impl: IWebTokenRequestResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidateCacheAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebTokenRequestResult>, ::windows::core::GetTrustLevel, ResponseData::<Impl, OFFSET>, ResponseStatus::<Impl, OFFSET>, ResponseError::<Impl, OFFSET>, InvalidateCacheAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenResponseImpl: Sized {
    fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderError(&self) -> ::windows::core::Result<WebProviderError>;
    fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenResponse";
}
#[cfg(feature = "implement_exclusive")]
impl IWebTokenResponseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenResponseImpl, const OFFSET: isize>() -> IWebTokenResponseVtbl {
        unsafe extern "system" fn Token<Impl: IWebTokenResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Token() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderError<Impl: IWebTokenResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebAccount<Impl: IWebTokenResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IWebTokenResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebTokenResponse>, ::windows::core::GetTrustLevel, Token::<Impl, OFFSET>, ProviderError::<Impl, OFFSET>, WebAccount::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenResponseFactoryImpl: Sized {
    fn CreateWithToken(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenResponse>;
    fn CreateWithTokenAndAccount(&self, token: &::windows::core::HSTRING, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<WebTokenResponse>;
    fn CreateWithTokenAccountAndError(&self, token: &::windows::core::HSTRING, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, error: &::core::option::Option<WebProviderError>) -> ::windows::core::Result<WebTokenResponse>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebTokenResponseFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenResponseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebTokenResponseFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenResponseFactoryImpl, const OFFSET: isize>() -> IWebTokenResponseFactoryVtbl {
        unsafe extern "system" fn CreateWithToken<Impl: IWebTokenResponseFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithToken(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithTokenAndAccount<Impl: IWebTokenResponseFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithTokenAndAccount(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithTokenAccountAndError<Impl: IWebTokenResponseFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccount: ::windows::core::RawPtr, error: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithTokenAccountAndError(
                &*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType),
                &*(&error as *const <WebProviderError as ::windows::core::Abi>::Abi as *const <WebProviderError as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebTokenResponseFactory>, ::windows::core::GetTrustLevel, CreateWithToken::<Impl, OFFSET>, CreateWithTokenAndAccount::<Impl, OFFSET>, CreateWithTokenAccountAndError::<Impl, OFFSET>)
    }
}
