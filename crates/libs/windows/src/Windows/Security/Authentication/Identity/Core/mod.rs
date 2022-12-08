#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMicrosoftAccountMultiFactorAuthenticationManager {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticationManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorAuthenticationManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fd340a5_f574_4320_a08e_0a19a82322aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetOneTimePassCodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: *mut ::core::ffi::c_void, codelength: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetOneTimePassCodeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AddDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: *mut ::core::ffi::c_void, authenticationtoken: *mut ::core::ffi::c_void, wnschannelid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateWnsChannelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: *mut ::core::ffi::c_void, channeluri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateWnsChannelAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountidlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessionsAndUnregisteredAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountidlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionsAndUnregisteredAccountsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ApproveSessionUsingAuthSessionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApproveSessionUsingAuthSessionInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ApproveSessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: *mut ::core::ffi::c_void, sessionid: *mut ::core::ffi::c_void, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApproveSessionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DenySessionUsingAuthSessionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authenticationsessioninfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DenySessionUsingAuthSessionInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DenySessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: *mut ::core::ffi::c_void, sessionid: *mut ::core::ffi::c_void, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DenySessionAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMicrosoftAccountMultiFactorAuthenticatorStatics {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticatorStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorAuthenticatorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd964c2e6_f446_4c71_8b79_6ea4024aa9b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMicrosoftAccountMultiFactorGetSessionsResult {
    type Vtable = IMicrosoftAccountMultiFactorGetSessionsResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorGetSessionsResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e23a9a0_e9fa_497a_95de_6d5747bf974c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Sessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sessions: usize,
    pub ServiceResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    type Vtable = IMicrosoftAccountMultiFactorOneTimeCodedInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82ba264b_d87c_4668_a976_40cfae547d08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Code: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TimeInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeInterval: usize,
    #[cfg(feature = "Foundation")]
    pub TimeToLive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToLive: usize,
    pub ServiceResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorSessionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMicrosoftAccountMultiFactorSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorSessionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorSessionInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f7eabb4_a278_4635_b765_b494eb260af4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorSessionInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UserAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplaySessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ApprovalStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorSessionApprovalStatus) -> ::windows::core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa7ec5fb_da3f_4088_a20d_5618afadb2e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Sessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sessions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UnregisteredAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UnregisteredAccounts: usize,
    pub ServiceResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorAuthenticationManager(::windows::core::IUnknown);
