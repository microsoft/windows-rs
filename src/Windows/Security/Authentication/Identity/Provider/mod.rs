#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthentication(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorAuthentication {
    type Vtable = ISecondaryAuthenticationFactorAuthentication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x020a16e5_6a25_40a3_8c00_50a023f619d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthentication_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicehmac: ::windows::runtime::RawPtr, sessionhmac: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, errorlogmessage: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorAuthenticationResult {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9cbb5987_ef6d_4bc2_bf49_4617515a0f9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SecondaryAuthenticationFactorAuthenticationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd4a5ee56_7291_4073_bc1f_ccb8f5afdf96);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorAuthenticationStageInfo {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x56fec28b_e8aa_4c0f_8e4c_a559e73add88);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SecondaryAuthenticationFactorAuthenticationStage) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SecondaryAuthenticationFactorAuthenticationScenario) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorAuthenticationStatics {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3f582656_28f8_4e0f_ae8c_5898b9ae2469);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, message: SecondaryAuthenticationFactorAuthenticationMessage, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, serviceauthenticationnonce: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics {
    type Vtable = ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x90499a19_7ef2_4523_951c_a417a24acf93);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, deviceinstancepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        deviceinstancepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode,
        devicefriendlyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        devicemodelnumber: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        deviceconfigurationdata: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorInfo {
    type Vtable = ISecondaryAuthenticationFactorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1e2ba861_8533_4fce_839b_ecb72410ac14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorInfo2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorInfo2 {
    type Vtable = ISecondaryAuthenticationFactorInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x14d981a3_fc26_4ff7_abc3_48e82a512a0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presencestate: SecondaryAuthenticationFactorDevicePresence, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistration(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorRegistration {
    type Vtable = ISecondaryAuthenticationFactorRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f4cbbb4_8cba_48b0_840d_dbb22a54c678);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceconfigurationdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, errorlogmessage: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistrationResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorRegistrationResult {
    type Vtable = ISecondaryAuthenticationFactorRegistrationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa4fe35f0_ade3_4981_af6b_ec195921682a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistrationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SecondaryAuthenticationFactorRegistrationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistrationStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryAuthenticationFactorRegistrationStatics {
    type Vtable = ISecondaryAuthenticationFactorRegistrationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1adf0f65_e3b7_4155_997f_b756ef65beba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistrationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        capabilities: SecondaryAuthenticationFactorDeviceCapabilities,
        devicefriendlyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        devicemodelnumber: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        devicekey: ::windows::runtime::RawPtr,
        mutualauthenticationkey: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, querytype: SecondaryAuthenticationFactorDeviceFindScope, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, deviceconfigurationdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SecondaryAuthenticationFactorAuthentication(pub ::windows::runtime::IInspectable);
impl SecondaryAuthenticationFactorAuthentication {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Storage_Streams`*"]
    pub fn ServiceAuthenticationHmac(&self) -> ::windows::runtime::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Storage_Streams`*"]
    pub fn SessionNonce(&self) -> ::windows::runtime::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Storage_Streams`*"]
    pub fn DeviceNonce(&self) -> ::windows::runtime::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Storage_Streams`*"]
    pub fn DeviceConfigurationData(&self) -> ::windows::runtime::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`, `Storage_Streams`*"]
    pub fn FinishAuthenticationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(&self, devicehmac: Param0, sessionhmac: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), devicehmac.into_param().abi(), sessionhmac.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`*"]
    pub fn AbortAuthenticationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, errorlogmessage: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), errorlogmessage.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`*"]
    pub fn ShowNotificationMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(devicename: Param0, message: SecondaryAuthenticationFactorAuthenticationMessage) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), devicename.into_param().abi(), message, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`, `Storage_Streams`*"]
    pub fn StartAuthenticationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(deviceid: Param0, serviceauthenticationnonce: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), serviceauthenticationnonce.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`*"]
    pub fn AuthenticationStageChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`*"]
    pub fn RemoveAuthenticationStageChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`*"]
    pub fn GetAuthenticationStageInfoAsync() -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>>(result__)
        })
    }
    pub fn ISecondaryAuthenticationFactorAuthenticationStatics<R, F: FnOnce(&ISecondaryAuthenticationFactorAuthenticationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SecondaryAuthenticationFactorAuthentication, ISecondaryAuthenticationFactorAuthenticationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorAuthentication {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication;{020a16e5-6a25-40a3-8c00-50a023f619d1})");
}
unsafe impl ::windows::runtime::Interface for SecondaryAuthenticationFactorAuthentication {
    type Vtable = ISecondaryAuthenticationFactorAuthentication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x020a16e5_6a25_40a3_8c00_50a023f619d1);
}
impl ::windows::runtime::RuntimeName for SecondaryAuthenticationFactorAuthentication {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication";
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthentication> for ::windows::runtime::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthentication) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthentication> for ::windows::runtime::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthentication) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthentication> for ::windows::runtime::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthentication) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthentication> for ::windows::runtime::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthentication) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthentication {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthentication {}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationMessage(pub i32);
impl SecondaryAuthenticationFactorAuthenticationMessage {
    pub const Invalid: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(0i32);
    pub const SwipeUpWelcome: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(1i32);
    pub const TapWelcome: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(2i32);
    pub const DeviceNeedsAttention: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(3i32);
    pub const LookingForDevice: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(4i32);
    pub const LookingForDevicePluggedin: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(5i32);
    pub const BluetoothIsDisabled: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(6i32);
    pub const NfcIsDisabled: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(7i32);
    pub const WiFiIsDisabled: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(8i32);
    pub const ExtraTapIsRequired: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(9i32);
    pub const DisabledByPolicy: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(10i32);
    pub const TapOnDeviceRequired: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(11i32);
    pub const HoldFinger: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(12i32);
    pub const ScanFinger: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(13i32);
    pub const UnauthorizedUser: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(14i32);
    pub const ReregisterRequired: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(15i32);
    pub const TryAgain: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(16i32);
    pub const SayPassphrase: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(17i32);
    pub const ReadyToSignIn: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(18i32);
    pub const UseAnotherSignInOption: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(19i32);
    pub const ConnectionRequired: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(20i32);
    pub const TimeLimitExceeded: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(21i32);
    pub const CanceledByUser: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(22i32);
    pub const CenterHand: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(23i32);
    pub const MoveHandCloser: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(24i32);
    pub const MoveHandFarther: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(25i32);
    pub const PlaceHandAbove: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(26i32);
    pub const RecognitionFailed: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(27i32);
    pub const DeviceUnavailable: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(28i32);
}
impl ::core::convert::From<i32> for SecondaryAuthenticationFactorAuthenticationMessage {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondaryAuthenticationFactorAuthenticationMessage {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorAuthenticationMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationMessage;i4)");
}
impl ::windows::runtime::DefaultType for SecondaryAuthenticationFactorAuthenticationMessage {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SecondaryAuthenticationFactorAuthenticationResult(pub ::windows::runtime::IInspectable);
impl SecondaryAuthenticationFactorAuthenticationResult {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<SecondaryAuthenticationFactorAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorAuthenticationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationStatus>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn Authentication(&self) -> ::windows::runtime::Result<SecondaryAuthenticationFactorAuthentication> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthentication>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorAuthenticationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult;{9cbb5987-ef6d-4bc2-bf49-4617515a0f9a})");
}
unsafe impl ::windows::runtime::Interface for SecondaryAuthenticationFactorAuthenticationResult {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9cbb5987_ef6d_4bc2_bf49_4617515a0f9a);
}
impl ::windows::runtime::RuntimeName for SecondaryAuthenticationFactorAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult";
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationResult> for ::windows::runtime::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationResult> for ::windows::runtime::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationResult> for ::windows::runtime::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationResult> for ::windows::runtime::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthenticationResult {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthenticationResult {}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationScenario(pub i32);
impl SecondaryAuthenticationFactorAuthenticationScenario {
    pub const SignIn: SecondaryAuthenticationFactorAuthenticationScenario = SecondaryAuthenticationFactorAuthenticationScenario(0i32);
    pub const CredentialPrompt: SecondaryAuthenticationFactorAuthenticationScenario = SecondaryAuthenticationFactorAuthenticationScenario(1i32);
}
impl ::core::convert::From<i32> for SecondaryAuthenticationFactorAuthenticationScenario {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondaryAuthenticationFactorAuthenticationScenario {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorAuthenticationScenario {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationScenario;i4)");
}
impl ::windows::runtime::DefaultType for SecondaryAuthenticationFactorAuthenticationScenario {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStage(pub i32);
impl SecondaryAuthenticationFactorAuthenticationStage {
    pub const NotStarted: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(0i32);
    pub const WaitingForUserConfirmation: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(1i32);
    pub const CollectingCredential: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(2i32);
    pub const SuspendingAuthentication: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(3i32);
    pub const CredentialCollected: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(4i32);
    pub const CredentialAuthenticated: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(5i32);
    pub const StoppingAuthentication: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(6i32);
    pub const ReadyForLock: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(7i32);
    pub const CheckingDevicePresence: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(8i32);
}
impl ::core::convert::From<i32> for SecondaryAuthenticationFactorAuthenticationStage {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondaryAuthenticationFactorAuthenticationStage {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorAuthenticationStage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStage;i4)");
}
impl ::windows::runtime::DefaultType for SecondaryAuthenticationFactorAuthenticationStage {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(pub ::windows::runtime::IInspectable);
impl SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn StageInfo(&self) -> ::windows::runtime::Result<SecondaryAuthenticationFactorAuthenticationStageInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationStageInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs;{d4a5ee56-7291-4073-bc1f-ccb8f5afdf96})");
}
unsafe impl ::windows::runtime::Interface for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd4a5ee56_7291_4073_bc1f_ccb8f5afdf96);
}
impl ::windows::runtime::RuntimeName for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs";
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SecondaryAuthenticationFactorAuthenticationStageInfo(pub ::windows::runtime::IInspectable);
impl SecondaryAuthenticationFactorAuthenticationStageInfo {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn Stage(&self) -> ::windows::runtime::Result<SecondaryAuthenticationFactorAuthenticationStage> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorAuthenticationStage = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationStage>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn Scenario(&self) -> ::windows::runtime::Result<SecondaryAuthenticationFactorAuthenticationScenario> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorAuthenticationScenario = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationScenario>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorAuthenticationStageInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo;{56fec28b-e8aa-4c0f-8e4c-a559e73add88})");
}
unsafe impl ::windows::runtime::Interface for SecondaryAuthenticationFactorAuthenticationStageInfo {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x56fec28b_e8aa_4c0f_8e4c_a559e73add88);
}
impl ::windows::runtime::RuntimeName for SecondaryAuthenticationFactorAuthenticationStageInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo";
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::runtime::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::runtime::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::runtime::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::runtime::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthenticationStageInfo {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthenticationStageInfo {}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStatus(pub i32);
impl SecondaryAuthenticationFactorAuthenticationStatus {
    pub const Failed: SecondaryAuthenticationFactorAuthenticationStatus = SecondaryAuthenticationFactorAuthenticationStatus(0i32);
    pub const Started: SecondaryAuthenticationFactorAuthenticationStatus = SecondaryAuthenticationFactorAuthenticationStatus(1i32);
    pub const UnknownDevice: SecondaryAuthenticationFactorAuthenticationStatus = SecondaryAuthenticationFactorAuthenticationStatus(2i32);
    pub const DisabledByPolicy: SecondaryAuthenticationFactorAuthenticationStatus = SecondaryAuthenticationFactorAuthenticationStatus(3i32);
    pub const InvalidAuthenticationStage: SecondaryAuthenticationFactorAuthenticationStatus = SecondaryAuthenticationFactorAuthenticationStatus(4i32);
}
impl ::core::convert::From<i32> for SecondaryAuthenticationFactorAuthenticationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondaryAuthenticationFactorAuthenticationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorAuthenticationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStatus;i4)");
}
impl ::windows::runtime::DefaultType for SecondaryAuthenticationFactorAuthenticationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDeviceCapabilities(pub u32);
impl SecondaryAuthenticationFactorDeviceCapabilities {
    pub const None: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(0u32);
    pub const SecureStorage: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(1u32);
    pub const StoreKeys: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(2u32);
    pub const ConfirmUserIntentToAuthenticate: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(4u32);
    pub const SupportSecureUserPresenceCheck: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(8u32);
    pub const TransmittedDataIsEncrypted: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(16u32);
    pub const HMacSha256: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(32u32);
    pub const CloseRangeDataTransmission: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(64u32);
}
impl ::core::convert::From<u32> for SecondaryAuthenticationFactorDeviceCapabilities {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondaryAuthenticationFactorDeviceCapabilities {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorDeviceCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDeviceCapabilities;u4)");
}
impl ::windows::runtime::DefaultType for SecondaryAuthenticationFactorDeviceCapabilities {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for SecondaryAuthenticationFactorDeviceCapabilities {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for SecondaryAuthenticationFactorDeviceCapabilities {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for SecondaryAuthenticationFactorDeviceCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for SecondaryAuthenticationFactorDeviceCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for SecondaryAuthenticationFactorDeviceCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDeviceFindScope(pub i32);
impl SecondaryAuthenticationFactorDeviceFindScope {
    pub const User: SecondaryAuthenticationFactorDeviceFindScope = SecondaryAuthenticationFactorDeviceFindScope(0i32);
    pub const AllUsers: SecondaryAuthenticationFactorDeviceFindScope = SecondaryAuthenticationFactorDeviceFindScope(1i32);
}
impl ::core::convert::From<i32> for SecondaryAuthenticationFactorDeviceFindScope {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondaryAuthenticationFactorDeviceFindScope {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorDeviceFindScope {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDeviceFindScope;i4)");
}
impl ::windows::runtime::DefaultType for SecondaryAuthenticationFactorDeviceFindScope {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresence(pub i32);
impl SecondaryAuthenticationFactorDevicePresence {
    pub const Absent: SecondaryAuthenticationFactorDevicePresence = SecondaryAuthenticationFactorDevicePresence(0i32);
    pub const Present: SecondaryAuthenticationFactorDevicePresence = SecondaryAuthenticationFactorDevicePresence(1i32);
}
impl ::core::convert::From<i32> for SecondaryAuthenticationFactorDevicePresence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondaryAuthenticationFactorDevicePresence {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorDevicePresence {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresence;i4)");
}
impl ::windows::runtime::DefaultType for SecondaryAuthenticationFactorDevicePresence {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringMode(pub i32);
impl SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    pub const Unsupported: SecondaryAuthenticationFactorDevicePresenceMonitoringMode = SecondaryAuthenticationFactorDevicePresenceMonitoringMode(0i32);
    pub const AppManaged: SecondaryAuthenticationFactorDevicePresenceMonitoringMode = SecondaryAuthenticationFactorDevicePresenceMonitoringMode(1i32);
    pub const SystemManaged: SecondaryAuthenticationFactorDevicePresenceMonitoringMode = SecondaryAuthenticationFactorDevicePresenceMonitoringMode(2i32);
}
impl ::core::convert::From<i32> for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringMode;i4)");
}
impl ::windows::runtime::DefaultType for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(pub i32);
impl SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    pub const Unsupported: SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus = SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(0i32);
    pub const Succeeded: SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus = SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(1i32);
    pub const DisabledByPolicy: SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus = SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(2i32);
}
impl ::core::convert::From<i32> for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus;i4)");
}
impl ::windows::runtime::DefaultType for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorFinishAuthenticationStatus(pub i32);
impl SecondaryAuthenticationFactorFinishAuthenticationStatus {
    pub const Failed: SecondaryAuthenticationFactorFinishAuthenticationStatus = SecondaryAuthenticationFactorFinishAuthenticationStatus(0i32);
    pub const Completed: SecondaryAuthenticationFactorFinishAuthenticationStatus = SecondaryAuthenticationFactorFinishAuthenticationStatus(1i32);
    pub const NonceExpired: SecondaryAuthenticationFactorFinishAuthenticationStatus = SecondaryAuthenticationFactorFinishAuthenticationStatus(2i32);
}
impl ::core::convert::From<i32> for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorFinishAuthenticationStatus;i4)");
}
impl ::windows::runtime::DefaultType for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SecondaryAuthenticationFactorInfo(pub ::windows::runtime::IInspectable);
impl SecondaryAuthenticationFactorInfo {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn DeviceFriendlyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn DeviceModelNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Storage_Streams`*"]
    pub fn DeviceConfigurationData(&self) -> ::windows::runtime::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn PresenceMonitoringMode(&self) -> ::windows::runtime::Result<SecondaryAuthenticationFactorDevicePresenceMonitoringMode> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryAuthenticationFactorInfo2>(self)?;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorDevicePresenceMonitoringMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorDevicePresenceMonitoringMode>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`*"]
    pub fn UpdateDevicePresenceAsync(&self, presencestate: SecondaryAuthenticationFactorDevicePresence) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryAuthenticationFactorInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), presencestate, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn IsAuthenticationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryAuthenticationFactorInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo;{1e2ba861-8533-4fce-839b-ecb72410ac14})");
}
unsafe impl ::windows::runtime::Interface for SecondaryAuthenticationFactorInfo {
    type Vtable = ISecondaryAuthenticationFactorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1e2ba861_8533_4fce_839b_ecb72410ac14);
}
impl ::windows::runtime::RuntimeName for SecondaryAuthenticationFactorInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo";
}
impl ::core::convert::From<SecondaryAuthenticationFactorInfo> for ::windows::runtime::IUnknown {
    fn from(value: SecondaryAuthenticationFactorInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorInfo> for ::windows::runtime::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorInfo> for ::windows::runtime::IInspectable {
    fn from(value: SecondaryAuthenticationFactorInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorInfo> for ::windows::runtime::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorInfo {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorInfo {}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SecondaryAuthenticationFactorRegistration(pub ::windows::runtime::IInspectable);
impl SecondaryAuthenticationFactorRegistration {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`, `Storage_Streams`*"]
    pub fn FinishRegisteringDeviceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(&self, deviceconfigurationdata: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceconfigurationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`*"]
    pub fn AbortRegisteringDeviceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, errorlogmessage: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), errorlogmessage.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`, `Storage_Streams`*"]
    pub fn RequestStartRegisteringDeviceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(
        deviceid: Param0,
        capabilities: SecondaryAuthenticationFactorDeviceCapabilities,
        devicefriendlyname: Param2,
        devicemodelnumber: Param3,
        devicekey: Param4,
        mutualauthenticationkey: Param5,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), capabilities, devicefriendlyname.into_param().abi(), devicemodelnumber.into_param().abi(), devicekey.into_param().abi(), mutualauthenticationkey.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllRegisteredDeviceInfoAsync(querytype: SecondaryAuthenticationFactorDeviceFindScope) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<SecondaryAuthenticationFactorInfo>>> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), querytype, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<SecondaryAuthenticationFactorInfo>>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`*"]
    pub fn UnregisterDeviceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`, `Storage_Streams`*"]
    pub fn UpdateDeviceConfigurationDataAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(deviceid: Param0, deviceconfigurationdata: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), deviceconfigurationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`*"]
    pub fn RegisterDevicePresenceMonitoringAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0, deviceinstancepath: Param1, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), deviceinstancepath.into_param().abi(), monitoringmode, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`, `Storage_Streams`*"]
    pub fn RegisterDevicePresenceMonitoringWithNewDeviceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(
        deviceid: Param0,
        deviceinstancepath: Param1,
        monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode,
        devicefriendlyname: Param3,
        devicemodelnumber: Param4,
        deviceconfigurationdata: Param5,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), deviceinstancepath.into_param().abi(), monitoringmode, devicefriendlyname.into_param().abi(), devicemodelnumber.into_param().abi(), deviceconfigurationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`, `Foundation`*"]
    pub fn UnregisterDevicePresenceMonitoringAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn IsDevicePresenceMonitoringSupported() -> ::windows::runtime::Result<bool> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ISecondaryAuthenticationFactorRegistrationStatics<R, F: FnOnce(&ISecondaryAuthenticationFactorRegistrationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SecondaryAuthenticationFactorRegistration, ISecondaryAuthenticationFactorRegistrationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics<R, F: FnOnce(&ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SecondaryAuthenticationFactorRegistration, ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorRegistration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration;{9f4cbbb4-8cba-48b0-840d-dbb22a54c678})");
}
unsafe impl ::windows::runtime::Interface for SecondaryAuthenticationFactorRegistration {
    type Vtable = ISecondaryAuthenticationFactorRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f4cbbb4_8cba_48b0_840d_dbb22a54c678);
}
impl ::windows::runtime::RuntimeName for SecondaryAuthenticationFactorRegistration {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration";
}
impl ::core::convert::From<SecondaryAuthenticationFactorRegistration> for ::windows::runtime::IUnknown {
    fn from(value: SecondaryAuthenticationFactorRegistration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistration> for ::windows::runtime::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorRegistration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorRegistration> for ::windows::runtime::IInspectable {
    fn from(value: SecondaryAuthenticationFactorRegistration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistration> for ::windows::runtime::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorRegistration {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorRegistration {}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SecondaryAuthenticationFactorRegistrationResult(pub ::windows::runtime::IInspectable);
impl SecondaryAuthenticationFactorRegistrationResult {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<SecondaryAuthenticationFactorRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorRegistrationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorRegistrationStatus>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
    pub fn Registration(&self) -> ::windows::runtime::Result<SecondaryAuthenticationFactorRegistration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorRegistration>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorRegistrationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult;{a4fe35f0-ade3-4981-af6b-ec195921682a})");
}
unsafe impl ::windows::runtime::Interface for SecondaryAuthenticationFactorRegistrationResult {
    type Vtable = ISecondaryAuthenticationFactorRegistrationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa4fe35f0_ade3_4981_af6b_ec195921682a);
}
impl ::windows::runtime::RuntimeName for SecondaryAuthenticationFactorRegistrationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult";
}
impl ::core::convert::From<SecondaryAuthenticationFactorRegistrationResult> for ::windows::runtime::IUnknown {
    fn from(value: SecondaryAuthenticationFactorRegistrationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistrationResult> for ::windows::runtime::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorRegistrationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorRegistrationResult> for ::windows::runtime::IInspectable {
    fn from(value: SecondaryAuthenticationFactorRegistrationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistrationResult> for ::windows::runtime::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorRegistrationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorRegistrationResult {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorRegistrationResult {}
#[doc = "*Required features: `Security_Authentication_Identity_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistrationStatus(pub i32);
impl SecondaryAuthenticationFactorRegistrationStatus {
    pub const Failed: SecondaryAuthenticationFactorRegistrationStatus = SecondaryAuthenticationFactorRegistrationStatus(0i32);
    pub const Started: SecondaryAuthenticationFactorRegistrationStatus = SecondaryAuthenticationFactorRegistrationStatus(1i32);
    pub const CanceledByUser: SecondaryAuthenticationFactorRegistrationStatus = SecondaryAuthenticationFactorRegistrationStatus(2i32);
    pub const PinSetupRequired: SecondaryAuthenticationFactorRegistrationStatus = SecondaryAuthenticationFactorRegistrationStatus(3i32);
    pub const DisabledByPolicy: SecondaryAuthenticationFactorRegistrationStatus = SecondaryAuthenticationFactorRegistrationStatus(4i32);
}
impl ::core::convert::From<i32> for SecondaryAuthenticationFactorRegistrationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecondaryAuthenticationFactorRegistrationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryAuthenticationFactorRegistrationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationStatus;i4)");
}
impl ::windows::runtime::DefaultType for SecondaryAuthenticationFactorRegistrationStatus {
    type DefaultType = Self;
}
