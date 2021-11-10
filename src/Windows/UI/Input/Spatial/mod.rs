#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialGestureRecognizer(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialGestureRecognizer {
    type Vtable = ISpatialGestureRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x71605bcc_0c35_4673_adbd_cc04caa6ef45);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGestureRecognizer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interaction: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settings: SpatialGestureSettings, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialGestureSettings) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialGestureRecognizerFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialGestureRecognizerFactory {
    type Vtable = ISpatialGestureRecognizerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x77214186_57b9_3150_8382_698b24e264d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGestureRecognizerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settings: SpatialGestureSettings, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialHoldCanceledEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialHoldCanceledEventArgs {
    type Vtable = ISpatialHoldCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5dfcb667_4caa_4093_8c35_b601a839f31b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialHoldCanceledEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialHoldCompletedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialHoldCompletedEventArgs {
    type Vtable = ISpatialHoldCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3f64470b_4cfd_43da_8dc4_e64552173971);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialHoldCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialHoldStartedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialHoldStartedEventArgs {
    type Vtable = ISpatialHoldStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8e343d79_acb6_4144_8615_2cfba8a3cb3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialHoldStartedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteraction(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteraction {
    type Vtable = ISpatialInteraction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfc967639_88e6_4646_9112_4344aaec9dfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteraction_abi(
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
pub struct ISpatialInteractionController(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionController {
    type Vtable = ISpatialInteractionController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f0e5ba3_0954_4e97_86c5_e7f30b114dfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Haptics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionController2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionController2 {
    type Vtable = ISpatialInteractionController2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x35b6d924_c7a2_49b7_b72e_5436b2fb8f9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionController2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionController3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionController3 {
    type Vtable = ISpatialInteractionController3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x628466a0_9d91_4a0b_888d_165e670a8cd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionController3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Power")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Power"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionControllerProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionControllerProperties {
    type Vtable = ISpatialInteractionControllerProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x61056fb1_7ba9_4e35_b93f_9272cba9b28b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionControllerProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionDetectedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionDetectedEventArgs {
    type Vtable = ISpatialInteractionDetectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x075878e4_5961_3b41_9dfb_cea5d89cc38a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionDetectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionDetectedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionDetectedEventArgs2 {
    type Vtable = ISpatialInteractionDetectedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7b263e93_5f13_419c_97d5_834678266aa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionDetectedEventArgs2_abi(
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
pub struct ISpatialInteractionManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionManager {
    type Vtable = ISpatialInteractionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x32a64ea8_a15a_3995_b8bd_80513cb5adef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Perception"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timestamp: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Perception")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionManagerStatics {
    type Vtable = ISpatialInteractionManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x00e31fa6_8ca2_30bf_91fe_d9cb4a008990);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManagerStatics_abi(
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
pub struct ISpatialInteractionManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionManagerStatics2 {
    type Vtable = ISpatialInteractionManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x93f16c52_b88a_5929_8d7c_48cb948b081c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: SpatialInteractionSourceKind, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSource {
    type Vtable = ISpatialInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfb5433ba_b0b3_3148_9f3b_e9f5de568f5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSource2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSource2 {
    type Vtable = ISpatialInteractionSource2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe4c5b70c_0470_4028_88c0_a0eb44d34efe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSource2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timestamp: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSource3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSource3 {
    type Vtable = ISpatialInteractionSource3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0406d9f9_9afd_44f9_85dc_700023a962e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSource3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceHandedness) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSource4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSource4 {
    type Vtable = ISpatialInteractionSource4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0073bc4d_df66_5a91_a2ba_cea3e5c58a19);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSource4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_People")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_People"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Perception_People"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Perception_People")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceEventArgs {
    type Vtable = ISpatialInteractionSourceEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x23b786cf_ec23_3979_b27c_eb0e12feb7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceEventArgs_abi(
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
pub struct ISpatialInteractionSourceEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceEventArgs2 {
    type Vtable = ISpatialInteractionSourceEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd8b4b467_e648_4d52_ab49_e0d227199f63);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionPressKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceLocation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceLocation {
    type Vtable = ISpatialInteractionSourceLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea4696c4_7e8b_30ca_bcc5_c77189cea30a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceLocation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceLocation2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceLocation2 {
    type Vtable = ISpatialInteractionSourceLocation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4c671045_3917_40fc_a9ac_31c9cf5ff91b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceLocation2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceLocation3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceLocation3 {
    type Vtable = ISpatialInteractionSourceLocation3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6702e65e_e915_4cfb_9c1b_0538efc86687);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceLocation3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourcePositionAccuracy) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceProperties {
    type Vtable = ISpatialInteractionSourceProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x05604542_3ef7_3222_9f53_63c9cb7e3bc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceState(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceState {
    type Vtable = ISpatialInteractionSourceState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd5c475ef_4b63_37ec_98b9_9fc652b9d2f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceState_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception"))] usize,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceState2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceState2 {
    type Vtable = ISpatialInteractionSourceState2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x45f6d0bd_1773_492e_9ba3_8ac1cbe77c08);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceState2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceState3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceState3 {
    type Vtable = ISpatialInteractionSourceState3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf2f00bc2_bd2b_4a01_a8fb_323e0158527c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceState3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_People")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_People"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialManipulationCanceledEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialManipulationCanceledEventArgs {
    type Vtable = ISpatialManipulationCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2d40d1cb_e7da_4220_b0bf_819301674780);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialManipulationCanceledEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialManipulationCompletedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialManipulationCompletedEventArgs {
    type Vtable = ISpatialManipulationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x05086802_f301_4343_9250_2fbaa5f87a37);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialManipulationCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialManipulationDelta(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialManipulationDelta {
    type Vtable = ISpatialManipulationDelta_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa7ec967a_d123_3a81_a15b_992923dcbe91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialManipulationDelta_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialManipulationStartedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialManipulationStartedEventArgs {
    type Vtable = ISpatialManipulationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa1d6bbce_42a5_377b_ada6_d28e3d384737);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialManipulationStartedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialManipulationUpdatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialManipulationUpdatedEventArgs {
    type Vtable = ISpatialManipulationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f230b9b_60c6_4dc6_bdc9_9f4a6f15fe49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialManipulationUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialNavigationCanceledEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialNavigationCanceledEventArgs {
    type Vtable = ISpatialNavigationCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xce503edc_e8a5_46f0_92d4_3c122b35112a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialNavigationCanceledEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialNavigationCompletedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialNavigationCompletedEventArgs {
    type Vtable = ISpatialNavigationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x012e80b7_af3b_42c2_9e41_baaa0e721f3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialNavigationCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialNavigationStartedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialNavigationStartedEventArgs {
    type Vtable = ISpatialNavigationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x754a348a_fb64_4656_8ebd_9deecaafe475);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialNavigationStartedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialNavigationUpdatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialNavigationUpdatedEventArgs {
    type Vtable = ISpatialNavigationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9b713fd7_839d_4a74_8732_45466fc044b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialNavigationUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialPointerInteractionSourcePose(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerInteractionSourcePose {
    type Vtable = ISpatialPointerInteractionSourcePose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa7104307_2c2b_4d3a_92a7_80ced7c4a0d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerInteractionSourcePose_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialPointerInteractionSourcePose2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerInteractionSourcePose2 {
    type Vtable = ISpatialPointerInteractionSourcePose2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xeccd86b8_52db_469f_9e3f_80c47f74bce9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerInteractionSourcePose2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourcePositionAccuracy) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialPointerPose(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerPose {
    type Vtable = ISpatialPointerPose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6953a42e_c17e_357d_97a1_7269d0ed2d10);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerPose_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception"))] usize,
    #[cfg(feature = "Perception_People")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_People"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialPointerPose2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerPose2 {
    type Vtable = ISpatialPointerPose2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9d202b17_954e_4e0c_96d1_b6790b6fc2fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerPose2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialPointerPose3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerPose3 {
    type Vtable = ISpatialPointerPose3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6342f3f0_ec49_5b4b_b8d1_d16cbb16be84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerPose3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_People")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_People"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialPointerPoseStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerPoseStatics {
    type Vtable = ISpatialPointerPoseStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa25591a9_aca1_3ee0_9816_785cfb2e3fb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerPoseStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Perception", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, timestamp: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Perception", feature = "Perception_Spatial")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialRecognitionEndedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialRecognitionEndedEventArgs {
    type Vtable = ISpatialRecognitionEndedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0e35f5cb_3f75_43f3_ac81_d1dc2df9b1fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialRecognitionEndedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialRecognitionStartedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialRecognitionStartedEventArgs {
    type Vtable = ISpatialRecognitionStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x24da128f_0008_4a6d_aa50_2a76f9cfb264);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialRecognitionStartedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gesture: SpatialGestureSettings, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialTappedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialTappedEventArgs {
    type Vtable = ISpatialTappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x296d83de_f444_4aa1_b2bf_9dc88d567da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialTappedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpatialInteractionSourceKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialGestureRecognizer(pub ::windows::runtime::IInspectable);
impl SpatialGestureRecognizer {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RecognitionStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionStartedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveRecognitionStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RecognitionEnded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionEndedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveRecognitionEnded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn Tapped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialTappedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveTapped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn HoldStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldStartedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveHoldStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn HoldCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveHoldCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn HoldCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCanceledEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveHoldCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn ManipulationStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationStartedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveManipulationStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn ManipulationUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveManipulationUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn ManipulationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveManipulationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn ManipulationCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCanceledEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveManipulationCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn NavigationStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationStartedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveNavigationStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn NavigationUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveNavigationUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn NavigationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveNavigationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn NavigationCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCanceledEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveNavigationCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn CaptureInteraction<'a, Param0: ::windows::runtime::IntoParam<'a, SpatialInteraction>>(&self, interaction: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::core::mem::transmute_copy(this), interaction.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn CancelPendingGestures(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn TrySetGestureSettings(&self, settings: SpatialGestureSettings) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::core::mem::transmute_copy(this), settings, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn GestureSettings(&self) -> ::windows::runtime::Result<SpatialGestureSettings> {
        let this = self;
        unsafe {
            let mut result__: SpatialGestureSettings = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialGestureSettings>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Create(settings: SpatialGestureSettings) -> ::windows::runtime::Result<SpatialGestureRecognizer> {
        Self::ISpatialGestureRecognizerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), settings, &mut result__).from_abi::<SpatialGestureRecognizer>(result__)
        })
    }
    pub fn ISpatialGestureRecognizerFactory<R, F: FnOnce(&ISpatialGestureRecognizerFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpatialGestureRecognizer, ISpatialGestureRecognizerFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialGestureRecognizer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialGestureRecognizer;{71605bcc-0c35-4673-adbd-cc04caa6ef45})");
}
unsafe impl ::windows::runtime::Interface for SpatialGestureRecognizer {
    type Vtable = ISpatialGestureRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x71605bcc_0c35_4673_adbd_cc04caa6ef45);
}
impl ::windows::runtime::RuntimeName for SpatialGestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialGestureRecognizer";
}
impl ::core::convert::From<SpatialGestureRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: SpatialGestureRecognizer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialGestureRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialGestureRecognizer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialGestureRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialGestureRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialGestureRecognizer> for ::windows::runtime::IInspectable {
    fn from(value: SpatialGestureRecognizer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialGestureRecognizer> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialGestureRecognizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialGestureRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialGestureRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialGestureRecognizer {}
unsafe impl ::core::marker::Sync for SpatialGestureRecognizer {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialGestureSettings(pub u32);
impl SpatialGestureSettings {
    pub const None: SpatialGestureSettings = SpatialGestureSettings(0u32);
    pub const Tap: SpatialGestureSettings = SpatialGestureSettings(1u32);
    pub const DoubleTap: SpatialGestureSettings = SpatialGestureSettings(2u32);
    pub const Hold: SpatialGestureSettings = SpatialGestureSettings(4u32);
    pub const ManipulationTranslate: SpatialGestureSettings = SpatialGestureSettings(8u32);
    pub const NavigationX: SpatialGestureSettings = SpatialGestureSettings(16u32);
    pub const NavigationY: SpatialGestureSettings = SpatialGestureSettings(32u32);
    pub const NavigationZ: SpatialGestureSettings = SpatialGestureSettings(64u32);
    pub const NavigationRailsX: SpatialGestureSettings = SpatialGestureSettings(128u32);
    pub const NavigationRailsY: SpatialGestureSettings = SpatialGestureSettings(256u32);
    pub const NavigationRailsZ: SpatialGestureSettings = SpatialGestureSettings(512u32);
}
impl ::core::convert::From<u32> for SpatialGestureSettings {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialGestureSettings {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialGestureSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialGestureSettings;u4)");
}
impl ::windows::runtime::DefaultType for SpatialGestureSettings {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for SpatialGestureSettings {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for SpatialGestureSettings {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for SpatialGestureSettings {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for SpatialGestureSettings {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for SpatialGestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialHoldCanceledEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialHoldCanceledEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialHoldCanceledEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs;{5dfcb667-4caa-4093-8c35-b601a839f31b})");
}
unsafe impl ::windows::runtime::Interface for SpatialHoldCanceledEventArgs {
    type Vtable = ISpatialHoldCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5dfcb667_4caa_4093_8c35_b601a839f31b);
}
impl ::windows::runtime::RuntimeName for SpatialHoldCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs";
}
impl ::core::convert::From<SpatialHoldCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialHoldCanceledEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialHoldCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialHoldCanceledEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialHoldCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialHoldCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialHoldCanceledEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialHoldCanceledEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialHoldCanceledEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialHoldCanceledEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialHoldCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialHoldCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialHoldCanceledEventArgs {}
unsafe impl ::core::marker::Sync for SpatialHoldCanceledEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialHoldCompletedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialHoldCompletedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialHoldCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs;{3f64470b-4cfd-43da-8dc4-e64552173971})");
}
unsafe impl ::windows::runtime::Interface for SpatialHoldCompletedEventArgs {
    type Vtable = ISpatialHoldCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3f64470b_4cfd_43da_8dc4_e64552173971);
}
impl ::windows::runtime::RuntimeName for SpatialHoldCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs";
}
impl ::core::convert::From<SpatialHoldCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialHoldCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialHoldCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialHoldCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialHoldCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialHoldCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialHoldCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialHoldCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialHoldCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialHoldCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialHoldCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialHoldCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialHoldCompletedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialHoldCompletedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialHoldStartedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialHoldStartedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialHoldStartedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs;{8e343d79-acb6-4144-8615-2cfba8a3cb3f})");
}
unsafe impl ::windows::runtime::Interface for SpatialHoldStartedEventArgs {
    type Vtable = ISpatialHoldStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8e343d79_acb6_4144_8615_2cfba8a3cb3f);
}
impl ::windows::runtime::RuntimeName for SpatialHoldStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs";
}
impl ::core::convert::From<SpatialHoldStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialHoldStartedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialHoldStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialHoldStartedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialHoldStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialHoldStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialHoldStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialHoldStartedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialHoldStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialHoldStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialHoldStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialHoldStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialHoldStartedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialHoldStartedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialInteraction(pub ::windows::runtime::IInspectable);
impl SpatialInteraction {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn SourceState(&self) -> ::windows::runtime::Result<SpatialInteractionSourceState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteraction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteraction;{fc967639-88e6-4646-9112-4344aaec9dfa})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteraction {
    type Vtable = ISpatialInteraction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfc967639_88e6_4646_9112_4344aaec9dfa);
}
impl ::windows::runtime::RuntimeName for SpatialInteraction {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteraction";
}
impl ::core::convert::From<SpatialInteraction> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteraction) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialInteraction> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteraction) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteraction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialInteraction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialInteraction> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteraction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialInteraction> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialInteraction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialInteraction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialInteraction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialInteraction {}
unsafe impl ::core::marker::Sync for SpatialInteraction {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialInteractionController(pub ::windows::runtime::IInspectable);
impl SpatialInteractionController {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn HasTouchpad(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn HasThumbstick(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Devices_Haptics`*"]
    pub fn SimpleHapticsController(&self) -> ::windows::runtime::Result<super::super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn VendorId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn ProductId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Version(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Storage_Streams`*"]
    pub fn TryGetRenderableModelAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionController2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[cfg(feature = "Devices_Power")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Devices_Power`*"]
    pub fn TryGetBatteryReport(&self) -> ::windows::runtime::Result<super::super::super::Devices::Power::BatteryReport> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionController3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Power::BatteryReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionController;{5f0e5ba3-0954-4e97-86c5-e7f30b114dfd})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionController {
    type Vtable = ISpatialInteractionController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f0e5ba3_0954_4e97_86c5_e7f30b114dfd);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionController {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionController";
}
impl ::core::convert::From<SpatialInteractionController> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialInteractionController> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialInteractionController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialInteractionController> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialInteractionController> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialInteractionController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialInteractionController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialInteractionController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionController {}
unsafe impl ::core::marker::Sync for SpatialInteractionController {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialInteractionControllerProperties(pub ::windows::runtime::IInspectable);
impl SpatialInteractionControllerProperties {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsTouchpadTouched(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsTouchpadPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsThumbstickPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn ThumbstickX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn ThumbstickY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn TouchpadX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn TouchpadY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionControllerProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionControllerProperties;{61056fb1-7ba9-4e35-b93f-9272cba9b28b})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionControllerProperties {
    type Vtable = ISpatialInteractionControllerProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x61056fb1_7ba9_4e35_b93f_9272cba9b28b);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionControllerProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionControllerProperties";
}
impl ::core::convert::From<SpatialInteractionControllerProperties> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionControllerProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialInteractionControllerProperties> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionControllerProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionControllerProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialInteractionControllerProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialInteractionControllerProperties> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionControllerProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialInteractionControllerProperties> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialInteractionControllerProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialInteractionControllerProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialInteractionControllerProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionControllerProperties {}
unsafe impl ::core::marker::Sync for SpatialInteractionControllerProperties {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialInteractionDetectedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialInteractionDetectedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Interaction(&self) -> ::windows::runtime::Result<SpatialInteraction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteraction>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSource(&self) -> ::windows::runtime::Result<SpatialInteractionSource> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionDetectedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSource>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionDetectedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs;{075878e4-5961-3b41-9dfb-cea5d89cc38a})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionDetectedEventArgs {
    type Vtable = ISpatialInteractionDetectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x075878e4_5961_3b41_9dfb_cea5d89cc38a);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionDetectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs";
}
impl ::core::convert::From<SpatialInteractionDetectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionDetectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialInteractionDetectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionDetectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialInteractionDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialInteractionDetectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionDetectedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialInteractionDetectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialInteractionDetectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialInteractionDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialInteractionDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionDetectedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialInteractionDetectedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialInteractionManager(pub ::windows::runtime::IInspectable);
impl SpatialInteractionManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn SourceDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveSourceDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn SourceLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveSourceLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn SourceUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveSourceUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn SourcePressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveSourcePressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn SourceReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveSourceReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn InteractionDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionDetectedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveInteractionDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Perception"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Collections`, `Perception`*"]
    pub fn GetDetectedSourcesAtTimestamp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<SpatialInteractionSourceState>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), timestamp.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<SpatialInteractionSourceState>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<SpatialInteractionManager> {
        Self::ISpatialInteractionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionManager>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsSourceKindSupported(kind: SpatialInteractionSourceKind) -> ::windows::runtime::Result<bool> {
        Self::ISpatialInteractionManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), kind, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ISpatialInteractionManagerStatics<R, F: FnOnce(&ISpatialInteractionManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpatialInteractionManager, ISpatialInteractionManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISpatialInteractionManagerStatics2<R, F: FnOnce(&ISpatialInteractionManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpatialInteractionManager, ISpatialInteractionManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionManager;{32a64ea8-a15a-3995-b8bd-80513cb5adef})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionManager {
    type Vtable = ISpatialInteractionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x32a64ea8_a15a_3995_b8bd_80513cb5adef);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionManager {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionManager";
}
impl ::core::convert::From<SpatialInteractionManager> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialInteractionManager> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialInteractionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialInteractionManager> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialInteractionManager> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialInteractionManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialInteractionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialInteractionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionManager {}
unsafe impl ::core::marker::Sync for SpatialInteractionManager {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialInteractionPressKind(pub i32);
impl SpatialInteractionPressKind {
    pub const None: SpatialInteractionPressKind = SpatialInteractionPressKind(0i32);
    pub const Select: SpatialInteractionPressKind = SpatialInteractionPressKind(1i32);
    pub const Menu: SpatialInteractionPressKind = SpatialInteractionPressKind(2i32);
    pub const Grasp: SpatialInteractionPressKind = SpatialInteractionPressKind(3i32);
    pub const Touchpad: SpatialInteractionPressKind = SpatialInteractionPressKind(4i32);
    pub const Thumbstick: SpatialInteractionPressKind = SpatialInteractionPressKind(5i32);
}
impl ::core::convert::From<i32> for SpatialInteractionPressKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialInteractionPressKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionPressKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionPressKind;i4)");
}
impl ::windows::runtime::DefaultType for SpatialInteractionPressKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialInteractionSource(pub ::windows::runtime::IInspectable);
impl SpatialInteractionSource {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsPointingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsMenuSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsGraspSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Controller(&self) -> ::windows::runtime::Result<SpatialInteractionController> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionController>(result__)
        }
    }
    #[cfg(feature = "Perception")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception`*"]
    pub fn TryGetStateAtTimestamp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows::runtime::Result<SpatialInteractionSourceState> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), timestamp.into_param().abi(), &mut result__).from_abi::<SpatialInteractionSourceState>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Handedness(&self) -> ::windows::runtime::Result<SpatialInteractionSourceHandedness> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource3>(self)?;
        unsafe {
            let mut result__: SpatialInteractionSourceHandedness = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceHandedness>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_People`*"]
    pub fn TryCreateHandMeshObserver(&self) -> ::windows::runtime::Result<super::super::super::Perception::People::HandMeshObserver> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::People::HandMeshObserver>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Perception_People"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Perception_People`*"]
    pub fn TryCreateHandMeshObserverAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Perception::People::HandMeshObserver>> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Perception::People::HandMeshObserver>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSource;{fb5433ba-b0b3-3148-9f3b-e9f5de568f5d})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionSource {
    type Vtable = ISpatialInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfb5433ba_b0b3_3148_9f3b_e9f5de568f5d);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionSource {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSource";
}
impl ::core::convert::From<SpatialInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialInteractionSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionSource {}
unsafe impl ::core::marker::Sync for SpatialInteractionSource {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialInteractionSourceEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialInteractionSourceEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn State(&self) -> ::windows::runtime::Result<SpatialInteractionSourceState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceState>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn PressKind(&self) -> ::windows::runtime::Result<SpatialInteractionPressKind> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceEventArgs2>(self)?;
        unsafe {
            let mut result__: SpatialInteractionPressKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionPressKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSourceEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs;{23b786cf-ec23-3979-b27c-eb0e12feb7c7})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionSourceEventArgs {
    type Vtable = ISpatialInteractionSourceEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x23b786cf_ec23_3979_b27c_eb0e12feb7c7);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionSourceEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs";
}
impl ::core::convert::From<SpatialInteractionSourceEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionSourceEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialInteractionSourceEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionSourceEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionSourceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialInteractionSourceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialInteractionSourceEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionSourceEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialInteractionSourceEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialInteractionSourceEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialInteractionSourceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialInteractionSourceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionSourceEventArgs {}
unsafe impl ::core::marker::Sync for SpatialInteractionSourceEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialInteractionSourceHandedness(pub i32);
impl SpatialInteractionSourceHandedness {
    pub const Unspecified: SpatialInteractionSourceHandedness = SpatialInteractionSourceHandedness(0i32);
    pub const Left: SpatialInteractionSourceHandedness = SpatialInteractionSourceHandedness(1i32);
    pub const Right: SpatialInteractionSourceHandedness = SpatialInteractionSourceHandedness(2i32);
}
impl ::core::convert::From<i32> for SpatialInteractionSourceHandedness {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialInteractionSourceHandedness {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSourceHandedness {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionSourceHandedness;i4)");
}
impl ::windows::runtime::DefaultType for SpatialInteractionSourceHandedness {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialInteractionSourceKind(pub i32);
impl SpatialInteractionSourceKind {
    pub const Other: SpatialInteractionSourceKind = SpatialInteractionSourceKind(0i32);
    pub const Hand: SpatialInteractionSourceKind = SpatialInteractionSourceKind(1i32);
    pub const Voice: SpatialInteractionSourceKind = SpatialInteractionSourceKind(2i32);
    pub const Controller: SpatialInteractionSourceKind = SpatialInteractionSourceKind(3i32);
}
impl ::core::convert::From<i32> for SpatialInteractionSourceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialInteractionSourceKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSourceKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionSourceKind;i4)");
}
impl ::windows::runtime::DefaultType for SpatialInteractionSourceKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialInteractionSourceLocation(pub ::windows::runtime::IInspectable);
impl SpatialInteractionSourceLocation {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Foundation_Numerics`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Foundation_Numerics`*"]
    pub fn Velocity(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Foundation_Numerics`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Quaternion>> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceLocation2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Quaternion>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn PositionAccuracy(&self) -> ::windows::runtime::Result<SpatialInteractionSourcePositionAccuracy> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__: SpatialInteractionSourcePositionAccuracy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourcePositionAccuracy>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Foundation_Numerics`*"]
    pub fn AngularVelocity(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn SourcePointerPose(&self) -> ::windows::runtime::Result<SpatialPointerInteractionSourcePose> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialPointerInteractionSourcePose>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSourceLocation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceLocation;{ea4696c4-7e8b-30ca-bcc5-c77189cea30a})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionSourceLocation {
    type Vtable = ISpatialInteractionSourceLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea4696c4_7e8b_30ca_bcc5_c77189cea30a);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionSourceLocation {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceLocation";
}
impl ::core::convert::From<SpatialInteractionSourceLocation> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionSourceLocation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialInteractionSourceLocation> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionSourceLocation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionSourceLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialInteractionSourceLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialInteractionSourceLocation> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionSourceLocation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialInteractionSourceLocation> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialInteractionSourceLocation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialInteractionSourceLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialInteractionSourceLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionSourceLocation {}
unsafe impl ::core::marker::Sync for SpatialInteractionSourceLocation {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialInteractionSourcePositionAccuracy(pub i32);
impl SpatialInteractionSourcePositionAccuracy {
    pub const High: SpatialInteractionSourcePositionAccuracy = SpatialInteractionSourcePositionAccuracy(0i32);
    pub const Approximate: SpatialInteractionSourcePositionAccuracy = SpatialInteractionSourcePositionAccuracy(1i32);
}
impl ::core::convert::From<i32> for SpatialInteractionSourcePositionAccuracy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialInteractionSourcePositionAccuracy {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSourcePositionAccuracy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionSourcePositionAccuracy;i4)");
}
impl ::windows::runtime::DefaultType for SpatialInteractionSourcePositionAccuracy {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialInteractionSourceProperties(pub ::windows::runtime::IInspectable);
impl SpatialInteractionSourceProperties {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn TryGetSourceLossMitigationDirection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn SourceLossRisk(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetLocation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialInteractionSourceLocation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialInteractionSourceLocation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSourceProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceProperties;{05604542-3ef7-3222-9f53-63c9cb7e3bc7})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionSourceProperties {
    type Vtable = ISpatialInteractionSourceProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x05604542_3ef7_3222_9f53_63c9cb7e3bc7);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionSourceProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceProperties";
}
impl ::core::convert::From<SpatialInteractionSourceProperties> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionSourceProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialInteractionSourceProperties> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionSourceProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionSourceProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialInteractionSourceProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialInteractionSourceProperties> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionSourceProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialInteractionSourceProperties> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialInteractionSourceProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialInteractionSourceProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialInteractionSourceProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionSourceProperties {}
unsafe impl ::core::marker::Sync for SpatialInteractionSourceProperties {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialInteractionSourceState(pub ::windows::runtime::IInspectable);
impl SpatialInteractionSourceState {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<SpatialInteractionSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSource>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<SpatialInteractionSourceProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceProperties>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Perception")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::PerceptionTimestamp>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsSelectPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsMenuPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsGrasped(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn SelectPressedValue(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn ControllerProperties(&self) -> ::windows::runtime::Result<SpatialInteractionControllerProperties> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionControllerProperties>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_People`*"]
    pub fn TryGetHandPose(&self) -> ::windows::runtime::Result<super::super::super::Perception::People::HandPose> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::People::HandPose>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSourceState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceState;{d5c475ef-4b63-37ec-98b9-9fc652b9d2f2})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionSourceState {
    type Vtable = ISpatialInteractionSourceState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd5c475ef_4b63_37ec_98b9_9fc652b9d2f2);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionSourceState {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceState";
}
impl ::core::convert::From<SpatialInteractionSourceState> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionSourceState) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialInteractionSourceState> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionSourceState) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionSourceState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialInteractionSourceState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialInteractionSourceState> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionSourceState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialInteractionSourceState> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialInteractionSourceState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialInteractionSourceState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialInteractionSourceState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionSourceState {}
unsafe impl ::core::marker::Sync for SpatialInteractionSourceState {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialManipulationCanceledEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialManipulationCanceledEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialManipulationCanceledEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs;{2d40d1cb-e7da-4220-b0bf-819301674780})");
}
unsafe impl ::windows::runtime::Interface for SpatialManipulationCanceledEventArgs {
    type Vtable = ISpatialManipulationCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2d40d1cb_e7da_4220_b0bf_819301674780);
}
impl ::windows::runtime::RuntimeName for SpatialManipulationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs";
}
impl ::core::convert::From<SpatialManipulationCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialManipulationCanceledEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialManipulationCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialManipulationCanceledEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialManipulationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialManipulationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialManipulationCanceledEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialManipulationCanceledEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialManipulationCanceledEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialManipulationCanceledEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialManipulationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialManipulationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialManipulationCanceledEventArgs {}
unsafe impl ::core::marker::Sync for SpatialManipulationCanceledEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialManipulationCompletedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialManipulationCompletedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetCumulativeDelta<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialManipulationDelta>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialManipulationCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs;{05086802-f301-4343-9250-2fbaa5f87a37})");
}
unsafe impl ::windows::runtime::Interface for SpatialManipulationCompletedEventArgs {
    type Vtable = ISpatialManipulationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x05086802_f301_4343_9250_2fbaa5f87a37);
}
impl ::windows::runtime::RuntimeName for SpatialManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs";
}
impl ::core::convert::From<SpatialManipulationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialManipulationCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialManipulationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialManipulationCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialManipulationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialManipulationCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialManipulationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialManipulationCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialManipulationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialManipulationCompletedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialManipulationDelta(pub ::windows::runtime::IInspectable);
impl SpatialManipulationDelta {
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn Translation(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialManipulationDelta {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationDelta;{a7ec967a-d123-3a81-a15b-992923dcbe91})");
}
unsafe impl ::windows::runtime::Interface for SpatialManipulationDelta {
    type Vtable = ISpatialManipulationDelta_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa7ec967a_d123_3a81_a15b_992923dcbe91);
}
impl ::windows::runtime::RuntimeName for SpatialManipulationDelta {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationDelta";
}
impl ::core::convert::From<SpatialManipulationDelta> for ::windows::runtime::IUnknown {
    fn from(value: SpatialManipulationDelta) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialManipulationDelta> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialManipulationDelta) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialManipulationDelta {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialManipulationDelta {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialManipulationDelta> for ::windows::runtime::IInspectable {
    fn from(value: SpatialManipulationDelta) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialManipulationDelta> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialManipulationDelta) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialManipulationDelta {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialManipulationDelta {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialManipulationDelta {}
unsafe impl ::core::marker::Sync for SpatialManipulationDelta {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialManipulationStartedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialManipulationStartedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialManipulationStartedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs;{a1d6bbce-42a5-377b-ada6-d28e3d384737})");
}
unsafe impl ::windows::runtime::Interface for SpatialManipulationStartedEventArgs {
    type Vtable = ISpatialManipulationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa1d6bbce_42a5_377b_ada6_d28e3d384737);
}
impl ::windows::runtime::RuntimeName for SpatialManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs";
}
impl ::core::convert::From<SpatialManipulationStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialManipulationStartedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialManipulationStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialManipulationStartedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialManipulationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialManipulationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialManipulationStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialManipulationStartedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialManipulationStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialManipulationStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialManipulationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialManipulationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialManipulationStartedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialManipulationStartedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialManipulationUpdatedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialManipulationUpdatedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetCumulativeDelta<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialManipulationDelta>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialManipulationUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs;{5f230b9b-60c6-4dc6-bdc9-9f4a6f15fe49})");
}
unsafe impl ::windows::runtime::Interface for SpatialManipulationUpdatedEventArgs {
    type Vtable = ISpatialManipulationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f230b9b_60c6_4dc6_bdc9_9f4a6f15fe49);
}
impl ::windows::runtime::RuntimeName for SpatialManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs";
}
impl ::core::convert::From<SpatialManipulationUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialManipulationUpdatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialManipulationUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialManipulationUpdatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialManipulationUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialManipulationUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialManipulationUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialManipulationUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialManipulationUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialManipulationUpdatedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialNavigationCanceledEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialNavigationCanceledEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialNavigationCanceledEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs;{ce503edc-e8a5-46f0-92d4-3c122b35112a})");
}
unsafe impl ::windows::runtime::Interface for SpatialNavigationCanceledEventArgs {
    type Vtable = ISpatialNavigationCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xce503edc_e8a5_46f0_92d4_3c122b35112a);
}
impl ::windows::runtime::RuntimeName for SpatialNavigationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs";
}
impl ::core::convert::From<SpatialNavigationCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialNavigationCanceledEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialNavigationCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialNavigationCanceledEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialNavigationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialNavigationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialNavigationCanceledEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialNavigationCanceledEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialNavigationCanceledEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialNavigationCanceledEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialNavigationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialNavigationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialNavigationCanceledEventArgs {}
unsafe impl ::core::marker::Sync for SpatialNavigationCanceledEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialNavigationCompletedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialNavigationCompletedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn NormalizedOffset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialNavigationCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs;{012e80b7-af3b-42c2-9e41-baaa0e721f3a})");
}
unsafe impl ::windows::runtime::Interface for SpatialNavigationCompletedEventArgs {
    type Vtable = ISpatialNavigationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x012e80b7_af3b_42c2_9e41_baaa0e721f3a);
}
impl ::windows::runtime::RuntimeName for SpatialNavigationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs";
}
impl ::core::convert::From<SpatialNavigationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialNavigationCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialNavigationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialNavigationCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialNavigationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialNavigationCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialNavigationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialNavigationCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialNavigationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialNavigationCompletedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialNavigationStartedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialNavigationStartedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsNavigatingX(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsNavigatingY(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsNavigatingZ(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialNavigationStartedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs;{754a348a-fb64-4656-8ebd-9deecaafe475})");
}
unsafe impl ::windows::runtime::Interface for SpatialNavigationStartedEventArgs {
    type Vtable = ISpatialNavigationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x754a348a_fb64_4656_8ebd_9deecaafe475);
}
impl ::windows::runtime::RuntimeName for SpatialNavigationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs";
}
impl ::core::convert::From<SpatialNavigationStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialNavigationStartedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialNavigationStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialNavigationStartedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialNavigationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialNavigationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialNavigationStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialNavigationStartedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialNavigationStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialNavigationStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialNavigationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialNavigationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialNavigationStartedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialNavigationStartedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialNavigationUpdatedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialNavigationUpdatedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn NormalizedOffset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialNavigationUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs;{9b713fd7-839d-4a74-8732-45466fc044b5})");
}
unsafe impl ::windows::runtime::Interface for SpatialNavigationUpdatedEventArgs {
    type Vtable = ISpatialNavigationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9b713fd7_839d_4a74_8732_45466fc044b5);
}
impl ::windows::runtime::RuntimeName for SpatialNavigationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs";
}
impl ::core::convert::From<SpatialNavigationUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialNavigationUpdatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialNavigationUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialNavigationUpdatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialNavigationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialNavigationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialNavigationUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialNavigationUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialNavigationUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialNavigationUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialNavigationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialNavigationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialNavigationUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialNavigationUpdatedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialPointerInteractionSourcePose(pub ::windows::runtime::IInspectable);
impl SpatialPointerInteractionSourcePose {
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn ForwardDirection(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn UpDirection(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = &::windows::runtime::Interface::cast::<ISpatialPointerInteractionSourcePose2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn PositionAccuracy(&self) -> ::windows::runtime::Result<SpatialInteractionSourcePositionAccuracy> {
        let this = &::windows::runtime::Interface::cast::<ISpatialPointerInteractionSourcePose2>(self)?;
        unsafe {
            let mut result__: SpatialInteractionSourcePositionAccuracy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourcePositionAccuracy>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialPointerInteractionSourcePose {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose;{a7104307-2c2b-4d3a-92a7-80ced7c4a0d0})");
}
unsafe impl ::windows::runtime::Interface for SpatialPointerInteractionSourcePose {
    type Vtable = ISpatialPointerInteractionSourcePose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa7104307_2c2b_4d3a_92a7_80ced7c4a0d0);
}
impl ::windows::runtime::RuntimeName for SpatialPointerInteractionSourcePose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose";
}
impl ::core::convert::From<SpatialPointerInteractionSourcePose> for ::windows::runtime::IUnknown {
    fn from(value: SpatialPointerInteractionSourcePose) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialPointerInteractionSourcePose> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialPointerInteractionSourcePose) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialPointerInteractionSourcePose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialPointerInteractionSourcePose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialPointerInteractionSourcePose> for ::windows::runtime::IInspectable {
    fn from(value: SpatialPointerInteractionSourcePose) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialPointerInteractionSourcePose> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialPointerInteractionSourcePose) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialPointerInteractionSourcePose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialPointerInteractionSourcePose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialPointerInteractionSourcePose {}
unsafe impl ::core::marker::Sync for SpatialPointerInteractionSourcePose {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialPointerPose(pub ::windows::runtime::IInspectable);
impl SpatialPointerPose {
    #[cfg(feature = "Perception")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::PerceptionTimestamp>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_People`*"]
    pub fn Head(&self) -> ::windows::runtime::Result<super::super::super::Perception::People::HeadPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::People::HeadPose>(result__)
        }
    }
    #[cfg(all(feature = "Perception", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception`, `Perception_Spatial`*"]
    pub fn TryGetAtTimestamp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Perception::PerceptionTimestamp>>(coordinatesystem: Param0, timestamp: Param1) -> ::windows::runtime::Result<SpatialPointerPose> {
        Self::ISpatialPointerPoseStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), timestamp.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn TryGetInteractionSourcePose<'a, Param0: ::windows::runtime::IntoParam<'a, SpatialInteractionSource>>(&self, source: Param0) -> ::windows::runtime::Result<SpatialPointerInteractionSourcePose> {
        let this = &::windows::runtime::Interface::cast::<ISpatialPointerPose2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<SpatialPointerInteractionSourcePose>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_People`*"]
    pub fn Eyes(&self) -> ::windows::runtime::Result<super::super::super::Perception::People::EyesPose> {
        let this = &::windows::runtime::Interface::cast::<ISpatialPointerPose3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::People::EyesPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsHeadCapturedBySystem(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialPointerPose3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ISpatialPointerPoseStatics<R, F: FnOnce(&ISpatialPointerPoseStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpatialPointerPose, ISpatialPointerPoseStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialPointerPose {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialPointerPose;{6953a42e-c17e-357d-97a1-7269d0ed2d10})");
}
unsafe impl ::windows::runtime::Interface for SpatialPointerPose {
    type Vtable = ISpatialPointerPose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6953a42e_c17e_357d_97a1_7269d0ed2d10);
}
impl ::windows::runtime::RuntimeName for SpatialPointerPose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialPointerPose";
}
impl ::core::convert::From<SpatialPointerPose> for ::windows::runtime::IUnknown {
    fn from(value: SpatialPointerPose) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialPointerPose> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialPointerPose) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialPointerPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialPointerPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialPointerPose> for ::windows::runtime::IInspectable {
    fn from(value: SpatialPointerPose) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialPointerPose> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialPointerPose) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialPointerPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialPointerPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialPointerPose {}
unsafe impl ::core::marker::Sync for SpatialPointerPose {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialRecognitionEndedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialRecognitionEndedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialRecognitionEndedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs;{0e35f5cb-3f75-43f3-ac81-d1dc2df9b1fb})");
}
unsafe impl ::windows::runtime::Interface for SpatialRecognitionEndedEventArgs {
    type Vtable = ISpatialRecognitionEndedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0e35f5cb_3f75_43f3_ac81_d1dc2df9b1fb);
}
impl ::windows::runtime::RuntimeName for SpatialRecognitionEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs";
}
impl ::core::convert::From<SpatialRecognitionEndedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialRecognitionEndedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialRecognitionEndedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialRecognitionEndedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialRecognitionEndedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialRecognitionEndedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialRecognitionEndedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialRecognitionEndedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialRecognitionEndedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialRecognitionEndedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialRecognitionEndedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialRecognitionEndedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialRecognitionEndedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialRecognitionEndedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialRecognitionStartedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialRecognitionStartedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsGesturePossible(&self, gesture: SpatialGestureSettings) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), gesture, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialRecognitionStartedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs;{24da128f-0008-4a6d-aa50-2a76f9cfb264})");
}
unsafe impl ::windows::runtime::Interface for SpatialRecognitionStartedEventArgs {
    type Vtable = ISpatialRecognitionStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x24da128f_0008_4a6d_aa50_2a76f9cfb264);
}
impl ::windows::runtime::RuntimeName for SpatialRecognitionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs";
}
impl ::core::convert::From<SpatialRecognitionStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialRecognitionStartedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialRecognitionStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialRecognitionStartedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialRecognitionStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialRecognitionStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialRecognitionStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialRecognitionStartedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialRecognitionStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialRecognitionStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialRecognitionStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialRecognitionStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialRecognitionStartedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialRecognitionStartedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialTappedEventArgs(pub ::windows::runtime::IInspectable);
impl SpatialTappedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn TapCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialTappedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialTappedEventArgs;{296d83de-f444-4aa1-b2bf-9dc88d567da6})");
}
unsafe impl ::windows::runtime::Interface for SpatialTappedEventArgs {
    type Vtable = ISpatialTappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x296d83de_f444_4aa1_b2bf_9dc88d567da6);
}
impl ::windows::runtime::RuntimeName for SpatialTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialTappedEventArgs";
}
impl ::core::convert::From<SpatialTappedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialTappedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialTappedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialTappedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialTappedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpatialTappedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialTappedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialTappedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialTappedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialTappedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialTappedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialTappedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialTappedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialTappedEventArgs {}
