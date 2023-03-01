#[doc(hidden)]
#[repr(transparent)]
pub struct IFindAllAccountsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFindAllAccountsResult {
    type Vtable = IFindAllAccountsResult_Vtbl;
}
impl ::core::clone::Clone for IFindAllAccountsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFindAllAccountsResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5812b5d_b72e_420c_86ab_aac0d7b7261f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindAllAccountsResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub Accounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    Accounts: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FindAllWebAccountsStatus) -> ::windows::core::HRESULT,
    pub ProviderError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountEventArgs {
    type Vtable = IWebAccountEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWebAccountEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAccountEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fb7037d_424e_44ec_977c_ef2415462a5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub Account: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Account: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountMonitor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountMonitor {
    type Vtable = IWebAccountMonitor_Vtbl;
}
impl ::core::clone::Clone for IWebAccountMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAccountMonitor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7445f5fd_aa9d_4619_8d5d_c138a4ede3e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountMonitor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub DefaultSignInAccountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
}
impl ::core::clone::Clone for IWebAccountMonitor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAccountMonitor2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7adc1f8_24b8_4f01_9ae5_24545e71233a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountMonitor2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AccountPictureUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
}
impl ::core::clone::Clone for IWebAuthenticationCoreManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAuthenticationCoreManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aca7c92_a581_4479_9c10_752eff44fd34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetTokenSilentlyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTokenSilentlyAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub GetTokenSilentlyWithWebAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    GetTokenSilentlyWithWebAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestTokenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestTokenAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub RequestTokenWithWebAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    RequestTokenWithWebAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAccountProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAccountProviderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAccountProviderWithAuthorityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, authority: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAccountProviderWithAuthorityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationCoreManagerStatics2 {
    type Vtable = IWebAuthenticationCoreManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IWebAuthenticationCoreManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAuthenticationCoreManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf584184a_8b57_4820_b6a4_70a5b6fcf44a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    pub FindAccountProviderWithAuthorityForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, authority: ::std::mem::MaybeUninit<::windows::core::HSTRING>, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "System")))]
    FindAccountProviderWithAuthorityForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationCoreManagerStatics3 {
    type Vtable = IWebAuthenticationCoreManagerStatics3_Vtbl;
}
impl ::core::clone::Clone for IWebAuthenticationCoreManagerStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAuthenticationCoreManagerStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2404eeb2_8924_4d93_ab3a_99688b419d56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub CreateWebAccountMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccounts: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    CreateWebAccountMonitor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationCoreManagerStatics4 {
    type Vtable = IWebAuthenticationCoreManagerStatics4_Vtbl;
}
impl ::core::clone::Clone for IWebAuthenticationCoreManagerStatics4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAuthenticationCoreManagerStatics4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54e633fe_96e0_41e8_9832_1298897c2aaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAllAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAllAccountsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAllAccountsWithClientIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, clientid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAllAccountsWithClientIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindSystemAccountProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindSystemAccountProviderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindSystemAccountProviderWithAuthorityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, authority: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindSystemAccountProviderWithAuthorityAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    pub FindSystemAccountProviderWithAuthorityForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountproviderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, authority: ::std::mem::MaybeUninit<::windows::core::HSTRING>, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "System")))]
    FindSystemAccountProviderWithAuthorityForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderError(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebProviderError {
    type Vtable = IWebProviderError_Vtbl;
}
impl ::core::clone::Clone for IWebProviderError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebProviderError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb191bb1_50c5_4809_8dca_09c99410245c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderError_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ErrorMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderErrorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebProviderErrorFactory {
    type Vtable = IWebProviderErrorFactory_Vtbl;
}
impl ::core::clone::Clone for IWebProviderErrorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebProviderErrorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3c40a2d_89ef_4e37_847f_a8b9d5a32910);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderErrorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorcode: u32, errormessage: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenRequest {
    type Vtable = IWebTokenRequest_Vtbl;
}
impl ::core::clone::Clone for IWebTokenRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebTokenRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb77b4d68_adcb_4673_b364_0cf7b35caf97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccountProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccountProvider: usize,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ClientId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PromptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebTokenRequestPromptType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenRequest2 {
    type Vtable = IWebTokenRequest2_Vtbl;
}
impl ::core::clone::Clone for IWebTokenRequest2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebTokenRequest2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd700c079_30c8_4397_9654_961c3be8b855);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequest2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenRequest3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenRequest3 {
    type Vtable = IWebTokenRequest3_Vtbl;
}
impl ::core::clone::Clone for IWebTokenRequest3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebTokenRequest3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a755b51_3bb1_41a5_a63d_90bc32c7db9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequest3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenRequestFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenRequestFactory {
    type Vtable = IWebTokenRequestFactory_Vtbl;
}
impl ::core::clone::Clone for IWebTokenRequestFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebTokenRequestFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cf2141c_0ff0_4c67_b84f_99ddbe4a72c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequestFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, scope: ::std::mem::MaybeUninit<::windows::core::HSTRING>, clientid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Create: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithPromptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, scope: ::std::mem::MaybeUninit<::windows::core::HSTRING>, clientid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, prompttype: WebTokenRequestPromptType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithPromptType: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithProvider: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, scope: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithScope: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenRequestResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenRequestResult {
    type Vtable = IWebTokenRequestResult_Vtbl;
}
impl ::core::clone::Clone for IWebTokenRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebTokenRequestResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc12a8305_d1f8_4483_8d54_38fe292784ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequestResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ResponseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResponseData: usize,
    pub ResponseStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebTokenRequestStatus) -> ::windows::core::HRESULT,
    pub ResponseError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InvalidateCacheAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvalidateCacheAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenResponse(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenResponse {
    type Vtable = IWebTokenResponse_Vtbl;
}
impl ::core::clone::Clone for IWebTokenResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebTokenResponse {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67a7c5ca_83f6_44c6_a3b1_0eb69e41fa8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenResponse_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Token: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProviderError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebTokenResponseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebTokenResponseFactory {
    type Vtable = IWebTokenResponseFactory_Vtbl;
}
impl ::core::clone::Clone for IWebTokenResponseFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebTokenResponseFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab6bf7f8_5450_4ef6_97f7_052b0431c0f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenResponseFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithTokenAndAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows::core::HSTRING>, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithTokenAndAccount: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithTokenAccountAndError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows::core::HSTRING>, webaccount: *mut ::core::ffi::c_void, error: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithTokenAccountAndError: usize,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct FindAllAccountsResult(::windows::core::IUnknown);
impl FindAllAccountsResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn Accounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>();
            (::windows::core::Interface::vtable(this).Accounts)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<FindAllWebAccountsStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<FindAllWebAccountsStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderError(&self) -> ::windows::core::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WebProviderError>();
            (::windows::core::Interface::vtable(this).ProviderError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for FindAllAccountsResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.FindAllAccountsResult;{a5812b5d-b72e-420c-86ab-aac0d7b7261f})");
}
impl ::core::clone::Clone for FindAllAccountsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FindAllAccountsResult {
    type Vtable = IFindAllAccountsResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FindAllAccountsResult {
    const IID: ::windows::core::GUID = <IFindAllAccountsResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FindAllAccountsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.FindAllAccountsResult";
}
::windows::imp::interface_hierarchy!(FindAllAccountsResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for FindAllAccountsResult {}
unsafe impl ::core::marker::Sync for FindAllAccountsResult {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebAccountEventArgs(::windows::core::IUnknown);
impl WebAccountEventArgs {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn Account(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Credentials::WebAccount>();
            (::windows::core::Interface::vtable(this).Account)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for WebAccountEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebAccountEventArgs;{6fb7037d-424e-44ec-977c-ef2415462a5a})");
}
impl ::core::clone::Clone for WebAccountEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebAccountEventArgs {
    type Vtable = IWebAccountEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebAccountEventArgs {
    const IID: ::windows::core::GUID = <IWebAccountEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountEventArgs {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAccountEventArgs";
}
::windows::imp::interface_hierarchy!(WebAccountEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebAccountEventArgs {}
unsafe impl ::core::marker::Sync for WebAccountEventArgs {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebAccountMonitor(::windows::core::IUnknown);
impl WebAccountMonitor {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated(&self, handler: &super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Updated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUpdated)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed(&self, handler: &super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Removed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRemoved)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DefaultSignInAccountChanged(&self, handler: &super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).DefaultSignInAccountChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDefaultSignInAccountChanged(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDefaultSignInAccountChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AccountPictureUpdated(&self, handler: &super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IWebAccountMonitor2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AccountPictureUpdated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountPictureUpdated(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebAccountMonitor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAccountPictureUpdated)(::windows::core::Interface::as_raw(this), token).ok() }
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
impl ::windows::core::RuntimeType for WebAccountMonitor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebAccountMonitor;{7445f5fd-aa9d-4619-8d5d-c138a4ede3e5})");
}
impl ::core::clone::Clone for WebAccountMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebAccountMonitor {
    type Vtable = IWebAccountMonitor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebAccountMonitor {
    const IID: ::windows::core::GUID = <IWebAccountMonitor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountMonitor {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAccountMonitor";
}
::windows::imp::interface_hierarchy!(WebAccountMonitor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebAccountMonitor {}
unsafe impl ::core::marker::Sync for WebAccountMonitor {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
pub struct WebAuthenticationCoreManager;
impl WebAuthenticationCoreManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTokenSilentlyAsync(request: &WebTokenRequest) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>();
            (::windows::core::Interface::vtable(this).GetTokenSilentlyAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn GetTokenSilentlyWithWebAccountAsync(request: &WebTokenRequest, webaccount: &super::super::super::Credentials::WebAccount) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>();
            (::windows::core::Interface::vtable(this).GetTokenSilentlyWithWebAccountAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request), ::core::mem::transmute_copy(webaccount), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestTokenAsync(request: &WebTokenRequest) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>();
            (::windows::core::Interface::vtable(this).RequestTokenAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn RequestTokenWithWebAccountAsync(request: &WebTokenRequest, webaccount: &super::super::super::Credentials::WebAccount) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>();
            (::windows::core::Interface::vtable(this).RequestTokenWithWebAccountAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request), ::core::mem::transmute_copy(webaccount), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindAccountAsync(provider: &super::super::super::Credentials::WebAccountProvider, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>();
            (::windows::core::Interface::vtable(this).FindAccountAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(provider), ::core::mem::transmute_copy(webaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindAccountProviderAsync(webaccountproviderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>();
            (::windows::core::Interface::vtable(this).FindAccountProviderAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountproviderid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindAccountProviderWithAuthorityAsync(webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>();
            (::windows::core::Interface::vtable(this).FindAccountProviderWithAuthorityAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountproviderid), ::core::mem::transmute_copy(authority), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    pub fn FindAccountProviderWithAuthorityForUserAsync(webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING, user: &super::super::super::super::System::User) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>();
            (::windows::core::Interface::vtable(this).FindAccountProviderWithAuthorityForUserAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountproviderid), ::core::mem::transmute_copy(authority), ::core::mem::transmute_copy(user), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn CreateWebAccountMonitor<P0>(webaccounts: P0) -> ::windows::core::Result<WebAccountMonitor>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::super::Foundation::Collections::IIterable<super::super::super::Credentials::WebAccount>>,
    {
        Self::IWebAuthenticationCoreManagerStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebAccountMonitor>();
            (::windows::core::Interface::vtable(this).CreateWebAccountMonitor)(::windows::core::Interface::as_raw(this), webaccounts.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindAllAccountsAsync(provider: &super::super::super::Credentials::WebAccountProvider) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>();
            (::windows::core::Interface::vtable(this).FindAllAccountsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(provider), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindAllAccountsWithClientIdAsync(provider: &super::super::super::Credentials::WebAccountProvider, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>();
            (::windows::core::Interface::vtable(this).FindAllAccountsWithClientIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(provider), ::core::mem::transmute_copy(clientid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindSystemAccountProviderAsync(webaccountproviderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>();
            (::windows::core::Interface::vtable(this).FindSystemAccountProviderAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountproviderid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn FindSystemAccountProviderWithAuthorityAsync(webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>();
            (::windows::core::Interface::vtable(this).FindSystemAccountProviderWithAuthorityAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountproviderid), ::core::mem::transmute_copy(authority), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    pub fn FindSystemAccountProviderWithAuthorityForUserAsync(webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING, user: &super::super::super::super::System::User) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>();
            (::windows::core::Interface::vtable(this).FindSystemAccountProviderWithAuthorityForUserAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountproviderid), ::core::mem::transmute_copy(authority), ::core::mem::transmute_copy(user), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics2<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics3<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics3> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics4<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics4> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for WebAuthenticationCoreManager {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager";
}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebProviderError(::windows::core::IUnknown);
impl WebProviderError {
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(errorcode: u32, errormessage: &::windows::core::HSTRING) -> ::windows::core::Result<WebProviderError> {
        Self::IWebProviderErrorFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebProviderError>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), errorcode, ::core::mem::transmute_copy(errormessage), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebProviderErrorFactory<R, F: FnOnce(&IWebProviderErrorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebProviderError, IWebProviderErrorFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for WebProviderError {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebProviderError;{db191bb1-50c5-4809-8dca-09c99410245c})");
}
impl ::core::clone::Clone for WebProviderError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebProviderError {
    type Vtable = IWebProviderError_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebProviderError {
    const IID: ::windows::core::GUID = <IWebProviderError as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebProviderError {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebProviderError";
}
::windows::imp::interface_hierarchy!(WebProviderError, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebProviderError {}
unsafe impl ::core::marker::Sync for WebProviderError {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebTokenRequest(::windows::core::IUnknown);
impl WebTokenRequest {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccountProvider(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Credentials::WebAccountProvider>();
            (::windows::core::Interface::vtable(this).WebAccountProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Scope(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Scope)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ClientId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ClientId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PromptType(&self) -> ::windows::core::Result<WebTokenRequestPromptType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WebTokenRequestPromptType>();
            (::windows::core::Interface::vtable(this).PromptType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::ComInterface::cast::<IWebTokenRequest2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).AppProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IWebTokenRequest3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CorrelationId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCorrelationId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebTokenRequest3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCorrelationId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn Create(provider: &super::super::super::Credentials::WebAccountProvider, scope: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebTokenRequest>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(provider), ::core::mem::transmute_copy(scope), ::core::mem::transmute_copy(clientid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithPromptType(provider: &super::super::super::Credentials::WebAccountProvider, scope: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, prompttype: WebTokenRequestPromptType) -> ::windows::core::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebTokenRequest>();
            (::windows::core::Interface::vtable(this).CreateWithPromptType)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(provider), ::core::mem::transmute_copy(scope), ::core::mem::transmute_copy(clientid), prompttype, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithProvider(provider: &super::super::super::Credentials::WebAccountProvider) -> ::windows::core::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebTokenRequest>();
            (::windows::core::Interface::vtable(this).CreateWithProvider)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(provider), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithScope(provider: &super::super::super::Credentials::WebAccountProvider, scope: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebTokenRequest>();
            (::windows::core::Interface::vtable(this).CreateWithScope)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(provider), ::core::mem::transmute_copy(scope), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebTokenRequestFactory<R, F: FnOnce(&IWebTokenRequestFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebTokenRequest, IWebTokenRequestFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for WebTokenRequest {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebTokenRequest;{b77b4d68-adcb-4673-b364-0cf7b35caf97})");
}
impl ::core::clone::Clone for WebTokenRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebTokenRequest {
    type Vtable = IWebTokenRequest_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebTokenRequest {
    const IID: ::windows::core::GUID = <IWebTokenRequest as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebTokenRequest {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenRequest";
}
::windows::imp::interface_hierarchy!(WebTokenRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebTokenRequest {}
unsafe impl ::core::marker::Sync for WebTokenRequest {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebTokenRequestResult(::windows::core::IUnknown);
impl WebTokenRequestResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResponseData(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<WebTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<WebTokenResponse>>();
            (::windows::core::Interface::vtable(this).ResponseData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResponseStatus(&self) -> ::windows::core::Result<WebTokenRequestStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WebTokenRequestStatus>();
            (::windows::core::Interface::vtable(this).ResponseStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResponseError(&self) -> ::windows::core::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WebProviderError>();
            (::windows::core::Interface::vtable(this).ResponseError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InvalidateCacheAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).InvalidateCacheAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for WebTokenRequestResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebTokenRequestResult;{c12a8305-d1f8-4483-8d54-38fe292784ff})");
}
impl ::core::clone::Clone for WebTokenRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebTokenRequestResult {
    type Vtable = IWebTokenRequestResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebTokenRequestResult {
    const IID: ::windows::core::GUID = <IWebTokenRequestResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebTokenRequestResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenRequestResult";
}
::windows::imp::interface_hierarchy!(WebTokenRequestResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebTokenRequestResult {}
unsafe impl ::core::marker::Sync for WebTokenRequestResult {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebTokenResponse(::windows::core::IUnknown);
impl WebTokenResponse {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebTokenResponse, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Token)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderError(&self) -> ::windows::core::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WebProviderError>();
            (::windows::core::Interface::vtable(this).ProviderError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Credentials::WebAccount>();
            (::windows::core::Interface::vtable(this).WebAccount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateWithToken(token: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenResponse> {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebTokenResponse>();
            (::windows::core::Interface::vtable(this).CreateWithToken)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithTokenAndAccount(token: &::windows::core::HSTRING, webaccount: &super::super::super::Credentials::WebAccount) -> ::windows::core::Result<WebTokenResponse> {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebTokenResponse>();
            (::windows::core::Interface::vtable(this).CreateWithTokenAndAccount)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(token), ::core::mem::transmute_copy(webaccount), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithTokenAccountAndError(token: &::windows::core::HSTRING, webaccount: &super::super::super::Credentials::WebAccount, error: &WebProviderError) -> ::windows::core::Result<WebTokenResponse> {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebTokenResponse>();
            (::windows::core::Interface::vtable(this).CreateWithTokenAccountAndError)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(token), ::core::mem::transmute_copy(webaccount), ::core::mem::transmute_copy(error), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebTokenResponseFactory<R, F: FnOnce(&IWebTokenResponseFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebTokenResponse, IWebTokenResponseFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for WebTokenResponse {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebTokenResponse;{67a7c5ca-83f6-44c6-a3b1-0eb69e41fa8a})");
}
impl ::core::clone::Clone for WebTokenResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebTokenResponse {
    type Vtable = IWebTokenResponse_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebTokenResponse {
    const IID: ::windows::core::GUID = <IWebTokenResponse as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenResponse";
}
::windows::imp::interface_hierarchy!(WebTokenResponse, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebTokenResponse {}
unsafe impl ::core::marker::Sync for WebTokenResponse {}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for FindAllWebAccountsStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FindAllWebAccountsStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindAllWebAccountsStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FindAllWebAccountsStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.FindAllWebAccountsStatus;i4)");
}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for WebTokenRequestPromptType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WebTokenRequestPromptType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebTokenRequestPromptType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WebTokenRequestPromptType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.WebTokenRequestPromptType;i4)");
}
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for WebTokenRequestStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WebTokenRequestStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebTokenRequestStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WebTokenRequestStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.WebTokenRequestStatus;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
