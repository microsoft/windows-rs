#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_WiFiDirect_Services")]
pub mod Services;
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectAdvertisement {
    type Vtable = IWiFiDirectAdvertisementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab511a2d_2a06_49a1_a584_61435c7905a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisement2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectAdvertisement2 {
    type Vtable = IWiFiDirectAdvertisement2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb759aa46_d816_491b_917a_b40d7dc403a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisementPublisher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectAdvertisementPublisher {
    type Vtable = IWiFiDirectAdvertisementPublisherVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35a2d1a_9b1f_45d9_925a_694d66df68ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisherVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IWiFiDirectAdvertisementPublisherStatusChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaafde53c_5481_46e6_90dd_32116518f192);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectError) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionListener(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionListener {
    type Vtable = IWiFiDirectConnectionListenerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x699c1b0d_8d13_4ee9_b9ec_9c72f8251f7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionListenerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionParameters {
    type Vtable = IWiFiDirectConnectionParametersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2e55405_5702_4b16_a02c_bbcd21ef6098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParametersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionParameters2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionParameters2 {
    type Vtable = IWiFiDirectConnectionParameters2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab3b0fbe_aa82_44b4_88c8_e3056b89801d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectPairingProcedure) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WiFiDirectPairingProcedure) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionParametersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionParametersStatics {
    type Vtable = IWiFiDirectConnectionParametersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x598af493_7642_456f_b9d8_e8a9eb1f401a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParametersStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configurationmethod: WiFiDirectConfigurationMethod, result__: *mut super::Enumeration::DevicePairingKinds) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionRequest {
    type Vtable = IWiFiDirectConnectionRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8eb99605_914f_49c3_a614_d18dc5b19b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionRequestedEventArgs {
    type Vtable = IWiFiDirectConnectionRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf99d20be_d38d_484f_8215_e7b65abf244c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectDevice {
    type Vtable = IWiFiDirectDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72deaaa8_72eb_4dae_8a28_8513355d2777);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectConnectionStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectDeviceStatics {
    type Vtable = IWiFiDirectDeviceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe86cb57c_3aac_4851_a792_482aaf931b04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectDeviceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectDeviceStatics2 {
    type Vtable = IWiFiDirectDeviceStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a953e49_b103_437e_9226_ab67971342f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WiFiDirectDeviceSelectorType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, connectionparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectInformationElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectInformationElement {
    type Vtable = IWiFiDirectInformationElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaffb72d6_76bb_497e_ac8b_dc72838bc309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectInformationElementStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectInformationElementStatics {
    type Vtable = IWiFiDirectInformationElementStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbd02f16_11a5_4e60_8caa_34772148378a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElementStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceinformation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectLegacySettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectLegacySettings {
    type Vtable = IWiFiDirectLegacySettingsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa64fdbba_f2fd_4567_a91b_f5c2f5321057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectLegacySettingsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectAdvertisement(::windows::core::IUnknown);
impl WiFiDirectAdvertisement {
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InformationElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetInformationElements<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn ListenStateDiscoverability(&self) -> ::windows::core::Result<WiFiDirectAdvertisementListenStateDiscoverability> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectAdvertisementListenStateDiscoverability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisementListenStateDiscoverability>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn SetListenStateDiscoverability(&self, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn IsAutonomousGroupOwnerEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn SetIsAutonomousGroupOwnerEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn LegacySettings(&self) -> ::windows::core::Result<WiFiDirectLegacySettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectLegacySettings>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedConfigurationMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &::windows::core::Interface::cast::<IWiFiDirectAdvertisement2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectAdvertisement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectAdvertisement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectAdvertisement {}
impl ::core::fmt::Debug for WiFiDirectAdvertisement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectAdvertisement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisement;{ab511a2d-2a06-49a1-a584-61435c7905a6})");
}
unsafe impl ::windows::core::Interface for WiFiDirectAdvertisement {
    type Vtable = IWiFiDirectAdvertisementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab511a2d_2a06_49a1_a584_61435c7905a6);
}
impl ::windows::core::RuntimeName for WiFiDirectAdvertisement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisement";
}
impl ::core::convert::From<WiFiDirectAdvertisement> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectAdvertisement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisement> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectAdvertisement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectAdvertisement> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectAdvertisement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisement> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectAdvertisement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectAdvertisement {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisement {}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementListenStateDiscoverability(pub i32);
impl WiFiDirectAdvertisementListenStateDiscoverability {
    pub const None: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
    pub const Intensive: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectAdvertisementListenStateDiscoverability {}
impl ::core::clone::Clone for WiFiDirectAdvertisementListenStateDiscoverability {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectAdvertisementListenStateDiscoverability {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiDirectAdvertisementListenStateDiscoverability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectAdvertisementListenStateDiscoverability {}
impl ::core::fmt::Debug for WiFiDirectAdvertisementListenStateDiscoverability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementListenStateDiscoverability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectAdvertisementListenStateDiscoverability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementListenStateDiscoverability;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectAdvertisementListenStateDiscoverability {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisher(::windows::core::IUnknown);
impl WiFiDirectAdvertisementPublisher {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectAdvertisementPublisher, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn Advertisement(&self) -> ::windows::core::Result<WiFiDirectAdvertisement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisement>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn Status(&self) -> ::windows::core::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectAdvertisementPublisherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisementPublisherStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for WiFiDirectAdvertisementPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectAdvertisementPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectAdvertisementPublisher {}
impl ::core::fmt::Debug for WiFiDirectAdvertisementPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementPublisher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectAdvertisementPublisher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisher;{b35a2d1a-9b1f-45d9-925a-694d66df68ef})");
}
unsafe impl ::windows::core::Interface for WiFiDirectAdvertisementPublisher {
    type Vtable = IWiFiDirectAdvertisementPublisherVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35a2d1a_9b1f_45d9_925a_694d66df68ef);
}
impl ::windows::core::RuntimeName for WiFiDirectAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisher";
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisher> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectAdvertisementPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisher> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectAdvertisementPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisher> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectAdvertisementPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisher> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectAdvertisementPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectAdvertisementPublisher {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisementPublisher {}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatus(pub i32);
impl WiFiDirectAdvertisementPublisherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiDirectAdvertisementPublisherStatus {}
impl ::core::clone::Clone for WiFiDirectAdvertisementPublisherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectAdvertisementPublisherStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiDirectAdvertisementPublisherStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectAdvertisementPublisherStatus {}
impl ::core::fmt::Debug for WiFiDirectAdvertisementPublisherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementPublisherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectAdvertisementPublisherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectAdvertisementPublisherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatusChangedEventArgs(::windows::core::IUnknown);
impl WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn Status(&self) -> ::windows::core::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectAdvertisementPublisherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisementPublisherStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn Error(&self) -> ::windows::core::Result<WiFiDirectError> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectError>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
impl ::core::fmt::Debug for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementPublisherStatusChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatusChangedEventArgs;{aafde53c-5481-46e6-90dd-32116518f192})");
}
unsafe impl ::windows::core::Interface for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IWiFiDirectAdvertisementPublisherStatusChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaafde53c_5481_46e6_90dd_32116518f192);
}
impl ::windows::core::RuntimeName for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatusChangedEventArgs";
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectConfigurationMethod(pub i32);
impl WiFiDirectConfigurationMethod {
    pub const ProvidePin: Self = Self(0i32);
    pub const DisplayPin: Self = Self(1i32);
    pub const PushButton: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectConfigurationMethod {}
impl ::core::clone::Clone for WiFiDirectConfigurationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectConfigurationMethod {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiDirectConfigurationMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectConfigurationMethod {}
impl ::core::fmt::Debug for WiFiDirectConfigurationMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConfigurationMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConfigurationMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConfigurationMethod;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectConfigurationMethod {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectConnectionListener(::windows::core::IUnknown);
impl WiFiDirectConnectionListener {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectConnectionListener, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for WiFiDirectConnectionListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectConnectionListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectConnectionListener {}
impl ::core::fmt::Debug for WiFiDirectConnectionListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionListener").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConnectionListener {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionListener;{699c1b0d-8d13-4ee9-b9ec-9c72f8251f7d})");
}
unsafe impl ::windows::core::Interface for WiFiDirectConnectionListener {
    type Vtable = IWiFiDirectConnectionListenerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x699c1b0d_8d13_4ee9_b9ec_9c72f8251f7d);
}
impl ::windows::core::RuntimeName for WiFiDirectConnectionListener {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionListener";
}
impl ::core::convert::From<WiFiDirectConnectionListener> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectConnectionListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionListener> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectConnectionListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectConnectionListener> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectConnectionListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionListener> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectConnectionListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectConnectionListener {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionListener {}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectConnectionParameters(::windows::core::IUnknown);
impl WiFiDirectConnectionParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectConnectionParameters, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn GroupOwnerIntent(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn SetGroupOwnerIntent(&self, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferenceOrderedConfigurationMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &::windows::core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn PreferredPairingProcedure(&self) -> ::windows::core::Result<WiFiDirectPairingProcedure> {
        let this = &::windows::core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__: WiFiDirectPairingProcedure = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectPairingProcedure>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn SetPreferredPairingProcedure(&self, value: WiFiDirectPairingProcedure) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Devices_Enumeration'*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn GetDevicePairingKinds(configurationmethod: WiFiDirectConfigurationMethod) -> ::windows::core::Result<super::Enumeration::DevicePairingKinds> {
        Self::IWiFiDirectConnectionParametersStatics(|this| unsafe {
            let mut result__: super::Enumeration::DevicePairingKinds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), configurationmethod, &mut result__).from_abi::<super::Enumeration::DevicePairingKinds>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectConnectionParametersStatics<R, F: FnOnce(&IWiFiDirectConnectionParametersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectConnectionParameters, IWiFiDirectConnectionParametersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WiFiDirectConnectionParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectConnectionParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectConnectionParameters {}
impl ::core::fmt::Debug for WiFiDirectConnectionParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConnectionParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionParameters;{b2e55405-5702-4b16-a02c-bbcd21ef6098})");
}
unsafe impl ::windows::core::Interface for WiFiDirectConnectionParameters {
    type Vtable = IWiFiDirectConnectionParametersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2e55405_5702_4b16_a02c_bbcd21ef6098);
}
impl ::windows::core::RuntimeName for WiFiDirectConnectionParameters {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionParameters";
}
impl ::core::convert::From<WiFiDirectConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectConnectionParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectConnectionParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectConnectionParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectConnectionParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl ::core::convert::TryFrom<WiFiDirectConnectionParameters> for super::Enumeration::IDevicePairingSettings {
    type Error = ::windows::core::Error;
    fn try_from(value: WiFiDirectConnectionParameters) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl ::core::convert::TryFrom<&WiFiDirectConnectionParameters> for super::Enumeration::IDevicePairingSettings {
    type Error = ::windows::core::Error;
    fn try_from(value: &WiFiDirectConnectionParameters) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl<'a> ::windows::core::IntoParam<'a, super::Enumeration::IDevicePairingSettings> for WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, super::Enumeration::IDevicePairingSettings> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl<'a> ::windows::core::IntoParam<'a, super::Enumeration::IDevicePairingSettings> for &WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, super::Enumeration::IDevicePairingSettings> {
        ::core::convert::TryInto::<super::Enumeration::IDevicePairingSettings>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectConnectionParameters {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionParameters {}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectConnectionRequest(::windows::core::IUnknown);
impl WiFiDirectConnectionRequest {
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Devices_Enumeration'*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectConnectionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectConnectionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectConnectionRequest {}
impl ::core::fmt::Debug for WiFiDirectConnectionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConnectionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionRequest;{8eb99605-914f-49c3-a614-d18dc5b19b43})");
}
unsafe impl ::windows::core::Interface for WiFiDirectConnectionRequest {
    type Vtable = IWiFiDirectConnectionRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8eb99605_914f_49c3_a614_d18dc5b19b43);
}
impl ::windows::core::RuntimeName for WiFiDirectConnectionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequest";
}
impl ::core::convert::From<WiFiDirectConnectionRequest> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectConnectionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequest> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectConnectionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectConnectionRequest> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectConnectionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequest> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectConnectionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<WiFiDirectConnectionRequest> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: WiFiDirectConnectionRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&WiFiDirectConnectionRequest> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &WiFiDirectConnectionRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectConnectionRequest {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionRequest {}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectConnectionRequestedEventArgs(::windows::core::IUnknown);
impl WiFiDirectConnectionRequestedEventArgs {
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn GetConnectionRequest(&self) -> ::windows::core::Result<WiFiDirectConnectionRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectConnectionRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectConnectionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectConnectionRequestedEventArgs {}
impl ::core::fmt::Debug for WiFiDirectConnectionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConnectionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionRequestedEventArgs;{f99d20be-d38d-484f-8215-e7b65abf244c})");
}
unsafe impl ::windows::core::Interface for WiFiDirectConnectionRequestedEventArgs {
    type Vtable = IWiFiDirectConnectionRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf99d20be_d38d_484f_8215_e7b65abf244c);
}
impl ::windows::core::RuntimeName for WiFiDirectConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequestedEventArgs";
}
impl ::core::convert::From<WiFiDirectConnectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectConnectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectConnectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectConnectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectConnectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectConnectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectConnectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionRequestedEventArgs {}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectConnectionStatus(pub i32);
impl WiFiDirectConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectConnectionStatus {}
impl ::core::clone::Clone for WiFiDirectConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectConnectionStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiDirectConnectionStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectConnectionStatus {}
impl ::core::fmt::Debug for WiFiDirectConnectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConnectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConnectionStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectConnectionStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectDevice(::windows::core::IUnknown);
impl WiFiDirectDevice {
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<WiFiDirectConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectConnectionStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation_Collections', 'Networking'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn GetConnectionEndpointPairs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn GetDeviceSelector2(r#type: WiFiDirectDeviceSelectorType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, WiFiDirectConnectionParameters>>(deviceid: Param0, connectionparameters: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>> {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), connectionparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectDeviceStatics<R, F: FnOnce(&IWiFiDirectDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IWiFiDirectDeviceStatics2<R, F: FnOnce(&IWiFiDirectDeviceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WiFiDirectDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectDevice {}
impl ::core::fmt::Debug for WiFiDirectDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectDevice;{72deaaa8-72eb-4dae-8a28-8513355d2777})");
}
unsafe impl ::windows::core::Interface for WiFiDirectDevice {
    type Vtable = IWiFiDirectDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72deaaa8_72eb_4dae_8a28_8513355d2777);
}
impl ::windows::core::RuntimeName for WiFiDirectDevice {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectDevice";
}
impl ::core::convert::From<WiFiDirectDevice> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectDevice> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiDirectDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectDevice> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectDevice> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiDirectDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<WiFiDirectDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: WiFiDirectDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&WiFiDirectDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &WiFiDirectDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for WiFiDirectDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &WiFiDirectDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectDevice {}
unsafe impl ::core::marker::Sync for WiFiDirectDevice {}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectDeviceSelectorType(pub i32);
impl WiFiDirectDeviceSelectorType {
    pub const DeviceInterface: Self = Self(0i32);
    pub const AssociationEndpoint: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectDeviceSelectorType {}
impl ::core::clone::Clone for WiFiDirectDeviceSelectorType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectDeviceSelectorType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiDirectDeviceSelectorType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectDeviceSelectorType {}
impl ::core::fmt::Debug for WiFiDirectDeviceSelectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectDeviceSelectorType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectDeviceSelectorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectDeviceSelectorType;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectDeviceSelectorType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectError(pub i32);
impl WiFiDirectError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectError {}
impl ::core::clone::Clone for WiFiDirectError {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectError {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiDirectError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectError {}
impl ::core::fmt::Debug for WiFiDirectError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectError;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectError {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectInformationElement(::windows::core::IUnknown);
impl WiFiDirectInformationElement {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectInformationElement, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Oui(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetOui<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn OuiType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn SetOuiType(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Foundation_Collections', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn CreateFromBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(buffer: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Devices_Enumeration', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))]
    pub fn CreateFromDeviceInformation<'a, Param0: ::windows::core::IntoParam<'a, super::Enumeration::DeviceInformation>>(deviceinformation: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceinformation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectInformationElementStatics<R, F: FnOnce(&IWiFiDirectInformationElementStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectInformationElement, IWiFiDirectInformationElementStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WiFiDirectInformationElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectInformationElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectInformationElement {}
impl ::core::fmt::Debug for WiFiDirectInformationElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectInformationElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectInformationElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectInformationElement;{affb72d6-76bb-497e-ac8b-dc72838bc309})");
}
unsafe impl ::windows::core::Interface for WiFiDirectInformationElement {
    type Vtable = IWiFiDirectInformationElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaffb72d6_76bb_497e_ac8b_dc72838bc309);
}
impl ::windows::core::RuntimeName for WiFiDirectInformationElement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectInformationElement";
}
impl ::core::convert::From<WiFiDirectInformationElement> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectInformationElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectInformationElement> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectInformationElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectInformationElement> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectInformationElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectInformationElement> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectInformationElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectInformationElement {}
unsafe impl ::core::marker::Sync for WiFiDirectInformationElement {}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectLegacySettings(::windows::core::IUnknown);
impl WiFiDirectLegacySettings {
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect'*"]
    pub fn SetSsid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn Passphrase(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_WiFiDirect', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetPassphrase<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for WiFiDirectLegacySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectLegacySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectLegacySettings {}
impl ::core::fmt::Debug for WiFiDirectLegacySettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectLegacySettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectLegacySettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectLegacySettings;{a64fdbba-f2fd-4567-a91b-f5c2f5321057})");
}
unsafe impl ::windows::core::Interface for WiFiDirectLegacySettings {
    type Vtable = IWiFiDirectLegacySettingsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa64fdbba_f2fd_4567_a91b_f5c2f5321057);
}
impl ::windows::core::RuntimeName for WiFiDirectLegacySettings {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectLegacySettings";
}
impl ::core::convert::From<WiFiDirectLegacySettings> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectLegacySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectLegacySettings> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectLegacySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectLegacySettings> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectLegacySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectLegacySettings> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectLegacySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectLegacySettings {}
unsafe impl ::core::marker::Sync for WiFiDirectLegacySettings {}
#[doc = "*Required features: 'Devices_WiFiDirect'*"]
#[repr(transparent)]
pub struct WiFiDirectPairingProcedure(pub i32);
impl WiFiDirectPairingProcedure {
    pub const GroupOwnerNegotiation: Self = Self(0i32);
    pub const Invitation: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectPairingProcedure {}
impl ::core::clone::Clone for WiFiDirectPairingProcedure {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectPairingProcedure {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiFiDirectPairingProcedure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectPairingProcedure {}
impl ::core::fmt::Debug for WiFiDirectPairingProcedure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectPairingProcedure").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectPairingProcedure {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectPairingProcedure;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectPairingProcedure {
    type DefaultType = Self;
}
