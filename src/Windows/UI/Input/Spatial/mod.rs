#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialGestureRecognizer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialGestureRecognizer {
    type Vtable = ISpatialGestureRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1902140364, 3125, 18035, [173, 189, 204, 4, 202, 166, 239, 69]);
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
pub struct ISpatialGestureRecognizerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialGestureRecognizerFactory {
    type Vtable = ISpatialGestureRecognizerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1998668166, 22457, 12624, [131, 130, 105, 139, 36, 226, 100, 208]);
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
pub struct ISpatialHoldCanceledEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialHoldCanceledEventArgs {
    type Vtable = ISpatialHoldCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1576842855, 19626, 16531, [140, 53, 182, 1, 168, 57, 243, 27]);
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
pub struct ISpatialHoldCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialHoldCompletedEventArgs {
    type Vtable = ISpatialHoldCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1063536395, 19709, 17370, [141, 196, 230, 69, 82, 23, 57, 113]);
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
pub struct ISpatialHoldStartedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialHoldStartedEventArgs {
    type Vtable = ISpatialHoldStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2385788281, 44214, 16708, [134, 21, 44, 251, 168, 163, 203, 63]);
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
pub struct ISpatialInteraction(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteraction {
    type Vtable = ISpatialInteraction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4237719097, 35046, 17990, [145, 18, 67, 68, 170, 236, 157, 250]);
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
pub struct ISpatialInteractionController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionController {
    type Vtable = ISpatialInteractionController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1594776483, 2388, 20119, [134, 197, 231, 243, 11, 17, 77, 253]);
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
pub struct ISpatialInteractionController2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionController2 {
    type Vtable = ISpatialInteractionController2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(901175588, 51106, 18871, [183, 46, 84, 54, 178, 251, 143, 156]);
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
pub struct ISpatialInteractionController3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionController3 {
    type Vtable = ISpatialInteractionController3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1652844192, 40337, 18955, [136, 141, 22, 94, 103, 10, 140, 213]);
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
pub struct ISpatialInteractionControllerProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionControllerProperties {
    type Vtable = ISpatialInteractionControllerProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1627746225, 31657, 20021, [185, 63, 146, 114, 203, 169, 178, 139]);
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
pub struct ISpatialInteractionDetectedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionDetectedEventArgs {
    type Vtable = ISpatialInteractionDetectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(123238628, 22881, 15169, [157, 251, 206, 165, 216, 156, 195, 138]);
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
pub struct ISpatialInteractionDetectedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionDetectedEventArgs2 {
    type Vtable = ISpatialInteractionDetectedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2066103955, 24339, 16796, [151, 213, 131, 70, 120, 38, 106, 166]);
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
pub struct ISpatialInteractionManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionManager {
    type Vtable = ISpatialInteractionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(849759912, 41306, 14741, [184, 189, 128, 81, 60, 181, 173, 239]);
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
pub struct ISpatialInteractionManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionManagerStatics {
    type Vtable = ISpatialInteractionManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(14884774, 36002, 12479, [145, 254, 217, 203, 74, 0, 137, 144]);
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
pub struct ISpatialInteractionManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionManagerStatics2 {
    type Vtable = ISpatialInteractionManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2482072658, 47242, 22825, [141, 124, 72, 203, 148, 139, 8, 28]);
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
pub struct ISpatialInteractionSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSource {
    type Vtable = ISpatialInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4216599482, 45235, 12616, [159, 59, 233, 245, 222, 86, 143, 93]);
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
pub struct ISpatialInteractionSource2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSource2 {
    type Vtable = ISpatialInteractionSource2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3838162700, 1136, 16424, [136, 192, 160, 235, 68, 211, 78, 254]);
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
pub struct ISpatialInteractionSource3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSource3 {
    type Vtable = ISpatialInteractionSource3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(67557881, 39677, 17657, [133, 220, 112, 0, 35, 169, 98, 227]);
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
pub struct ISpatialInteractionSource4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSource4 {
    type Vtable = ISpatialInteractionSource4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(7584845, 57190, 23185, [162, 186, 206, 163, 229, 197, 138, 25]);
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
pub struct ISpatialInteractionSourceEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceEventArgs {
    type Vtable = ISpatialInteractionSourceEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(599230159, 60451, 14713, [178, 124, 235, 14, 18, 254, 183, 199]);
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
pub struct ISpatialInteractionSourceEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceEventArgs2 {
    type Vtable = ISpatialInteractionSourceEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3635721319, 58952, 19794, [171, 73, 224, 210, 39, 25, 159, 99]);
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
pub struct ISpatialInteractionSourceLocation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceLocation {
    type Vtable = ISpatialInteractionSourceLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3930494660, 32395, 12490, [188, 197, 199, 113, 137, 206, 163, 10]);
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
pub struct ISpatialInteractionSourceLocation2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceLocation2 {
    type Vtable = ISpatialInteractionSourceLocation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1281822789, 14615, 16636, [169, 172, 49, 201, 207, 95, 249, 27]);
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
pub struct ISpatialInteractionSourceLocation3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceLocation3 {
    type Vtable = ISpatialInteractionSourceLocation3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1728243294, 59669, 19707, [156, 27, 5, 56, 239, 200, 102, 135]);
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
pub struct ISpatialInteractionSourceProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceProperties {
    type Vtable = ISpatialInteractionSourceProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(90195266, 16119, 12834, [159, 83, 99, 201, 203, 126, 59, 199]);
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
pub struct ISpatialInteractionSourceState(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceState {
    type Vtable = ISpatialInteractionSourceState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3586422255, 19299, 14316, [152, 185, 159, 198, 82, 185, 210, 242]);
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
pub struct ISpatialInteractionSourceState2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceState2 {
    type Vtable = ISpatialInteractionSourceState2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1173803197, 6003, 18734, [155, 163, 138, 193, 203, 231, 124, 8]);
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
pub struct ISpatialInteractionSourceState3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialInteractionSourceState3 {
    type Vtable = ISpatialInteractionSourceState3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4075817922, 48427, 18945, [168, 251, 50, 62, 1, 88, 82, 124]);
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
pub struct ISpatialManipulationCanceledEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialManipulationCanceledEventArgs {
    type Vtable = ISpatialManipulationCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(759222731, 59354, 16928, [176, 191, 129, 147, 1, 103, 71, 128]);
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
pub struct ISpatialManipulationCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialManipulationCompletedEventArgs {
    type Vtable = ISpatialManipulationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(84436994, 62209, 17219, [146, 80, 47, 186, 165, 248, 122, 55]);
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
pub struct ISpatialManipulationDelta(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialManipulationDelta {
    type Vtable = ISpatialManipulationDelta_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2817300090, 53539, 14977, [161, 91, 153, 41, 35, 220, 190, 145]);
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
pub struct ISpatialManipulationStartedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialManipulationStartedEventArgs {
    type Vtable = ISpatialManipulationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2715204558, 17061, 14203, [173, 166, 210, 142, 61, 56, 71, 55]);
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
pub struct ISpatialManipulationUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialManipulationUpdatedEventArgs {
    type Vtable = ISpatialManipulationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1596132251, 24774, 19910, [189, 201, 159, 74, 111, 21, 254, 73]);
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
pub struct ISpatialNavigationCanceledEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialNavigationCanceledEventArgs {
    type Vtable = ISpatialNavigationCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3461365468, 59557, 18160, [146, 212, 60, 18, 43, 53, 17, 42]);
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
pub struct ISpatialNavigationCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialNavigationCompletedEventArgs {
    type Vtable = ISpatialNavigationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(19824823, 44859, 17090, [158, 65, 186, 170, 14, 114, 31, 58]);
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
pub struct ISpatialNavigationStartedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialNavigationStartedEventArgs {
    type Vtable = ISpatialNavigationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1967797386, 64356, 18006, [142, 189, 157, 238, 202, 175, 228, 117]);
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
pub struct ISpatialNavigationUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialNavigationUpdatedEventArgs {
    type Vtable = ISpatialNavigationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2607890391, 33693, 19060, [135, 50, 69, 70, 111, 192, 68, 181]);
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
pub struct ISpatialPointerInteractionSourcePose(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerInteractionSourcePose {
    type Vtable = ISpatialPointerInteractionSourcePose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2802860807, 11307, 19770, [146, 167, 128, 206, 215, 196, 160, 208]);
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
pub struct ISpatialPointerInteractionSourcePose2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerInteractionSourcePose2 {
    type Vtable = ISpatialPointerInteractionSourcePose2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3972892344, 21211, 18079, [158, 63, 128, 196, 127, 116, 188, 233]);
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
pub struct ISpatialPointerPose(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerPose {
    type Vtable = ISpatialPointerPose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1767089198, 49534, 13693, [151, 161, 114, 105, 208, 237, 45, 16]);
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
pub struct ISpatialPointerPose2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerPose2 {
    type Vtable = ISpatialPointerPose2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2636131095, 38222, 19980, [150, 209, 182, 121, 11, 111, 194, 253]);
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
pub struct ISpatialPointerPose3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerPose3 {
    type Vtable = ISpatialPointerPose3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1665332208, 60489, 23371, [184, 209, 209, 108, 187, 22, 190, 132]);
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
pub struct ISpatialPointerPoseStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialPointerPoseStatics {
    type Vtable = ISpatialPointerPoseStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2723516841, 44193, 16096, [152, 22, 120, 92, 251, 46, 63, 184]);
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
pub struct ISpatialRecognitionEndedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialRecognitionEndedEventArgs {
    type Vtable = ISpatialRecognitionEndedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(238417355, 16245, 17395, [172, 129, 209, 220, 45, 249, 177, 251]);
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
pub struct ISpatialRecognitionStartedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialRecognitionStartedEventArgs {
    type Vtable = ISpatialRecognitionStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(618271375, 8, 19053, [170, 80, 42, 118, 249, 207, 178, 100]);
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
pub struct ISpatialTappedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialTappedEventArgs {
    type Vtable = ISpatialTappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(695043038, 62532, 19105, [178, 191, 157, 200, 141, 86, 125, 166]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialGestureRecognizer(::windows::runtime::IInspectable);
impl SpatialGestureRecognizer {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RecognitionStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionStartedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveRecognitionStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RecognitionEnded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionEndedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveRecognitionEnded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn Tapped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialTappedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveTapped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn HoldStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldStartedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveHoldStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn HoldCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveHoldCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn HoldCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCanceledEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveHoldCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn ManipulationStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationStartedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveManipulationStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn ManipulationUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveManipulationUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn ManipulationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveManipulationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn ManipulationCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCanceledEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveManipulationCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn NavigationStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationStartedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveNavigationStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn NavigationUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveNavigationUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn NavigationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveNavigationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn NavigationCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCanceledEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveNavigationCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn CaptureInteraction<'a, Param0: ::windows::runtime::IntoParam<'a, SpatialInteraction>>(&self, interaction: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), interaction.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn CancelPendingGestures(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn TrySetGestureSettings(&self, settings: SpatialGestureSettings) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), settings, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn GestureSettings(&self) -> ::windows::runtime::Result<SpatialGestureSettings> {
        let this = self;
        unsafe {
            let mut result__: SpatialGestureSettings = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialGestureSettings>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Create(settings: SpatialGestureSettings) -> ::windows::runtime::Result<SpatialGestureRecognizer> {
        Self::ISpatialGestureRecognizerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), settings, &mut result__).from_abi::<SpatialGestureRecognizer>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1902140364, 3125, 18035, [173, 189, 204, 4, 202, 166, 239, 69]);
}
impl ::windows::runtime::RuntimeName for SpatialGestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialGestureRecognizer";
}
impl ::std::convert::From<SpatialGestureRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: SpatialGestureRecognizer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialGestureRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialGestureRecognizer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialGestureRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialGestureRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialGestureRecognizer> for ::windows::runtime::IInspectable {
    fn from(value: SpatialGestureRecognizer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialGestureRecognizer> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialGestureRecognizer {}
unsafe impl ::std::marker::Sync for SpatialGestureRecognizer {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for SpatialGestureSettings {
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
impl ::std::ops::BitOr for SpatialGestureSettings {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SpatialGestureSettings {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SpatialGestureSettings {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SpatialGestureSettings {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SpatialGestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialHoldCanceledEventArgs(::windows::runtime::IInspectable);
impl SpatialHoldCanceledEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialHoldCanceledEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs;{5dfcb667-4caa-4093-8c35-b601a839f31b})");
}
unsafe impl ::windows::runtime::Interface for SpatialHoldCanceledEventArgs {
    type Vtable = ISpatialHoldCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1576842855, 19626, 16531, [140, 53, 182, 1, 168, 57, 243, 27]);
}
impl ::windows::runtime::RuntimeName for SpatialHoldCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs";
}
impl ::std::convert::From<SpatialHoldCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialHoldCanceledEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialHoldCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialHoldCanceledEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialHoldCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialHoldCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialHoldCanceledEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialHoldCanceledEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialHoldCanceledEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialHoldCanceledEventArgs {}
unsafe impl ::std::marker::Sync for SpatialHoldCanceledEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialHoldCompletedEventArgs(::windows::runtime::IInspectable);
impl SpatialHoldCompletedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialHoldCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs;{3f64470b-4cfd-43da-8dc4-e64552173971})");
}
unsafe impl ::windows::runtime::Interface for SpatialHoldCompletedEventArgs {
    type Vtable = ISpatialHoldCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1063536395, 19709, 17370, [141, 196, 230, 69, 82, 23, 57, 113]);
}
impl ::windows::runtime::RuntimeName for SpatialHoldCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs";
}
impl ::std::convert::From<SpatialHoldCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialHoldCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialHoldCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialHoldCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialHoldCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialHoldCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialHoldCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialHoldCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialHoldCompletedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialHoldCompletedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialHoldCompletedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialHoldStartedEventArgs(::windows::runtime::IInspectable);
impl SpatialHoldStartedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialHoldStartedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs;{8e343d79-acb6-4144-8615-2cfba8a3cb3f})");
}
unsafe impl ::windows::runtime::Interface for SpatialHoldStartedEventArgs {
    type Vtable = ISpatialHoldStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2385788281, 44214, 16708, [134, 21, 44, 251, 168, 163, 203, 63]);
}
impl ::windows::runtime::RuntimeName for SpatialHoldStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs";
}
impl ::std::convert::From<SpatialHoldStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialHoldStartedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialHoldStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialHoldStartedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialHoldStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialHoldStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialHoldStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialHoldStartedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialHoldStartedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialHoldStartedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialHoldStartedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialInteraction(::windows::runtime::IInspectable);
impl SpatialInteraction {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn SourceState(&self) -> ::windows::runtime::Result<SpatialInteractionSourceState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteraction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteraction;{fc967639-88e6-4646-9112-4344aaec9dfa})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteraction {
    type Vtable = ISpatialInteraction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4237719097, 35046, 17990, [145, 18, 67, 68, 170, 236, 157, 250]);
}
impl ::windows::runtime::RuntimeName for SpatialInteraction {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteraction";
}
impl ::std::convert::From<SpatialInteraction> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteraction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialInteraction> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteraction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteraction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialInteraction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialInteraction> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteraction) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialInteraction> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialInteraction {}
unsafe impl ::std::marker::Sync for SpatialInteraction {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialInteractionController(::windows::runtime::IInspectable);
impl SpatialInteractionController {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn HasTouchpad(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn HasThumbstick(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Devices_Haptics`*"]
    pub fn SimpleHapticsController(&self) -> ::windows::runtime::Result<super::super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn VendorId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn ProductId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Version(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Storage_Streams`*"]
    pub fn TryGetRenderableModelAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionController2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[cfg(feature = "Devices_Power")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Devices_Power`*"]
    pub fn TryGetBatteryReport(&self) -> ::windows::runtime::Result<super::super::super::Devices::Power::BatteryReport> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionController3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Power::BatteryReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionController;{5f0e5ba3-0954-4e97-86c5-e7f30b114dfd})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionController {
    type Vtable = ISpatialInteractionController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1594776483, 2388, 20119, [134, 197, 231, 243, 11, 17, 77, 253]);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionController {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionController";
}
impl ::std::convert::From<SpatialInteractionController> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionController) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialInteractionController> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionController) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialInteractionController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialInteractionController> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionController) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialInteractionController> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialInteractionController {}
unsafe impl ::std::marker::Sync for SpatialInteractionController {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialInteractionControllerProperties(::windows::runtime::IInspectable);
impl SpatialInteractionControllerProperties {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsTouchpadTouched(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsTouchpadPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsThumbstickPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn ThumbstickX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn ThumbstickY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn TouchpadX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn TouchpadY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionControllerProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionControllerProperties;{61056fb1-7ba9-4e35-b93f-9272cba9b28b})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionControllerProperties {
    type Vtable = ISpatialInteractionControllerProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1627746225, 31657, 20021, [185, 63, 146, 114, 203, 169, 178, 139]);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionControllerProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionControllerProperties";
}
impl ::std::convert::From<SpatialInteractionControllerProperties> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionControllerProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialInteractionControllerProperties> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionControllerProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionControllerProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialInteractionControllerProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialInteractionControllerProperties> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionControllerProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialInteractionControllerProperties> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialInteractionControllerProperties {}
unsafe impl ::std::marker::Sync for SpatialInteractionControllerProperties {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialInteractionDetectedEventArgs(::windows::runtime::IInspectable);
impl SpatialInteractionDetectedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Interaction(&self) -> ::windows::runtime::Result<SpatialInteraction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteraction>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSource(&self) -> ::windows::runtime::Result<SpatialInteractionSource> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionDetectedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSource>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionDetectedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs;{075878e4-5961-3b41-9dfb-cea5d89cc38a})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionDetectedEventArgs {
    type Vtable = ISpatialInteractionDetectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(123238628, 22881, 15169, [157, 251, 206, 165, 216, 156, 195, 138]);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionDetectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs";
}
impl ::std::convert::From<SpatialInteractionDetectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionDetectedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialInteractionDetectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionDetectedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialInteractionDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialInteractionDetectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionDetectedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialInteractionDetectedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialInteractionDetectedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialInteractionDetectedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialInteractionManager(::windows::runtime::IInspectable);
impl SpatialInteractionManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn SourceDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveSourceDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn SourceLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveSourceLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn SourceUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveSourceUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn SourcePressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveSourcePressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn SourceReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveSourceReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn InteractionDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionDetectedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`*"]
    pub fn RemoveInteractionDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Perception"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Collections`, `Perception`*"]
    pub fn GetDetectedSourcesAtTimestamp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<SpatialInteractionSourceState>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), timestamp.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<SpatialInteractionSourceState>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<SpatialInteractionManager> {
        Self::ISpatialInteractionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionManager>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsSourceKindSupported(kind: SpatialInteractionSourceKind) -> ::windows::runtime::Result<bool> {
        Self::ISpatialInteractionManagerStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), kind, &mut result__).from_abi::<bool>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(849759912, 41306, 14741, [184, 189, 128, 81, 60, 181, 173, 239]);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionManager {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionManager";
}
impl ::std::convert::From<SpatialInteractionManager> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialInteractionManager> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialInteractionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialInteractionManager> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialInteractionManager> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialInteractionManager {}
unsafe impl ::std::marker::Sync for SpatialInteractionManager {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for SpatialInteractionPressKind {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialInteractionSource(::windows::runtime::IInspectable);
impl SpatialInteractionSource {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsPointingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsMenuSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsGraspSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Controller(&self) -> ::windows::runtime::Result<SpatialInteractionController> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionController>(result__)
        }
    }
    #[cfg(feature = "Perception")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception`*"]
    pub fn TryGetStateAtTimestamp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows::runtime::Result<SpatialInteractionSourceState> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), timestamp.into_param().abi(), &mut result__).from_abi::<SpatialInteractionSourceState>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Handedness(&self) -> ::windows::runtime::Result<SpatialInteractionSourceHandedness> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource3>(self)?;
        unsafe {
            let mut result__: SpatialInteractionSourceHandedness = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceHandedness>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_People`*"]
    pub fn TryCreateHandMeshObserver(&self) -> ::windows::runtime::Result<super::super::super::Perception::People::HandMeshObserver> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::People::HandMeshObserver>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Perception_People"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Perception_People`*"]
    pub fn TryCreateHandMeshObserverAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Perception::People::HandMeshObserver>> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSource4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Perception::People::HandMeshObserver>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSource;{fb5433ba-b0b3-3148-9f3b-e9f5de568f5d})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionSource {
    type Vtable = ISpatialInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4216599482, 45235, 12616, [159, 59, 233, 245, 222, 86, 143, 93]);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionSource {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSource";
}
impl ::std::convert::From<SpatialInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialInteractionSource> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialInteractionSource {}
unsafe impl ::std::marker::Sync for SpatialInteractionSource {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialInteractionSourceEventArgs(::windows::runtime::IInspectable);
impl SpatialInteractionSourceEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn State(&self) -> ::windows::runtime::Result<SpatialInteractionSourceState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceState>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn PressKind(&self) -> ::windows::runtime::Result<SpatialInteractionPressKind> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceEventArgs2>(self)?;
        unsafe {
            let mut result__: SpatialInteractionPressKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionPressKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSourceEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs;{23b786cf-ec23-3979-b27c-eb0e12feb7c7})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionSourceEventArgs {
    type Vtable = ISpatialInteractionSourceEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(599230159, 60451, 14713, [178, 124, 235, 14, 18, 254, 183, 199]);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionSourceEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs";
}
impl ::std::convert::From<SpatialInteractionSourceEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionSourceEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialInteractionSourceEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionSourceEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionSourceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialInteractionSourceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialInteractionSourceEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionSourceEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialInteractionSourceEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialInteractionSourceEventArgs {}
unsafe impl ::std::marker::Sync for SpatialInteractionSourceEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialInteractionSourceHandedness(pub i32);
impl SpatialInteractionSourceHandedness {
    pub const Unspecified: SpatialInteractionSourceHandedness = SpatialInteractionSourceHandedness(0i32);
    pub const Left: SpatialInteractionSourceHandedness = SpatialInteractionSourceHandedness(1i32);
    pub const Right: SpatialInteractionSourceHandedness = SpatialInteractionSourceHandedness(2i32);
}
impl ::std::convert::From<i32> for SpatialInteractionSourceHandedness {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialInteractionSourceKind(pub i32);
impl SpatialInteractionSourceKind {
    pub const Other: SpatialInteractionSourceKind = SpatialInteractionSourceKind(0i32);
    pub const Hand: SpatialInteractionSourceKind = SpatialInteractionSourceKind(1i32);
    pub const Voice: SpatialInteractionSourceKind = SpatialInteractionSourceKind(2i32);
    pub const Controller: SpatialInteractionSourceKind = SpatialInteractionSourceKind(3i32);
}
impl ::std::convert::From<i32> for SpatialInteractionSourceKind {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialInteractionSourceLocation(::windows::runtime::IInspectable);
impl SpatialInteractionSourceLocation {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Foundation_Numerics`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Foundation_Numerics`*"]
    pub fn Velocity(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Foundation_Numerics`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Quaternion>> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceLocation2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Quaternion>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn PositionAccuracy(&self) -> ::windows::runtime::Result<SpatialInteractionSourcePositionAccuracy> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__: SpatialInteractionSourcePositionAccuracy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourcePositionAccuracy>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Foundation_Numerics`*"]
    pub fn AngularVelocity(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn SourcePointerPose(&self) -> ::windows::runtime::Result<SpatialPointerInteractionSourcePose> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialPointerInteractionSourcePose>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSourceLocation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceLocation;{ea4696c4-7e8b-30ca-bcc5-c77189cea30a})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionSourceLocation {
    type Vtable = ISpatialInteractionSourceLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3930494660, 32395, 12490, [188, 197, 199, 113, 137, 206, 163, 10]);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionSourceLocation {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceLocation";
}
impl ::std::convert::From<SpatialInteractionSourceLocation> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionSourceLocation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialInteractionSourceLocation> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionSourceLocation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionSourceLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialInteractionSourceLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialInteractionSourceLocation> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionSourceLocation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialInteractionSourceLocation> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialInteractionSourceLocation {}
unsafe impl ::std::marker::Sync for SpatialInteractionSourceLocation {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialInteractionSourcePositionAccuracy(pub i32);
impl SpatialInteractionSourcePositionAccuracy {
    pub const High: SpatialInteractionSourcePositionAccuracy = SpatialInteractionSourcePositionAccuracy(0i32);
    pub const Approximate: SpatialInteractionSourcePositionAccuracy = SpatialInteractionSourcePositionAccuracy(1i32);
}
impl ::std::convert::From<i32> for SpatialInteractionSourcePositionAccuracy {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialInteractionSourceProperties(::windows::runtime::IInspectable);
impl SpatialInteractionSourceProperties {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn TryGetSourceLossMitigationDirection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn SourceLossRisk(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetLocation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialInteractionSourceLocation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialInteractionSourceLocation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSourceProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceProperties;{05604542-3ef7-3222-9f53-63c9cb7e3bc7})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionSourceProperties {
    type Vtable = ISpatialInteractionSourceProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(90195266, 16119, 12834, [159, 83, 99, 201, 203, 126, 59, 199]);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionSourceProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceProperties";
}
impl ::std::convert::From<SpatialInteractionSourceProperties> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionSourceProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialInteractionSourceProperties> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionSourceProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionSourceProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialInteractionSourceProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialInteractionSourceProperties> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionSourceProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialInteractionSourceProperties> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialInteractionSourceProperties {}
unsafe impl ::std::marker::Sync for SpatialInteractionSourceProperties {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialInteractionSourceState(::windows::runtime::IInspectable);
impl SpatialInteractionSourceState {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<SpatialInteractionSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSource>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<SpatialInteractionSourceProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceProperties>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Perception")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::PerceptionTimestamp>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsSelectPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsMenuPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsGrasped(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn SelectPressedValue(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn ControllerProperties(&self) -> ::windows::runtime::Result<SpatialInteractionControllerProperties> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionControllerProperties>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_People`*"]
    pub fn TryGetHandPose(&self) -> ::windows::runtime::Result<super::super::super::Perception::People::HandPose> {
        let this = &::windows::runtime::Interface::cast::<ISpatialInteractionSourceState3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::People::HandPose>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialInteractionSourceState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceState;{d5c475ef-4b63-37ec-98b9-9fc652b9d2f2})");
}
unsafe impl ::windows::runtime::Interface for SpatialInteractionSourceState {
    type Vtable = ISpatialInteractionSourceState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3586422255, 19299, 14316, [152, 185, 159, 198, 82, 185, 210, 242]);
}
impl ::windows::runtime::RuntimeName for SpatialInteractionSourceState {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceState";
}
impl ::std::convert::From<SpatialInteractionSourceState> for ::windows::runtime::IUnknown {
    fn from(value: SpatialInteractionSourceState) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialInteractionSourceState> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialInteractionSourceState) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialInteractionSourceState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialInteractionSourceState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialInteractionSourceState> for ::windows::runtime::IInspectable {
    fn from(value: SpatialInteractionSourceState) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialInteractionSourceState> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialInteractionSourceState {}
unsafe impl ::std::marker::Sync for SpatialInteractionSourceState {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialManipulationCanceledEventArgs(::windows::runtime::IInspectable);
impl SpatialManipulationCanceledEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialManipulationCanceledEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs;{2d40d1cb-e7da-4220-b0bf-819301674780})");
}
unsafe impl ::windows::runtime::Interface for SpatialManipulationCanceledEventArgs {
    type Vtable = ISpatialManipulationCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(759222731, 59354, 16928, [176, 191, 129, 147, 1, 103, 71, 128]);
}
impl ::windows::runtime::RuntimeName for SpatialManipulationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs";
}
impl ::std::convert::From<SpatialManipulationCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialManipulationCanceledEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialManipulationCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialManipulationCanceledEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialManipulationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialManipulationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialManipulationCanceledEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialManipulationCanceledEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialManipulationCanceledEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialManipulationCanceledEventArgs {}
unsafe impl ::std::marker::Sync for SpatialManipulationCanceledEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialManipulationCompletedEventArgs(::windows::runtime::IInspectable);
impl SpatialManipulationCompletedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetCumulativeDelta<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialManipulationDelta>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialManipulationCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs;{05086802-f301-4343-9250-2fbaa5f87a37})");
}
unsafe impl ::windows::runtime::Interface for SpatialManipulationCompletedEventArgs {
    type Vtable = ISpatialManipulationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(84436994, 62209, 17219, [146, 80, 47, 186, 165, 248, 122, 55]);
}
impl ::windows::runtime::RuntimeName for SpatialManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs";
}
impl ::std::convert::From<SpatialManipulationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialManipulationCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialManipulationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialManipulationCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialManipulationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialManipulationCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialManipulationCompletedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialManipulationCompletedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialManipulationCompletedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialManipulationDelta(::windows::runtime::IInspectable);
impl SpatialManipulationDelta {
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn Translation(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialManipulationDelta {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationDelta;{a7ec967a-d123-3a81-a15b-992923dcbe91})");
}
unsafe impl ::windows::runtime::Interface for SpatialManipulationDelta {
    type Vtable = ISpatialManipulationDelta_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2817300090, 53539, 14977, [161, 91, 153, 41, 35, 220, 190, 145]);
}
impl ::windows::runtime::RuntimeName for SpatialManipulationDelta {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationDelta";
}
impl ::std::convert::From<SpatialManipulationDelta> for ::windows::runtime::IUnknown {
    fn from(value: SpatialManipulationDelta) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialManipulationDelta> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialManipulationDelta) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialManipulationDelta {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialManipulationDelta {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialManipulationDelta> for ::windows::runtime::IInspectable {
    fn from(value: SpatialManipulationDelta) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialManipulationDelta> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialManipulationDelta {}
unsafe impl ::std::marker::Sync for SpatialManipulationDelta {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialManipulationStartedEventArgs(::windows::runtime::IInspectable);
impl SpatialManipulationStartedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialManipulationStartedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs;{a1d6bbce-42a5-377b-ada6-d28e3d384737})");
}
unsafe impl ::windows::runtime::Interface for SpatialManipulationStartedEventArgs {
    type Vtable = ISpatialManipulationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2715204558, 17061, 14203, [173, 166, 210, 142, 61, 56, 71, 55]);
}
impl ::windows::runtime::RuntimeName for SpatialManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs";
}
impl ::std::convert::From<SpatialManipulationStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialManipulationStartedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialManipulationStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialManipulationStartedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialManipulationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialManipulationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialManipulationStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialManipulationStartedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialManipulationStartedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialManipulationStartedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialManipulationStartedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialManipulationUpdatedEventArgs(::windows::runtime::IInspectable);
impl SpatialManipulationUpdatedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetCumulativeDelta<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialManipulationDelta>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialManipulationUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs;{5f230b9b-60c6-4dc6-bdc9-9f4a6f15fe49})");
}
unsafe impl ::windows::runtime::Interface for SpatialManipulationUpdatedEventArgs {
    type Vtable = ISpatialManipulationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1596132251, 24774, 19910, [189, 201, 159, 74, 111, 21, 254, 73]);
}
impl ::windows::runtime::RuntimeName for SpatialManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs";
}
impl ::std::convert::From<SpatialManipulationUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialManipulationUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialManipulationUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialManipulationUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialManipulationUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialManipulationUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialManipulationUpdatedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialManipulationUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialManipulationUpdatedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialNavigationCanceledEventArgs(::windows::runtime::IInspectable);
impl SpatialNavigationCanceledEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialNavigationCanceledEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs;{ce503edc-e8a5-46f0-92d4-3c122b35112a})");
}
unsafe impl ::windows::runtime::Interface for SpatialNavigationCanceledEventArgs {
    type Vtable = ISpatialNavigationCanceledEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3461365468, 59557, 18160, [146, 212, 60, 18, 43, 53, 17, 42]);
}
impl ::windows::runtime::RuntimeName for SpatialNavigationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs";
}
impl ::std::convert::From<SpatialNavigationCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialNavigationCanceledEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialNavigationCanceledEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialNavigationCanceledEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialNavigationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialNavigationCanceledEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialNavigationCanceledEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialNavigationCanceledEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialNavigationCanceledEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialNavigationCanceledEventArgs {}
unsafe impl ::std::marker::Sync for SpatialNavigationCanceledEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialNavigationCompletedEventArgs(::windows::runtime::IInspectable);
impl SpatialNavigationCompletedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn NormalizedOffset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialNavigationCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs;{012e80b7-af3b-42c2-9e41-baaa0e721f3a})");
}
unsafe impl ::windows::runtime::Interface for SpatialNavigationCompletedEventArgs {
    type Vtable = ISpatialNavigationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(19824823, 44859, 17090, [158, 65, 186, 170, 14, 114, 31, 58]);
}
impl ::windows::runtime::RuntimeName for SpatialNavigationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs";
}
impl ::std::convert::From<SpatialNavigationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialNavigationCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialNavigationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialNavigationCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialNavigationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialNavigationCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialNavigationCompletedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialNavigationCompletedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialNavigationCompletedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialNavigationStartedEventArgs(::windows::runtime::IInspectable);
impl SpatialNavigationStartedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsNavigatingX(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsNavigatingY(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsNavigatingZ(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialNavigationStartedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs;{754a348a-fb64-4656-8ebd-9deecaafe475})");
}
unsafe impl ::windows::runtime::Interface for SpatialNavigationStartedEventArgs {
    type Vtable = ISpatialNavigationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1967797386, 64356, 18006, [142, 189, 157, 238, 202, 175, 228, 117]);
}
impl ::windows::runtime::RuntimeName for SpatialNavigationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs";
}
impl ::std::convert::From<SpatialNavigationStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialNavigationStartedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialNavigationStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialNavigationStartedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialNavigationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialNavigationStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialNavigationStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialNavigationStartedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialNavigationStartedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialNavigationStartedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialNavigationStartedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialNavigationUpdatedEventArgs(::windows::runtime::IInspectable);
impl SpatialNavigationUpdatedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn NormalizedOffset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialNavigationUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs;{9b713fd7-839d-4a74-8732-45466fc044b5})");
}
unsafe impl ::windows::runtime::Interface for SpatialNavigationUpdatedEventArgs {
    type Vtable = ISpatialNavigationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2607890391, 33693, 19060, [135, 50, 69, 70, 111, 192, 68, 181]);
}
impl ::windows::runtime::RuntimeName for SpatialNavigationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs";
}
impl ::std::convert::From<SpatialNavigationUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialNavigationUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialNavigationUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialNavigationUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialNavigationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialNavigationUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialNavigationUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialNavigationUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialNavigationUpdatedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialNavigationUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialNavigationUpdatedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialPointerInteractionSourcePose(::windows::runtime::IInspectable);
impl SpatialPointerInteractionSourcePose {
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn ForwardDirection(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn UpDirection(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Foundation_Numerics`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = &::windows::runtime::Interface::cast::<ISpatialPointerInteractionSourcePose2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Quaternion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn PositionAccuracy(&self) -> ::windows::runtime::Result<SpatialInteractionSourcePositionAccuracy> {
        let this = &::windows::runtime::Interface::cast::<ISpatialPointerInteractionSourcePose2>(self)?;
        unsafe {
            let mut result__: SpatialInteractionSourcePositionAccuracy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourcePositionAccuracy>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialPointerInteractionSourcePose {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose;{a7104307-2c2b-4d3a-92a7-80ced7c4a0d0})");
}
unsafe impl ::windows::runtime::Interface for SpatialPointerInteractionSourcePose {
    type Vtable = ISpatialPointerInteractionSourcePose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2802860807, 11307, 19770, [146, 167, 128, 206, 215, 196, 160, 208]);
}
impl ::windows::runtime::RuntimeName for SpatialPointerInteractionSourcePose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose";
}
impl ::std::convert::From<SpatialPointerInteractionSourcePose> for ::windows::runtime::IUnknown {
    fn from(value: SpatialPointerInteractionSourcePose) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialPointerInteractionSourcePose> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialPointerInteractionSourcePose) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialPointerInteractionSourcePose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialPointerInteractionSourcePose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialPointerInteractionSourcePose> for ::windows::runtime::IInspectable {
    fn from(value: SpatialPointerInteractionSourcePose) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialPointerInteractionSourcePose> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialPointerInteractionSourcePose {}
unsafe impl ::std::marker::Sync for SpatialPointerInteractionSourcePose {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialPointerPose(::windows::runtime::IInspectable);
impl SpatialPointerPose {
    #[cfg(feature = "Perception")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::PerceptionTimestamp>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_People`*"]
    pub fn Head(&self) -> ::windows::runtime::Result<super::super::super::Perception::People::HeadPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::People::HeadPose>(result__)
        }
    }
    #[cfg(all(feature = "Perception", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception`, `Perception_Spatial`*"]
    pub fn TryGetAtTimestamp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Perception::PerceptionTimestamp>>(coordinatesystem: Param0, timestamp: Param1) -> ::windows::runtime::Result<SpatialPointerPose> {
        Self::ISpatialPointerPoseStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), timestamp.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn TryGetInteractionSourcePose<'a, Param0: ::windows::runtime::IntoParam<'a, SpatialInteractionSource>>(&self, source: Param0) -> ::windows::runtime::Result<SpatialPointerInteractionSourcePose> {
        let this = &::windows::runtime::Interface::cast::<ISpatialPointerPose2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<SpatialPointerInteractionSourcePose>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_People`*"]
    pub fn Eyes(&self) -> ::windows::runtime::Result<super::super::super::Perception::People::EyesPose> {
        let this = &::windows::runtime::Interface::cast::<ISpatialPointerPose3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Perception::People::EyesPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsHeadCapturedBySystem(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialPointerPose3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1767089198, 49534, 13693, [151, 161, 114, 105, 208, 237, 45, 16]);
}
impl ::windows::runtime::RuntimeName for SpatialPointerPose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialPointerPose";
}
impl ::std::convert::From<SpatialPointerPose> for ::windows::runtime::IUnknown {
    fn from(value: SpatialPointerPose) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialPointerPose> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialPointerPose) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialPointerPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialPointerPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialPointerPose> for ::windows::runtime::IInspectable {
    fn from(value: SpatialPointerPose) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialPointerPose> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialPointerPose {}
unsafe impl ::std::marker::Sync for SpatialPointerPose {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialRecognitionEndedEventArgs(::windows::runtime::IInspectable);
impl SpatialRecognitionEndedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialRecognitionEndedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs;{0e35f5cb-3f75-43f3-ac81-d1dc2df9b1fb})");
}
unsafe impl ::windows::runtime::Interface for SpatialRecognitionEndedEventArgs {
    type Vtable = ISpatialRecognitionEndedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(238417355, 16245, 17395, [172, 129, 209, 220, 45, 249, 177, 251]);
}
impl ::windows::runtime::RuntimeName for SpatialRecognitionEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs";
}
impl ::std::convert::From<SpatialRecognitionEndedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialRecognitionEndedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialRecognitionEndedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialRecognitionEndedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialRecognitionEndedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialRecognitionEndedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialRecognitionEndedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialRecognitionEndedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialRecognitionEndedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialRecognitionEndedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialRecognitionEndedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialRecognitionStartedEventArgs(::windows::runtime::IInspectable);
impl SpatialRecognitionStartedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn IsGesturePossible(&self, gesture: SpatialGestureSettings) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), gesture, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialRecognitionStartedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs;{24da128f-0008-4a6d-aa50-2a76f9cfb264})");
}
unsafe impl ::windows::runtime::Interface for SpatialRecognitionStartedEventArgs {
    type Vtable = ISpatialRecognitionStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(618271375, 8, 19053, [170, 80, 42, 118, 249, 207, 178, 100]);
}
impl ::windows::runtime::RuntimeName for SpatialRecognitionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs";
}
impl ::std::convert::From<SpatialRecognitionStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialRecognitionStartedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialRecognitionStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialRecognitionStartedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialRecognitionStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialRecognitionStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialRecognitionStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialRecognitionStartedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialRecognitionStartedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialRecognitionStartedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialRecognitionStartedEventArgs {}
#[doc = "*Required features: `UI_Input_Spatial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpatialTappedEventArgs(::windows::runtime::IInspectable);
impl SpatialTappedEventArgs {
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn InteractionSourceKind(&self) -> ::windows::runtime::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__: SpatialInteractionSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `UI_Input_Spatial`, `Perception_Spatial`*"]
    pub fn TryGetPointerPose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialPointerPose>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Spatial`*"]
    pub fn TapCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialTappedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialTappedEventArgs;{296d83de-f444-4aa1-b2bf-9dc88d567da6})");
}
unsafe impl ::windows::runtime::Interface for SpatialTappedEventArgs {
    type Vtable = ISpatialTappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(695043038, 62532, 19105, [178, 191, 157, 200, 141, 86, 125, 166]);
}
impl ::windows::runtime::RuntimeName for SpatialTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialTappedEventArgs";
}
impl ::std::convert::From<SpatialTappedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialTappedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialTappedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialTappedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialTappedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialTappedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpatialTappedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialTappedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialTappedEventArgs> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for SpatialTappedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialTappedEventArgs {}
