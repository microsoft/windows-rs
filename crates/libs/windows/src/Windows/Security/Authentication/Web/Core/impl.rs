#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IFindAllAccountsResult_Impl: Sized {
    fn Accounts(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>;
    fn Status(&mut self) -> ::windows::core::Result<FindAllWebAccountsStatus>;
    fn ProviderError(&mut self) -> ::windows::core::Result<WebProviderError>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFindAllAccountsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IFindAllAccountsResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IFindAllAccountsResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindAllAccountsResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFindAllAccountsResult_Vtbl {
        unsafe extern "system" fn Accounts<Impl: IFindAllAccountsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Status<Impl: IFindAllAccountsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FindAllWebAccountsStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderError<Impl: IFindAllAccountsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFindAllAccountsResult, BASE_OFFSET>(),
            Accounts: Accounts::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            ProviderError: ProviderError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFindAllAccountsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAccountEventArgs_Impl: Sized {
    fn Account(&mut self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountEventArgs {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAccountEventArgs";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountEventArgs_Vtbl {
        unsafe extern "system" fn Account<Impl: IWebAccountEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountEventArgs, BASE_OFFSET>(), Account: Account::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebAccountMonitor_Impl: Sized {
    fn Updated(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DefaultSignInAccountChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDefaultSignInAccountChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountMonitor {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAccountMonitor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAccountMonitor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountMonitor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountMonitor_Vtbl {
        unsafe extern "system" fn Updated<Impl: IWebAccountMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUpdated<Impl: IWebAccountMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IWebAccountMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRemoved<Impl: IWebAccountMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultSignInAccountChanged<Impl: IWebAccountMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDefaultSignInAccountChanged<Impl: IWebAccountMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDefaultSignInAccountChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountMonitor, BASE_OFFSET>(),
            Updated: Updated::<Impl, IMPL_OFFSET>,
            RemoveUpdated: RemoveUpdated::<Impl, IMPL_OFFSET>,
            Removed: Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved: RemoveRemoved::<Impl, IMPL_OFFSET>,
            DefaultSignInAccountChanged: DefaultSignInAccountChanged::<Impl, IMPL_OFFSET>,
            RemoveDefaultSignInAccountChanged: RemoveDefaultSignInAccountChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebAccountMonitor2_Impl: Sized {
    fn AccountPictureUpdated(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountPictureUpdated(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountMonitor2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAccountMonitor2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAccountMonitor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountMonitor2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountMonitor2_Vtbl {
        unsafe extern "system" fn AccountPictureUpdated<Impl: IWebAccountMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAccountPictureUpdated<Impl: IWebAccountMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccountPictureUpdated(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountMonitor2, BASE_OFFSET>(),
            AccountPictureUpdated: AccountPictureUpdated::<Impl, IMPL_OFFSET>,
            RemoveAccountPictureUpdated: RemoveAccountPictureUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountMonitor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAuthenticationCoreManagerStatics_Impl: Sized {
    fn GetTokenSilentlyAsync(&mut self, request: &::core::option::Option<WebTokenRequest>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn GetTokenSilentlyWithWebAccountAsync(&mut self, request: &::core::option::Option<WebTokenRequest>, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn RequestTokenAsync(&mut self, request: &::core::option::Option<WebTokenRequest>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn RequestTokenWithWebAccountAsync(&mut self, request: &::core::option::Option<WebTokenRequest>, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn FindAccountAsync(&mut self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn FindAccountProviderAsync(&mut self, webaccountproviderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
    fn FindAccountProviderWithAuthorityAsync(&mut self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAuthenticationCoreManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationCoreManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAuthenticationCoreManagerStatics_Vtbl {
        unsafe extern "system" fn GetTokenSilentlyAsync<Impl: IWebAuthenticationCoreManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTokenSilentlyWithWebAccountAsync<Impl: IWebAuthenticationCoreManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestTokenAsync<Impl: IWebAuthenticationCoreManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestTokenWithWebAccountAsync<Impl: IWebAuthenticationCoreManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAccountAsync<Impl: IWebAuthenticationCoreManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAccountProviderAsync<Impl: IWebAuthenticationCoreManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAccountProviderWithAuthorityAsync<Impl: IWebAuthenticationCoreManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAuthenticationCoreManagerStatics, BASE_OFFSET>(),
            GetTokenSilentlyAsync: GetTokenSilentlyAsync::<Impl, IMPL_OFFSET>,
            GetTokenSilentlyWithWebAccountAsync: GetTokenSilentlyWithWebAccountAsync::<Impl, IMPL_OFFSET>,
            RequestTokenAsync: RequestTokenAsync::<Impl, IMPL_OFFSET>,
            RequestTokenWithWebAccountAsync: RequestTokenWithWebAccountAsync::<Impl, IMPL_OFFSET>,
            FindAccountAsync: FindAccountAsync::<Impl, IMPL_OFFSET>,
            FindAccountProviderAsync: FindAccountProviderAsync::<Impl, IMPL_OFFSET>,
            FindAccountProviderWithAuthorityAsync: FindAccountProviderWithAuthorityAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationCoreManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System", feature = "implement_exclusive"))]
pub trait IWebAuthenticationCoreManagerStatics2_Impl: Sized + IWebAuthenticationCoreManagerStatics_Impl {
    fn FindAccountProviderWithAuthorityForUserAsync(&mut self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING, user: &::core::option::Option<super::super::super::super::System::User>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerStatics2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System", feature = "implement_exclusive"))]
impl IWebAuthenticationCoreManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationCoreManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAuthenticationCoreManagerStatics2_Vtbl {
        unsafe extern "system" fn FindAccountProviderWithAuthorityForUserAsync<Impl: IWebAuthenticationCoreManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAuthenticationCoreManagerStatics2, BASE_OFFSET>(),
            FindAccountProviderWithAuthorityForUserAsync: FindAccountProviderWithAuthorityForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationCoreManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAuthenticationCoreManagerStatics3_Impl: Sized + IWebAuthenticationCoreManagerStatics_Impl {
    fn CreateWebAccountMonitor(&mut self, webaccounts: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::super::Credentials::WebAccount>>) -> ::windows::core::Result<WebAccountMonitor>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerStatics3 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAuthenticationCoreManagerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationCoreManagerStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAuthenticationCoreManagerStatics3_Vtbl {
        unsafe extern "system" fn CreateWebAccountMonitor<Impl: IWebAuthenticationCoreManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccounts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAuthenticationCoreManagerStatics3, BASE_OFFSET>(),
            CreateWebAccountMonitor: CreateWebAccountMonitor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationCoreManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System", feature = "implement_exclusive"))]
pub trait IWebAuthenticationCoreManagerStatics4_Impl: Sized + IWebAuthenticationCoreManagerStatics_Impl {
    fn FindAllAccountsAsync(&mut self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>;
    fn FindAllAccountsWithClientIdAsync(&mut self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>;
    fn FindSystemAccountProviderAsync(&mut self, webaccountproviderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
    fn FindSystemAccountProviderWithAuthorityAsync(&mut self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
    fn FindSystemAccountProviderWithAuthorityForUserAsync(&mut self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING, user: &::core::option::Option<super::super::super::super::System::User>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerStatics4 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics4";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System", feature = "implement_exclusive"))]
impl IWebAuthenticationCoreManagerStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationCoreManagerStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAuthenticationCoreManagerStatics4_Vtbl {
        unsafe extern "system" fn FindAllAccountsAsync<Impl: IWebAuthenticationCoreManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAllAccountsWithClientIdAsync<Impl: IWebAuthenticationCoreManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindSystemAccountProviderAsync<Impl: IWebAuthenticationCoreManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindSystemAccountProviderWithAuthorityAsync<Impl: IWebAuthenticationCoreManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindSystemAccountProviderWithAuthorityForUserAsync<Impl: IWebAuthenticationCoreManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAuthenticationCoreManagerStatics4, BASE_OFFSET>(),
            FindAllAccountsAsync: FindAllAccountsAsync::<Impl, IMPL_OFFSET>,
            FindAllAccountsWithClientIdAsync: FindAllAccountsWithClientIdAsync::<Impl, IMPL_OFFSET>,
            FindSystemAccountProviderAsync: FindSystemAccountProviderAsync::<Impl, IMPL_OFFSET>,
            FindSystemAccountProviderWithAuthorityAsync: FindSystemAccountProviderWithAuthorityAsync::<Impl, IMPL_OFFSET>,
            FindSystemAccountProviderWithAuthorityForUserAsync: FindSystemAccountProviderWithAuthorityForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationCoreManagerStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWebProviderError_Impl: Sized {
    fn ErrorCode(&mut self) -> ::windows::core::Result<u32>;
    fn ErrorMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebProviderError {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebProviderError";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWebProviderError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderError_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderError_Vtbl {
        unsafe extern "system" fn ErrorCode<Impl: IWebProviderError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ErrorMessage<Impl: IWebProviderError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IWebProviderError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebProviderError, BASE_OFFSET>(),
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            ErrorMessage: ErrorMessage::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebProviderError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderErrorFactory_Impl: Sized {
    fn Create(&mut self, errorcode: u32, errormessage: &::windows::core::HSTRING) -> ::windows::core::Result<WebProviderError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebProviderErrorFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebProviderErrorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebProviderErrorFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderErrorFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderErrorFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IWebProviderErrorFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: u32, errormessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebProviderErrorFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebProviderErrorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebTokenRequest_Impl: Sized {
    fn WebAccountProvider(&mut self) -> ::windows::core::Result<super::super::super::Credentials::WebAccountProvider>;
    fn Scope(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ClientId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PromptType(&mut self) -> ::windows::core::Result<WebTokenRequestPromptType>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebTokenRequest {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenRequest";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebTokenRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebTokenRequest_Vtbl {
        unsafe extern "system" fn WebAccountProvider<Impl: IWebTokenRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Scope<Impl: IWebTokenRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClientId<Impl: IWebTokenRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PromptType<Impl: IWebTokenRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebTokenRequestPromptType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IWebTokenRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebTokenRequest, BASE_OFFSET>(),
            WebAccountProvider: WebAccountProvider::<Impl, IMPL_OFFSET>,
            Scope: Scope::<Impl, IMPL_OFFSET>,
            ClientId: ClientId::<Impl, IMPL_OFFSET>,
            PromptType: PromptType::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebTokenRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWebTokenRequest2_Impl: Sized {
    fn AppProperties(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebTokenRequest2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenRequest2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWebTokenRequest2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenRequest2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebTokenRequest2_Vtbl {
        unsafe extern "system" fn AppProperties<Impl: IWebTokenRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebTokenRequest2, BASE_OFFSET>(), AppProperties: AppProperties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebTokenRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenRequest3_Impl: Sized {
    fn CorrelationId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCorrelationId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebTokenRequest3 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenRequest3";
}
#[cfg(feature = "implement_exclusive")]
impl IWebTokenRequest3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenRequest3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebTokenRequest3_Vtbl {
        unsafe extern "system" fn CorrelationId<Impl: IWebTokenRequest3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCorrelationId<Impl: IWebTokenRequest3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCorrelationId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebTokenRequest3, BASE_OFFSET>(),
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
            SetCorrelationId: SetCorrelationId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebTokenRequest3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebTokenRequestFactory_Impl: Sized {
    fn Create(&mut self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, scope: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenRequest>;
    fn CreateWithPromptType(&mut self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, scope: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, prompttype: WebTokenRequestPromptType) -> ::windows::core::Result<WebTokenRequest>;
    fn CreateWithProvider(&mut self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>) -> ::windows::core::Result<WebTokenRequest>;
    fn CreateWithScope(&mut self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, scope: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenRequest>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebTokenRequestFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenRequestFactory";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebTokenRequestFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenRequestFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebTokenRequestFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IWebTokenRequestFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithPromptType<Impl: IWebTokenRequestFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, prompttype: WebTokenRequestPromptType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithProvider<Impl: IWebTokenRequestFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithScope<Impl: IWebTokenRequestFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebTokenRequestFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithPromptType: CreateWithPromptType::<Impl, IMPL_OFFSET>,
            CreateWithProvider: CreateWithProvider::<Impl, IMPL_OFFSET>,
            CreateWithScope: CreateWithScope::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebTokenRequestFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWebTokenRequestResult_Impl: Sized {
    fn ResponseData(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<WebTokenResponse>>;
    fn ResponseStatus(&mut self) -> ::windows::core::Result<WebTokenRequestStatus>;
    fn ResponseError(&mut self) -> ::windows::core::Result<WebProviderError>;
    fn InvalidateCacheAsync(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebTokenRequestResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenRequestResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWebTokenRequestResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenRequestResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebTokenRequestResult_Vtbl {
        unsafe extern "system" fn ResponseData<Impl: IWebTokenRequestResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResponseStatus<Impl: IWebTokenRequestResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebTokenRequestStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResponseError<Impl: IWebTokenRequestResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InvalidateCacheAsync<Impl: IWebTokenRequestResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebTokenRequestResult, BASE_OFFSET>(),
            ResponseData: ResponseData::<Impl, IMPL_OFFSET>,
            ResponseStatus: ResponseStatus::<Impl, IMPL_OFFSET>,
            ResponseError: ResponseError::<Impl, IMPL_OFFSET>,
            InvalidateCacheAsync: InvalidateCacheAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebTokenRequestResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebTokenResponse_Impl: Sized {
    fn Token(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderError(&mut self) -> ::windows::core::Result<WebProviderError>;
    fn WebAccount(&mut self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenResponse";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebTokenResponse_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenResponse_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebTokenResponse_Vtbl {
        unsafe extern "system" fn Token<Impl: IWebTokenResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderError<Impl: IWebTokenResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WebAccount<Impl: IWebTokenResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IWebTokenResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebTokenResponse, BASE_OFFSET>(),
            Token: Token::<Impl, IMPL_OFFSET>,
            ProviderError: ProviderError::<Impl, IMPL_OFFSET>,
            WebAccount: WebAccount::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebTokenResponse as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebTokenResponseFactory_Impl: Sized {
    fn CreateWithToken(&mut self, token: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenResponse>;
    fn CreateWithTokenAndAccount(&mut self, token: &::windows::core::HSTRING, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<WebTokenResponse>;
    fn CreateWithTokenAccountAndError(&mut self, token: &::windows::core::HSTRING, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, error: &::core::option::Option<WebProviderError>) -> ::windows::core::Result<WebTokenResponse>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebTokenResponseFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.IWebTokenResponseFactory";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebTokenResponseFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebTokenResponseFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebTokenResponseFactory_Vtbl {
        unsafe extern "system" fn CreateWithToken<Impl: IWebTokenResponseFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithTokenAndAccount<Impl: IWebTokenResponseFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithTokenAccountAndError<Impl: IWebTokenResponseFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccount: ::windows::core::RawPtr, error: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebTokenResponseFactory, BASE_OFFSET>(),
            CreateWithToken: CreateWithToken::<Impl, IMPL_OFFSET>,
            CreateWithTokenAndAccount: CreateWithTokenAndAccount::<Impl, IMPL_OFFSET>,
            CreateWithTokenAccountAndError: CreateWithTokenAccountAndError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebTokenResponseFactory as ::windows::core::Interface>::IID
    }
}
