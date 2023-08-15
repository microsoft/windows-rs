#[doc(hidden)]
#[repr(transparent)]
pub struct ICardAddedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICardAddedEventArgs {
    type Vtable = ICardAddedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICardAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICardAddedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18bbef98_f18b_4dd3_b118_dfb2c8e23cc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICardAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICardRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICardRemovedEventArgs {
    type Vtable = ICardRemovedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICardRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICardRemovedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15331aaf_22d7_4945_afc9_03b46f42a6cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICardRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownSmartCardAppletIds(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownSmartCardAppletIds {
    type Vtable = IKnownSmartCardAppletIds_Vtbl;
}
impl ::core::clone::Clone for IKnownSmartCardAppletIds {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IKnownSmartCardAppletIds {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b04d8d8_95b4_4c88_8cea_411e55511efc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSmartCardAppletIds_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub PaymentSystemEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PaymentSystemEnvironment: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ProximityPaymentSystemEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProximityPaymentSystemEnvironment: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCard(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCard {
    type Vtable = ISmartCard_Vtbl;
}
impl ::core::clone::Clone for ISmartCard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCard {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b718871_6434_43f4_b55a_6a29623870aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCard_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStatusAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetAnswerToResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetAnswerToResetAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroup {
    type Vtable = ISmartCardAppletIdGroup_Vtbl;
}
impl ::core::clone::Clone for ISmartCardAppletIdGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardAppletIdGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7db165e6_6264_56f4_5e03_c86385395eb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroup_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub AppletIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    AppletIds: usize,
    pub SmartCardEmulationCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulationCategory) -> ::windows_core::HRESULT,
    pub SetSmartCardEmulationCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardEmulationCategory) -> ::windows_core::HRESULT,
    pub SmartCardEmulationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulationType) -> ::windows_core::HRESULT,
    pub SetSmartCardEmulationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardEmulationType) -> ::windows_core::HRESULT,
    pub AutomaticEnablement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutomaticEnablement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroup2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroup2 {
    type Vtable = ISmartCardAppletIdGroup2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardAppletIdGroup2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardAppletIdGroup2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b0ef9dc_9956_4a62_8d4e_d37a68ebc3a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroup2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogo: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub SecureUserAuthenticationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSecureUserAuthenticationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroupFactory {
    type Vtable = ISmartCardAppletIdGroupFactory_Vtbl;
}
impl ::core::clone::Clone for ISmartCardAppletIdGroupFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardAppletIdGroupFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9105eb4d_4a65_4e41_8061_cbe83f3695e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, appletids: *mut ::core::ffi::c_void, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupRegistration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroupRegistration {
    type Vtable = ISmartCardAppletIdGroupRegistration_Vtbl;
}
impl ::core::clone::Clone for ISmartCardAppletIdGroupRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardAppletIdGroupRegistration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf1208d1_31bb_5596_43b1_6d69a0257b3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupRegistration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ActivationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardAppletIdGroupActivationPolicy) -> ::windows_core::HRESULT,
    pub AppletIdGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestActivationPolicyChangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: SmartCardAppletIdGroupActivationPolicy, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestActivationPolicyChangeAsync: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAutomaticResponseApdusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apdus: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAutomaticResponseApdusAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupRegistration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroupRegistration2 {
    type Vtable = ISmartCardAppletIdGroupRegistration2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardAppletIdGroupRegistration2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardAppletIdGroupRegistration2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f5508d8_98a7_4f2e_91d9_6cfcceda407f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupRegistration2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SmartCardReaderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroupStatics {
    type Vtable = ISmartCardAppletIdGroupStatics_Vtbl;
}
impl ::core::clone::Clone for ISmartCardAppletIdGroupStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardAppletIdGroupStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab2899a9_e76c_45cf_bf1d_90eaa6205927);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxAppletIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAutomaticResponseApdu {
    type Vtable = ISmartCardAutomaticResponseApdu_Vtbl;
}
impl ::core::clone::Clone for ISmartCardAutomaticResponseApdu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardAutomaticResponseApdu {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52152bab_c63e_4531_a857_d756d99b986a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetCommandApdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetCommandApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApduBitMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApduBitMask: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetCommandApduBitMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetCommandApduBitMask: usize,
    pub ShouldMatchLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldMatchLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AppletId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetAppletId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetAppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ResponseApdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ResponseApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetResponseApdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetResponseApdu: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAutomaticResponseApdu2 {
    type Vtable = ISmartCardAutomaticResponseApdu2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardAutomaticResponseApdu2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardAutomaticResponseApdu2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44aebb14_559d_4531_4e51_89db6fa8a57a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InputState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputState: usize,
    #[cfg(feature = "Foundation")]
    pub SetInputState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInputState: usize,
    #[cfg(feature = "Foundation")]
    pub OutputState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutputState: usize,
    #[cfg(feature = "Foundation")]
    pub SetOutputState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOutputState: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAutomaticResponseApdu3 {
    type Vtable = ISmartCardAutomaticResponseApdu3_Vtbl;
}
impl ::core::clone::Clone for ISmartCardAutomaticResponseApdu3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardAutomaticResponseApdu3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf43da74_6576_4392_9367_fe3bc9e2d496);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllowWhenCryptogramGeneratorNotPrepared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowWhenCryptogramGeneratorNotPrepared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApduFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAutomaticResponseApduFactory {
    type Vtable = ISmartCardAutomaticResponseApduFactory_Vtbl;
}
impl ::core::clone::Clone for ISmartCardAutomaticResponseApduFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardAutomaticResponseApduFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe97ea2fa_d02c_4c55_b02a_8cff7fa9f05b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApduFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandapdu: *mut ::core::ffi::c_void, responseapdu: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardChallengeContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardChallengeContext {
    type Vtable = ISmartCardChallengeContext_Vtbl;
}
impl ::core::clone::Clone for ISmartCardChallengeContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardChallengeContext {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x192a5319_c9c4_4947_81cc_44794a61ef91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardChallengeContext_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Challenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Challenge: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub VerifyResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    VerifyResponseAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProvisionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: *mut ::core::ffi::c_void, formatcard: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProvisionAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProvisionAsyncWithNewCardId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: *mut ::core::ffi::c_void, formatcard: bool, newcardid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProvisionAsyncWithNewCardId: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ChangeAdministrativeKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: *mut ::core::ffi::c_void, newadministrativekey: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ChangeAdministrativeKeyAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardConnect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardConnect {
    type Vtable = ISmartCardConnect_Vtbl;
}
impl ::core::clone::Clone for ISmartCardConnect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardConnect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2fdf87e5_028d_491e_a058_3382c3986f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardConnect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardConnection {
    type Vtable = ISmartCardConnection_Vtbl;
}
impl ::core::clone::Clone for ISmartCardConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7edb991a_a81a_47bc_a649_156be6b7f231);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TransmitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TransmitAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGenerator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGenerator {
    type Vtable = ISmartCardCryptogramGenerator_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramGenerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe39f587b_edd3_4e49_b594_0ff5e4d0c76f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGenerator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramAlgorithms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialPackageFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialPackageFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialPackageConfirmationResponseFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialPackageConfirmationResponseFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedSmartCardCryptogramStorageKeyCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedSmartCardCryptogramStorageKeyCapabilities: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteCryptogramMaterialStorageKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagekeyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteCryptogramMaterialStorageKeyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateCryptogramMaterialStorageKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, algorithm: SmartCardCryptogramStorageKeyAlgorithm, capabilities: SmartCardCryptogramStorageKeyCapabilities, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateCryptogramMaterialStorageKeyAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core"))]
    pub RequestCryptogramMaterialStorageKeyInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, format: super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Core")))]
    RequestCryptogramMaterialStorageKeyInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ImportCryptogramMaterialPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: SmartCardCryptogramMaterialPackageFormat, storagekeyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, materialpackagename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, cryptogrammaterialpackage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ImportCryptogramMaterialPackageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryProvePossessionOfCryptogramMaterialPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, responseformat: SmartCardCryptogramMaterialPackageConfirmationResponseFormat, materialpackagename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, materialname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, challenge: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryProvePossessionOfCryptogramMaterialPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUnlockCryptogramMaterialForUseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnlockCryptogramMaterialForUseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteCryptogramMaterialPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, materialpackagename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteCryptogramMaterialPackageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGenerator2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGenerator2 {
    type Vtable = ISmartCardCryptogramGenerator2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramGenerator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramGenerator2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7116aa34_5d6d_4b4a_96a3_efa47d2a7e25);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGenerator2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ValidateRequestApduAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, apdutovalidate: *mut ::core::ffi::c_void, cryptogramplacementsteps: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ValidateRequestApduAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAllCryptogramStorageKeyCharacteristicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAllCryptogramStorageKeyCharacteristicsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAllCryptogramMaterialPackageCharacteristicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAllCryptogramMaterialPackageCharacteristicsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagekeyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAllCryptogramMaterialCharacteristicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, materialpackagename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAllCryptogramMaterialCharacteristicsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGeneratorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGeneratorStatics {
    type Vtable = ISmartCardCryptogramGeneratorStatics_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramGeneratorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramGeneratorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09933910_cb9c_4015_967d_5234f3b02900);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGeneratorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetSmartCardCryptogramGeneratorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSmartCardCryptogramGeneratorAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGeneratorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGeneratorStatics2 {
    type Vtable = ISmartCardCryptogramGeneratorStatics2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramGeneratorStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramGeneratorStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09bdf5e5_b4bd_4e23_a588_74469204c128);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGeneratorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2798e029_d687_4c92_86c6_399e9a0ecb09);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e6a8a5c_9773_46c4_a32f_b1e543159e04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c7ce857_a7e7_489d_b9d6_368061515012);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialCharacteristics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramMaterialCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialCharacteristics_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramMaterialCharacteristics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramMaterialCharacteristics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc9ac5cc_c1d7_4153_923b_a2d43c6c8d49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialCharacteristics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaterialName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedAlgorithms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedProofOfPossessionAlgorithms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedProofOfPossessionAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedValidations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedValidations: usize,
    pub MaterialType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramMaterialType) -> ::windows_core::HRESULT,
    pub ProtectionMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramMaterialProtectionMethod) -> ::windows_core::HRESULT,
    pub ProtectionVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaterialLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialPackageCharacteristics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramMaterialPackageCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialPackageCharacteristics_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramMaterialPackageCharacteristics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramMaterialPackageCharacteristics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xffb58e1f_0692_4c47_93cf_34d91f9dcd00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialPackageCharacteristics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PackageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StorageKeyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DateImported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateImported: usize,
    pub PackageFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramMaterialPackageFormat) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialPossessionProof(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramMaterialPossessionProof {
    type Vtable = ISmartCardCryptogramMaterialPossessionProof_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramMaterialPossessionProof {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramMaterialPossessionProof {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5b9ab8c_a141_4135_9add_b0d2e3aa1fc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialPossessionProof_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Proof: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Proof: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramPlacementStep(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramPlacementStep {
    type Vtable = ISmartCardCryptogramPlacementStep_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramPlacementStep {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramPlacementStep {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x947b03eb_8342_4792_a2e5_925636378a53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramPlacementStep_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Algorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramAlgorithm) -> ::windows_core::HRESULT,
    pub SetAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardCryptogramAlgorithm) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SourceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SourceData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSourceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSourceData: usize,
    pub CryptogramMaterialPackageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCryptogramMaterialPackageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CryptogramMaterialName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCryptogramMaterialName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TemplateOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetTemplateOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub CryptogramOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetCryptogramOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub CryptogramLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetCryptogramLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub CryptogramPlacementOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramPlacementOptions) -> ::windows_core::HRESULT,
    pub SetCryptogramPlacementOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardCryptogramPlacementOptions) -> ::windows_core::HRESULT,
    pub ChainedOutputStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetChainedOutputStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyCharacteristics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramStorageKeyCharacteristics {
    type Vtable = ISmartCardCryptogramStorageKeyCharacteristics_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramStorageKeyCharacteristics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramStorageKeyCharacteristics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8552546e_4457_4825_b464_635471a39f5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyCharacteristics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StorageKeyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DateCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateCreated: usize,
    pub Algorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramStorageKeyAlgorithm) -> ::windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramStorageKeyCapabilities) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramStorageKeyInfo {
    type Vtable = ISmartCardCryptogramStorageKeyInfo_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramStorageKeyInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramStorageKeyInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77b0f00d_b097_4f61_a26a_9561639c9c3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Core")]
    pub PublicKeyBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Core"))]
    PublicKeyBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PublicKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublicKey: usize,
    pub AttestationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptographicKeyAttestationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Attestation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Attestation: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AttestationCertificateChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttestationCertificateChain: usize,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramStorageKeyCapabilities) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramStorageKeyInfo2 {
    type Vtable = ISmartCardCryptogramStorageKeyInfo2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardCryptogramStorageKeyInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardCryptogramStorageKeyInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x000440f9_f7fd_417d_89e1_fbb0382adc4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyInfo2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OperationalRequirements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulator {
    type Vtable = ISmartCardEmulator_Vtbl;
}
impl ::core::clone::Clone for ISmartCardEmulator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardEmulator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfb906b2_875e_47e5_8077_e8bff1b1c6fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EnablementPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulatorEnablementPolicy) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulator2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulator2 {
    type Vtable = ISmartCardEmulator2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardEmulator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardEmulator2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe3fc0b8_8529_411a_807b_48edc2a0ab44);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulator2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ApduReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApduReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveApduReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveApduReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionDeactivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionDeactivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionDeactivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionDeactivated: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsHostCardEmulationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorApduReceivedEventArgs {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISmartCardEmulatorApduReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardEmulatorApduReceivedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd55d1576_69d2_5333_5b5f_f8c0d6e9f09f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApdu: usize,
    pub ConnectionProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryRespondAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseapdu: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryRespondAsync: usize,
    pub AutomaticResponseStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardAutomaticResponseStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorApduReceivedEventArgs2 {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardEmulatorApduReceivedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardEmulatorApduReceivedEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bf93df0_22e1_4238_8610_94ce4a965425);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryRespondWithStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseapdu: *mut ::core::ffi::c_void, nextstate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryRespondWithStateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgsWithCryptograms(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorApduReceivedEventArgsWithCryptograms {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgsWithCryptograms_Vtbl;
}
impl ::core::clone::Clone for ISmartCardEmulatorApduReceivedEventArgsWithCryptograms {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardEmulatorApduReceivedEventArgsWithCryptograms {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd550bac7_b7bf_4e29_9294_0c4ac3c941bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgsWithCryptograms_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub TryRespondWithCryptogramsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responsetemplate: *mut ::core::ffi::c_void, cryptogramplacementsteps: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    TryRespondWithCryptogramsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub TryRespondWithCryptogramsAndStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responsetemplate: *mut ::core::ffi::c_void, cryptogramplacementsteps: *mut ::core::ffi::c_void, nextstate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    TryRespondWithCryptogramsAndStateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorConnectionDeactivatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorConnectionDeactivatedEventArgs {
    type Vtable = ISmartCardEmulatorConnectionDeactivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISmartCardEmulatorConnectionDeactivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardEmulatorConnectionDeactivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2186d8d3_c5eb_5262_43df_62a0a1b55557);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorConnectionDeactivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ConnectionProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulatorConnectionDeactivatedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorConnectionProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorConnectionProperties {
    type Vtable = ISmartCardEmulatorConnectionProperties_Vtbl;
}
impl ::core::clone::Clone for ISmartCardEmulatorConnectionProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardEmulatorConnectionProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e2ca5ee_f969_507d_6cf9_34e2d18df311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorConnectionProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulatorConnectionSource) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorStatics {
    type Vtable = ISmartCardEmulatorStatics_Vtbl;
}
impl ::core::clone::Clone for ISmartCardEmulatorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardEmulatorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a9bfc4b_c4d3_494f_b8a2_6215d81e85b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorStatics2 {
    type Vtable = ISmartCardEmulatorStatics2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardEmulatorStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardEmulatorStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69ae9f8a_b775_488b_8436_6c1e28ed731f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppletIdGroupRegistrationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppletIdGroupRegistrationsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RegisterAppletIdGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appletidgroup: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterAppletIdGroupAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnregisterAppletIdGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnregisterAppletIdGroupAsync: usize,
    pub MaxAppletIdGroupRegistrations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorStatics3 {
    type Vtable = ISmartCardEmulatorStatics3_Vtbl;
}
impl ::core::clone::Clone for ISmartCardEmulatorStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardEmulatorStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59ea142a_9f09_43f5_8565_cfa8148e4cb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardPinPolicy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardPinPolicy {
    type Vtable = ISmartCardPinPolicy_Vtbl;
}
impl ::core::clone::Clone for ISmartCardPinPolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardPinPolicy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x183ce184_4db6_4841_ac9e_2ac1f39b7304);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardPinPolicy_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MinLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMinLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub UppercaseLetters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub SetUppercaseLetters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub LowercaseLetters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub SetLowercaseLetters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub Digits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub SetDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub SpecialCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub SetSpecialCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardPinResetDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardPinResetDeferral {
    type Vtable = ISmartCardPinResetDeferral_Vtbl;
}
impl ::core::clone::Clone for ISmartCardPinResetDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardPinResetDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18c94aac_7805_4004_85e4_bbefac8f6884);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardPinResetDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardPinResetRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardPinResetRequest {
    type Vtable = ISmartCardPinResetRequest_Vtbl;
}
impl ::core::clone::Clone for ISmartCardPinResetRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardPinResetRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12fe3c4d_5fb9_4e8e_9ff6_61f475124fef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardPinResetRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Challenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Challenge: usize,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetResponse: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardProvisioning(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardProvisioning {
    type Vtable = ISmartCardProvisioning_Vtbl;
}
impl ::core::clone::Clone for ISmartCardProvisioning {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardProvisioning {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19eeedbd_1fab_477c_b712_1a2c5af1fd6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioning_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetChallengeContextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetChallengeContextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPinChangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPinChangeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPinResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPinResetAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardProvisioning2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardProvisioning2 {
    type Vtable = ISmartCardProvisioning2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardProvisioning2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardProvisioning2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10fd28eb_3f79_4b66_9b7c_11c149b7d0bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioning2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetAuthorityKeyContainerNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAuthorityKeyContainerNameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardProvisioningStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardProvisioningStatics {
    type Vtable = ISmartCardProvisioningStatics_Vtbl;
}
impl ::core::clone::Clone for ISmartCardProvisioningStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardProvisioningStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13882848_0d13_4e70_9735_51daeca5254f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioningStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromSmartCardAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, card: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromSmartCardAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestVirtualSmartCardCreationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, administrativekey: *mut ::core::ffi::c_void, pinpolicy: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestVirtualSmartCardCreationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestVirtualSmartCardCreationAsyncWithCardId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, administrativekey: *mut ::core::ffi::c_void, pinpolicy: *mut ::core::ffi::c_void, cardid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestVirtualSmartCardCreationAsyncWithCardId: usize,
    #[cfg(feature = "Foundation")]
    pub RequestVirtualSmartCardDeletionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, card: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestVirtualSmartCardDeletionAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardProvisioningStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardProvisioningStatics2 {
    type Vtable = ISmartCardProvisioningStatics2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardProvisioningStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardProvisioningStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3447c6a8_c9a0_4bd6_b50d_251f4e8d3a62);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioningStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestAttestedVirtualSmartCardCreationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, administrativekey: *mut ::core::ffi::c_void, pinpolicy: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestAttestedVirtualSmartCardCreationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestAttestedVirtualSmartCardCreationAsyncWithCardId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, administrativekey: *mut ::core::ffi::c_void, pinpolicy: *mut ::core::ffi::c_void, cardid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestAttestedVirtualSmartCardCreationAsyncWithCardId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardReader {
    type Vtable = ISmartCardReader_Vtbl;
}
impl ::core::clone::Clone for ISmartCardReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1074b4e0_54c2_4df0_817a_14c14378f06c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardReaderKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStatusAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllCardsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllCardsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CardAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CardAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCardAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCardAdded: usize,
    #[cfg(feature = "Foundation")]
    pub CardRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CardRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCardRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCardRemoved: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardReaderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardReaderStatics {
    type Vtable = ISmartCardReaderStatics_Vtbl;
}
impl ::core::clone::Clone for ISmartCardReaderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardReaderStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x103c04e1_a1ca_48f2_a281_5b6f669af107);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardReaderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorWithKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SmartCardReaderKind, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardTriggerDetails {
    type Vtable = ISmartCardTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for ISmartCardTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f9bf11e_39ef_4f2b_b44f_0a9155b177bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TriggerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardTriggerType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SourceAppletId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SourceAppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub TriggerData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TriggerData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardTriggerDetails2 {
    type Vtable = ISmartCardTriggerDetails2_Vtbl;
}
impl ::core::clone::Clone for ISmartCardTriggerDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardTriggerDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2945c569_8975_4a51_9e1a_5f8a76ee51af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Emulator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryLaunchCurrentAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryLaunchCurrentAppAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryLaunchCurrentAppWithBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: ::std::mem::MaybeUninit<::windows_core::HSTRING>, behavior: SmartCardLaunchBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryLaunchCurrentAppWithBehaviorAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardTriggerDetails3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardTriggerDetails3 {
    type Vtable = ISmartCardTriggerDetails3_Vtbl;
}
impl ::core::clone::Clone for ISmartCardTriggerDetails3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISmartCardTriggerDetails3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3e2c27d_18c6_4ba8_8376_ef03d4912666);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct CardAddedEventArgs(::windows_core::IUnknown);
impl CardAddedEventArgs {
    pub fn SmartCard(&self) -> ::windows_core::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmartCard)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CardAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CardAddedEventArgs {}
impl ::core::fmt::Debug for CardAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CardAddedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CardAddedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.CardAddedEventArgs;{18bbef98-f18b-4dd3-b118-dfb2c8e23cc6})");
}
impl ::core::clone::Clone for CardAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CardAddedEventArgs {
    type Vtable = ICardAddedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CardAddedEventArgs {
    const IID: ::windows_core::GUID = <ICardAddedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CardAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.CardAddedEventArgs";
}
::windows_core::imp::interface_hierarchy!(CardAddedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CardAddedEventArgs {}
unsafe impl ::core::marker::Sync for CardAddedEventArgs {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct CardRemovedEventArgs(::windows_core::IUnknown);
impl CardRemovedEventArgs {
    pub fn SmartCard(&self) -> ::windows_core::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmartCard)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CardRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CardRemovedEventArgs {}
impl ::core::fmt::Debug for CardRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CardRemovedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CardRemovedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.CardRemovedEventArgs;{15331aaf-22d7-4945-afc9-03b46f42a6cd})");
}
impl ::core::clone::Clone for CardRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CardRemovedEventArgs {
    type Vtable = ICardRemovedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CardRemovedEventArgs {
    const IID: ::windows_core::GUID = <ICardRemovedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CardRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.CardRemovedEventArgs";
}
::windows_core::imp::interface_hierarchy!(CardRemovedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CardRemovedEventArgs {}
unsafe impl ::core::marker::Sync for CardRemovedEventArgs {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
pub struct KnownSmartCardAppletIds;
impl KnownSmartCardAppletIds {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PaymentSystemEnvironment() -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        Self::IKnownSmartCardAppletIds(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PaymentSystemEnvironment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ProximityPaymentSystemEnvironment() -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        Self::IKnownSmartCardAppletIds(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProximityPaymentSystemEnvironment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownSmartCardAppletIds<R, F: FnOnce(&IKnownSmartCardAppletIds) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KnownSmartCardAppletIds, IKnownSmartCardAppletIds> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for KnownSmartCardAppletIds {
    const NAME: &'static str = "Windows.Devices.SmartCards.KnownSmartCardAppletIds";
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCard(::windows_core::IUnknown);
impl SmartCard {
    pub fn Reader(&self) -> ::windows_core::Result<SmartCardReader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetStatusAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStatusAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetAnswerToResetAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAnswerToResetAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardConnection>> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardConnect>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCard {}
impl ::core::fmt::Debug for SmartCard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCard").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCard {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCard;{1b718871-6434-43f4-b55a-6a29623870aa})");
}
impl ::core::clone::Clone for SmartCard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCard {
    type Vtable = ISmartCard_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCard {
    const IID: ::windows_core::GUID = <ISmartCard as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCard {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCard";
}
::windows_core::imp::interface_hierarchy!(SmartCard, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCard {}
unsafe impl ::core::marker::Sync for SmartCard {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardAppletIdGroup(::windows_core::IUnknown);
impl SmartCardAppletIdGroup {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardAppletIdGroup, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn AppletIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppletIds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SmartCardEmulationCategory(&self) -> ::windows_core::Result<SmartCardEmulationCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmartCardEmulationCategory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSmartCardEmulationCategory(&self, value: SmartCardEmulationCategory) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSmartCardEmulationCategory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SmartCardEmulationType(&self) -> ::windows_core::Result<SmartCardEmulationType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmartCardEmulationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSmartCardEmulationType(&self, value: SmartCardEmulationType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSmartCardEmulationType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticEnablement(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutomaticEnablement)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutomaticEnablement(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutomaticEnablement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetLogo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLogo)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SecureUserAuthenticationRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SecureUserAuthenticationRequired)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSecureUserAuthenticationRequired(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSecureUserAuthenticationRequired)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Create<P0>(displayname: &::windows_core::HSTRING, appletids: P0, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType) -> ::windows_core::Result<SmartCardAppletIdGroup>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>>,
    {
        Self::ISmartCardAppletIdGroupFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(displayname), appletids.try_into_param()?.abi(), emulationcategory, emulationtype, &mut result__).from_abi(result__)
        })
    }
    pub fn MaxAppletIds() -> ::windows_core::Result<u16> {
        Self::ISmartCardAppletIdGroupStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxAppletIds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardAppletIdGroupFactory<R, F: FnOnce(&ISmartCardAppletIdGroupFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardAppletIdGroup, ISmartCardAppletIdGroupFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISmartCardAppletIdGroupStatics<R, F: FnOnce(&ISmartCardAppletIdGroupStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardAppletIdGroup, ISmartCardAppletIdGroupStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmartCardAppletIdGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardAppletIdGroup {}
impl ::core::fmt::Debug for SmartCardAppletIdGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardAppletIdGroup").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardAppletIdGroup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardAppletIdGroup;{7db165e6-6264-56f4-5e03-c86385395eb1})");
}
impl ::core::clone::Clone for SmartCardAppletIdGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardAppletIdGroup {
    type Vtable = ISmartCardAppletIdGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardAppletIdGroup {
    const IID: ::windows_core::GUID = <ISmartCardAppletIdGroup as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardAppletIdGroup {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAppletIdGroup";
}
::windows_core::imp::interface_hierarchy!(SmartCardAppletIdGroup, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardAppletIdGroup {}
unsafe impl ::core::marker::Sync for SmartCardAppletIdGroup {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardAppletIdGroupRegistration(::windows_core::IUnknown);
impl SmartCardAppletIdGroupRegistration {
    pub fn ActivationPolicy(&self) -> ::windows_core::Result<SmartCardAppletIdGroupActivationPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivationPolicy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppletIdGroup(&self) -> ::windows_core::Result<SmartCardAppletIdGroup> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppletIdGroup)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestActivationPolicyChangeAsync(&self, policy: SmartCardAppletIdGroupActivationPolicy) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardActivationPolicyChangeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestActivationPolicyChangeAsync)(::windows_core::Interface::as_raw(this), policy, &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAutomaticResponseApdusAsync<P0>(&self, apdus: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<SmartCardAutomaticResponseApdu>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAutomaticResponseApdusAsync)(::windows_core::Interface::as_raw(this), apdus.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SmartCardReaderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAppletIdGroupRegistration2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmartCardReaderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPropertiesAsync<P0>(&self, props: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::ValueSet>,
    {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAppletIdGroupRegistration2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPropertiesAsync)(::windows_core::Interface::as_raw(this), props.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardAppletIdGroupRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardAppletIdGroupRegistration {}
impl ::core::fmt::Debug for SmartCardAppletIdGroupRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardAppletIdGroupRegistration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardAppletIdGroupRegistration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardAppletIdGroupRegistration;{df1208d1-31bb-5596-43b1-6d69a0257b3a})");
}
impl ::core::clone::Clone for SmartCardAppletIdGroupRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardAppletIdGroupRegistration {
    type Vtable = ISmartCardAppletIdGroupRegistration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardAppletIdGroupRegistration {
    const IID: ::windows_core::GUID = <ISmartCardAppletIdGroupRegistration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardAppletIdGroupRegistration {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAppletIdGroupRegistration";
}
::windows_core::imp::interface_hierarchy!(SmartCardAppletIdGroupRegistration, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardAppletIdGroupRegistration {}
unsafe impl ::core::marker::Sync for SmartCardAppletIdGroupRegistration {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardAutomaticResponseApdu(::windows_core::IUnknown);
impl SmartCardAutomaticResponseApdu {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CommandApdu(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommandApdu)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetCommandApdu<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommandApdu)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CommandApduBitMask(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommandApduBitMask)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetCommandApduBitMask<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommandApduBitMask)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn ShouldMatchLength(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldMatchLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShouldMatchLength(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldMatchLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AppletId(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppletId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetAppletId<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppletId)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ResponseApdu(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseApdu)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetResponseApdu<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResponseApdu)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputState(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInputState<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInputState)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OutputState(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OutputState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOutputState<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutputState)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn AllowWhenCryptogramGeneratorNotPrepared(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAutomaticResponseApdu3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowWhenCryptogramGeneratorNotPrepared)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowWhenCryptogramGeneratorNotPrepared(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardAutomaticResponseApdu3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowWhenCryptogramGeneratorNotPrepared)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0, P1>(commandapdu: P0, responseapdu: P1) -> ::windows_core::Result<SmartCardAutomaticResponseApdu>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ISmartCardAutomaticResponseApduFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), commandapdu.try_into_param()?.abi(), responseapdu.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardAutomaticResponseApduFactory<R, F: FnOnce(&ISmartCardAutomaticResponseApduFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardAutomaticResponseApdu, ISmartCardAutomaticResponseApduFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmartCardAutomaticResponseApdu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardAutomaticResponseApdu {}
impl ::core::fmt::Debug for SmartCardAutomaticResponseApdu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardAutomaticResponseApdu").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardAutomaticResponseApdu {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardAutomaticResponseApdu;{52152bab-c63e-4531-a857-d756d99b986a})");
}
impl ::core::clone::Clone for SmartCardAutomaticResponseApdu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardAutomaticResponseApdu {
    type Vtable = ISmartCardAutomaticResponseApdu_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardAutomaticResponseApdu {
    const IID: ::windows_core::GUID = <ISmartCardAutomaticResponseApdu as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardAutomaticResponseApdu {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAutomaticResponseApdu";
}
::windows_core::imp::interface_hierarchy!(SmartCardAutomaticResponseApdu, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardAutomaticResponseApdu {}
unsafe impl ::core::marker::Sync for SmartCardAutomaticResponseApdu {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardChallengeContext(::windows_core::IUnknown);
impl SmartCardChallengeContext {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Challenge(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Challenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn VerifyResponseAsync<P0>(&self, response: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VerifyResponseAsync)(::windows_core::Interface::as_raw(this), response.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProvisionAsync<P0>(&self, response: P0, formatcard: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProvisionAsync)(::windows_core::Interface::as_raw(this), response.try_into_param()?.abi(), formatcard, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProvisionAsyncWithNewCardId<P0>(&self, response: P0, formatcard: bool, newcardid: ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProvisionAsyncWithNewCardId)(::windows_core::Interface::as_raw(this), response.try_into_param()?.abi(), formatcard, newcardid, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ChangeAdministrativeKeyAsync<P0, P1>(&self, response: P0, newadministrativekey: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangeAdministrativeKeyAsync)(::windows_core::Interface::as_raw(this), response.try_into_param()?.abi(), newadministrativekey.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardChallengeContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardChallengeContext {}
impl ::core::fmt::Debug for SmartCardChallengeContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardChallengeContext").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardChallengeContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardChallengeContext;{192a5319-c9c4-4947-81cc-44794a61ef91})");
}
impl ::core::clone::Clone for SmartCardChallengeContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardChallengeContext {
    type Vtable = ISmartCardChallengeContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardChallengeContext {
    const IID: ::windows_core::GUID = <ISmartCardChallengeContext as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardChallengeContext {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardChallengeContext";
}
::windows_core::imp::interface_hierarchy!(SmartCardChallengeContext, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for SmartCardChallengeContext {}
unsafe impl ::core::marker::Send for SmartCardChallengeContext {}
unsafe impl ::core::marker::Sync for SmartCardChallengeContext {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardConnection(::windows_core::IUnknown);
impl SmartCardConnection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TransmitAsync<P0>(&self, command: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransmitAsync)(::windows_core::Interface::as_raw(this), command.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardConnection {}
impl ::core::fmt::Debug for SmartCardConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardConnection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardConnection;{7edb991a-a81a-47bc-a649-156be6b7f231})");
}
impl ::core::clone::Clone for SmartCardConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardConnection {
    type Vtable = ISmartCardConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardConnection {
    const IID: ::windows_core::GUID = <ISmartCardConnection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardConnection {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardConnection";
}
::windows_core::imp::interface_hierarchy!(SmartCardConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for SmartCardConnection {}
unsafe impl ::core::marker::Send for SmartCardConnection {}
unsafe impl ::core::marker::Sync for SmartCardConnection {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramGenerator(::windows_core::IUnknown);
impl SmartCardCryptogramGenerator {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramMaterialTypes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCryptogramMaterialTypes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramAlgorithms(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCryptogramAlgorithms)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramMaterialPackageFormats(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCryptogramMaterialPackageFormats)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramMaterialPackageConfirmationResponseFormats(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCryptogramMaterialPackageConfirmationResponseFormats)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedSmartCardCryptogramStorageKeyCapabilities(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCapabilities>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedSmartCardCryptogramStorageKeyCapabilities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteCryptogramMaterialStorageKeyAsync(&self, storagekeyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteCryptogramMaterialStorageKeyAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(storagekeyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateCryptogramMaterialStorageKeyAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: &::windows_core::HSTRING, algorithm: SmartCardCryptogramStorageKeyAlgorithm, capabilities: SmartCardCryptogramStorageKeyCapabilities) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCryptogramMaterialStorageKeyAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, ::core::mem::transmute_copy(storagekeyname), algorithm, capabilities, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Cryptography_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core"))]
    pub fn RequestCryptogramMaterialStorageKeyInfoAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: &::windows_core::HSTRING, format: super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramStorageKeyInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCryptogramMaterialStorageKeyInfoAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, ::core::mem::transmute_copy(storagekeyname), format, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ImportCryptogramMaterialPackageAsync<P0>(&self, format: SmartCardCryptogramMaterialPackageFormat, storagekeyname: &::windows_core::HSTRING, materialpackagename: &::windows_core::HSTRING, cryptogrammaterialpackage: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImportCryptogramMaterialPackageAsync)(::windows_core::Interface::as_raw(this), format, ::core::mem::transmute_copy(storagekeyname), ::core::mem::transmute_copy(materialpackagename), cryptogrammaterialpackage.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TryProvePossessionOfCryptogramMaterialPackageAsync<P0>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, responseformat: SmartCardCryptogramMaterialPackageConfirmationResponseFormat, materialpackagename: &::windows_core::HSTRING, materialname: &::windows_core::HSTRING, challenge: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramMaterialPossessionProof>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryProvePossessionOfCryptogramMaterialPackageAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, responseformat, ::core::mem::transmute_copy(materialpackagename), ::core::mem::transmute_copy(materialname), challenge.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUnlockCryptogramMaterialForUseAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestUnlockCryptogramMaterialForUseAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteCryptogramMaterialPackageAsync(&self, materialpackagename: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteCryptogramMaterialPackageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(materialpackagename), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn ValidateRequestApduAsync<P0, P1>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, apdutovalidate: P0, cryptogramplacementsteps: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValidateRequestApduAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, apdutovalidate.try_into_param()?.abi(), cryptogramplacementsteps.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAllCryptogramStorageKeyCharacteristicsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult>> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllCryptogramStorageKeyCharacteristicsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAllCryptogramMaterialPackageCharacteristicsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllCryptogramMaterialPackageCharacteristicsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync(&self, storagekeyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(storagekeyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAllCryptogramMaterialCharacteristicsAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, materialpackagename: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult>> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllCryptogramMaterialCharacteristicsAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, ::core::mem::transmute_copy(materialpackagename), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSmartCardCryptogramGeneratorAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGenerator>> {
        Self::ISmartCardCryptogramGeneratorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSmartCardCryptogramGeneratorAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::ISmartCardCryptogramGeneratorStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardCryptogramGeneratorStatics<R, F: FnOnce(&ISmartCardCryptogramGeneratorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardCryptogramGenerator, ISmartCardCryptogramGeneratorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISmartCardCryptogramGeneratorStatics2<R, F: FnOnce(&ISmartCardCryptogramGeneratorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardCryptogramGenerator, ISmartCardCryptogramGeneratorStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramGenerator {}
impl ::core::fmt::Debug for SmartCardCryptogramGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramGenerator").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramGenerator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGenerator;{e39f587b-edd3-4e49-b594-0ff5e4d0c76f})");
}
impl ::core::clone::Clone for SmartCardCryptogramGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramGenerator {
    type Vtable = ISmartCardCryptogramGenerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardCryptogramGenerator {
    const IID: ::windows_core::GUID = <ISmartCardCryptogramGenerator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramGenerator {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGenerator";
}
::windows_core::imp::interface_hierarchy!(SmartCardCryptogramGenerator, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardCryptogramGenerator {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGenerator {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult(::windows_core::IUnknown);
impl SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OperationStatus(&self) -> ::windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OperationStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Characteristics)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
impl ::core::fmt::Debug for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult;{2798e029-d687-4c92-86c6-399e9a0ecb09})");
}
impl ::core::clone::Clone for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const IID: ::windows_core::GUID = <ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult";
}
::windows_core::imp::interface_hierarchy!(SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult(::windows_core::IUnknown);
impl SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OperationStatus(&self) -> ::windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OperationStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Characteristics)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
impl ::core::fmt::Debug for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult;{4e6a8a5c-9773-46c4-a32f-b1e543159e04})");
}
impl ::core::clone::Clone for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const IID: ::windows_core::GUID = <ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult";
}
::windows_core::imp::interface_hierarchy!(SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult(::windows_core::IUnknown);
impl SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OperationStatus(&self) -> ::windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OperationStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Characteristics)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
impl ::core::fmt::Debug for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult;{8c7ce857-a7e7-489d-b9d6-368061515012})");
}
impl ::core::clone::Clone for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const IID: ::windows_core::GUID = <ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult";
}
::windows_core::imp::interface_hierarchy!(SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialCharacteristics(::windows_core::IUnknown);
impl SmartCardCryptogramMaterialCharacteristics {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardCryptogramMaterialCharacteristics, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MaterialName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaterialName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedAlgorithms(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowedAlgorithms)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedProofOfPossessionAlgorithms(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowedProofOfPossessionAlgorithms)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedValidations(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowedValidations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaterialType(&self) -> ::windows_core::Result<SmartCardCryptogramMaterialType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaterialType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProtectionMethod(&self) -> ::windows_core::Result<SmartCardCryptogramMaterialProtectionMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionMethod)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProtectionVersion(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaterialLength(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaterialLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramMaterialCharacteristics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramMaterialCharacteristics {}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialCharacteristics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialCharacteristics").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialCharacteristics {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramMaterialCharacteristics;{fc9ac5cc-c1d7-4153-923b-a2d43c6c8d49})");
}
impl ::core::clone::Clone for SmartCardCryptogramMaterialCharacteristics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramMaterialCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialCharacteristics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardCryptogramMaterialCharacteristics {
    const IID: ::windows_core::GUID = <ISmartCardCryptogramMaterialCharacteristics as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramMaterialCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialCharacteristics";
}
::windows_core::imp::interface_hierarchy!(SmartCardCryptogramMaterialCharacteristics, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardCryptogramMaterialCharacteristics {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramMaterialCharacteristics {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPackageCharacteristics(::windows_core::IUnknown);
impl SmartCardCryptogramMaterialPackageCharacteristics {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardCryptogramMaterialPackageCharacteristics, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PackageName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StorageKeyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StorageKeyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateImported(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DateImported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PackageFormat(&self) -> ::windows_core::Result<SmartCardCryptogramMaterialPackageFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramMaterialPackageCharacteristics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramMaterialPackageCharacteristics {}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialPackageCharacteristics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialPackageCharacteristics").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialPackageCharacteristics {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageCharacteristics;{ffb58e1f-0692-4c47-93cf-34d91f9dcd00})");
}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPackageCharacteristics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramMaterialPackageCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialPackageCharacteristics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardCryptogramMaterialPackageCharacteristics {
    const IID: ::windows_core::GUID = <ISmartCardCryptogramMaterialPackageCharacteristics as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramMaterialPackageCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageCharacteristics";
}
::windows_core::imp::interface_hierarchy!(SmartCardCryptogramMaterialPackageCharacteristics, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardCryptogramMaterialPackageCharacteristics {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramMaterialPackageCharacteristics {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPossessionProof(::windows_core::IUnknown);
impl SmartCardCryptogramMaterialPossessionProof {
    pub fn OperationStatus(&self) -> ::windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OperationStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Proof(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Proof)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramMaterialPossessionProof {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramMaterialPossessionProof {}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialPossessionProof {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialPossessionProof").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialPossessionProof {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPossessionProof;{e5b9ab8c-a141-4135-9add-b0d2e3aa1fc9})");
}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPossessionProof {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramMaterialPossessionProof {
    type Vtable = ISmartCardCryptogramMaterialPossessionProof_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardCryptogramMaterialPossessionProof {
    const IID: ::windows_core::GUID = <ISmartCardCryptogramMaterialPossessionProof as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramMaterialPossessionProof {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialPossessionProof";
}
::windows_core::imp::interface_hierarchy!(SmartCardCryptogramMaterialPossessionProof, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardCryptogramMaterialPossessionProof {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramMaterialPossessionProof {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramPlacementStep(::windows_core::IUnknown);
impl SmartCardCryptogramPlacementStep {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardCryptogramPlacementStep, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Algorithm(&self) -> ::windows_core::Result<SmartCardCryptogramAlgorithm> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Algorithm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlgorithm(&self, value: SmartCardCryptogramAlgorithm) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlgorithm)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SourceData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSourceData<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceData)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn CryptogramMaterialPackageName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CryptogramMaterialPackageName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCryptogramMaterialPackageName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCryptogramMaterialPackageName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CryptogramMaterialName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CryptogramMaterialName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCryptogramMaterialName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCryptogramMaterialName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TemplateOffset(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TemplateOffset)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTemplateOffset(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTemplateOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CryptogramOffset(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CryptogramOffset)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCryptogramOffset(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCryptogramOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CryptogramLength(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CryptogramLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCryptogramLength(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCryptogramLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CryptogramPlacementOptions(&self) -> ::windows_core::Result<SmartCardCryptogramPlacementOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CryptogramPlacementOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCryptogramPlacementOptions(&self, value: SmartCardCryptogramPlacementOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCryptogramPlacementOptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ChainedOutputStep(&self) -> ::windows_core::Result<SmartCardCryptogramPlacementStep> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChainedOutputStep)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChainedOutputStep<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SmartCardCryptogramPlacementStep>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChainedOutputStep)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramPlacementStep {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramPlacementStep {}
impl ::core::fmt::Debug for SmartCardCryptogramPlacementStep {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramPlacementStep").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramPlacementStep {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramPlacementStep;{947b03eb-8342-4792-a2e5-925636378a53})");
}
impl ::core::clone::Clone for SmartCardCryptogramPlacementStep {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramPlacementStep {
    type Vtable = ISmartCardCryptogramPlacementStep_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardCryptogramPlacementStep {
    const IID: ::windows_core::GUID = <ISmartCardCryptogramPlacementStep as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramPlacementStep {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramPlacementStep";
}
::windows_core::imp::interface_hierarchy!(SmartCardCryptogramPlacementStep, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardCryptogramPlacementStep {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramPlacementStep {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyCharacteristics(::windows_core::IUnknown);
impl SmartCardCryptogramStorageKeyCharacteristics {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardCryptogramStorageKeyCharacteristics, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn StorageKeyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StorageKeyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DateCreated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Algorithm(&self) -> ::windows_core::Result<SmartCardCryptogramStorageKeyAlgorithm> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Algorithm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows_core::Result<SmartCardCryptogramStorageKeyCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramStorageKeyCharacteristics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramStorageKeyCharacteristics {}
impl ::core::fmt::Debug for SmartCardCryptogramStorageKeyCharacteristics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramStorageKeyCharacteristics").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramStorageKeyCharacteristics {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyCharacteristics;{8552546e-4457-4825-b464-635471a39f5c})");
}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyCharacteristics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramStorageKeyCharacteristics {
    type Vtable = ISmartCardCryptogramStorageKeyCharacteristics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardCryptogramStorageKeyCharacteristics {
    const IID: ::windows_core::GUID = <ISmartCardCryptogramStorageKeyCharacteristics as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramStorageKeyCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyCharacteristics";
}
::windows_core::imp::interface_hierarchy!(SmartCardCryptogramStorageKeyCharacteristics, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardCryptogramStorageKeyCharacteristics {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramStorageKeyCharacteristics {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyInfo(::windows_core::IUnknown);
impl SmartCardCryptogramStorageKeyInfo {
    pub fn OperationStatus(&self) -> ::windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OperationStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
    #[cfg(feature = "Security_Cryptography_Core")]
    pub fn PublicKeyBlobType(&self) -> ::windows_core::Result<super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PublicKeyBlobType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PublicKey(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PublicKey)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AttestationStatus(&self) -> ::windows_core::Result<SmartCardCryptographicKeyAttestationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttestationStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Attestation(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attestation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AttestationCertificateChain(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttestationCertificateChain)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows_core::Result<SmartCardCryptogramStorageKeyCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OperationalRequirements(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardCryptogramStorageKeyInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OperationalRequirements)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramStorageKeyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramStorageKeyInfo {}
impl ::core::fmt::Debug for SmartCardCryptogramStorageKeyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramStorageKeyInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramStorageKeyInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyInfo;{77b0f00d-b097-4f61-a26a-9561639c9c3a})");
}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramStorageKeyInfo {
    type Vtable = ISmartCardCryptogramStorageKeyInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardCryptogramStorageKeyInfo {
    const IID: ::windows_core::GUID = <ISmartCardCryptogramStorageKeyInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramStorageKeyInfo {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyInfo";
}
::windows_core::imp::interface_hierarchy!(SmartCardCryptogramStorageKeyInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardCryptogramStorageKeyInfo {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramStorageKeyInfo {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardEmulator(::windows_core::IUnknown);
impl SmartCardEmulator {
    pub fn EnablementPolicy(&self) -> ::windows_core::Result<SmartCardEmulatorEnablementPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnablementPolicy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApduReceived<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorApduReceivedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApduReceived)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveApduReceived(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveApduReceived)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionDeactivated<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorConnectionDeactivatedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionDeactivated)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionDeactivated(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveConnectionDeactivated)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsHostCardEmulationSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHostCardEmulationSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardEmulator>> {
        Self::ISmartCardEmulatorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppletIdGroupRegistrationsAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCardAppletIdGroupRegistration>>> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAppletIdGroupRegistrationsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RegisterAppletIdGroupAsync<P0>(appletidgroup: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardAppletIdGroupRegistration>>
    where
        P0: ::windows_core::IntoParam<SmartCardAppletIdGroup>,
    {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisterAppletIdGroupAsync)(::windows_core::Interface::as_raw(this), appletidgroup.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnregisterAppletIdGroupAsync<P0>(registration: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<SmartCardAppletIdGroupRegistration>,
    {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnregisterAppletIdGroupAsync)(::windows_core::Interface::as_raw(this), registration.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn MaxAppletIdGroupRegistrations() -> ::windows_core::Result<u16> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxAppletIdGroupRegistrations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::ISmartCardEmulatorStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardEmulatorStatics<R, F: FnOnce(&ISmartCardEmulatorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISmartCardEmulatorStatics2<R, F: FnOnce(&ISmartCardEmulatorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISmartCardEmulatorStatics3<R, F: FnOnce(&ISmartCardEmulatorStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmartCardEmulator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardEmulator {}
impl ::core::fmt::Debug for SmartCardEmulator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulator").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardEmulator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulator;{dfb906b2-875e-47e5-8077-e8bff1b1c6fb})");
}
impl ::core::clone::Clone for SmartCardEmulator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardEmulator {
    type Vtable = ISmartCardEmulator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardEmulator {
    const IID: ::windows_core::GUID = <ISmartCardEmulator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardEmulator {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulator";
}
::windows_core::imp::interface_hierarchy!(SmartCardEmulator, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardEmulator {}
unsafe impl ::core::marker::Sync for SmartCardEmulator {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardEmulatorApduReceivedEventArgs(::windows_core::IUnknown);
impl SmartCardEmulatorApduReceivedEventArgs {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CommandApdu(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommandApdu)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConnectionProperties(&self) -> ::windows_core::Result<SmartCardEmulatorConnectionProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TryRespondAsync<P0>(&self, responseapdu: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRespondAsync)(::windows_core::Interface::as_raw(this), responseapdu.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn AutomaticResponseStatus(&self) -> ::windows_core::Result<SmartCardAutomaticResponseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutomaticResponseStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardEmulatorApduReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TryRespondWithStateAsync<P0, P1>(&self, responseapdu: P0, nextstate: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISmartCardEmulatorApduReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRespondWithStateAsync)(::windows_core::Interface::as_raw(this), responseapdu.try_into_param()?.abi(), nextstate.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn TryRespondWithCryptogramsAsync<P0, P1>(&self, responsetemplate: P0, cryptogramplacementsteps: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISmartCardEmulatorApduReceivedEventArgsWithCryptograms>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRespondWithCryptogramsAsync)(::windows_core::Interface::as_raw(this), responsetemplate.try_into_param()?.abi(), cryptogramplacementsteps.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn TryRespondWithCryptogramsAndStateAsync<P0, P1, P2>(&self, responsetemplate: P0, cryptogramplacementsteps: P1, nextstate: P2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>,
        P2: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISmartCardEmulatorApduReceivedEventArgsWithCryptograms>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRespondWithCryptogramsAndStateAsync)(::windows_core::Interface::as_raw(this), responsetemplate.try_into_param()?.abi(), cryptogramplacementsteps.try_into_param()?.abi(), nextstate.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardEmulatorApduReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardEmulatorApduReceivedEventArgs {}
impl ::core::fmt::Debug for SmartCardEmulatorApduReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorApduReceivedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardEmulatorApduReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulatorApduReceivedEventArgs;{d55d1576-69d2-5333-5b5f-f8c0d6e9f09f})");
}
impl ::core::clone::Clone for SmartCardEmulatorApduReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardEmulatorApduReceivedEventArgs {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardEmulatorApduReceivedEventArgs {
    const IID: ::windows_core::GUID = <ISmartCardEmulatorApduReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardEmulatorApduReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorApduReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SmartCardEmulatorApduReceivedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardEmulatorApduReceivedEventArgs {}
unsafe impl ::core::marker::Sync for SmartCardEmulatorApduReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionDeactivatedEventArgs(::windows_core::IUnknown);
impl SmartCardEmulatorConnectionDeactivatedEventArgs {
    pub fn ConnectionProperties(&self) -> ::windows_core::Result<SmartCardEmulatorConnectionProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Reason(&self) -> ::windows_core::Result<SmartCardEmulatorConnectionDeactivatedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardEmulatorConnectionDeactivatedEventArgs {}
impl ::core::fmt::Debug for SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorConnectionDeactivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardEmulatorConnectionDeactivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulatorConnectionDeactivatedEventArgs;{2186d8d3-c5eb-5262-43df-62a0a1b55557})");
}
impl ::core::clone::Clone for SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardEmulatorConnectionDeactivatedEventArgs {
    type Vtable = ISmartCardEmulatorConnectionDeactivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardEmulatorConnectionDeactivatedEventArgs {
    const IID: ::windows_core::GUID = <ISmartCardEmulatorConnectionDeactivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardEmulatorConnectionDeactivatedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorConnectionDeactivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SmartCardEmulatorConnectionDeactivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardEmulatorConnectionDeactivatedEventArgs {}
unsafe impl ::core::marker::Sync for SmartCardEmulatorConnectionDeactivatedEventArgs {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionProperties(::windows_core::IUnknown);
impl SmartCardEmulatorConnectionProperties {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Source(&self) -> ::windows_core::Result<SmartCardEmulatorConnectionSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardEmulatorConnectionProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardEmulatorConnectionProperties {}
impl ::core::fmt::Debug for SmartCardEmulatorConnectionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorConnectionProperties").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardEmulatorConnectionProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulatorConnectionProperties;{4e2ca5ee-f969-507d-6cf9-34e2d18df311})");
}
impl ::core::clone::Clone for SmartCardEmulatorConnectionProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardEmulatorConnectionProperties {
    type Vtable = ISmartCardEmulatorConnectionProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardEmulatorConnectionProperties {
    const IID: ::windows_core::GUID = <ISmartCardEmulatorConnectionProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardEmulatorConnectionProperties {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorConnectionProperties";
}
::windows_core::imp::interface_hierarchy!(SmartCardEmulatorConnectionProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardEmulatorConnectionProperties {}
unsafe impl ::core::marker::Sync for SmartCardEmulatorConnectionProperties {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardPinPolicy(::windows_core::IUnknown);
impl SmartCardPinPolicy {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardPinPolicy, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MinLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMinLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UppercaseLetters(&self) -> ::windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UppercaseLetters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUppercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUppercaseLetters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LowercaseLetters(&self) -> ::windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LowercaseLetters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLowercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLowercaseLetters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Digits(&self) -> ::windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Digits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDigits(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpecialCharacters(&self) -> ::windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpecialCharacters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSpecialCharacters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpecialCharacters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for SmartCardPinPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardPinPolicy {}
impl ::core::fmt::Debug for SmartCardPinPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardPinPolicy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardPinPolicy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardPinPolicy;{183ce184-4db6-4841-ac9e-2ac1f39b7304})");
}
impl ::core::clone::Clone for SmartCardPinPolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardPinPolicy {
    type Vtable = ISmartCardPinPolicy_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardPinPolicy {
    const IID: ::windows_core::GUID = <ISmartCardPinPolicy as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardPinPolicy {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinPolicy";
}
::windows_core::imp::interface_hierarchy!(SmartCardPinPolicy, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardPinPolicy {}
unsafe impl ::core::marker::Sync for SmartCardPinPolicy {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardPinResetDeferral(::windows_core::IUnknown);
impl SmartCardPinResetDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for SmartCardPinResetDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardPinResetDeferral {}
impl ::core::fmt::Debug for SmartCardPinResetDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardPinResetDeferral").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardPinResetDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardPinResetDeferral;{18c94aac-7805-4004-85e4-bbefac8f6884})");
}
impl ::core::clone::Clone for SmartCardPinResetDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardPinResetDeferral {
    type Vtable = ISmartCardPinResetDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardPinResetDeferral {
    const IID: ::windows_core::GUID = <ISmartCardPinResetDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardPinResetDeferral {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinResetDeferral";
}
::windows_core::imp::interface_hierarchy!(SmartCardPinResetDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardPinResetDeferral {}
unsafe impl ::core::marker::Sync for SmartCardPinResetDeferral {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardPinResetRequest(::windows_core::IUnknown);
impl SmartCardPinResetRequest {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Challenge(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Challenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<SmartCardPinResetDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetResponse<P0>(&self, response: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResponse)(::windows_core::Interface::as_raw(this), response.try_into_param()?.abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for SmartCardPinResetRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardPinResetRequest {}
impl ::core::fmt::Debug for SmartCardPinResetRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardPinResetRequest").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardPinResetRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardPinResetRequest;{12fe3c4d-5fb9-4e8e-9ff6-61f475124fef})");
}
impl ::core::clone::Clone for SmartCardPinResetRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardPinResetRequest {
    type Vtable = ISmartCardPinResetRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardPinResetRequest {
    const IID: ::windows_core::GUID = <ISmartCardPinResetRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardPinResetRequest {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinResetRequest";
}
::windows_core::imp::interface_hierarchy!(SmartCardPinResetRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardPinResetRequest {}
unsafe impl ::core::marker::Sync for SmartCardPinResetRequest {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardProvisioning(::windows_core::IUnknown);
impl SmartCardProvisioning {
    pub fn SmartCard(&self) -> ::windows_core::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmartCard)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIdAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIdAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetNameAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNameAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetChallengeContextAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardChallengeContext>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetChallengeContextAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPinChangeAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestPinChangeAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPinResetAsync<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<SmartCardPinResetHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestPinResetAsync)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAuthorityKeyContainerNameAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardProvisioning2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAuthorityKeyContainerNameAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromSmartCardAsync<P0>(card: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>
    where
        P0: ::windows_core::IntoParam<SmartCard>,
    {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromSmartCardAsync)(::windows_core::Interface::as_raw(this), card.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RequestVirtualSmartCardCreationAsync<P0, P1>(friendlyname: &::windows_core::HSTRING, administrativekey: P0, pinpolicy: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::IntoParam<SmartCardPinPolicy>,
    {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestVirtualSmartCardCreationAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(friendlyname), administrativekey.try_into_param()?.abi(), pinpolicy.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RequestVirtualSmartCardCreationAsyncWithCardId<P0, P1>(friendlyname: &::windows_core::HSTRING, administrativekey: P0, pinpolicy: P1, cardid: ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::IntoParam<SmartCardPinPolicy>,
    {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestVirtualSmartCardCreationAsyncWithCardId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(friendlyname), administrativekey.try_into_param()?.abi(), pinpolicy.into_param().abi(), cardid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestVirtualSmartCardDeletionAsync<P0>(card: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<SmartCard>,
    {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestVirtualSmartCardDeletionAsync)(::windows_core::Interface::as_raw(this), card.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RequestAttestedVirtualSmartCardCreationAsync<P0, P1>(friendlyname: &::windows_core::HSTRING, administrativekey: P0, pinpolicy: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::IntoParam<SmartCardPinPolicy>,
    {
        Self::ISmartCardProvisioningStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAttestedVirtualSmartCardCreationAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(friendlyname), administrativekey.try_into_param()?.abi(), pinpolicy.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RequestAttestedVirtualSmartCardCreationAsyncWithCardId<P0, P1>(friendlyname: &::windows_core::HSTRING, administrativekey: P0, pinpolicy: P1, cardid: ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::IntoParam<SmartCardPinPolicy>,
    {
        Self::ISmartCardProvisioningStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAttestedVirtualSmartCardCreationAsyncWithCardId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(friendlyname), administrativekey.try_into_param()?.abi(), pinpolicy.into_param().abi(), cardid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardProvisioningStatics<R, F: FnOnce(&ISmartCardProvisioningStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardProvisioning, ISmartCardProvisioningStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISmartCardProvisioningStatics2<R, F: FnOnce(&ISmartCardProvisioningStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardProvisioning, ISmartCardProvisioningStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmartCardProvisioning {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardProvisioning {}
impl ::core::fmt::Debug for SmartCardProvisioning {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardProvisioning").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardProvisioning {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardProvisioning;{19eeedbd-1fab-477c-b712-1a2c5af1fd6e})");
}
impl ::core::clone::Clone for SmartCardProvisioning {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardProvisioning {
    type Vtable = ISmartCardProvisioning_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardProvisioning {
    const IID: ::windows_core::GUID = <ISmartCardProvisioning as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardProvisioning {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardProvisioning";
}
::windows_core::imp::interface_hierarchy!(SmartCardProvisioning, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardProvisioning {}
unsafe impl ::core::marker::Sync for SmartCardProvisioning {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardReader(::windows_core::IUnknown);
impl SmartCardReader {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<SmartCardReaderKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetStatusAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardReaderStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStatusAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllCardsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCard>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllCardsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CardAdded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SmartCardReader, CardAddedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CardAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCardAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCardAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CardRemoved<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SmartCardReader, CardRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CardRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCardRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCardRemoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorWithKind(kind: SmartCardReaderKind) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorWithKind)(::windows_core::Interface::as_raw(this), kind, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardReader>> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardReaderStatics<R, F: FnOnce(&ISmartCardReaderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardReader, ISmartCardReaderStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmartCardReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardReader {}
impl ::core::fmt::Debug for SmartCardReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardReader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardReader;{1074b4e0-54c2-4df0-817a-14c14378f06c})");
}
impl ::core::clone::Clone for SmartCardReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardReader {
    type Vtable = ISmartCardReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardReader {
    const IID: ::windows_core::GUID = <ISmartCardReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardReader {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardReader";
}
::windows_core::imp::interface_hierarchy!(SmartCardReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardReader {}
unsafe impl ::core::marker::Sync for SmartCardReader {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardTriggerDetails(::windows_core::IUnknown);
impl SmartCardTriggerDetails {
    pub fn TriggerType(&self) -> ::windows_core::Result<SmartCardTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SourceAppletId(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceAppletId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn TriggerData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Emulator(&self) -> ::windows_core::Result<SmartCardEmulator> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Emulator)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryLaunchCurrentAppAsync(&self, arguments: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryLaunchCurrentAppAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(arguments), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryLaunchCurrentAppWithBehaviorAsync(&self, arguments: &::windows_core::HSTRING, behavior: SmartCardLaunchBehavior) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryLaunchCurrentAppWithBehaviorAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(arguments), behavior, &mut result__).from_abi(result__)
        }
    }
    pub fn SmartCard(&self) -> ::windows_core::Result<SmartCard> {
        let this = &::windows_core::ComInterface::cast::<ISmartCardTriggerDetails3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmartCard)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmartCardTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardTriggerDetails {}
impl ::core::fmt::Debug for SmartCardTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardTriggerDetails;{5f9bf11e-39ef-4f2b-b44f-0a9155b177bc})");
}
impl ::core::clone::Clone for SmartCardTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SmartCardTriggerDetails {
    type Vtable = ISmartCardTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardTriggerDetails {
    const IID: ::windows_core::GUID = <ISmartCardTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardTriggerDetails {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(SmartCardTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SmartCardTriggerDetails {}
unsafe impl ::core::marker::Sync for SmartCardTriggerDetails {}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardActivationPolicyChangeResult(pub i32);
impl SmartCardActivationPolicyChangeResult {
    pub const Denied: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardActivationPolicyChangeResult {}
impl ::core::clone::Clone for SmartCardActivationPolicyChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardActivationPolicyChangeResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardActivationPolicyChangeResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardActivationPolicyChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardActivationPolicyChangeResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardActivationPolicyChangeResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardActivationPolicyChangeResult;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardAppletIdGroupActivationPolicy(pub i32);
impl SmartCardAppletIdGroupActivationPolicy {
    pub const Disabled: Self = Self(0i32);
    pub const ForegroundOverride: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardAppletIdGroupActivationPolicy {}
impl ::core::clone::Clone for SmartCardAppletIdGroupActivationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardAppletIdGroupActivationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardAppletIdGroupActivationPolicy {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardAppletIdGroupActivationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardAppletIdGroupActivationPolicy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardAppletIdGroupActivationPolicy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardAppletIdGroupActivationPolicy;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardAutomaticResponseStatus(pub i32);
impl SmartCardAutomaticResponseStatus {
    pub const None: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const UnknownError: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardAutomaticResponseStatus {}
impl ::core::clone::Clone for SmartCardAutomaticResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardAutomaticResponseStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardAutomaticResponseStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardAutomaticResponseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardAutomaticResponseStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardAutomaticResponseStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardAutomaticResponseStatus;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardCryptogramAlgorithm(pub i32);
impl SmartCardCryptogramAlgorithm {
    pub const None: Self = Self(0i32);
    pub const CbcMac: Self = Self(1i32);
    pub const Cvc3Umd: Self = Self(2i32);
    pub const DecimalizedMsd: Self = Self(3i32);
    pub const Cvc3MD: Self = Self(4i32);
    pub const Sha1: Self = Self(5i32);
    pub const SignedDynamicApplicationData: Self = Self(6i32);
    pub const RsaPkcs1: Self = Self(7i32);
    pub const Sha256Hmac: Self = Self(8i32);
}
impl ::core::marker::Copy for SmartCardCryptogramAlgorithm {}
impl ::core::clone::Clone for SmartCardCryptogramAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardCryptogramAlgorithm {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardCryptogramAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramAlgorithm").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramAlgorithm {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramAlgorithm;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardCryptogramGeneratorOperationStatus(pub i32);
impl SmartCardCryptogramGeneratorOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const AuthorizationFailed: Self = Self(1i32);
    pub const AuthorizationCanceled: Self = Self(2i32);
    pub const AuthorizationRequired: Self = Self(3i32);
    pub const CryptogramMaterialPackageStorageKeyExists: Self = Self(4i32);
    pub const NoCryptogramMaterialPackageStorageKey: Self = Self(5i32);
    pub const NoCryptogramMaterialPackage: Self = Self(6i32);
    pub const UnsupportedCryptogramMaterialPackage: Self = Self(7i32);
    pub const UnknownCryptogramMaterialName: Self = Self(8i32);
    pub const InvalidCryptogramMaterialUsage: Self = Self(9i32);
    pub const ApduResponseNotSent: Self = Self(10i32);
    pub const OtherError: Self = Self(11i32);
    pub const ValidationFailed: Self = Self(12i32);
    pub const NotSupported: Self = Self(13i32);
}
impl ::core::marker::Copy for SmartCardCryptogramGeneratorOperationStatus {}
impl ::core::clone::Clone for SmartCardCryptogramGeneratorOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramGeneratorOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardCryptogramGeneratorOperationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardCryptogramGeneratorOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramGeneratorOperationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramGeneratorOperationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramGeneratorOperationStatus;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardCryptogramMaterialPackageConfirmationResponseFormat(pub i32);
impl SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    pub const None: Self = Self(0i32);
    pub const VisaHmac: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialPackageConfirmationResponseFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageConfirmationResponseFormat;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardCryptogramMaterialPackageFormat(pub i32);
impl SmartCardCryptogramMaterialPackageFormat {
    pub const None: Self = Self(0i32);
    pub const JweRsaPki: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialPackageFormat {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPackageFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramMaterialPackageFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardCryptogramMaterialPackageFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialPackageFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialPackageFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialPackageFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageFormat;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardCryptogramMaterialProtectionMethod(pub i32);
impl SmartCardCryptogramMaterialProtectionMethod {
    pub const None: Self = Self(0i32);
    pub const WhiteBoxing: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialProtectionMethod {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialProtectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramMaterialProtectionMethod {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardCryptogramMaterialProtectionMethod {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialProtectionMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialProtectionMethod").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialProtectionMethod {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialProtectionMethod;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardCryptogramMaterialType(pub i32);
impl SmartCardCryptogramMaterialType {
    pub const None: Self = Self(0i32);
    pub const StaticDataAuthentication: Self = Self(1i32);
    pub const TripleDes112: Self = Self(2i32);
    pub const Aes: Self = Self(3i32);
    pub const RsaPkcs1: Self = Self(4i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialType {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramMaterialType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardCryptogramMaterialType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialType;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardCryptogramPlacementOptions(pub u32);
impl SmartCardCryptogramPlacementOptions {
    pub const None: Self = Self(0u32);
    pub const UnitsAreInNibbles: Self = Self(1u32);
    pub const ChainOutput: Self = Self(2u32);
}
impl ::core::marker::Copy for SmartCardCryptogramPlacementOptions {}
impl ::core::clone::Clone for SmartCardCryptogramPlacementOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramPlacementOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardCryptogramPlacementOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardCryptogramPlacementOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramPlacementOptions").field(&self.0).finish()
    }
}
impl SmartCardCryptogramPlacementOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SmartCardCryptogramPlacementOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SmartCardCryptogramPlacementOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramPlacementOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramPlacementOptions;u4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardCryptogramStorageKeyAlgorithm(pub i32);
impl SmartCardCryptogramStorageKeyAlgorithm {
    pub const None: Self = Self(0i32);
    pub const Rsa2048: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramStorageKeyAlgorithm {}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramStorageKeyAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardCryptogramStorageKeyAlgorithm {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardCryptogramStorageKeyAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramStorageKeyAlgorithm").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramStorageKeyAlgorithm {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyAlgorithm;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardCryptogramStorageKeyCapabilities(pub u32);
impl SmartCardCryptogramStorageKeyCapabilities {
    pub const None: Self = Self(0u32);
    pub const HardwareProtection: Self = Self(1u32);
    pub const UnlockPrompt: Self = Self(2u32);
}
impl ::core::marker::Copy for SmartCardCryptogramStorageKeyCapabilities {}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramStorageKeyCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardCryptogramStorageKeyCapabilities {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardCryptogramStorageKeyCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramStorageKeyCapabilities").field(&self.0).finish()
    }
}
impl SmartCardCryptogramStorageKeyCapabilities {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SmartCardCryptogramStorageKeyCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SmartCardCryptogramStorageKeyCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptogramStorageKeyCapabilities {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyCapabilities;u4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardCryptographicKeyAttestationStatus(pub i32);
impl SmartCardCryptographicKeyAttestationStatus {
    pub const NoAttestation: Self = Self(0i32);
    pub const SoftwareKeyWithoutTpm: Self = Self(1i32);
    pub const SoftwareKeyWithTpm: Self = Self(2i32);
    pub const TpmKeyUnknownAttestationStatus: Self = Self(3i32);
    pub const TpmKeyWithoutAttestationCapability: Self = Self(4i32);
    pub const TpmKeyWithTemporaryAttestationFailure: Self = Self(5i32);
    pub const TpmKeyWithLongTermAttestationFailure: Self = Self(6i32);
    pub const TpmKeyWithAttestation: Self = Self(7i32);
}
impl ::core::marker::Copy for SmartCardCryptographicKeyAttestationStatus {}
impl ::core::clone::Clone for SmartCardCryptographicKeyAttestationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptographicKeyAttestationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardCryptographicKeyAttestationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardCryptographicKeyAttestationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptographicKeyAttestationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardCryptographicKeyAttestationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptographicKeyAttestationStatus;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardEmulationCategory(pub i32);
impl SmartCardEmulationCategory {
    pub const Other: Self = Self(0i32);
    pub const Payment: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulationCategory {}
impl ::core::clone::Clone for SmartCardEmulationCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardEmulationCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardEmulationCategory {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardEmulationCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulationCategory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardEmulationCategory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulationCategory;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardEmulationType(pub i32);
impl SmartCardEmulationType {
    pub const Host: Self = Self(0i32);
    pub const Uicc: Self = Self(1i32);
    pub const EmbeddedSE: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardEmulationType {}
impl ::core::clone::Clone for SmartCardEmulationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardEmulationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardEmulationType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardEmulationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulationType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardEmulationType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulationType;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardEmulatorConnectionDeactivatedReason(pub i32);
impl SmartCardEmulatorConnectionDeactivatedReason {
    pub const ConnectionLost: Self = Self(0i32);
    pub const ConnectionRedirected: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulatorConnectionDeactivatedReason {}
impl ::core::clone::Clone for SmartCardEmulatorConnectionDeactivatedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardEmulatorConnectionDeactivatedReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardEmulatorConnectionDeactivatedReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardEmulatorConnectionDeactivatedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorConnectionDeactivatedReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardEmulatorConnectionDeactivatedReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorConnectionDeactivatedReason;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardEmulatorConnectionSource(pub i32);
impl SmartCardEmulatorConnectionSource {
    pub const Unknown: Self = Self(0i32);
    pub const NfcReader: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulatorConnectionSource {}
impl ::core::clone::Clone for SmartCardEmulatorConnectionSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardEmulatorConnectionSource {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardEmulatorConnectionSource {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardEmulatorConnectionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorConnectionSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardEmulatorConnectionSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorConnectionSource;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardEmulatorEnablementPolicy(pub i32);
impl SmartCardEmulatorEnablementPolicy {
    pub const Never: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const ScreenOn: Self = Self(2i32);
    pub const ScreenUnlocked: Self = Self(3i32);
}
impl ::core::marker::Copy for SmartCardEmulatorEnablementPolicy {}
impl ::core::clone::Clone for SmartCardEmulatorEnablementPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardEmulatorEnablementPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardEmulatorEnablementPolicy {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardEmulatorEnablementPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorEnablementPolicy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardEmulatorEnablementPolicy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorEnablementPolicy;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardLaunchBehavior(pub i32);
impl SmartCardLaunchBehavior {
    pub const Default: Self = Self(0i32);
    pub const AboveLock: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardLaunchBehavior {}
impl ::core::clone::Clone for SmartCardLaunchBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardLaunchBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardLaunchBehavior {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardLaunchBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardLaunchBehavior").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardLaunchBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardLaunchBehavior;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardPinCharacterPolicyOption(pub i32);
impl SmartCardPinCharacterPolicyOption {
    pub const Allow: Self = Self(0i32);
    pub const RequireAtLeastOne: Self = Self(1i32);
    pub const Disallow: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardPinCharacterPolicyOption {}
impl ::core::clone::Clone for SmartCardPinCharacterPolicyOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardPinCharacterPolicyOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardPinCharacterPolicyOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardPinCharacterPolicyOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardPinCharacterPolicyOption").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardPinCharacterPolicyOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardPinCharacterPolicyOption;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardReaderKind(pub i32);
impl SmartCardReaderKind {
    pub const Any: Self = Self(0i32);
    pub const Generic: Self = Self(1i32);
    pub const Tpm: Self = Self(2i32);
    pub const Nfc: Self = Self(3i32);
    pub const Uicc: Self = Self(4i32);
    pub const EmbeddedSE: Self = Self(5i32);
}
impl ::core::marker::Copy for SmartCardReaderKind {}
impl ::core::clone::Clone for SmartCardReaderKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardReaderKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardReaderKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardReaderKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardReaderKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardReaderKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardReaderKind;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardReaderStatus(pub i32);
impl SmartCardReaderStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Exclusive: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardReaderStatus {}
impl ::core::clone::Clone for SmartCardReaderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardReaderStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardReaderStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardReaderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardReaderStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardReaderStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardReaderStatus;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardStatus(pub i32);
impl SmartCardStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Shared: Self = Self(2i32);
    pub const Exclusive: Self = Self(3i32);
    pub const Unresponsive: Self = Self(4i32);
}
impl ::core::marker::Copy for SmartCardStatus {}
impl ::core::clone::Clone for SmartCardStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardStatus;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardTriggerType(pub i32);
impl SmartCardTriggerType {
    pub const EmulatorTransaction: Self = Self(0i32);
    pub const EmulatorNearFieldEntry: Self = Self(1i32);
    pub const EmulatorNearFieldExit: Self = Self(2i32);
    pub const EmulatorHostApplicationActivated: Self = Self(3i32);
    pub const EmulatorAppletIdGroupRegistrationChanged: Self = Self(4i32);
    pub const ReaderCardAdded: Self = Self(5i32);
}
impl ::core::marker::Copy for SmartCardTriggerType {}
impl ::core::clone::Clone for SmartCardTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardTriggerType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardTriggerType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardTriggerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardTriggerType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardTriggerType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardTriggerType;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmartCardUnlockPromptingBehavior(pub i32);
impl SmartCardUnlockPromptingBehavior {
    pub const AllowUnlockPrompt: Self = Self(0i32);
    pub const RequireUnlockPrompt: Self = Self(1i32);
    pub const PreventUnlockPrompt: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardUnlockPromptingBehavior {}
impl ::core::clone::Clone for SmartCardUnlockPromptingBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardUnlockPromptingBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SmartCardUnlockPromptingBehavior {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SmartCardUnlockPromptingBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardUnlockPromptingBehavior").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SmartCardUnlockPromptingBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardUnlockPromptingBehavior;i4)");
}
#[doc = "*Required features: `\"Devices_SmartCards\"`*"]
#[repr(transparent)]
pub struct SmartCardPinResetHandler(pub ::windows_core::IUnknown);
impl SmartCardPinResetHandler {
    pub fn new<F: FnMut(::core::option::Option<&SmartCardProvisioning>, ::core::option::Option<&SmartCardPinResetRequest>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SmartCardPinResetHandlerBox::<F> { vtable: &SmartCardPinResetHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, request: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SmartCardProvisioning>,
        P1: ::windows_core::IntoParam<SmartCardPinResetRequest>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), request.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct SmartCardPinResetHandlerBox<F: FnMut(::core::option::Option<&SmartCardProvisioning>, ::core::option::Option<&SmartCardPinResetRequest>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SmartCardPinResetHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&SmartCardProvisioning>, ::core::option::Option<&SmartCardPinResetRequest>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> SmartCardPinResetHandlerBox<F> {
    const VTABLE: SmartCardPinResetHandler_Vtbl = SmartCardPinResetHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<SmartCardPinResetHandler as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&request)).into()
    }
}
impl ::core::cmp::PartialEq for SmartCardPinResetHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardPinResetHandler {}
impl ::core::fmt::Debug for SmartCardPinResetHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardPinResetHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for SmartCardPinResetHandler {
    type Vtable = SmartCardPinResetHandler_Vtbl;
}
impl ::core::clone::Clone for SmartCardPinResetHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for SmartCardPinResetHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x138d5e40_f3bc_4a5c_b41d_4b4ef684e237);
}
impl ::windows_core::RuntimeType for SmartCardPinResetHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{138d5e40-f3bc-4a5c-b41d-4b4ef684e237}");
}
#[repr(C)]
#[doc(hidden)]
pub struct SmartCardPinResetHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
