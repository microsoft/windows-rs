#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct FindAllAccountsResult(::windows::core::IUnknown);
impl FindAllAccountsResult {
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn Accounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Accounts)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<FindAllWebAccountsStatus> {
        let this = self;
        unsafe {
            let mut result__: FindAllWebAccountsStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FindAllWebAccountsStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn ProviderError(&self) -> ::windows::core::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebProviderError>(result__)
        }
    }
}
impl ::core::clone::Clone for FindAllAccountsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FindAllAccountsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FindAllAccountsResult {}
impl ::core::fmt::Debug for FindAllAccountsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindAllAccountsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FindAllAccountsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.FindAllAccountsResult;{a5812b5d-b72e-420c-86ab-aac0d7b7261f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FindAllAccountsResult {
    type Vtable = IFindAllAccountsResult_Vtbl;
    const IID: ::windows::core::GUID = <IFindAllAccountsResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FindAllAccountsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.FindAllAccountsResult";
}
impl ::core::convert::From<FindAllAccountsResult> for ::windows::core::IUnknown {
    fn from(value: FindAllAccountsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindAllAccountsResult> for ::windows::core::IUnknown {
    fn from(value: &FindAllAccountsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FindAllAccountsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FindAllAccountsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FindAllAccountsResult> for ::windows::core::IInspectable {
    fn from(value: FindAllAccountsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindAllAccountsResult> for ::windows::core::IInspectable {
    fn from(value: &FindAllAccountsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FindAllAccountsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FindAllAccountsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FindAllAccountsResult {}
unsafe impl ::core::marker::Sync for FindAllAccountsResult {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FindAllWebAccountsStatus(pub i32);
impl FindAllWebAccountsStatus {
    pub const Success: Self = Self(0i32);
    pub const NotAllowedByProvider: Self = Self(1i32);
    pub const NotSupportedByProvider: Self = Self(2i32);
    pub const ProviderError: Self = Self(3i32);
}
impl ::core::marker::Copy for FindAllWebAccountsStatus {}
impl ::core::clone::Clone for FindAllWebAccountsStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FindAllWebAccountsStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FindAllWebAccountsStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for FindAllWebAccountsStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindAllWebAccountsStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FindAllWebAccountsStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.FindAllWebAccountsStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFindAllAccountsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFindAllAccountsResult {
    type Vtable = IFindAllAccountsResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5812b5d_b72e_420c_86ab_aac0d7b7261f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindAllAccountsResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub Accounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    Accounts: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FindAllWebAccountsStatus) -> ::windows::core::HRESULT,
    pub ProviderError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountEventArgs {
    type Vtable = IWebAccountEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fb7037d_424e_44ec_977c_ef2415462a5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub Account: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Account: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountMonitor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountMonitor {
    type Vtable = IWebAccountMonitor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7445f5fd_aa9d_4619_8d5d_c138a4ede3e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountMonitor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub DefaultSignInAccountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultSignInAccountChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefaultSignInAccountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefaultSignInAccountChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountMonitor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountMonitor2 {
    type Vtable = IWebAccountMonitor2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7adc1f8_24b8_4f01_9ae5_24545e71233a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountMonitor2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AccountPictureUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountPictureUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountPictureUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountPictureUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationCoreManagerStatics {
    type Vtable = IWebAuthenticationCoreManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aca7c92_a581_4479_9c10_752eff44fd34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetTokenSilentlyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTokenSilentlyAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub GetTokenSilentlyWithWebAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    GetTokenSilentlyWithWebAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestTokenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestTokenAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub RequestTokenWithWebAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    RequestTokenWithWebAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAccountProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAccountProviderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAccountProviderWithAuthorityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAccountProviderWithAuthorityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationCoreManagerStatics2 {
    type Vtable = IWebAuthenticationCoreManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf584184a_8b57_4820_b6a4_70a5b6fcf44a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    pub FindAccountProviderWithAuthorityForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "System")))]
    FindAccountProviderWithAuthorityForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationCoreManagerStatics3 {
    type Vtable = IWebAuthenticationCoreManagerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2404eeb2_8924_4d93_ab3a_99688b419d56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub CreateWebAccountMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccounts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    CreateWebAccountMonitor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationCoreManagerStatics4 {
    type Vtable = IWebAuthenticationCoreManagerStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54e633fe_96e0_41e8_9832_1298897c2aaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAllAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAllAccountsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAllAccountsWithClientIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAllAccountsWithClientIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindSystemAccountProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindSystemAccountProviderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindSystemAccountProviderWithAuthorityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindSystemAccountProviderWithAuthorityAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    pub FindSystemAccountProviderWithAuthorityForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authority: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "System")))]
    FindSystemAccountProviderWithAuthorityForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderError(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebProviderError {
    type Vtable = IWebProviderError_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb191bb1_50c5_4809_8dca_09c99410245c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderError_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ErrorMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderErrorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebProviderErrorFactory {
    type Vtable = IWebProviderErrorFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3c40a2d_89ef_4e37_847f_a8b9d5a32910);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderErrorFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorcode: u32, errormessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenRequest {
    type Vtable = IWebTokenRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb77b4d68_adcb_4673_b364_0cf7b35caf97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccountProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccountProvider: usize,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ClientId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PromptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebTokenRequestPromptType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenRequest2 {
    type Vtable = IWebTokenRequest2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd700c079_30c8_4397_9654_961c3be8b855);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequest2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenRequest3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenRequest3 {
    type Vtable = IWebTokenRequest3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a755b51_3bb1_41a5_a63d_90bc32c7db9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequest3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenRequestFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenRequestFactory {
    type Vtable = IWebTokenRequestFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cf2141c_0ff0_4c67_b84f_99ddbe4a72c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequestFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Create: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithPromptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, prompttype: WebTokenRequestPromptType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithPromptType: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithProvider: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, scope: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithScope: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenRequestResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenRequestResult {
    type Vtable = IWebTokenRequestResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc12a8305_d1f8_4483_8d54_38fe292784ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequestResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ResponseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResponseData: usize,
    pub ResponseStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebTokenRequestStatus) -> ::windows::core::HRESULT,
    pub ResponseError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InvalidateCacheAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvalidateCacheAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenResponse(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenResponse {
    type Vtable = IWebTokenResponse_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67a7c5ca_83f6_44c6_a3b1_0eb69e41fa8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenResponse_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Token: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProviderError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenResponseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenResponseFactory {
    type Vtable = IWebTokenResponseFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab6bf7f8_5450_4ef6_97f7_052b0431c0f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenResponseFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateWithToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithTokenAndAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithTokenAndAccount: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithTokenAccountAndError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccount: ::windows::core::RawPtr, error: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithTokenAccountAndError: usize,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebAccountEventArgs(::windows::core::IUnknown);
impl WebAccountEventArgs {
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn Account(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Account)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Credentials::WebAccount>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountEventArgs {}
impl ::core::fmt::Debug for WebAccountEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebAccountEventArgs;{6fb7037d-424e-44ec-977c-ef2415462a5a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountEventArgs {
    type Vtable = IWebAccountEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountEventArgs {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAccountEventArgs";
}
impl ::core::convert::From<WebAccountEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebAccountEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebAccountEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebAccountEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebAccountEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebAccountEventArgs {}
unsafe impl ::core::marker::Sync for WebAccountEventArgs {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebAccountMonitor(::windows::core::IUnknown);
impl WebAccountMonitor {
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Updated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Removed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRemoved)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DefaultSignInAccountChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultSignInAccountChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDefaultSignInAccountChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDefaultSignInAccountChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AccountPictureUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWebAccountMonitor2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountPictureUpdated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountPictureUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountMonitor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAccountPictureUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for WebAccountMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountMonitor {}
impl ::core::fmt::Debug for WebAccountMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountMonitor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebAccountMonitor;{7445f5fd-aa9d-4619-8d5d-c138a4ede3e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountMonitor {
    type Vtable = IWebAccountMonitor_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountMonitor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountMonitor {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAccountMonitor";
}
impl ::core::convert::From<WebAccountMonitor> for ::windows::core::IUnknown {
    fn from(value: WebAccountMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountMonitor> for ::windows::core::IUnknown {
    fn from(value: &WebAccountMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountMonitor> for ::windows::core::IInspectable {
    fn from(value: WebAccountMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountMonitor> for ::windows::core::IInspectable {
    fn from(value: &WebAccountMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebAccountMonitor {}
unsafe impl ::core::marker::Sync for WebAccountMonitor {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
pub struct WebAuthenticationCoreManager {}
impl WebAuthenticationCoreManager {
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTokenSilentlyAsync<'a, Param0: ::windows::core::IntoParam<'a, WebTokenRequest>>(request: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTokenSilentlyAsync)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn GetTokenSilentlyWithWebAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, WebTokenRequest>, Param1: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>>(request: Param0, webaccount: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTokenSilentlyWithWebAccountAsync)(::core::mem::transmute_copy(this), request.into_param().abi(), webaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestTokenAsync<'a, Param0: ::windows::core::IntoParam<'a, WebTokenRequest>>(request: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestTokenAsync)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn RequestTokenWithWebAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, WebTokenRequest>, Param1: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>>(request: Param0, webaccount: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestTokenWithWebAccountAsync)(::core::mem::transmute_copy(this), request.into_param().abi(), webaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(provider: Param0, webaccountid: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAccountAsync)(::core::mem::transmute_copy(this), provider.into_param().abi(), webaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindAccountProviderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(webaccountproviderid: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAccountProviderAsync)(::core::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindAccountProviderWithAuthorityAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(webaccountproviderid: Param0, authority: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAccountProviderWithAuthorityAsync)(::core::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), authority.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    pub fn FindAccountProviderWithAuthorityForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::System::User>>(webaccountproviderid: Param0, authority: Param1, user: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAccountProviderWithAuthorityForUserAsync)(::core::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), authority.into_param().abi(), user.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn CreateWebAccountMonitor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<super::super::super::Credentials::WebAccount>>>(webaccounts: Param0) -> ::windows::core::Result<WebAccountMonitor> {
        Self::IWebAuthenticationCoreManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWebAccountMonitor)(::core::mem::transmute_copy(this), webaccounts.into_param().abi(), &mut result__).from_abi::<WebAccountMonitor>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindAllAccountsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>>(provider: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllAccountsAsync)(::core::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindAllAccountsWithClientIdAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(provider: Param0, clientid: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllAccountsWithClientIdAsync)(::core::mem::transmute_copy(this), provider.into_param().abi(), clientid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindSystemAccountProviderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(webaccountproviderid: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindSystemAccountProviderAsync)(::core::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindSystemAccountProviderWithAuthorityAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(webaccountproviderid: Param0, authority: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindSystemAccountProviderWithAuthorityAsync)(::core::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), authority.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    pub fn FindSystemAccountProviderWithAuthorityForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::System::User>>(webaccountproviderid: Param0, authority: Param1, user: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindSystemAccountProviderWithAuthorityForUserAsync)(::core::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), authority.into_param().abi(), user.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics2<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics3<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics4<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for WebAuthenticationCoreManager {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager";
}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebProviderError(::windows::core::IUnknown);
impl WebProviderError {
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn ErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorMessage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn Create<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(errorcode: u32, errormessage: Param1) -> ::windows::core::Result<WebProviderError> {
        Self::IWebProviderErrorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), errorcode, errormessage.into_param().abi(), &mut result__).from_abi::<WebProviderError>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebProviderErrorFactory<R, F: FnOnce(&IWebProviderErrorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebProviderError, IWebProviderErrorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebProviderError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebProviderError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebProviderError {}
impl ::core::fmt::Debug for WebProviderError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebProviderError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebProviderError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebProviderError;{db191bb1-50c5-4809-8dca-09c99410245c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebProviderError {
    type Vtable = IWebProviderError_Vtbl;
    const IID: ::windows::core::GUID = <IWebProviderError as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebProviderError {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebProviderError";
}
impl ::core::convert::From<WebProviderError> for ::windows::core::IUnknown {
    fn from(value: WebProviderError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebProviderError> for ::windows::core::IUnknown {
    fn from(value: &WebProviderError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebProviderError> for ::windows::core::IInspectable {
    fn from(value: WebProviderError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebProviderError> for ::windows::core::IInspectable {
    fn from(value: &WebProviderError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebProviderError {}
unsafe impl ::core::marker::Sync for WebProviderError {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebTokenRequest(::windows::core::IUnknown);
impl WebTokenRequest {
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccountProvider(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebAccountProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Credentials::WebAccountProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn Scope(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Scope)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn ClientId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClientId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn PromptType(&self) -> ::windows::core::Result<WebTokenRequestPromptType> {
        let this = self;
        unsafe {
            let mut result__: WebTokenRequestPromptType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PromptType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebTokenRequestPromptType>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IWebTokenRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IWebTokenRequest3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CorrelationId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn SetCorrelationId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebTokenRequest3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCorrelationId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(provider: Param0, scope: Param1, clientid: Param2) -> ::windows::core::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), provider.into_param().abi(), scope.into_param().abi(), clientid.into_param().abi(), &mut result__).from_abi::<WebTokenRequest>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithPromptType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(provider: Param0, scope: Param1, clientid: Param2, prompttype: WebTokenRequestPromptType) -> ::windows::core::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithPromptType)(::core::mem::transmute_copy(this), provider.into_param().abi(), scope.into_param().abi(), clientid.into_param().abi(), prompttype, &mut result__).from_abi::<WebTokenRequest>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>>(provider: Param0) -> ::windows::core::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithProvider)(::core::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<WebTokenRequest>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithScope<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(provider: Param0, scope: Param1) -> ::windows::core::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithScope)(::core::mem::transmute_copy(this), provider.into_param().abi(), scope.into_param().abi(), &mut result__).from_abi::<WebTokenRequest>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebTokenRequestFactory<R, F: FnOnce(&IWebTokenRequestFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebTokenRequest, IWebTokenRequestFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebTokenRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebTokenRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebTokenRequest {}
impl ::core::fmt::Debug for WebTokenRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebTokenRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebTokenRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebTokenRequest;{b77b4d68-adcb-4673-b364-0cf7b35caf97})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebTokenRequest {
    type Vtable = IWebTokenRequest_Vtbl;
    const IID: ::windows::core::GUID = <IWebTokenRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebTokenRequest {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenRequest";
}
impl ::core::convert::From<WebTokenRequest> for ::windows::core::IUnknown {
    fn from(value: WebTokenRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebTokenRequest> for ::windows::core::IUnknown {
    fn from(value: &WebTokenRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebTokenRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebTokenRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebTokenRequest> for ::windows::core::IInspectable {
    fn from(value: WebTokenRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebTokenRequest> for ::windows::core::IInspectable {
    fn from(value: &WebTokenRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebTokenRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebTokenRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebTokenRequest {}
unsafe impl ::core::marker::Sync for WebTokenRequest {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WebTokenRequestPromptType(pub i32);
impl WebTokenRequestPromptType {
    pub const Default: Self = Self(0i32);
    pub const ForceAuthentication: Self = Self(1i32);
}
impl ::core::marker::Copy for WebTokenRequestPromptType {}
impl ::core::clone::Clone for WebTokenRequestPromptType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebTokenRequestPromptType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WebTokenRequestPromptType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebTokenRequestPromptType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebTokenRequestPromptType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebTokenRequestPromptType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.WebTokenRequestPromptType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebTokenRequestResult(::windows::core::IUnknown);
impl WebTokenRequestResult {
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResponseData(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<WebTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResponseData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<WebTokenResponse>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn ResponseStatus(&self) -> ::windows::core::Result<WebTokenRequestStatus> {
        let this = self;
        unsafe {
            let mut result__: WebTokenRequestStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResponseStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebTokenRequestStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn ResponseError(&self) -> ::windows::core::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResponseError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebProviderError>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InvalidateCacheAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InvalidateCacheAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for WebTokenRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebTokenRequestResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebTokenRequestResult {}
impl ::core::fmt::Debug for WebTokenRequestResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebTokenRequestResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebTokenRequestResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebTokenRequestResult;{c12a8305-d1f8-4483-8d54-38fe292784ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebTokenRequestResult {
    type Vtable = IWebTokenRequestResult_Vtbl;
    const IID: ::windows::core::GUID = <IWebTokenRequestResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebTokenRequestResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenRequestResult";
}
impl ::core::convert::From<WebTokenRequestResult> for ::windows::core::IUnknown {
    fn from(value: WebTokenRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebTokenRequestResult> for ::windows::core::IUnknown {
    fn from(value: &WebTokenRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebTokenRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebTokenRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebTokenRequestResult> for ::windows::core::IInspectable {
    fn from(value: WebTokenRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebTokenRequestResult> for ::windows::core::IInspectable {
    fn from(value: &WebTokenRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebTokenRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebTokenRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebTokenRequestResult {}
unsafe impl ::core::marker::Sync for WebTokenRequestResult {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WebTokenRequestStatus(pub i32);
impl WebTokenRequestStatus {
    pub const Success: Self = Self(0i32);
    pub const UserCancel: Self = Self(1i32);
    pub const AccountSwitch: Self = Self(2i32);
    pub const UserInteractionRequired: Self = Self(3i32);
    pub const AccountProviderNotAvailable: Self = Self(4i32);
    pub const ProviderError: Self = Self(5i32);
}
impl ::core::marker::Copy for WebTokenRequestStatus {}
impl ::core::clone::Clone for WebTokenRequestStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebTokenRequestStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WebTokenRequestStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebTokenRequestStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebTokenRequestStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebTokenRequestStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.WebTokenRequestStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebTokenResponse(::windows::core::IUnknown);
impl WebTokenResponse {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebTokenResponse, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Token)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn ProviderError(&self) -> ::windows::core::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebProviderError>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebAccount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Credentials::WebAccount>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    pub fn CreateWithToken<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(token: Param0) -> ::windows::core::Result<WebTokenResponse> {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithToken)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<WebTokenResponse>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithTokenAndAccount<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>>(token: Param0, webaccount: Param1) -> ::windows::core::Result<WebTokenResponse> {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithTokenAndAccount)(::core::mem::transmute_copy(this), token.into_param().abi(), webaccount.into_param().abi(), &mut result__).from_abi::<WebTokenResponse>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithTokenAccountAndError<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>, Param2: ::windows::core::IntoParam<'a, WebProviderError>>(token: Param0, webaccount: Param1, error: Param2) -> ::windows::core::Result<WebTokenResponse> {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithTokenAccountAndError)(::core::mem::transmute_copy(this), token.into_param().abi(), webaccount.into_param().abi(), error.into_param().abi(), &mut result__).from_abi::<WebTokenResponse>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebTokenResponseFactory<R, F: FnOnce(&IWebTokenResponseFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebTokenResponse, IWebTokenResponseFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebTokenResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebTokenResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebTokenResponse {}
impl ::core::fmt::Debug for WebTokenResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebTokenResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebTokenResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebTokenResponse;{67a7c5ca-83f6-44c6-a3b1-0eb69e41fa8a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebTokenResponse {
    type Vtable = IWebTokenResponse_Vtbl;
    const IID: ::windows::core::GUID = <IWebTokenResponse as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenResponse";
}
impl ::core::convert::From<WebTokenResponse> for ::windows::core::IUnknown {
    fn from(value: WebTokenResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebTokenResponse> for ::windows::core::IUnknown {
    fn from(value: &WebTokenResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebTokenResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebTokenResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebTokenResponse> for ::windows::core::IInspectable {
    fn from(value: WebTokenResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebTokenResponse> for ::windows::core::IInspectable {
    fn from(value: &WebTokenResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebTokenResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebTokenResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebTokenResponse {}
unsafe impl ::core::marker::Sync for WebTokenResponse {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
