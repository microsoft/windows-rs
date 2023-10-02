#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicrosoftAccountMultiFactorAuthenticationManager {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticationManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicrosoftAccountMultiFactorAuthenticationManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0fd340a5_f574_4320_a08e_0a19a82322aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetOneTimePassCodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, codelength: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetOneTimePassCodeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AddDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, authenticationtoken: ::std::mem::MaybeUninit<::windows_core::HSTRING>, wnschannelid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateWnsChannelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, channeluri: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateWnsChannelAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountidlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessionsAndUnregisteredAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountidlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionsAndUnregisteredAccountsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ApproveSessionUsingAuthSessionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApproveSessionUsingAuthSessionInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ApproveSessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, sessionid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApproveSessionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DenySessionUsingAuthSessionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authenticationsessioninfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DenySessionUsingAuthSessionInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DenySessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, sessionid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DenySessionAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicrosoftAccountMultiFactorAuthenticatorStatics {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticatorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicrosoftAccountMultiFactorAuthenticatorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd964c2e6_f446_4c71_8b79_6ea4024aa9b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicrosoftAccountMultiFactorGetSessionsResult {
    type Vtable = IMicrosoftAccountMultiFactorGetSessionsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicrosoftAccountMultiFactorGetSessionsResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e23a9a0_e9fa_497a_95de_6d5747bf974c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Sessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sessions: usize,
    pub ServiceResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    type Vtable = IMicrosoftAccountMultiFactorOneTimeCodedInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82ba264b_d87c_4668_a976_40cfae547d08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Code: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TimeInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeInterval: usize,
    #[cfg(feature = "Foundation")]
    pub TimeToLive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToLive: usize,
    pub ServiceResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMicrosoftAccountMultiFactorSessionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicrosoftAccountMultiFactorSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorSessionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicrosoftAccountMultiFactorSessionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f7eabb4_a278_4635_b765_b494eb260af4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorSessionInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UserAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplaySessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ApprovalStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorSessionApprovalStatus) -> ::windows_core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorAuthenticationType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa7ec5fb_da3f_4088_a20d_5618afadb2e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Sessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sessions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UnregisteredAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UnregisteredAccounts: usize,
    pub ServiceResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MicrosoftAccountMultiFactorAuthenticationManager(::windows_core::IUnknown);
