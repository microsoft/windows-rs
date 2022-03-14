#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthentication(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthentication {
    type Vtable = ISecondaryAuthenticationFactorAuthentication_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x020a16e5_6a25_40a3_8c00_50a023f619d1);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthentication_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub ServiceAuthenticationHmac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    ServiceAuthenticationHmac: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SessionNonce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SessionNonce: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub DeviceNonce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    DeviceNonce: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub DeviceConfigurationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    DeviceConfigurationData: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub FinishAuthenticationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicehmac: ::windows::core::RawPtr, sessionhmac: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    FinishAuthenticationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AbortAuthenticationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorlogmessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AbortAuthenticationAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationResult(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthenticationResult {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cbb5987_ef6d_4bc2_bf49_4617515a0f9a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub Authentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Authentication: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4a5ee56_7291_4073_bc1f_ccb8f5afdf96);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub StageInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StageInfo: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageInfo(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthenticationStageInfo {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56fec28b_e8aa_4c0f_8e4c_a559e73add88);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Stage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationStage) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stage: usize,
    #[cfg(feature = "deprecated")]
    pub Scenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationScenario) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Scenario: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthenticationStatics {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f582656_28f8_4e0f_ae8c_5898b9ae2469);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ShowNotificationMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: SecondaryAuthenticationFactorAuthenticationMessage, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ShowNotificationMessageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub StartAuthenticationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serviceauthenticationnonce: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    StartAuthenticationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AuthenticationStageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AuthenticationStageChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAuthenticationStageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAuthenticationStageChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetAuthenticationStageInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetAuthenticationStageInfoAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics {
    type Vtable = ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90499a19_7ef2_4523_951c_a417a24acf93);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RegisterDevicePresenceMonitoringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceinstancepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RegisterDevicePresenceMonitoringAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub RegisterDevicePresenceMonitoringWithNewDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceinstancepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, devicefriendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicemodelnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    RegisterDevicePresenceMonitoringWithNewDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub UnregisterDevicePresenceMonitoringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    UnregisterDevicePresenceMonitoringAsync: usize,
    #[cfg(feature = "deprecated")]
    pub IsDevicePresenceMonitoringSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsDevicePresenceMonitoringSupported: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorInfo(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorInfo {
    type Vtable = ISecondaryAuthenticationFactorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e2ba861_8533_4fce_839b_ecb72410ac14);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceFriendlyName: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceModelNumber: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub DeviceConfigurationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    DeviceConfigurationData: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorInfo2(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorInfo2 {
    type Vtable = ISecondaryAuthenticationFactorInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14d981a3_fc26_4ff7_abc3_48e82a512a0a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub PresenceMonitoringMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PresenceMonitoringMode: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub UpdateDevicePresenceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presencestate: SecondaryAuthenticationFactorDevicePresence, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    UpdateDevicePresenceAsync: usize,
    #[cfg(feature = "deprecated")]
    pub IsAuthenticationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsAuthenticationSupported: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistration(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorRegistration {
    type Vtable = ISecondaryAuthenticationFactorRegistration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f4cbbb4_8cba_48b0_840d_dbb22a54c678);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub FinishRegisteringDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    FinishRegisteringDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AbortRegisteringDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorlogmessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AbortRegisteringDeviceAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistrationResult(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorRegistrationResult {
    type Vtable = ISecondaryAuthenticationFactorRegistrationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4fe35f0_ade3_4981_af6b_ec195921682a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistrationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorRegistrationStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub Registration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Registration: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistrationStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorRegistrationStatics {
    type Vtable = ISecondaryAuthenticationFactorRegistrationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1adf0f65_e3b7_4155_997f_b756ef65beba);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistrationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub RequestStartRegisteringDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, devicefriendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicemodelnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicekey: ::windows::core::RawPtr, mutualauthenticationkey: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    RequestStartRegisteringDeviceAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllRegisteredDeviceInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytype: SecondaryAuthenticationFactorDeviceFindScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllRegisteredDeviceInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub UnregisterDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    UnregisterDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub UpdateDeviceConfigurationDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    UpdateDeviceConfigurationDataAsync: usize,
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthentication(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthentication {
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn ServiceAuthenticationHmac(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceAuthenticationHmac)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SessionNonce(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionNonce)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn DeviceNonce(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceNonce)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn DeviceConfigurationData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceConfigurationData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn FinishAuthenticationAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(&self, devicehmac: Param0, sessionhmac: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FinishAuthenticationAsync)(::core::mem::transmute_copy(this), devicehmac.into_param().abi(), sessionhmac.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AbortAuthenticationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, errorlogmessage: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AbortAuthenticationAsync)(::core::mem::transmute_copy(this), errorlogmessage.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ShowNotificationMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(devicename: Param0, message: SecondaryAuthenticationFactorAuthenticationMessage) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowNotificationMessageAsync)(::core::mem::transmute_copy(this), devicename.into_param().abi(), message, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn StartAuthenticationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(deviceid: Param0, serviceauthenticationnonce: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartAuthenticationAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), serviceauthenticationnonce.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AuthenticationStageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationStageChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveAuthenticationStageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveAuthenticationStageChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetAuthenticationStageInfoAsync() -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAuthenticationStageInfoAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISecondaryAuthenticationFactorAuthenticationStatics<R, F: FnOnce(&ISecondaryAuthenticationFactorAuthenticationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecondaryAuthenticationFactorAuthentication, ISecondaryAuthenticationFactorAuthenticationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthentication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthentication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthentication {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorAuthentication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorAuthentication").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthentication {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication;{020a16e5-6a25-40a3-8c00-50a023f619d1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorAuthentication {
    type Vtable = ISecondaryAuthenticationFactorAuthentication_Vtbl;
    const IID: ::windows::core::GUID = <ISecondaryAuthenticationFactorAuthentication as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorAuthentication {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorAuthentication> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthentication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthentication> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthentication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorAuthentication> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthentication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthentication> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthentication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthentication {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthentication {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondaryAuthenticationFactorAuthenticationMessage(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationMessage {
    pub const Invalid: Self = Self(0i32);
    pub const SwipeUpWelcome: Self = Self(1i32);
    pub const TapWelcome: Self = Self(2i32);
    pub const DeviceNeedsAttention: Self = Self(3i32);
    pub const LookingForDevice: Self = Self(4i32);
    pub const LookingForDevicePluggedin: Self = Self(5i32);
    pub const BluetoothIsDisabled: Self = Self(6i32);
    pub const NfcIsDisabled: Self = Self(7i32);
    pub const WiFiIsDisabled: Self = Self(8i32);
    pub const ExtraTapIsRequired: Self = Self(9i32);
    pub const DisabledByPolicy: Self = Self(10i32);
    pub const TapOnDeviceRequired: Self = Self(11i32);
    pub const HoldFinger: Self = Self(12i32);
    pub const ScanFinger: Self = Self(13i32);
    pub const UnauthorizedUser: Self = Self(14i32);
    pub const ReregisterRequired: Self = Self(15i32);
    pub const TryAgain: Self = Self(16i32);
    pub const SayPassphrase: Self = Self(17i32);
    pub const ReadyToSignIn: Self = Self(18i32);
    pub const UseAnotherSignInOption: Self = Self(19i32);
    pub const ConnectionRequired: Self = Self(20i32);
    pub const TimeLimitExceeded: Self = Self(21i32);
    pub const CanceledByUser: Self = Self(22i32);
    pub const CenterHand: Self = Self(23i32);
    pub const MoveHandCloser: Self = Self(24i32);
    pub const MoveHandFarther: Self = Self(25i32);
    pub const PlaceHandAbove: Self = Self(26i32);
    pub const RecognitionFailed: Self = Self(27i32);
    pub const DeviceUnavailable: Self = Self(28i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationMessage {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SecondaryAuthenticationFactorAuthenticationMessage {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorAuthenticationMessage {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorAuthenticationMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorAuthenticationMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationMessage;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationResult(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationResult {
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorAuthenticationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Authentication(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthentication> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Authentication)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthentication>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationResult {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorAuthenticationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorAuthenticationResult").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult;{9cbb5987-ef6d-4bc2-bf49-4617515a0f9a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorAuthenticationResult {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationResult_Vtbl;
    const IID: ::windows::core::GUID = <ISecondaryAuthenticationFactorAuthenticationResult as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationResult> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationResult> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationResult> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationResult> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthenticationResult {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthenticationResult {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondaryAuthenticationFactorAuthenticationScenario(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationScenario {
    pub const SignIn: Self = Self(0i32);
    pub const CredentialPrompt: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationScenario {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationScenario {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SecondaryAuthenticationFactorAuthenticationScenario {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorAuthenticationScenario {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorAuthenticationScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorAuthenticationScenario").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationScenario {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationScenario;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondaryAuthenticationFactorAuthenticationStage(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationStage {
    pub const NotStarted: Self = Self(0i32);
    pub const WaitingForUserConfirmation: Self = Self(1i32);
    pub const CollectingCredential: Self = Self(2i32);
    pub const SuspendingAuthentication: Self = Self(3i32);
    pub const CredentialCollected: Self = Self(4i32);
    pub const CredentialAuthenticated: Self = Self(5i32);
    pub const StoppingAuthentication: Self = Self(6i32);
    pub const ReadyForLock: Self = Self(7i32);
    pub const CheckingDevicePresence: Self = Self(8i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationStage {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationStage {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SecondaryAuthenticationFactorAuthenticationStage {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorAuthenticationStage {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorAuthenticationStage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorAuthenticationStage").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationStage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStage;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn StageInfo(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStageInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StageInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationStageInfo>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs;{d4a5ee56-7291-4073-bc1f-ccb8f5afdf96})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStageInfo(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationStageInfo {
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Stage(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStage> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorAuthenticationStage = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationStage>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Scenario(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationScenario> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorAuthenticationScenario = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Scenario)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationScenario>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationStageInfo {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorAuthenticationStageInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationStageInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo;{56fec28b-e8aa-4c0f-8e4c-a559e73add88})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorAuthenticationStageInfo {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageInfo_Vtbl;
    const IID: ::windows::core::GUID = <ISecondaryAuthenticationFactorAuthenticationStageInfo as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorAuthenticationStageInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthenticationStageInfo {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthenticationStageInfo {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondaryAuthenticationFactorAuthenticationStatus(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationStatus {
    pub const Failed: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const UnknownDevice: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const InvalidAuthenticationStage: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SecondaryAuthenticationFactorAuthenticationStatus {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorAuthenticationStatus {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorAuthenticationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorAuthenticationStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondaryAuthenticationFactorDeviceCapabilities(pub u32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorDeviceCapabilities {
    pub const None: Self = Self(0u32);
    pub const SecureStorage: Self = Self(1u32);
    pub const StoreKeys: Self = Self(2u32);
    pub const ConfirmUserIntentToAuthenticate: Self = Self(4u32);
    pub const SupportSecureUserPresenceCheck: Self = Self(8u32);
    pub const TransmittedDataIsEncrypted: Self = Self(16u32);
    pub const HMacSha256: Self = Self(32u32);
    pub const CloseRangeDataTransmission: Self = Self(64u32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorDeviceCapabilities {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorDeviceCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SecondaryAuthenticationFactorDeviceCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorDeviceCapabilities {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorDeviceCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorDeviceCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SecondaryAuthenticationFactorDeviceCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SecondaryAuthenticationFactorDeviceCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SecondaryAuthenticationFactorDeviceCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SecondaryAuthenticationFactorDeviceCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SecondaryAuthenticationFactorDeviceCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorDeviceCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDeviceCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondaryAuthenticationFactorDeviceFindScope(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorDeviceFindScope {
    pub const User: Self = Self(0i32);
    pub const AllUsers: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorDeviceFindScope {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorDeviceFindScope {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SecondaryAuthenticationFactorDeviceFindScope {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorDeviceFindScope {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorDeviceFindScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorDeviceFindScope").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorDeviceFindScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDeviceFindScope;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondaryAuthenticationFactorDevicePresence(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorDevicePresence {
    pub const Absent: Self = Self(0i32);
    pub const Present: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorDevicePresence {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorDevicePresence {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SecondaryAuthenticationFactorDevicePresence {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorDevicePresence {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorDevicePresence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorDevicePresence").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorDevicePresence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringMode(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    pub const Unsupported: Self = Self(0i32);
    pub const AppManaged: Self = Self(1i32);
    pub const SystemManaged: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorDevicePresenceMonitoringMode").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    pub const Unsupported: Self = Self(0i32);
    pub const Succeeded: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondaryAuthenticationFactorFinishAuthenticationStatus(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorFinishAuthenticationStatus {
    pub const Failed: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const NonceExpired: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorFinishAuthenticationStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorFinishAuthenticationStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorFinishAuthenticationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorInfo(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorInfo {
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceFriendlyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceModelNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn DeviceConfigurationData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceConfigurationData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PresenceMonitoringMode(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorDevicePresenceMonitoringMode> {
        let this = &::windows::core::Interface::cast::<ISecondaryAuthenticationFactorInfo2>(self)?;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorDevicePresenceMonitoringMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PresenceMonitoringMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorDevicePresenceMonitoringMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn UpdateDevicePresenceAsync(&self, presencestate: SecondaryAuthenticationFactorDevicePresence) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<ISecondaryAuthenticationFactorInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateDevicePresenceAsync)(::core::mem::transmute_copy(this), presencestate, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsAuthenticationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISecondaryAuthenticationFactorInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAuthenticationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SecondaryAuthenticationFactorInfo {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo;{1e2ba861-8533-4fce-839b-ecb72410ac14})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorInfo {
    type Vtable = ISecondaryAuthenticationFactorInfo_Vtbl;
    const IID: ::windows::core::GUID = <ISecondaryAuthenticationFactorInfo as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorInfo> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorInfo> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorInfo> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorInfo> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorInfo {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorInfo {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistration(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorRegistration {
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RegisterDevicePresenceMonitoringAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0, deviceinstancepath: Param1, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterDevicePresenceMonitoringAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), deviceinstancepath.into_param().abi(), monitoringmode, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn RegisterDevicePresenceMonitoringWithNewDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(deviceid: Param0, deviceinstancepath: Param1, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, devicefriendlyname: Param3, devicemodelnumber: Param4, deviceconfigurationdata: Param5) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterDevicePresenceMonitoringWithNewDeviceAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), deviceinstancepath.into_param().abi(), monitoringmode, devicefriendlyname.into_param().abi(), devicemodelnumber.into_param().abi(), deviceconfigurationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn UnregisterDevicePresenceMonitoringAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnregisterDevicePresenceMonitoringAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsDevicePresenceMonitoringSupported() -> ::windows::core::Result<bool> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDevicePresenceMonitoringSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn FinishRegisteringDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(&self, deviceconfigurationdata: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FinishRegisteringDeviceAsync)(::core::mem::transmute_copy(this), deviceconfigurationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AbortRegisteringDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, errorlogmessage: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AbortRegisteringDeviceAsync)(::core::mem::transmute_copy(this), errorlogmessage.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn RequestStartRegisteringDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>, Param5: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(deviceid: Param0, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, devicefriendlyname: Param2, devicemodelnumber: Param3, devicekey: Param4, mutualauthenticationkey: Param5) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStartRegisteringDeviceAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), capabilities, devicefriendlyname.into_param().abi(), devicemodelnumber.into_param().abi(), devicekey.into_param().abi(), mutualauthenticationkey.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllRegisteredDeviceInfoAsync(querytype: SecondaryAuthenticationFactorDeviceFindScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<SecondaryAuthenticationFactorInfo>>> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllRegisteredDeviceInfoAsync)(::core::mem::transmute_copy(this), querytype, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<SecondaryAuthenticationFactorInfo>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn UnregisterDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnregisterDeviceAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn UpdateDeviceConfigurationDataAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(deviceid: Param0, deviceconfigurationdata: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateDeviceConfigurationDataAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), deviceconfigurationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics<R, F: FnOnce(&ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecondaryAuthenticationFactorRegistration, ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISecondaryAuthenticationFactorRegistrationStatics<R, F: FnOnce(&ISecondaryAuthenticationFactorRegistrationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecondaryAuthenticationFactorRegistration, ISecondaryAuthenticationFactorRegistrationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SecondaryAuthenticationFactorRegistration {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorRegistration").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorRegistration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration;{9f4cbbb4-8cba-48b0-840d-dbb22a54c678})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorRegistration {
    type Vtable = ISecondaryAuthenticationFactorRegistration_Vtbl;
    const IID: ::windows::core::GUID = <ISecondaryAuthenticationFactorRegistration as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorRegistration {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorRegistration> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistration> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorRegistration> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistration> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorRegistration {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorRegistration {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistrationResult(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorRegistrationResult {
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorRegistrationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorRegistrationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Registration(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorRegistration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Registration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorRegistration>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorRegistrationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorRegistrationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SecondaryAuthenticationFactorRegistrationResult {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorRegistrationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorRegistrationResult").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorRegistrationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult;{a4fe35f0-ade3-4981-af6b-ec195921682a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorRegistrationResult {
    type Vtable = ISecondaryAuthenticationFactorRegistrationResult_Vtbl;
    const IID: ::windows::core::GUID = <ISecondaryAuthenticationFactorRegistrationResult as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorRegistrationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SecondaryAuthenticationFactorRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorRegistrationResult {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorRegistrationResult {}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondaryAuthenticationFactorRegistrationStatus(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorRegistrationStatus {
    pub const Failed: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const CanceledByUser: Self = Self(2i32);
    pub const PinSetupRequired: Self = Self(3i32);
    pub const DisabledByPolicy: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorRegistrationStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SecondaryAuthenticationFactorRegistrationStatus {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorRegistrationStatus {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorRegistrationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorRegistrationStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorRegistrationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