impl MicrosoftAccountMultiFactorAuthenticationManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetOneTimePassCodeAsync(&self, useraccountid: &::windows::core::HSTRING, codelength: u32) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOneTimePassCodeAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(useraccountid), codelength, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddDeviceAsync(&self, useraccountid: &::windows::core::HSTRING, authenticationtoken: &::windows::core::HSTRING, wnschannelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddDeviceAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(useraccountid), ::core::mem::transmute_copy(authenticationtoken), ::core::mem::transmute_copy(wnschannelid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceAsync(&self, useraccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveDeviceAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(useraccountid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateWnsChannelAsync(&self, useraccountid: &::windows::core::HSTRING, channeluri: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateWnsChannelAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(useraccountid), ::core::mem::transmute_copy(channeluri), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessionsAsync<P0, E0>(&self, useraccountidlist: P0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSessionsAsync)(::windows::core::Vtable::as_raw(this), useraccountidlist.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessionsAndUnregisteredAccountsAsync<P0, E0>(&self, useraccountidlist: P0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSessionsAndUnregisteredAccountsAsync)(::windows::core::Vtable::as_raw(this), useraccountidlist.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApproveSessionUsingAuthSessionInfoAsync(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: &MicrosoftAccountMultiFactorSessionInfo) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ApproveSessionUsingAuthSessionInfoAsync)(::windows::core::Vtable::as_raw(this), sessionauthentictionstatus, ::core::mem::transmute_copy(authenticationsessioninfo), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApproveSessionAsync(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ApproveSessionAsync)(::windows::core::Vtable::as_raw(this), sessionauthentictionstatus, ::core::mem::transmute_copy(useraccountid), ::core::mem::transmute_copy(sessionid), sessionauthenticationtype, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DenySessionUsingAuthSessionInfoAsync(&self, authenticationsessioninfo: &MicrosoftAccountMultiFactorSessionInfo) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DenySessionUsingAuthSessionInfoAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(authenticationsessioninfo), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DenySessionAsync(&self, useraccountid: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DenySessionAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(useraccountid), ::core::mem::transmute_copy(sessionid), sessionauthenticationtype, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Current() -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationManager> {
        Self::IMicrosoftAccountMultiFactorAuthenticatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Current)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMicrosoftAccountMultiFactorAuthenticatorStatics<R, F: FnOnce(&IMicrosoftAccountMultiFactorAuthenticatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MicrosoftAccountMultiFactorAuthenticationManager, IMicrosoftAccountMultiFactorAuthenticatorStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorAuthenticationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorAuthenticationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorAuthenticationManager {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorAuthenticationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorAuthenticationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorAuthenticationManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationManager;{0fd340a5-f574-4320-a08e-0a19a82322aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MicrosoftAccountMultiFactorAuthenticationManager {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticationManager_Vtbl;
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorAuthenticationManager {
    const IID: ::windows::core::GUID = <IMicrosoftAccountMultiFactorAuthenticationManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorAuthenticationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationManager";
}
::windows::core::interface_hierarchy!(MicrosoftAccountMultiFactorAuthenticationManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorAuthenticationManager {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorAuthenticationManager {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorGetSessionsResult(::windows::core::IUnknown);
impl MicrosoftAccountMultiFactorGetSessionsResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sessions(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Sessions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceResponse)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorGetSessionsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorGetSessionsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorGetSessionsResult {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorGetSessionsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorGetSessionsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorGetSessionsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorGetSessionsResult;{4e23a9a0-e9fa-497a-95de-6d5747bf974c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MicrosoftAccountMultiFactorGetSessionsResult {
    type Vtable = IMicrosoftAccountMultiFactorGetSessionsResult_Vtbl;
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorGetSessionsResult {
    const IID: ::windows::core::GUID = <IMicrosoftAccountMultiFactorGetSessionsResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorGetSessionsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorGetSessionsResult";
}
::windows::core::interface_hierarchy!(MicrosoftAccountMultiFactorGetSessionsResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorGetSessionsResult {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorGetSessionsResult {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorOneTimeCodedInfo(::windows::core::IUnknown);
impl MicrosoftAccountMultiFactorOneTimeCodedInfo {
    pub fn Code(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeInterval(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeInterval)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToLive(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeToLive)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceResponse)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorOneTimeCodedInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorOneTimeCodedInfo;{82ba264b-d87c-4668-a976-40cfae547d08})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    type Vtable = IMicrosoftAccountMultiFactorOneTimeCodedInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const IID: ::windows::core::GUID = <IMicrosoftAccountMultiFactorOneTimeCodedInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorOneTimeCodedInfo";
}
::windows::core::interface_hierarchy!(MicrosoftAccountMultiFactorOneTimeCodedInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionInfo(::windows::core::IUnknown);
impl MicrosoftAccountMultiFactorSessionInfo {
    pub fn UserAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserAccountId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplaySessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplaySessionId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ApprovalStatus(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorSessionApprovalStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ApprovalStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AuthenticationType(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AuthenticationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorSessionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorSessionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorSessionInfo {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorSessionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorSessionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorSessionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionInfo;{5f7eabb4-a278-4635-b765-b494eb260af4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MicrosoftAccountMultiFactorSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorSessionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorSessionInfo {
    const IID: ::windows::core::GUID = <IMicrosoftAccountMultiFactorSessionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionInfo";
}
::windows::core::interface_hierarchy!(MicrosoftAccountMultiFactorSessionInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorSessionInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorSessionInfo {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(::windows::core::IUnknown);
impl MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sessions(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Sessions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UnregisteredAccounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnregisteredAccounts)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceResponse)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo;{aa7ec5fb-da3f-4088-a20d-5618afadb2e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const IID: ::windows::core::GUID = <IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo";
}
::windows::core::interface_hierarchy!(MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
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
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorAuthenticationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorAuthenticationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorAuthenticationType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorAuthenticationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
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
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorServiceResponse {
    type Abi = Self;
}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorServiceResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorServiceResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorServiceResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorServiceResponse;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
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
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorSessionApprovalStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorSessionApprovalStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorSessionApprovalStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionApprovalStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
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
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorSessionAuthenticationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionAuthenticationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
