#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMicrosoftAccountMultiFactorAuthenticationManagerImpl: Sized {
    fn GetOneTimePassCodeAsync(&self, useraccountid: &::windows::core::HSTRING, codelength: u32) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>>;
    fn AddDeviceAsync(&self, useraccountid: &::windows::core::HSTRING, authenticationtoken: &::windows::core::HSTRING, wnschannelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn RemoveDeviceAsync(&self, useraccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn UpdateWnsChannelAsync(&self, useraccountid: &::windows::core::HSTRING, channeluri: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn GetSessionsAsync(&self, useraccountidlist: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>>;
    fn GetSessionsAndUnregisteredAccountsAsync(&self, useraccountidlist: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>>;
    fn ApproveSessionUsingAuthSessionInfoAsync(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: &::core::option::Option<MicrosoftAccountMultiFactorSessionInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn ApproveSessionAsync(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn DenySessionUsingAuthSessionInfoAsync(&self, authenticationsessioninfo: &::core::option::Option<MicrosoftAccountMultiFactorSessionInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn DenySessionAsync(&self, useraccountid: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorAuthenticationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorAuthenticationManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMicrosoftAccountMultiFactorAuthenticationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorAuthenticationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorAuthenticationManagerVtbl {
        unsafe extern "system" fn GetOneTimePassCodeAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, codelength: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOneTimePassCodeAsync(&*(&useraccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), codelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDeviceAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authenticationtoken: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, wnschannelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDeviceAsync(
                &*(&useraccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&authenticationtoken as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&wnschannelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDeviceAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveDeviceAsync(&*(&useraccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateWnsChannelAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, channeluri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateWnsChannelAsync(&*(&useraccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&channeluri as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionsAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountidlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionsAsync(&*(&useraccountidlist as *const <super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionsAndUnregisteredAccountsAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountidlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionsAndUnregisteredAccountsAsync(&*(&useraccountidlist as *const <super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApproveSessionUsingAuthSessionInfoAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApproveSessionUsingAuthSessionInfoAsync(sessionauthentictionstatus, &*(&authenticationsessioninfo as *const <MicrosoftAccountMultiFactorSessionInfo as ::windows::core::Abi>::Abi as *const <MicrosoftAccountMultiFactorSessionInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApproveSessionAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApproveSessionAsync(sessionauthentictionstatus, &*(&useraccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&sessionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), sessionauthenticationtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DenySessionUsingAuthSessionInfoAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authenticationsessioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DenySessionUsingAuthSessionInfoAsync(&*(&authenticationsessioninfo as *const <MicrosoftAccountMultiFactorSessionInfo as ::windows::core::Abi>::Abi as *const <MicrosoftAccountMultiFactorSessionInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DenySessionAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DenySessionAsync(&*(&useraccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&sessionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), sessionauthenticationtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMicrosoftAccountMultiFactorAuthenticationManager>,
            ::windows::core::GetTrustLevel,
            GetOneTimePassCodeAsync::<Impl, IMPL_OFFSET>,
            AddDeviceAsync::<Impl, IMPL_OFFSET>,
            RemoveDeviceAsync::<Impl, IMPL_OFFSET>,
            UpdateWnsChannelAsync::<Impl, IMPL_OFFSET>,
            GetSessionsAsync::<Impl, IMPL_OFFSET>,
            GetSessionsAndUnregisteredAccountsAsync::<Impl, IMPL_OFFSET>,
            ApproveSessionUsingAuthSessionInfoAsync::<Impl, IMPL_OFFSET>,
            ApproveSessionAsync::<Impl, IMPL_OFFSET>,
            DenySessionUsingAuthSessionInfoAsync::<Impl, IMPL_OFFSET>,
            DenySessionAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorAuthenticationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMicrosoftAccountMultiFactorAuthenticatorStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorAuthenticatorStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorAuthenticatorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMicrosoftAccountMultiFactorAuthenticatorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorAuthenticatorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorAuthenticatorStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IMicrosoftAccountMultiFactorAuthenticatorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMicrosoftAccountMultiFactorAuthenticatorStatics>, ::windows::core::GetTrustLevel, Current::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorAuthenticatorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMicrosoftAccountMultiFactorGetSessionsResultImpl: Sized {
    fn Sessions(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>;
    fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorGetSessionsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorGetSessionsResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMicrosoftAccountMultiFactorGetSessionsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorGetSessionsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorGetSessionsResultVtbl {
        unsafe extern "system" fn Sessions<Impl: IMicrosoftAccountMultiFactorGetSessionsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sessions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceResponse<Impl: IMicrosoftAccountMultiFactorGetSessionsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceResponse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMicrosoftAccountMultiFactorGetSessionsResult>, ::windows::core::GetTrustLevel, Sessions::<Impl, IMPL_OFFSET>, ServiceResponse::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorGetSessionsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMicrosoftAccountMultiFactorOneTimeCodedInfoImpl: Sized {
    fn Code(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TimeInterval(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn TimeToLive(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorOneTimeCodedInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMicrosoftAccountMultiFactorOneTimeCodedInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorOneTimeCodedInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorOneTimeCodedInfoVtbl {
        unsafe extern "system" fn Code<Impl: IMicrosoftAccountMultiFactorOneTimeCodedInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeInterval<Impl: IMicrosoftAccountMultiFactorOneTimeCodedInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeToLive<Impl: IMicrosoftAccountMultiFactorOneTimeCodedInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeToLive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceResponse<Impl: IMicrosoftAccountMultiFactorOneTimeCodedInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceResponse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMicrosoftAccountMultiFactorOneTimeCodedInfo>, ::windows::core::GetTrustLevel, Code::<Impl, IMPL_OFFSET>, TimeInterval::<Impl, IMPL_OFFSET>, TimeToLive::<Impl, IMPL_OFFSET>, ServiceResponse::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorOneTimeCodedInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMicrosoftAccountMultiFactorSessionInfoImpl: Sized {
    fn UserAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplaySessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApprovalStatus(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorSessionApprovalStatus>;
    fn AuthenticationType(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationType>;
    fn RequestTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorSessionInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMicrosoftAccountMultiFactorSessionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorSessionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorSessionInfoVtbl {
        unsafe extern "system" fn UserAccountId<Impl: IMicrosoftAccountMultiFactorSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionId<Impl: IMicrosoftAccountMultiFactorSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplaySessionId<Impl: IMicrosoftAccountMultiFactorSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplaySessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApprovalStatus<Impl: IMicrosoftAccountMultiFactorSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorSessionApprovalStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApprovalStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationType<Impl: IMicrosoftAccountMultiFactorSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestTime<Impl: IMicrosoftAccountMultiFactorSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationTime<Impl: IMicrosoftAccountMultiFactorSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMicrosoftAccountMultiFactorSessionInfo>,
            ::windows::core::GetTrustLevel,
            UserAccountId::<Impl, IMPL_OFFSET>,
            SessionId::<Impl, IMPL_OFFSET>,
            DisplaySessionId::<Impl, IMPL_OFFSET>,
            ApprovalStatus::<Impl, IMPL_OFFSET>,
            AuthenticationType::<Impl, IMPL_OFFSET>,
            RequestTime::<Impl, IMPL_OFFSET>,
            ExpirationTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorSessionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoImpl: Sized {
    fn Sessions(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>;
    fn UnregisteredAccounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoVtbl {
        unsafe extern "system" fn Sessions<Impl: IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sessions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisteredAccounts<Impl: IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisteredAccounts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceResponse<Impl: IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceResponse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>, ::windows::core::GetTrustLevel, Sessions::<Impl, IMPL_OFFSET>, UnregisteredAccounts::<Impl, IMPL_OFFSET>, ServiceResponse::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo as ::windows::core::Interface>::IID
    }
}