impl MicrosoftAccountMultiFactorAuthenticationManager {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetOneTimePassCodeAsync(&self, useraccountid: &::windows_core::HSTRING, codelength: u32) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOneTimePassCodeAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(useraccountid), codelength, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AddDeviceAsync(&self, useraccountid: &::windows_core::HSTRING, authenticationtoken: &::windows_core::HSTRING, wnschannelid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddDeviceAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(useraccountid), ::core::mem::transmute_copy(authenticationtoken), ::core::mem::transmute_copy(wnschannelid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceAsync(&self, useraccountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveDeviceAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(useraccountid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateWnsChannelAsync(&self, useraccountid: &::windows_core::HSTRING, channeluri: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateWnsChannelAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(useraccountid), ::core::mem::transmute_copy(channeluri), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessionsAsync<P0>(&self, useraccountidlist: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSessionsAsync)(::windows_core::Interface::as_raw(this), useraccountidlist.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessionsAndUnregisteredAccountsAsync<P0>(&self, useraccountidlist: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSessionsAndUnregisteredAccountsAsync)(::windows_core::Interface::as_raw(this), useraccountidlist.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ApproveSessionUsingAuthSessionInfoAsync<P0>(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>
    where
        P0: ::windows_core::IntoParam<MicrosoftAccountMultiFactorSessionInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApproveSessionUsingAuthSessionInfoAsync)(::windows_core::Interface::as_raw(this), sessionauthentictionstatus, authenticationsessioninfo.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ApproveSessionAsync(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: &::windows_core::HSTRING, sessionid: &::windows_core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApproveSessionAsync)(::windows_core::Interface::as_raw(this), sessionauthentictionstatus, ::core::mem::transmute_copy(useraccountid), ::core::mem::transmute_copy(sessionid), sessionauthenticationtype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DenySessionUsingAuthSessionInfoAsync<P0>(&self, authenticationsessioninfo: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>
    where
        P0: ::windows_core::IntoParam<MicrosoftAccountMultiFactorSessionInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DenySessionUsingAuthSessionInfoAsync)(::windows_core::Interface::as_raw(this), authenticationsessioninfo.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DenySessionAsync(&self, useraccountid: &::windows_core::HSTRING, sessionid: &::windows_core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DenySessionAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(useraccountid), ::core::mem::transmute_copy(sessionid), sessionauthenticationtype, &mut result__).from_abi(result__)
        }
    }
    pub fn Current() -> ::windows_core::Result<MicrosoftAccountMultiFactorAuthenticationManager> {
        Self::IMicrosoftAccountMultiFactorAuthenticatorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMicrosoftAccountMultiFactorAuthenticatorStatics<R, F: FnOnce(&IMicrosoftAccountMultiFactorAuthenticatorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MicrosoftAccountMultiFactorAuthenticationManager, IMicrosoftAccountMultiFactorAuthenticatorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MicrosoftAccountMultiFactorAuthenticationManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationManager;{0fd340a5-f574-4320-a08e-0a19a82322aa})");
}
unsafe impl ::windows_core::Interface for MicrosoftAccountMultiFactorAuthenticationManager {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticationManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MicrosoftAccountMultiFactorAuthenticationManager {
    const IID: ::windows_core::GUID = <IMicrosoftAccountMultiFactorAuthenticationManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MicrosoftAccountMultiFactorAuthenticationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationManager";
}
::windows_core::imp::interface_hierarchy!(MicrosoftAccountMultiFactorAuthenticationManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorAuthenticationManager {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorAuthenticationManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MicrosoftAccountMultiFactorGetSessionsResult(::windows_core::IUnknown);
impl MicrosoftAccountMultiFactorGetSessionsResult {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sessions(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Sessions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceResponse(&self) -> ::windows_core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceResponse)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MicrosoftAccountMultiFactorGetSessionsResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorGetSessionsResult;{4e23a9a0-e9fa-497a-95de-6d5747bf974c})");
}
unsafe impl ::windows_core::Interface for MicrosoftAccountMultiFactorGetSessionsResult {
    type Vtable = IMicrosoftAccountMultiFactorGetSessionsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MicrosoftAccountMultiFactorGetSessionsResult {
    const IID: ::windows_core::GUID = <IMicrosoftAccountMultiFactorGetSessionsResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MicrosoftAccountMultiFactorGetSessionsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorGetSessionsResult";
}
::windows_core::imp::interface_hierarchy!(MicrosoftAccountMultiFactorGetSessionsResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorGetSessionsResult {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorGetSessionsResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MicrosoftAccountMultiFactorOneTimeCodedInfo(::windows_core::IUnknown);
impl MicrosoftAccountMultiFactorOneTimeCodedInfo {
    pub fn Code(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Code)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TimeInterval(&self) -> ::windows_core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToLive(&self) -> ::windows_core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeToLive)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceResponse(&self) -> ::windows_core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceResponse)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorOneTimeCodedInfo;{82ba264b-d87c-4668-a976-40cfae547d08})");
}
unsafe impl ::windows_core::Interface for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    type Vtable = IMicrosoftAccountMultiFactorOneTimeCodedInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const IID: ::windows_core::GUID = <IMicrosoftAccountMultiFactorOneTimeCodedInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorOneTimeCodedInfo";
}
::windows_core::imp::interface_hierarchy!(MicrosoftAccountMultiFactorOneTimeCodedInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MicrosoftAccountMultiFactorSessionInfo(::windows_core::IUnknown);
impl MicrosoftAccountMultiFactorSessionInfo {
    pub fn UserAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SessionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplaySessionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplaySessionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ApprovalStatus(&self) -> ::windows_core::Result<MicrosoftAccountMultiFactorSessionApprovalStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApprovalStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AuthenticationType(&self) -> ::windows_core::Result<MicrosoftAccountMultiFactorAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestTime(&self) -> ::windows_core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows_core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MicrosoftAccountMultiFactorSessionInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionInfo;{5f7eabb4-a278-4635-b765-b494eb260af4})");
}
unsafe impl ::windows_core::Interface for MicrosoftAccountMultiFactorSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorSessionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MicrosoftAccountMultiFactorSessionInfo {
    const IID: ::windows_core::GUID = <IMicrosoftAccountMultiFactorSessionInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MicrosoftAccountMultiFactorSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionInfo";
}
::windows_core::imp::interface_hierarchy!(MicrosoftAccountMultiFactorSessionInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorSessionInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorSessionInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(::windows_core::IUnknown);
impl MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sessions(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Sessions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UnregisteredAccounts(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnregisteredAccounts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceResponse(&self) -> ::windows_core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceResponse)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo;{aa7ec5fb-da3f-4088-a20d-5618afadb2e5})");
}
unsafe impl ::windows_core::Interface for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const IID: ::windows_core::GUID = <IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo";
}
::windows_core::imp::interface_hierarchy!(MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MicrosoftAccountMultiFactorAuthenticationType(pub i32);
impl MicrosoftAccountMultiFactorAuthenticationType {
    pub const User: Self = Self(0i32);
    pub const Device: Self = Self(1i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorAuthenticationType {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MicrosoftAccountMultiFactorAuthenticationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MicrosoftAccountMultiFactorAuthenticationType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorAuthenticationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorAuthenticationType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MicrosoftAccountMultiFactorAuthenticationType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MicrosoftAccountMultiFactorServiceResponse(pub i32);
impl MicrosoftAccountMultiFactorServiceResponse {
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const NoNetworkConnection: Self = Self(2i32);
    pub const ServiceUnavailable: Self = Self(3i32);
    pub const TotpSetupDenied: Self = Self(4i32);
    pub const NgcNotSetup: Self = Self(5i32);
    pub const SessionAlreadyDenied: Self = Self(6i32);
    pub const SessionAlreadyApproved: Self = Self(7i32);
    pub const SessionExpired: Self = Self(8i32);
    pub const NgcNonceExpired: Self = Self(9i32);
    pub const InvalidSessionId: Self = Self(10i32);
    pub const InvalidSessionType: Self = Self(11i32);
    pub const InvalidOperation: Self = Self(12i32);
    pub const InvalidStateTransition: Self = Self(13i32);
    pub const DeviceNotFound: Self = Self(14i32);
    pub const FlowDisabled: Self = Self(15i32);
    pub const SessionNotApproved: Self = Self(16i32);
    pub const OperationCanceledByUser: Self = Self(17i32);
    pub const NgcDisabledByServer: Self = Self(18i32);
    pub const NgcKeyNotFoundOnServer: Self = Self(19i32);
    pub const UIRequired: Self = Self(20i32);
    pub const DeviceIdChanged: Self = Self(21i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorServiceResponse {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorServiceResponse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MicrosoftAccountMultiFactorServiceResponse {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MicrosoftAccountMultiFactorServiceResponse {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorServiceResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorServiceResponse").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MicrosoftAccountMultiFactorServiceResponse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorServiceResponse;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MicrosoftAccountMultiFactorSessionApprovalStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionApprovalStatus {
    pub const Pending: Self = Self(0i32);
    pub const Approved: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorSessionApprovalStatus {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MicrosoftAccountMultiFactorSessionApprovalStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorSessionApprovalStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MicrosoftAccountMultiFactorSessionApprovalStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionApprovalStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MicrosoftAccountMultiFactorSessionAuthenticationStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    pub const Authenticated: Self = Self(0i32);
    pub const Unauthenticated: Self = Self(1i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorSessionAuthenticationStatus {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorSessionAuthenticationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionAuthenticationStatus;i4)");
}
