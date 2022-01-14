#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMicrosoftAccountMultiFactorAuthenticationManager_Impl: Sized {
    fn GetOneTimePassCodeAsync(&mut self, useraccountid: &::windows::core::HSTRING, codelength: u32) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>>;
    fn AddDeviceAsync(&mut self, useraccountid: &::windows::core::HSTRING, authenticationtoken: &::windows::core::HSTRING, wnschannelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn RemoveDeviceAsync(&mut self, useraccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn UpdateWnsChannelAsync(&mut self, useraccountid: &::windows::core::HSTRING, channeluri: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn GetSessionsAsync(&mut self, useraccountidlist: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>>;
    fn GetSessionsAndUnregisteredAccountsAsync(&mut self, useraccountidlist: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>>;
    fn ApproveSessionUsingAuthSessionInfoAsync(&mut self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: &::core::option::Option<MicrosoftAccountMultiFactorSessionInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn ApproveSessionAsync(&mut self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn DenySessionUsingAuthSessionInfoAsync(&mut self, authenticationsessioninfo: &::core::option::Option<MicrosoftAccountMultiFactorSessionInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn DenySessionAsync(&mut self, useraccountid: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorAuthenticationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorAuthenticationManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMicrosoftAccountMultiFactorAuthenticationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorAuthenticationManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorAuthenticationManager_Vtbl {
        unsafe extern "system" fn GetOneTimePassCodeAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, codelength: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddDeviceAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authenticationtoken: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, wnschannelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDeviceAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateWnsChannelAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, channeluri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSessionsAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountidlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSessionsAndUnregisteredAccountsAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountidlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ApproveSessionUsingAuthSessionInfoAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ApproveSessionAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DenySessionUsingAuthSessionInfoAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authenticationsessioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DenySessionAsync<Impl: IMicrosoftAccountMultiFactorAuthenticationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMicrosoftAccountMultiFactorAuthenticationManager, BASE_OFFSET>(),
            GetOneTimePassCodeAsync: GetOneTimePassCodeAsync::<Impl, IMPL_OFFSET>,
            AddDeviceAsync: AddDeviceAsync::<Impl, IMPL_OFFSET>,
            RemoveDeviceAsync: RemoveDeviceAsync::<Impl, IMPL_OFFSET>,
            UpdateWnsChannelAsync: UpdateWnsChannelAsync::<Impl, IMPL_OFFSET>,
            GetSessionsAsync: GetSessionsAsync::<Impl, IMPL_OFFSET>,
            GetSessionsAndUnregisteredAccountsAsync: GetSessionsAndUnregisteredAccountsAsync::<Impl, IMPL_OFFSET>,
            ApproveSessionUsingAuthSessionInfoAsync: ApproveSessionUsingAuthSessionInfoAsync::<Impl, IMPL_OFFSET>,
            ApproveSessionAsync: ApproveSessionAsync::<Impl, IMPL_OFFSET>,
            DenySessionUsingAuthSessionInfoAsync: DenySessionUsingAuthSessionInfoAsync::<Impl, IMPL_OFFSET>,
            DenySessionAsync: DenySessionAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorAuthenticationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMicrosoftAccountMultiFactorAuthenticatorStatics_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorAuthenticatorStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorAuthenticatorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMicrosoftAccountMultiFactorAuthenticatorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorAuthenticatorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorAuthenticatorStatics_Vtbl {
        unsafe extern "system" fn Current<Impl: IMicrosoftAccountMultiFactorAuthenticatorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMicrosoftAccountMultiFactorAuthenticatorStatics, BASE_OFFSET>(),
            Current: Current::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorAuthenticatorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMicrosoftAccountMultiFactorGetSessionsResult_Impl: Sized {
    fn Sessions(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>;
    fn ServiceResponse(&mut self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorGetSessionsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorGetSessionsResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMicrosoftAccountMultiFactorGetSessionsResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorGetSessionsResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorGetSessionsResult_Vtbl {
        unsafe extern "system" fn Sessions<Impl: IMicrosoftAccountMultiFactorGetSessionsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServiceResponse<Impl: IMicrosoftAccountMultiFactorGetSessionsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMicrosoftAccountMultiFactorGetSessionsResult, BASE_OFFSET>(),
            Sessions: Sessions::<Impl, IMPL_OFFSET>,
            ServiceResponse: ServiceResponse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorGetSessionsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMicrosoftAccountMultiFactorOneTimeCodedInfo_Impl: Sized {
    fn Code(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TimeInterval(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn TimeToLive(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn ServiceResponse(&mut self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorOneTimeCodedInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMicrosoftAccountMultiFactorOneTimeCodedInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorOneTimeCodedInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorOneTimeCodedInfo_Vtbl {
        unsafe extern "system" fn Code<Impl: IMicrosoftAccountMultiFactorOneTimeCodedInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimeInterval<Impl: IMicrosoftAccountMultiFactorOneTimeCodedInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimeToLive<Impl: IMicrosoftAccountMultiFactorOneTimeCodedInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServiceResponse<Impl: IMicrosoftAccountMultiFactorOneTimeCodedInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMicrosoftAccountMultiFactorOneTimeCodedInfo, BASE_OFFSET>(),
            Code: Code::<Impl, IMPL_OFFSET>,
            TimeInterval: TimeInterval::<Impl, IMPL_OFFSET>,
            TimeToLive: TimeToLive::<Impl, IMPL_OFFSET>,
            ServiceResponse: ServiceResponse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorOneTimeCodedInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMicrosoftAccountMultiFactorSessionInfo_Impl: Sized {
    fn UserAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplaySessionId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApprovalStatus(&mut self) -> ::windows::core::Result<MicrosoftAccountMultiFactorSessionApprovalStatus>;
    fn AuthenticationType(&mut self) -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationType>;
    fn RequestTime(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
    fn ExpirationTime(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorSessionInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMicrosoftAccountMultiFactorSessionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorSessionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorSessionInfo_Vtbl {
        unsafe extern "system" fn UserAccountId<Impl: IMicrosoftAccountMultiFactorSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SessionId<Impl: IMicrosoftAccountMultiFactorSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplaySessionId<Impl: IMicrosoftAccountMultiFactorSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ApprovalStatus<Impl: IMicrosoftAccountMultiFactorSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorSessionApprovalStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AuthenticationType<Impl: IMicrosoftAccountMultiFactorSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestTime<Impl: IMicrosoftAccountMultiFactorSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpirationTime<Impl: IMicrosoftAccountMultiFactorSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMicrosoftAccountMultiFactorSessionInfo, BASE_OFFSET>(),
            UserAccountId: UserAccountId::<Impl, IMPL_OFFSET>,
            SessionId: SessionId::<Impl, IMPL_OFFSET>,
            DisplaySessionId: DisplaySessionId::<Impl, IMPL_OFFSET>,
            ApprovalStatus: ApprovalStatus::<Impl, IMPL_OFFSET>,
            AuthenticationType: AuthenticationType::<Impl, IMPL_OFFSET>,
            RequestTime: RequestTime::<Impl, IMPL_OFFSET>,
            ExpirationTime: ExpirationTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorSessionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Impl: Sized {
    fn Sessions(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>;
    fn UnregisteredAccounts(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ServiceResponse(&mut self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Vtbl {
        unsafe extern "system" fn Sessions<Impl: IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UnregisteredAccounts<Impl: IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServiceResponse<Impl: IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo, BASE_OFFSET>(),
            Sessions: Sessions::<Impl, IMPL_OFFSET>,
            UnregisteredAccounts: UnregisteredAccounts::<Impl, IMPL_OFFSET>,
            ServiceResponse: ServiceResponse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo as ::windows::core::Interface>::IID
    }
}
